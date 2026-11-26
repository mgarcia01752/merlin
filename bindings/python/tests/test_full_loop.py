#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Installed native downstream full-loop contracts retain actual science and controls."""
import json
from datetime import timedelta
import os
import signal
import sys
from pathlib import Path
import threading
import time
import unittest

import merlin
import merlin.downstream as m

FIXTURES = Path(__file__).resolve().parents[3] / 'examples/measurements/single-channel'
# Independent host-clock observation fixtures; neither value is a simulation budget.
PROGRESS_AGE_OBSERVATION_MILLISECONDS = 5
PAUSE_OBSERVATION_MILLISECONDS = 30


def inputs():
    profile = m.DPD.from_file(FIXTURES / 'modulation_profile.json', m.ProfileId(0))
    transmission = m.TransmissionConfiguration()
    original = json.loads((FIXTURES / 'rxmer.json').read_bytes())
    original['values'] = [80.0] * 7480
    original['pnm_header']['capture_time'] = 1466967
    measured = m.RxMer.from_bytes('clean', json.dumps(original).encode())
    measurements = m.MeasurementSequence(profile, transmission, [measured], m.MeasurementSequenceOptions('single', 1000))
    return transmission, profile, m.SourceInput.prbs15(m.PrbsSeed(32767)), measurements, m.ChannelConfiguration()


def options(**changes):
    policy = {'duration_ms': 1, 'max_iterations': 10, 'max_runtime_ms': 60000}
    policy.update(changes)
    return m.FullLoopOptions.from_json(json.dumps(policy).encode())


def request(**changes):
    return m.Simulation.prepare_full_loop(*inputs(), options=options(**changes))


