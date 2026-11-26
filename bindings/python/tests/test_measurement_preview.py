#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Installed native measurement views preserve original scientific identity."""
import hashlib
import json
from pathlib import Path
import unittest

import merlin.downstream as m

ROOT = Path(__file__).resolve().parents[3]
FIXTURES = ROOT / 'examples/measurements/single-channel'


class MeasurementPreviewTests(unittest.TestCase):
    def test_original_rxmer_literal_frequency_units_and_owned_view(self):
        original = (FIXTURES / 'rxmer.json').read_bytes()
        capture = m.RxMer.from_bytes('original', original)
        result = capture.preview(count=1)
        self.assertEqual(result['projection'], 'rxmer_db')
        self.assertEqual(result['view']['total'], 7480)
        self.assertEqual(result['view']['axis_label'], 'RxMER (dB)')
        self.assertEqual(result['view']['points'][0]['x'], 0.5)
        self.assertEqual(result['view']['points'][0]['y'], 0.5)
        self.assertIn('640.000–640.000 MHz', result['view']['points'][0]['readout'])
        self.assertIn('42.750', result['view']['points'][0]['readout'])
        self.assertEqual(result['provenance']['original_epoch'], 1380970)
        self.assertEqual(result['provenance']['original_sha256'], hashlib.sha256(original).hexdigest())
        self.assertLessEqual(len(capture.preview()['view']['points']), 1024)
        result['view']['points'][0]['y'] = 42
        self.assertEqual(capture.preview(count=1)['view']['points'][0]['y'], 0.5)
        del capture
        self.assertEqual(result['grid']['sample_count'], 7480)

    def test_missing_values_viewport_bounds_and_atomic_rejection(self):
        record = json.loads((FIXTURES / 'rxmer.json').read_bytes())
        record['values'][0] = None
        capture = m.RxMer.from_bytes('gap', json.dumps(record).encode())
        original = capture.provenance
        first = capture.preview(count=2)['view']['points'][0]
        self.assertTrue(first['gap'])
        self.assertIsNone(first['y'])
        for start, count in [(7480, 1), (0, 7481), (2**64-1, 2)]:
            with self.assertRaises(m.ApiError):
                capture.preview(start=start, count=count)
        self.assertEqual(capture.preview(start=99, count=0)['view']['start'], 0)
        self.assertEqual(capture.provenance, original)
        self.assertIsNone(capture.values[0])

    def test_original_h_literal_projections_zero_gaps_and_invalid_metric(self):
        record = json.loads((FIXTURES / 'channel_estimation.json').read_bytes())
        record['iq'] = [[1, 0], [0, 1], [-1, 0], [0, 0], None]
        record['data_length'] = 20
        capture = m.ChannelEstimate.from_bytes('literal H', json.dumps(record).encode())
        real = capture.preview('real')
        self.assertEqual([p['y'] for p in real['view']['points']], [0.25, 0.5, 0.75, 0.5, None])
        self.assertEqual(real['projection'], 'real')
        imaginary = capture.preview('imaginary')
        self.assertIn('1.000', imaginary['view']['points'][1]['readout'])
        phase = capture.preview('phase_degrees')
        self.assertIn('90.000', phase['view']['points'][1]['readout'])
        self.assertIn('180.000', phase['view']['points'][2]['readout'])
        self.assertEqual(phase['view']['axis_label'], 'Phase (degrees)')
        gain = capture.preview()
        self.assertIn('0.000', gain['view']['points'][0]['readout'])
        self.assertTrue(gain['view']['points'][3]['gap'])
        self.assertTrue(gain['view']['points'][4]['gap'])
        before = capture.provenance
        with self.assertRaises(m.ApiError) as raised:
            capture.preview('equalizer')
        self.assertEqual(raised.exception.code, m.ErrorCode.InvalidInput)
        self.assertEqual(capture.provenance, before)

    def test_original_channel_frame_preserves_values_lineage_and_owned_copies(self):
        document = json.loads((FIXTURES / 'channel_estimation.json').read_bytes())
        document['iq'][0] = [3.0, 4.0]  # Independent complex/missing-value anchors.
        document['iq'][1] = None
        original = json.dumps(document).encode()
        capture = m.ChannelEstimate.from_bytes('original H', original)
        before = capture.provenance
        frame = capture.frame()
        self.assertEqual(frame['iq'][0], [3.0, 4.0])
        self.assertIsNone(frame['iq'][1])
        self.assertEqual(frame['key']['grid'], capture.grid)
        self.assertEqual(frame['capture_time'], before['original_epoch'])
        self.assertEqual(frame['source']['sha256'], before['decoded_sha256'])
        frame['iq'][0][0] = 99
        frame['key']['channel_id'] = 99
        self.assertEqual(capture.frame()['iq'][0], [3.0, 4.0])
        self.assertEqual(capture.frame()['key']['channel_id'], 34)
        self.assertEqual(capture.provenance, before)
        retained = capture.frame()
        del capture
        self.assertEqual(retained['iq'][0], [3.0, 4.0])
        binary = (ROOT / 'examples/workbench/single/ds-channel-estimate.bin').read_bytes()
        capture = m.ChannelEstimate.from_bytes('wrong.json', binary)
        frame = capture.frame()
        self.assertEqual(frame['capture_time'], 1391100)
        self.assertEqual(frame['key']['mac_address'], [0, 202, 0, 0, 0, 1])
        self.assertEqual(len(frame['iq']), 7480)
        self.assertEqual(frame['source']['sha256'], capture.provenance['decoded_sha256'])
        self.assertNotEqual(frame['source']['sha256'], capture.provenance['original_sha256'])
        self.assertEqual(capture.provenance['original_sha256'], hashlib.sha256(binary).hexdigest())
        self.assertEqual(capture.source_kind, 'decoded_json_channel_response')


if __name__ == '__main__':
    unittest.main()
