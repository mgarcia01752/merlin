#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Installed native Designer operations keep original baselines and exact saved bytes."""
import json
from pathlib import Path
import unittest

import merlin as root
import merlin.downstream as m

ROOT = Path(__file__).resolve().parents[3]
MEASUREMENTS = ROOT / 'examples/measurements/single-channel'
SAVED = ROOT / 'crates/pnm-input/tests/fixtures/designer-flat.json'


def profile():
    return m.DPD.from_file(MEASUREMENTS / 'modulation_profile.json', m.ProfileId(0))


def tilt(db):
    return json.dumps({'tilt': {'scope': {'mode': 'all'}, 'tilt_db': db}}).encode()


class DesignerAdmissionTests(unittest.TestCase):
    def test_saved_envelope_preserves_exact_bytes_mask_epochs_and_owned_result(self):
        original = SAVED.read_bytes()
        design = m.RxMerDesign.from_bytes('exact original', original)
        self.assertEqual(design.original_bytes, original)
        self.assertTrue(design.is_synthetic)
        self.assertEqual((design.position.index, design.position.count), (1, 1))
        output = design.materialize()
        self.assertEqual(output.source_kind, 'merlin_synthetic_rxmer')
        self.assertEqual(output.provenance['original_epoch'], 1700000000)
        self.assertEqual([v is not None for v in output.values], json.loads(original)['baseline']['valid'])
        with self.assertRaises(m.ApiError):
            design.check_profile(profile())
        copied_recipe = design.recipe
        copied_recipe['tilt'] = None
        self.assertIsNotNone(design.recipe['tilt'])
        del design
        self.assertEqual(output.source_kind, 'merlin_synthetic_rxmer')

    def test_typed_flat_creation_keeps_profile_identity_and_source_kinds(self):
        dpd = profile()
        design = m.RxMerDesign.flat(dpd, m.MerDb(48), m.CaptureTime(7))
        self.assertIsNone(design.check_profile(dpd))
        copied_key = dpd.key
        copied_key['channel_id'] = 99
        self.assertEqual(dpd.key['channel_id'], 34)
        output = design.materialize()
        self.assertEqual(output.grid, dpd.key['grid'])
        self.assertEqual(output.provenance['original_epoch'], 7)
        self.assertTrue(all(v.value == 48 for v in output.values if v is not None))
        self.assertEqual(output.source_kind, 'merlin_synthetic_rxmer')
        loaded = m.RxMerDesign.from_bytes('reload', design.original_bytes)
        self.assertEqual([None if v is None else v.value for v in loaded.materialize().values],
                         [None if v is None else v.value for v in output.values])
        plain = m.RxMer.from_file(MEASUREMENTS / 'rxmer.json')
        self.assertEqual(plain.source_kind, 'decoded_json_rxmer')
        channel = m.ChannelEstimate.from_file(MEASUREMENTS / 'channel_estimation.json')
        self.assertEqual(channel.source_kind, 'decoded_json_channel_response')
        self.assertIs(root.RxMerDesign, m.RxMerDesign)

    def test_replacement_uses_original_imported_baseline_without_compounding(self):
        original = json.loads((MEASUREMENTS / 'rxmer.json').read_bytes())
        original['values'] = [48] * 7480
        encoded = json.dumps(original).encode()
        design = m.RxMerDesign.imported(profile(), 'ordinary baseline', encoded, tilt(-6))
        self.assertFalse(design.is_synthetic)
        self.assertEqual(design.materialize().values[-1].value, 42)
        replaced = design.with_recipe(tilt(-3))
        self.assertEqual(replaced.materialize().values[0].value, 48)
        self.assertEqual(replaced.materialize().values[-1].value, 45)
        self.assertEqual(design.materialize().values[-1].value, 42)
        restored = design.with_recipe(b'{}')
        self.assertTrue(all(v.value == 48 for v in restored.materialize().values))
        self.assertEqual(replaced.materialize().source_kind, 'merlin_designed_rxmer')
        self.assertEqual(json.loads(replaced.original_bytes)['baseline']['original']['byte_length'], len(encoded))
        self.assertEqual(m.RxMerDesign.from_bytes('reload', replaced.original_bytes).materialize().values[-1].value, 45)

    def test_rejections_preserve_original_design_and_frozen_positions(self):
        for index, count in [(0, 1), (2, 1), (1, 0), (1, 257)]:
            with self.assertRaises(m.ApiError):
                m.RxMerDesignPosition(index, count)
        position = m.RxMerDesignPosition()
        with self.assertRaises(AttributeError):
            position.index = 0
        original = SAVED.read_bytes()
        design = m.RxMerDesign.from_bytes('original', original)
        for recipe in [b'{bad', b'{"unrecognized":1}', b' ' * 65537, tilt(61)]:
            with self.assertRaises(m.ApiError):
                design.with_recipe(recipe)
        bad = json.loads(original)
        bad['merlin_rxmer_designer'] = 2
        with self.assertRaises(m.ApiError):
            m.RxMerDesign.from_bytes('bad version', json.dumps(bad).encode())
        with self.assertRaises(m.ApiError):
            m.RxMerDesign.imported(profile(), 'nested', original)
        random = b'{"random_variation":{"standard_deviation_db":1,"seed":41}}'
        with self.assertRaises(m.ApiError):
            m.RxMerDesign.imported(profile(), 'ordinary', (MEASUREMENTS / 'rxmer.json').read_bytes(), random)
        with self.assertRaises(m.ApiError) as limit:
            m.RxMerDesign.from_bytes('too large', b' ' * 16777217)
        self.assertEqual(limit.exception.code, m.ErrorCode.ResourceLimit)
        excessive = json.loads(original)
        excessive['baseline']['key']['grid']['sample_count'] = 65536
        excessive['baseline']['valid'] = [False] * 65536
        excessive['recipe'] = {}
        excessive['capture_count'] = 65
        with self.assertRaises(m.ApiError):
            m.RxMerDesign.from_bytes('too many values', json.dumps(excessive).encode())
        self.assertEqual(design.original_bytes, original)

    def test_source_catalog_designs_preserve_raw_ids_masks_and_original_baselines(self):
        document = json.loads((MEASUREMENTS / 'modulation_profile.json').read_bytes())
        next(v for v in document['profiles'] if v['profile_id'] == 0)['profile_id'] = 255
        original = json.dumps(document).encode()
        capture = m.ProfileCapture.from_bytes('raw profile', original)
        flat = m.RxMerDesign.flat_capture(capture, 255, m.MerDb(48), m.CaptureTime(7), tilt(-6))
        old = m.RxMerDesign.flat(profile(), m.MerDb(48), m.CaptureTime(7), tilt(-6))
        self.assertEqual(flat.original_bytes, old.original_bytes)
        self.assertIsNone(flat.check_capture(capture, 255))
        baseline = json.loads((MEASUREMENTS / 'rxmer.json').read_bytes())
        baseline['values'] = [48] * 7480
        baseline = json.dumps(baseline).encode()
        imported = m.RxMerDesign.imported_capture(capture, 255, 'original', baseline, tilt(-6))
        old = m.RxMerDesign.imported(profile(), 'original', baseline, tilt(-6))
        self.assertEqual(imported.original_bytes, old.original_bytes)
        self.assertEqual(imported.materialize().values[-1].value, 42)
        with self.assertRaises(m.ApiError):
            m.ProfileId(255)
        with self.assertRaises(m.ApiError) as absent:
            flat.check_capture(capture, 254)
        self.assertEqual(absent.exception.code, m.ErrorCode.InvalidIdentifier)
        with self.assertRaises(m.ApiError):
            m.RxMerDesign.flat_capture(capture, 254, m.MerDb(48), m.CaptureTime(7))
        with self.assertRaises(m.ApiError):
            m.RxMerDesign.imported_capture(capture, 255, 'nested', flat.original_bytes)
        document['channel_id'] = 99
        other = m.ProfileCapture.from_bytes('other', json.dumps(document).encode())
        with self.assertRaises(m.ApiError) as mismatch:
            flat.check_capture(other, 255)
        self.assertEqual(mismatch.exception.code, m.ErrorCode.IncompatibleGrid)
        with self.assertRaises(m.ApiError):
            m.RxMerDesign.imported_capture(other, 255, 'original', baseline)
        del capture
        self.assertEqual(imported.materialize().values[-1].value, 42)
