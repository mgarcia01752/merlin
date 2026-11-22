#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Authoritative configured upstream allocation; no Python pilot-placement calculation."""
import inspect
import json
import os
from pathlib import Path
import threading
import time
import unittest

import merlin.upstream as u
from test_upstream_experiment import configuration as running_configuration

PROFILE_ID = 224
DATA_IUC = 9
PROFILE_RANDOMIZER_SEED = 1
SOURCE_PRBS15_SEED = 1
PREFIX_SAMPLES = 96
ROLLOFF_SAMPLES = 32
FRAME_SYMBOLS = 6
BANDWIDTH_HZ = 20_000_000
DATA_LOADING = 4
FIRST_50KHZ_CARRIER = 100
FIRST_25KHZ_CARRIER = 200
MINISLOT_50KHZ_CARRIERS = 8
MINISLOT_25KHZ_CARRIERS = 16
FIRST_50KHZ_PILOT_PATTERN = 1
FIRST_25KHZ_PILOT_PATTERN = 8
EXPECTED_25KHZ_FFT_SAMPLES = 4096
OVER_25KHZ_FFT_CARRIERS = 4097
PROFILE_MINISLOT_COUNT = 4
ZERO_LOADED_MINISLOT_INDEX = 1
VIEWPORT_BORDER_CARRIERS = 1
GRANT_ID = 17
GRANT_EPOCH_SAMPLE = 4096
FIRST_FRAME_ID = 11
GRANT_FRAME_COUNT = 2
SELECTED_FRAME_INDEX = 1
EXPECTED_DEFAULT_SYMBOL_COUNT = 1
EXPECTED_MAX_GRID_CELLS = 16384
OVER_GRID_CELL_BUDGET = 16385
EXPECTED_SCHEMA_VERSION = 1
EXPECTED_FIRST_RECTANGLE_CELLS = 204
EXPECTED_FRAME_BITS = 328
EXPECTED_GRANT_BITS = 656
EXPECTED_SECOND_FRAME_PREFIX = 16960
EXPECTED_SAMPLE_RATE_HZ = 102400000
CONTROL_DEADLINE_SECONDS = 30
CONTROL_POLL_SECONDS = 0.001
BUSY_RUN_GRANTS = 128
BUSY_RUN_CHUNK_SAMPLES = 127


def configured(spacing=50, pattern=None):
    """Construct independently labelled literal geometry through Rust's existing validators."""
    first, width, default_pattern = {
        50: (FIRST_50KHZ_CARRIER, MINISLOT_50KHZ_CARRIERS, FIRST_50KHZ_PILOT_PATTERN),
        25: (FIRST_25KHZ_CARRIER, MINISLOT_25KHZ_CARRIERS, FIRST_25KHZ_PILOT_PATTERN),
    }[spacing]
    if pattern is None:
        selected_pattern = default_pattern
    else:
        selected_pattern = pattern
    waveform = u.Waveform(spacing, PREFIX_SAMPLES, ROLLOFF_SAMPLES, FRAME_SYMBOLS, BANDWIDTH_HZ)
    slots = []
    for index in range(PROFILE_MINISLOT_COUNT):
        if index == ZERO_LOADED_MINISLOT_INDEX:
            loading, pilot_pattern = 0, 0
        else:
            loading, pilot_pattern = DATA_LOADING, selected_pattern
        slots.append(u.Minislot(first + index * width, loading, pilot_pattern))
    profile = u.Profile(PROFILE_ID, DATA_IUC, PROFILE_RANDOMIZER_SEED, slots)
    grant = u.SyntheticGrant(GRANT_ID, PROFILE_ID, GRANT_EPOCH_SAMPLE,
                             FIRST_FRAME_ID, GRANT_FRAME_COUNT, [0, 1, 2])
    config = u.UsConfiguration(waveform, [profile], grant, [first - VIEWPORT_BORDER_CARRIERS])
    simulation = u.Simulation()
    simulation.configure(u.ExperimentConfig(config, SOURCE_PRBS15_SEED, u.ReceiverConfig()))
    return simulation, first, width


def rectangle(simulation, first, width):
    """Select original configured coordinates; all roles and timing come back from Rust."""
    return simulation.resource_grid(frame_index=SELECTED_FRAME_INDEX,
                                    symbol_count=0, first_carrier=first - VIEWPORT_BORDER_CARRIERS,
                                    carrier_count=width * PROFILE_MINISLOT_COUNT + 2 * VIEWPORT_BORDER_CARRIERS)


