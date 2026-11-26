# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
"""Installed direction-project admission and separate Rust-process execution parity."""
import json
from collections.abc import Sequence
import os
from pathlib import Path
import tempfile
import unittest

import merlin
from merlin import common, downstream, upstream

ROOT = Path(__file__).resolve().parents[3]
DOWNSTREAM = ROOT / 'examples/workbench/single/workbench.merlin.json'
UPSTREAM = ROOT / 'examples/cli/upstream/us-50khz.json'
# A transport split deliberately unrelated to waveform symbol dimensions.
CHUNK_SAMPLES = 257
# Small actual I/Q history, independent of transport chunks and receiver estimates.
RETAINED_IQ_POINTS = 128
# Cartesian storage for one 4096-bin estimate; also admits the smaller FFT.
ESTIMATE_SCALAR_SLOTS = 8192
# One actual transmitted LDPC word's input-metric budget, independent of cell retention.
WORD_LLR_BUDGET = 16200


def legacy():
    """Preserve the UI fixture's scientific settings and select its active profile."""
    value = json.loads(DOWNSTREAM.read_bytes())
    value['receiver']['all_profiles'] = False
    return value


def envelope(direction, contents):
    """Build the independently specified schema-3 transport; Rust validates all settings."""
    return {'schema_version': 3, 'application': 'MERLIN',
            'direction': direction, 'contents': contents}


def upstream_value(kind='configuration', path=UPSTREAM):
    """Copy existing source settings without implementing any scientific calculation."""
    original = path.read_text()
    source = json.loads(original)
    settings = source['simulation'] if kind == 'configuration' else source['configuration']['simulation']
    options = {key: settings[key] for key in ['grants', 'maximum_samples', 'chunk_samples',
               'timeout_ms', 'retained_points', 'retained_taps']}
    options.update(chunk_samples=CHUNK_SAMPLES, retained_points=RETAINED_IQ_POINTS)
    return envelope('upstream', {'input': {'kind': kind, 'document': original},
        'options': options, 'receiver_observations': {'maximum_frames': 1,
        'maximum_scalar_slots': ESTIMATE_SCALAR_SLOTS, 'soft': {'maximum_cells': 4,
        'maximum_words': 1, 'maximum_word_llrs': WORD_LLR_BUDGET}}})


def admit(value):
    """Send complete immutable transport bytes to the installed native Rust adapter."""
    return common.Project.from_bytes(json.dumps(value).encode())


def reference():
    """Require the acceptance runner's separately executed Rust project reference."""
    path = os.environ.get('MERLIN_UPSTREAM_RUST_PROJECT_JSON')
    if path is None:
        raise AssertionError('Installed project parity requires a separate Rust reference')
    return json.loads(Path(path).read_text())