class FullLoopTests(unittest.TestCase):
    def until(self, control, predicate):
        deadline = time.monotonic() + 30
        while not predicate(control.snapshot()):
            self.assertLess(time.monotonic(), deadline)
            time.sleep(0.002)

    def test_complete_real_fec_original_ownership_and_immutable_requests(self):
        accepted = inputs()
        prepared = m.Simulation.prepare_full_loop(*accepted, options=options())
        self.assertEqual(prepared.settings['duration'], {'secs': 0, 'nanos': 1000000})
        before = prepared.options.settings
        before['duration_ms'] = 999
        self.assertEqual(prepared.options.settings['duration_ms'], 1)
        control = m.FullLoopControl()
        self.assertIsNone(control.last_progress_age_ms())
        self.assertEqual(control.snapshot(), {'state': 'idle', 'pause_requested': False,
                                             'cancellation_requested': False, 'progress': None})
        output = prepared.run(control)
        report = output.report()
        self.assertEqual(report['status'], 'complete')
        self.assertEqual(control.snapshot()['state'], 'completed')
        self.assertEqual(control.snapshot()['progress']['phase'], 'finished')
        age_before = control.last_progress_age_ms()
        time.sleep(timedelta(milliseconds=PROGRESS_AGE_OBSERVATION_MILLISECONDS).total_seconds())
        self.assertGreaterEqual(control.last_progress_age_ms(), age_before + PROGRESS_AGE_OBSERVATION_MILLISECONDS)
        self.assertGreater(report['summary']['comparison']['bits_tested'], 0)
        self.assertEqual(report['summary']['comparison']['bit_errors'], 0)
        self.assertEqual(report['summary']['payload_ldpc_failures'], 0)
        self.assertEqual(report['summary']['payload_bch_failures'], 0)
        self.assertGreater(report['summary']['recovered_payloads'], 0)
        recovery = output.fec_recovery()
        self.assertEqual(recovery, {'generated': report['summary']['generated_payloads'],
                                    'recovered': report['summary']['fec']['total_cw'],
                                    'rejected': 0, 'mismatched': 0,
                                    'unrecovered': report['unrecovered_generated_payloads']})
        expected_recovery = recovery.copy()
        recovery['recovered'] = -1
        self.assertEqual(output.fec_recovery(), expected_recovery)
        self.assertIsNone(output.recovered_bytes())
        retained = output.input_summary()
        self.assertEqual(retained['source_kind'], 'prbs15')
        self.assertIs(merlin.FullLoopResult, m.FullLoopResult)
        with self.assertRaises(m.ApiError) as raised:
            prepared.run(control)
        self.assertEqual(raised.exception.code, m.ErrorCode.Busy)
        report['summary']['comparison']['bit_errors'] = 123
        del prepared, accepted, control
        self.assertEqual(output.report()['summary']['comparison']['bit_errors'], 0)
        self.assertEqual(output.input_summary()['profile_sources'], retained['profile_sources'])
        self.assertEqual(output.fec_recovery(), expected_recovery)

    def test_pause_acknowledgement_resume_cancel_busy_and_partial_fec(self):
        prepared = request(duration_ms=1000, max_retained_words=2)
        control = m.FullLoopControl()
        results, errors = [], []

        def run():
            try:
                results.append(prepared.run(control))
            except BaseException as cause:
                errors.append(cause)

        worker = threading.Thread(target=run)
        worker.start()
        self.until(control, lambda value: value['progress'] and value['progress']['recovered_payloads'] > 0)
        control.pause()
        self.until(control, lambda value: value['state'] == 'paused')
        before = control.snapshot()['progress']
        age_before = control.last_progress_age_ms()
        time.sleep(timedelta(milliseconds=PAUSE_OBSERVATION_MILLISECONDS).total_seconds())
        self.assertGreaterEqual(control.last_progress_age_ms(), age_before + PAUSE_OBSERVATION_MILLISECONDS)
        self.assertEqual(before, control.snapshot()['progress'])
        with self.assertRaises(m.ApiError) as raised:
            prepared.run(control)
        self.assertEqual(raised.exception.code, m.ErrorCode.Busy)
        control.resume()
        self.until(control, lambda value: value['progress']['completed_source_symbols'] > before['completed_source_symbols'])
        control.pause()
        self.until(control, lambda value: value['state'] == 'paused')
        control.cancel()
        worker.join(timeout=20)
        self.assertFalse(worker.is_alive())
        self.assertEqual(errors, [])
        self.assertEqual(control.snapshot()['state'], 'cancelled')
        report = results[0].report()
        self.assertEqual(report['status'], 'cancelled')
        self.assertGreater(report['summary']['comparison']['bits_tested'], 0)
        self.assertEqual(report['summary']['comparison']['bit_errors'], 0)
        self.assertGreaterEqual(report['summary']['recovered_payloads'], before['recovered_payloads'])
        self.assertGreater(len(report['retained_words']), 0)
        self.assertLessEqual(len(report['retained_words']), 2)
        self.assertFalse(control.snapshot()['pause_requested'])
        recovery = results[0].fec_recovery()
        self.assertEqual(recovery['generated'], report['summary']['generated_payloads'])
        self.assertEqual(recovery['unrecovered'], report['unrecovered_generated_payloads'])
        self.assertEqual(recovery['recovered'], report['summary']['fec']['total_cw'])
        self.assertEqual((recovery['rejected'], recovery['mismatched']), (0, 0))

    def test_initial_pause_precancel_and_exact_finite_file_recovery(self):
        prepared = request()
        control = m.FullLoopControl()
        control.pause()
        outputs = []
        worker = threading.Thread(target=lambda: outputs.append(prepared.run(control)))
        worker.start()
        self.until(control, lambda value: value['state'] == 'paused')
        self.assertIsNone(control.snapshot()['progress'])
        self.assertIsNone(control.last_progress_age_ms())
        control.cancel()
        worker.join(timeout=20)
        self.assertFalse(worker.is_alive())
        self.assertEqual(outputs[0].report()['summary']['comparison']['bits_tested'], 0)
        self.assertEqual(outputs[0].fec_recovery(),
                         {'generated': 0, 'recovered': 0, 'rejected': 0,
                          'mismatched': 0, 'unrecovered': 0})
        args = list(inputs())
        args[2] = m.SourceInput.from_bytes('file', b'exact source\r\n')
        finite = m.Simulation.prepare_full_loop(*args, options=options())
        output = finite.run()
        self.assertEqual(output.recovered_bytes(), b'exact source\r\n')
        self.assertEqual(output.report()['summary']['comparison']['bits_tested'], 112)
        self.assertTrue(output.report()['recovered_file']['complete'])
        recovery = output.fec_recovery()
        self.assertGreater(recovery['recovered'], 0)
        self.assertEqual((recovery['rejected'], recovery['mismatched']), (0, 0))
        stopped = m.FullLoopControl()
        stopped.cancel()
        self.assertEqual(finite.run(stopped).report()['status'], 'cancelled')

    def test_runtime_limit_narrows_immutably_and_preserves_admission_and_retention(self):
        prepared = request().with_observations(m.FullLoopObservationPolicy())
        settings = prepared.settings
        policy = prepared.observation_policy.settings
        # Independent 30-second limit narrows this fixture's 60-second parent.
        limited = prepared.with_runtime_limit(30_000)
        self.assertEqual(limited.options.settings['max_runtime_ms'], 30_000)
        self.assertEqual(prepared.options.settings['max_runtime_ms'], 60_000)
        self.assertEqual(limited.settings, settings)
        self.assertEqual(limited.observation_policy.settings, policy)
        self.assertEqual(limited.response_tolerance.value, prepared.response_tolerance.value)
        self.assertEqual(prepared.with_runtime_limit(60_000).settings, settings)
        # Zero, one above parent, one above the 300-second owner ceiling, and u64 maximum.
        for rejected in [0, 60_001, 300_001, 2**64 - 1]:
            with self.assertRaises(m.ApiError):
                prepared.with_runtime_limit(rejected)
        with self.assertRaises(m.ApiError) as widened:
            prepared.with_runtime_limit(60_001)
        self.assertEqual(widened.exception.code, m.ErrorCode.ResourceLimit)
        for invalid_native_integer in [-1, 2**64]:
            with self.assertRaises(OverflowError):
                prepared.with_runtime_limit(invalid_native_integer)
        self.assertEqual(prepared.settings, settings)
        self.assertEqual(prepared.options.settings['max_runtime_ms'], 60_000)

    def test_runtime_limit_returns_actual_partial_outcome_without_mutating_parent(self):
        # One second of simulated observation, with an independent one-ms active host limit.
        prepared = request(duration_ms=1_000)
        settings = prepared.settings
        limited = prepared.with_runtime_limit(1)
        control = m.FullLoopControl()
        output = limited.run(control)
        self.assertEqual(output.report()['status'], 'runtime_exceeded')
        self.assertEqual(control.snapshot()['state'], 'runtime_exceeded')
        self.assertEqual(output.report()['settings'], settings)
        self.assertEqual(prepared.settings, settings)
        self.assertEqual(prepared.options.settings['max_runtime_ms'], 60_000)

    def test_rejections_leave_original_inputs_and_prepared_options_unchanged(self):
        for data in [b'{', b'{"unknown":1}', b'{"duration_ms":1001}', b'{"max_runtime_ms":0}', b' ' * 65537]:
            with self.assertRaises(m.ApiError):
                m.FullLoopOptions.from_json(data)
        args = list(inputs())
        before = args[1].sources
        changed = args[0].settings
        changed['cyclic_prefix_samples'] = 1024
        args[0] = m.TransmissionConfiguration.from_json(json.dumps(changed).encode())
        with self.assertRaises(m.ApiError) as raised:
            m.Simulation.prepare_full_loop(*args, options=options())
        self.assertEqual(raised.exception.code, m.ErrorCode.IncompatibleGrid)
        self.assertEqual(args[1].sources, before)
        args = inputs()
        response = m.ChannelEstimate.from_file(FIXTURES / 'channel_estimation.json')
        with self.assertRaises(m.ApiError) as raised:
            m.Simulation.prepare_full_loop(*args, response=response, options=options())
        self.assertEqual(raised.exception.code, m.ErrorCode.AssociationFailed)

    @unittest.skipUnless(os.environ.get('MERLIN_FULL_LOOP_PARITY'), 'Direct Rust process reference required')
    def test_actual_direct_rust_process_counts_fec_settings_and_words_match(self):
        reference = json.loads(Path(os.environ['MERLIN_FULL_LOOP_PARITY']).read_text())
        prepared = request()
        output = prepared.run()
        report = output.report()
        self.assertEqual(report['status'], reference['status'])
        for name in ['comparison', 'recovered_payloads', 'payload_ldpc_failures', 'payload_bch_failures', 'generated_payloads']:
            self.assertEqual(report['summary'][name], reference['summary'][name])
        self.assertEqual(report['retained_words'], reference['words'])
        for name in ['transmission', 'layout', 'duration', 'ncp_modulation', 'prbs_seed', 'receiver_mode', 'channel_filter', 'mce', 'constellation', 'protected_constellation', 'max_iterations', 'receive_block_samples', 'data_bit_loading']:
            self.assertEqual(prepared.settings[name], reference['settings'][name])
        for name in ['mode', 'interval_symbols', 'tolerance_seconds']:
            self.assertEqual(prepared.settings['playback'][name], reference['settings']['playback'][name])
        self.assertEqual(output.input_summary()['key'], reference['inputs']['key'])
        self.assertEqual(output.fec_recovery(), reference['recovery'])

    @unittest.skipUnless(sys.platform == 'linux', 'Local Linux signal lifecycle acceptance')
    def test_interpreter_interrupt_cancels_a_paused_native_worker_and_keeps_partial_report(self):
        prepared = request()
        control = m.FullLoopControl()
        control.pause()
        delivered = []

        def interrupt():
            deadline = time.monotonic() + 10
            while control.snapshot()['state'] != 'paused':
                if time.monotonic() >= deadline:
                    control.cancel()
                    return
                time.sleep(0.002)
            delivered.append(True)
            os.kill(os.getpid(), signal.SIGINT)

        sender = threading.Thread(target=interrupt)
        sender.start()
        result = prepared.run(control)
        sender.join(timeout=15)
        self.assertFalse(sender.is_alive())
        self.assertEqual(delivered, [True])
        self.assertEqual(result.report()['status'], 'cancelled')
        self.assertEqual(result.report()['summary']['comparison']['bits_tested'], 0)