class UpstreamResourceGridTests(unittest.TestCase):
    def test_literal_physical_roles_timing_original_capacity_and_owned_importable_snapshot(self):
        self.assertTrue(u.ResourceGridSnapshot.__module__.startswith('merlin.upstream'))
        signature = inspect.signature(u.Simulation.resource_grid)
        self.assertEqual(signature.parameters['symbol_count'].default, EXPECTED_DEFAULT_SYMBOL_COUNT)
        self.assertEqual(signature.parameters['max_cells'].default, EXPECTED_MAX_GRID_CELLS)
        simulation, first, width = configured()
        snapshot = rectangle(simulation, first, width)
        text = snapshot.to_json()
        view = json.loads(text)
        grid = view['grid']
        self.assertEqual(view['format'], 'merlin.upstream.resource_grid')
        self.assertEqual(view['schema_version'], EXPECTED_SCHEMA_VERSION)
        self.assertEqual(grid['sample_rate_hz'], EXPECTED_SAMPLE_RATE_HZ)
        self.assertEqual(grid['frame_prefix_sample'], EXPECTED_SECOND_FRAME_PREFIX)
        self.assertEqual(len(grid['cells']), EXPECTED_FIRST_RECTANGLE_CELLS)
        self.assertEqual(grid['frame_bits'], EXPECTED_FRAME_BITS)
        self.assertEqual(grid['grant_bits'], EXPECTED_GRANT_BITS)
        pilots = [(cell['frame_symbol'], cell['physical_carrier']) for cell in grid['cells']
                  if cell['role'] == 'known_pilot']
        self.assertEqual(pilots, [(0, 100), (0, 107), (0, 116), (0, 123),
                                 (2, 100), (2, 107), (2, 116), (2, 123)])
        self.assertEqual(grid['counts'], {'data': 80, 'complementary': 8, 'known_pilot': 8,
                                         'zero_loaded': 48, 'excluded': 6, 'unallocated': 54})
        for cell in grid['cells']:
            if cell['role'] == 'zero_loaded':
                self.assertTrue(cell['grant_selected'])
                self.assertEqual(cell['cell_loading'], 0)
                self.assertIsNone(cell['visit_index'])
                self.assertIsNone(cell['amplitude_gain'])
                self.assertIsNone(cell['grant_bit_offset'])
        zoom = json.loads(simulation.resource_grid(first_carrier=first, carrier_count=1).to_json())
        self.assertEqual(zoom['grid']['grant_bits'], grid['grant_bits'])
        self.assertEqual(zoom['configuration_generation'], view['configuration_generation'])
        simulation.configure(running_configuration())
        self.assertNotEqual(json.loads(simulation.resource_grid().to_json())['configuration_generation'],
                            view['configuration_generation'])
        del simulation
        self.assertEqual(snapshot.to_json(), text)

    def test_all_patterns_and_shared_bounds_keep_physical_holes_and_original_state(self):
        with self.assertRaises(u.ApiError) as missing:
            u.Simulation().resource_grid()
        self.assertEqual(missing.exception.code, u.ErrorCode.MissingSetting)
        for spacing, patterns in [(50, range(1, 8)), (25, range(8, 15))]:
            for pattern in patterns:
                simulation, first, width = configured(spacing, pattern)
                view = json.loads(rectangle(simulation, first, width).to_json())
                grid = view['grid']
                self.assertEqual(sum(grid['counts'].values()), len(grid['cells']))
                self.assertTrue(all(cell['pilot_pattern'] == pattern for cell in grid['cells']
                                    if cell['visit_index'] is not None))
                self.assertEqual(grid['counts']['zero_loaded'], width * FRAME_SYMBOLS)
                self.assertGreater(grid['counts']['known_pilot'], 0)
        simulation, _, _ = configured(25)
        original = simulation.resource_grid().to_json()
        for arguments in [dict(frame_index=GRANT_FRAME_COUNT), dict(first_symbol=FRAME_SYMBOLS),
                          dict(first_carrier=EXPECTED_25KHZ_FFT_SAMPLES), dict(carrier_count=OVER_25KHZ_FFT_CARRIERS),
                          dict(max_cells=0), dict(max_cells=1), dict(max_cells=OVER_GRID_CELL_BUDGET),
                          dict(symbol_count=0)]:
            with self.assertRaises(u.ApiError):
                simulation.resource_grid(**arguments)
        for arguments in [dict(first_carrier=-1), dict(symbol_count=1.5)]:
            with self.assertRaises((OverflowError, TypeError)):
                simulation.resource_grid(**arguments)
        self.assertEqual(simulation.resource_grid().to_json(), original)

    def test_busy_session_guard_and_cancel_restore_resource_query_without_hidden_execution(self):
        simulation = u.Simulation(u.ExperimentOptions(grants=BUSY_RUN_GRANTS, chunk_samples=BUSY_RUN_CHUNK_SAMPLES))
        simulation.configure(running_configuration())
        before = simulation.resource_grid().to_json()
        control = u.RunControl()
        control.pause()
        outputs, failures = [], []

        def run():
            try:
                outputs.append(simulation.run_with_control(control))
            except BaseException as cause:
                failures.append(cause)

        worker = threading.Thread(target=run)
        worker.start()
        try:
            deadline = time.monotonic() + CONTROL_DEADLINE_SECONDS
            while json.loads(control.progress_json())['status'] != 'Paused':
                self.assertLess(time.monotonic(), deadline)
                time.sleep(CONTROL_POLL_SECONDS)
            with self.assertRaises(u.ApiError) as busy:
                simulation.resource_grid()
            self.assertEqual(busy.exception.code, u.ErrorCode.Busy)
        finally:
            control.cancel()
            worker.join(CONTROL_DEADLINE_SECONDS)
        self.assertFalse(worker.is_alive())
        self.assertEqual(failures, [])
        self.assertEqual(outputs[0].status, 'Cancelled')
        self.assertEqual(simulation.resource_grid().to_json(), before)

    @unittest.skipUnless(os.environ.get('MERLIN_UPSTREAM_RUST_RESOURCE_GRID_JSON'), 'Direct Rust grid reference required')
    def test_separate_rust_process_grid_coordinates_roles_and_metadata_match_at_both_spacings(self):
        references = json.loads(Path(os.environ['MERLIN_UPSTREAM_RUST_RESOURCE_GRID_JSON']).read_text())
        for spacing, expected in zip([50, 25], references):
            simulation, first, width = configured(spacing)
            actual = json.loads(rectangle(simulation, first, width).to_json())

            # Generation identities are process-local; preservation is tested within sessions.
            self.assertGreater(actual['configuration_generation'], 0)
            self.assertEqual(actual['grid'], expected['grid'])
            for field in ['format', 'schema_version', 'transmission_planned']:
                self.assertEqual(actual[field], expected[field])
