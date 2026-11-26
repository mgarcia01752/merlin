#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Installed public saved-project preparation preserves Rust-owned simulation settings."""
import base64
import hashlib
import json
import os
from pathlib import Path
import unittest

import merlin.downstream as m
from test_full_loop import inputs, options

ROOT = Path(__file__).resolve().parents[3]
SINGLE = ROOT / 'examples/workbench/downstream/single/workbench.merlin.json'
MULTI = ROOT / 'examples/workbench/downstream/multi-rxmer/workbench.merlin.json'
# Independent source bytes exercise both end bits and an embedded zero octet.
FILE_BYTES = bytes([0x81, 0x00, 0x5a])
# Finite file-plus-fill length, independent of codec and observation budgets.
FILE_TARGET_BITS = 64
# A fill recurrence seed distinct from the dormant continuous-source seed.
FILE_FILL_SEED = 7
# H/profile distance in this fixture; its RxMER/profile tolerance is zero seconds.
RESPONSE_EPOCH_DISTANCE_SECONDS = 17


def document(path=SINGLE):
    """Copy the bundled UI-produced project and choose only its active profile."""
    value = json.loads(path.read_bytes())
    value['receiver']['all_profiles'] = False
    return value


def prepare(value):
    """Dispatch unchanged JSON through the shared Rust saved-project validator."""
    return m.Workbench.prepare_full_loop(json.dumps(value).encode())