class ProjectTests(unittest.TestCase):
    def test_real_common_namespace_legacy_originals_idle_ownership_and_precancel(self):
        self.assertEqual(common.Project.__module__, 'merlin.common')
        self.assertIs(merlin.Simulation, downstream.Simulation)
        for version in [1, 2]:  # Both historical downstream file-format versions.
            saved = legacy()
            saved['schema_version'] = version
            if version == 2:
                saved['receiver']['selected_profile_ids'] = [saved['sections']['profile']['profile_id']]
            original = json.dumps(saved, indent=2).encode()
            project = common.Project.from_bytes(original)
            self.assertEqual(project.direction, 'downstream')
            self.assertEqual(project.original_document, original)
            exported = project.to_json()
            wire = json.loads(exported)
            self.assertEqual(wire['schema_version'], 3)
            self.assertEqual(wire['contents']['workbench'].encode(), original)
            restored = common.Project.from_bytes(exported.encode())
            plan = restored.downstream_plan()
            with self.assertRaises(downstream.ApiError) as caught:
                restored.upstream_simulation()
            self.assertEqual(caught.exception.code, common.ErrorCode.InvalidInput)
            del restored, project
            request = plan.request(0)
            self.assertEqual(request.options.settings['noise_seed'], saved['receiver']['noise_seed'])
            control = downstream.FullLoopControl()
            control.cancel()
            self.assertEqual(request.run(control).report()['summary']['comparison']['bits_tested'], 0)

    def test_saved_downstream_actual_fec_observations_and_separate_rust_parity(self):
        expected = reference()['downstream']
        project = common.Project.from_bytes(expected['project'].encode())
        plan = project.downstream_plan()
        self.assertEqual(plan.info, expected['plan'])
        request = plan.request(0)
        self.assertEqual(request.settings, expected['settings'])
        self.assertEqual(request.observation_policy.settings, downstream.FullLoopObservationPolicy().settings)
        del project, plan
        result = request.run()
        self.assertEqual(result.report()['status'], expected['status'])
        self.assertEqual(result.report()['summary'], expected['summary'])
        self.assertEqual(result.input_summary(), expected['inputs'])
        self.assertEqual(result.observation_stats(), expected['observation_stats'])
        self.assertGreater(result.report()['summary']['comparison']['bits_tested'], 0)

    def test_saved_upstream_configuration_measured_sources_actual_run_and_rust_parity(self):
        expected = reference()
        ds_project = common.Project.from_bytes(expected['downstream']['project'].encode())
        keys = ['configuration', 'options', 'planned', 'status', 'generated_source_bits',
                'compared_source_bits', 'bit_errors', 'ber', 'reference_recovered',
                'not_started_grants', 'source_configuration_json', 'source_configuration_original',
                'source_configuration_sha256', 'measurement_provenance_json']
        for item in expected['upstream']:
            project = common.Project.from_bytes(item['project'].encode())
            self.assertEqual(project.direction, 'upstream')
            self.assertEqual(common.Project.collection_info([ds_project, project]), item['collection'])
            session = project.upstream_simulation()
            self.assertIsInstance(session, upstream.Simulation)
            with self.assertRaises(upstream.ApiError) as caught:
                project.downstream_plan()
            self.assertEqual(caught.exception.code, common.ErrorCode.InvalidInput)
            del project
            result = session.run()
            report = json.loads(result.to_json())
            for key in keys:
                self.assertEqual(report[key], item['report'][key], key)
            self.assertEqual([grant['words'] for grant in report['grants']],
                             [grant['words'] for grant in item['report']['grants']])
            observations = result.receiver_observations()
            for key in ['policy', 'completed_frames', 'retained_frames', 'dropped_frames', 'used_scalar_slots']:
                self.assertEqual(observations[key], item['receiver_observations'][key])
            self.assertTrue(report['reference_recovered'])
            self.assertEqual(report['options']['chunk_samples'], CHUNK_SAMPLES)
            self.assertEqual(report['options']['retained_points'], RETAINED_IQ_POINTS)

    def test_25khz_original_configuration_and_measured_lineage_survive_round_trip(self):
        for kind, path in [('configuration', ROOT / 'examples/cli/upstream/us-25khz.json'),
                           ('measured', ROOT / 'examples/measurements/upstream/measured/capture-driven-25khz.json')]:
            project = admit(upstream_value(kind, path))
            restored = common.Project.from_bytes(project.to_json().encode())
            self.assertEqual(json.loads(restored.to_json())['contents']['input']['document'], path.read_text())
            result = restored.upstream_simulation().run()
            self.assertTrue(result.reference_recovered)
            report = json.loads(result.to_json())
            if kind == 'measured':
                provenance = json.loads(report['measurement_provenance_json'])
                self.assertEqual(bytes(provenance['original_plan_bytes']), path.read_bytes())
            else:
                self.assertEqual(bytes(report['source_configuration_original']), path.read_bytes())

    def test_invalid_envelopes_policies_duplicate_fields_and_collections_are_atomic(self):
        accepted = admit(upstream_value())
        before = accepted.to_json()
        for key, value in [('direction', 'downstream'), ('schema_version', 99),
                           ('application', 'other'), ('unknown', True)]:
            candidate = upstream_value()
            candidate[key] = value
            with self.assertRaises(downstream.ApiError):
                admit(candidate)
        candidate = upstream_value()
        candidate['contents']['options']['chunk_samples'] = 0
        with self.assertRaises(downstream.ApiError):
            admit(candidate)
        text = json.dumps(upstream_value(), separators=(',', ':'))
        duplicate = text.replace('"chunk_samples":257', '"chunk_samples":257,"chunk_samples":258')
        self.assertNotEqual(duplicate, text)
        with self.assertRaises(downstream.ApiError) as caught:
            common.Project.from_bytes(duplicate.encode())
        self.assertEqual(caught.exception.code, common.ErrorCode.InvalidInput)
        with self.assertRaises(downstream.ApiError) as caught:
            common.Project.collection_info([accepted, accepted])
        self.assertEqual(caught.exception.code, common.ErrorCode.InvalidInput)
        with self.assertRaises(downstream.ApiError) as caught:
            common.Project.collection_info([accepted, accepted, accepted])
        self.assertEqual(caught.exception.code, common.ErrorCode.ResourceLimit)
        class IndexedProjectSequence(Sequence):
            def __len__(self):
                return 1

            def __getitem__(self, index):
                if index != 0:
                    raise IndexError(index)
                return accepted

            def __iter__(self):
                raise AssertionError('Native admission must use only the bounded index range')

        self.assertEqual(common.Project.collection_info(IndexedProjectSequence())['projects'], 1)
        self.assertEqual(common.Project.collection_info([])['projects'], 0)
        self.assertEqual(accepted.to_json(), before)

    def test_readonly_file_limits_and_nonregular_inputs(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / 'opaque.data'
            data = json.dumps(legacy()).encode()
            path.write_bytes(data)
            project = common.Project.from_file(str(path))
            self.assertEqual(project.original_document, data)
            self.assertEqual(path.read_bytes(), data)
            with self.assertRaises(downstream.ApiError) as caught:
                common.Project.from_file(directory)
            self.assertEqual(caught.exception.code, common.ErrorCode.Io)
            # Independent saved-document policy anchor; sparse extension allocates no payload.
            with path.open('wb') as stream:
                stream.truncate(144 * 1024 * 1024 + 1)
            with self.assertRaises(downstream.ApiError) as caught:
                common.Project.from_file(str(path))
            self.assertEqual(caught.exception.code, common.ErrorCode.ResourceLimit)
            if hasattr(os, 'mkfifo'):
                fifo = Path(directory) / 'pipe'
                os.mkfifo(fifo)
                with self.assertRaises(downstream.ApiError) as caught:
                    common.Project.from_file(str(fifo))
                self.assertEqual(caught.exception.code, common.ErrorCode.Io)


if __name__ == '__main__':
    unittest.main()
