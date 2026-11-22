#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Installed native actual TX descriptor inspection with original bytes and transmission coverage."""
import json
import os
from pathlib import Path
import unittest

import merlin.downstream as m
from test_full_loop import request


class FullLoopMessageTests(unittest.TestCase):
    def test_actual_packet_bytes_meanings_offsets_coverage_and_owned_queries(self):
        result = request().with_observations(m.FullLoopObservationPolicy()).run()
        report = result.plc_messages()
        self.assertEqual(report['reference_plane'], 'tx_plc_message_construction')
        self.assertIsNone(report['issue'])
        self.assertEqual(report['retained_frames'], report['processed_frames'])
        self.assertLessEqual(report['retained_frames'], 64)
        self.assertLessEqual(len(report['messages']), 32)
        profiles = {message['descriptor']['profile_id'] for message in report['messages']}
        self.assertTrue({None, 0, 255}.issubset(profiles))
        samples = result.report()['summary']['total_transmitted_samples']
        for message in report['messages']:
            wire = message['packet_bytes']
            self.assertEqual(wire[24], message['message_type'])
            self.assertEqual(wire[12:18], message['source_address'])
            for field in message['descriptor']['fields']:
                offset = field['offset']
                self.assertEqual(wire[offset], field['kind'])
                self.assertEqual(wire[offset+1], len(field['value']))
                self.assertEqual(wire[offset+2:offset+2+len(field['value'])], field['value'])
            symbol = message['last_frame_context']['channel_symbol_index']
            self.assertEqual(message['last_frame_end_sample'], (symbol+128)*8704)
            self.assertEqual(message['completely_transmitted'], samples >= message['last_frame_end_sample'])
        ocd = next(message for message in report['messages'] if message['message_type'] == 49)
        fields = {field['kind']: field['meaning'] for field in ocd['descriptor']['fields'] if field['kind'] != 5}
        self.assertEqual(fields[0], dict(kind='numerology', fft_samples=8192, spacing_hz=25000))
        self.assertEqual(fields[1], dict(kind='cyclic_prefix', samples=512))
        self.assertEqual(fields[2], dict(kind='roll_off', samples=128))
        self.assertEqual(fields[3], dict(kind='subcarrier_zero_frequency', frequency_hz=631100000))
        report['messages'][0]['packet_bytes'][0] = -1
        self.assertEqual(result.plc_messages()['messages'][0]['packet_bytes'][0], 194)

    def test_absent_and_truncated_messages_do_not_fabricate_packets_or_change_science(self):
        with self.assertRaises(m.ApiError):
            request().run().plc_messages()
        result = request().with_observations(m.FullLoopObservationPolicy.from_json(
            b'{"max_elements_per_event":16}')).run()
        report = result.plc_messages()
        self.assertEqual(report['messages'], [])
        self.assertEqual(report['processed_frames'], 0)
        self.assertEqual(report['issue']['retained_frame'], 0)
        self.assertEqual(report['issue']['error']['code'], 'MissingCoverage')
        self.assertEqual(result.report()['summary']['comparison']['bit_errors'], 0)

    @unittest.skipUnless(os.environ.get('MERLIN_FULL_LOOP_PARITY'), 'Direct Rust process reference required')
    def test_direct_rust_process_actual_packet_bytes_and_typed_views_match(self):
        expected = json.loads(Path(os.environ['MERLIN_FULL_LOOP_PARITY']).read_text())['messages']
        result = request().with_observations(m.FullLoopObservationPolicy()).run()
        self.assertEqual(result.plc_messages(), expected)