class WorkbenchFullLoopTests(unittest.TestCase):
    def test_installed_idle_preparation_settings_ownership_and_precancel(self):
        value = document()
        plan = prepare(value)
        self.assertIsInstance(plan, m.FullLoopSweepPlan)
        self.assertEqual(len(plan.info['selected_profile_ids']), 1)
        request = plan.request(0)
        self.assertEqual(request.options.settings['duration_ms'], 1)
        self.assertEqual(request.options.settings['noise_seed'], 41)
        self.assertEqual(request.response_tolerance.value,
                         value['sections']['measurements']['response_tolerance'])
        copied = request.settings
        copied['prbs_seed'] = 1
        self.assertEqual(request.settings['prbs_seed'], value['sections']['source']['options']['seed'])
        control = m.FullLoopControl()
        control.cancel()
        output = request.run(control)
        self.assertEqual(output.report()['summary']['comparison']['bits_tested'], 0)
        summary = output.input_summary()
        self.assertEqual(summary['profile_sources'][0]['original_sha256'],
                         value['sections']['profile']['file']['sha256'])

    def test_unnamed_saved_file_keeps_original_label_recovery_and_capture_names(self):
        value = document()
        value['sections']['source']['file'] = {
            'name': '', 'byte_length': len(FILE_BYTES),
            'sha256': hashlib.sha256(FILE_BYTES).hexdigest(),
            'base64': base64.b64encode(FILE_BYTES).decode()}
        value['sections']['source']['options'].update(
            mode='file', append=True, target_bits=FILE_TARGET_BITS,
            fill_seed=FILE_FILL_SEED, seed=0)
        request = prepare(value).request(0)
        self.assertEqual(request.settings['source']['payload']['target_bits'], FILE_TARGET_BITS)
        result = request.run()
        self.assertEqual(result.input_summary()['source_kind'], 'file')
        self.assertEqual(result.input_summary()['source_sha256'],
                         value['sections']['source']['file']['sha256'])
        self.assertEqual(result.recovered_bytes(), FILE_BYTES)
        invalid = json.loads(json.dumps(value))
        invalid['sections']['profile']['file']['name'] = ''
        with self.assertRaises(m.ApiError):
            prepare(invalid)
        invalid = json.loads(json.dumps(value))
        invalid['sections']['source']['file']['sha256'] = 'invalid'
        with self.assertRaises(m.ApiError):
            prepare(invalid)

    def test_saved_file_fill_and_timeline_settings_and_legacy_behavior(self):
        value = document()
        value['sections']['source']['file'] = {
            'name': 'opaque.bin', 'byte_length': len(FILE_BYTES),
            'sha256': hashlib.sha256(FILE_BYTES).hexdigest(),
            'base64': base64.b64encode(FILE_BYTES).decode()}
        value['sections']['source']['options'].update(
            mode='file', append=True, target_bits=FILE_TARGET_BITS,
            fill_seed=FILE_FILL_SEED, seed=0)
        request = prepare(value).request(0)
        source = request.settings['source']
        self.assertEqual(source['kind'], 'file')
        self.assertEqual(source['payload']['target_bits'], FILE_TARGET_BITS)
        self.assertEqual(source['payload']['fill']['seed'], FILE_FILL_SEED)
        self.assertEqual(request.run().recovered_bytes(), FILE_BYTES)
        with self.assertRaises(m.ApiError) as caught:
            m.Workbench.from_bytes(json.dumps(value).encode())
        self.assertEqual(caught.exception.code, m.ErrorCode.UnsupportedCapability)
        for mode in ['hold', 'interpolate']:
            value = document(MULTI)
            originals = value['sections']['measurements']['rxmer']
            selected = [originals[-1]['sha256'], originals[0]['sha256']]
            value['sections']['measurements']['options'].update(
                mode=mode, interval_symbols='1', selected_captures=selected)
            playback = prepare(value).request(0).settings['playback']
            self.assertEqual(playback['mode'], mode)
            self.assertEqual(playback['interval_symbols'], 1)
            self.assertEqual(len(playback['frames']), 2)
            expected = [m.RxMer.from_bytes('original', base64.b64decode(item['base64']))
                        for item in [originals[0], originals[-1]]]
            self.assertEqual([frame['capture_time'] for frame in playback['frames']],
                             [capture.provenance['original_epoch'] for capture in expected])

    def test_response_tolerance_is_independent_and_original_constructor_is_compatible(self):
        accepted = inputs()
        response = json.loads((ROOT / 'examples/measurements/single-channel/channel_estimation.json').read_bytes())
        # The source fixture epoch is a literal independent anchor used by test_full_loop.
        response['pnm_header']['capture_time'] = 1466967 + RESPONSE_EPOCH_DISTANCE_SECONDS
        response = m.ChannelEstimate.from_bytes('independent H', json.dumps(response).encode())
        with self.assertRaises(m.ApiError) as caught:
            m.Simulation.prepare_full_loop(*accepted, response=response, options=options())
        self.assertEqual(caught.exception.code, m.ErrorCode.AssociationFailed)
        request = m.Simulation.prepare_full_loop_with_response_tolerance(
            *accepted, m.AssociationTolerance(RESPONSE_EPOCH_DISTANCE_SECONDS),
            response=response, options=options())
        self.assertEqual(request.response_tolerance.value, RESPONSE_EPOCH_DISTANCE_SECONDS)
        self.assertEqual(request.settings['playback']['tolerance_seconds'], 0)
        with self.assertRaises(m.ApiError) as caught:
            m.Simulation.prepare_full_loop_with_response_tolerance(
                *accepted, m.AssociationTolerance(RESPONSE_EPOCH_DISTANCE_SECONDS - 1),
                response=response, options=options())
        self.assertEqual(caught.exception.code, m.ErrorCode.AssociationFailed)

    def test_invalid_project_rejection_preserves_accepted_plan(self):
        plan = prepare(document())
        before = plan.request(0).settings
        value = document(MULTI)
        value['sections']['measurements']['options']['selected_captures'] = [
            value['sections']['measurements']['rxmer'][0]['sha256']]
        value['sections']['measurements']['rxmer'][1]['sha256'] = 'bad'
        with self.assertRaises(m.ApiError) as caught:
            prepare(value)
        self.assertEqual(caught.exception.code, m.ErrorCode.InvalidInput)
        value = document()
        value['schema_version'] = 99
        with self.assertRaises(m.ApiError) as caught:
            prepare(value)
        self.assertEqual(caught.exception.code, m.ErrorCode.UnsupportedCapability)
        self.assertEqual(plan.request(0).settings, before)

    def test_separate_rust_process_saved_settings_actual_fec_and_original_inputs_match(self):
        path = os.environ.get('MERLIN_UPSTREAM_RUST_SAVED_FULL_LOOP_JSON')
        self.assertIsNotNone(path, 'Installed parity gate must provide a separate Rust reference')
        expected = json.loads(Path(path).read_text())
        plan = prepare(document())
        request = plan.request(0)
        self.assertEqual(plan.info, expected['plan'])
        self.assertEqual(request.settings, expected['settings'])
        self.assertEqual(request.response_tolerance.value, expected['response_tolerance'])
        result = request.run()
        report = result.report()
        self.assertEqual(report['status'], expected['status'])
        self.assertEqual(report['summary'], expected['summary'])
        self.assertEqual(result.input_summary(), expected['inputs'])
        self.assertGreater(report['summary']['comparison']['bits_tested'], 0)


if __name__ == '__main__':
    unittest.main()
