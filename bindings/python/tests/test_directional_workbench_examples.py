# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
"""Run actual saved upstream files through an installed wheel and separate Rust parity."""
import json
import os
from pathlib import Path
import unittest

from merlin.common import Project

ROOT = Path(__file__).resolve().parents[3] / 'examples/workbench'


class DirectionalWorkbenchExamplesTests(unittest.TestCase):
    def test_saved_files_run_with_exact_originals_and_independent_rust_parity(self):
        """Both numerologies and both channel sources recover actual transmitted bits."""
        reference = os.environ.get('MERLIN_WORKBENCH_EXAMPLES_RUST_REFERENCE_JSON')
        self.assertIsNotNone(reference, 'Separate Rust saved-file reference is required')
        expected = json.loads(Path(reference).read_text())['cases']
        manifest = json.loads((ROOT / 'manifest.json').read_bytes())
        upstream = {row['workbench']: row for row in manifest['bundles']
                    if row['direction'] == 'upstream'}
        self.assertEqual(set(upstream), {case['workbench'] for case in expected})
        for case in expected:
            with self.subTest(workbench=case['workbench']):
                path = ROOT / case['workbench']
                original = path.read_bytes()
                project = Project.from_file(str(path))
                self.assertEqual(project.direction, 'upstream')
                self.assertEqual(project.original_document, original)
                saved = json.loads(original)
                companion = ROOT / upstream[case['workbench']]['source_document']
                self.assertEqual(saved['contents']['input']['document'].encode(), companion.read_bytes())
                restored = Project.from_bytes(project.to_json().encode())
                result = restored.upstream_simulation().run()
                report = json.loads(result.to_json())
                for key in ('configuration', 'options', 'planned', 'status',
                            'generated_source_bits', 'compared_source_bits',
                            'bit_errors', 'ber', 'reference_recovered',
                            'not_started_grants', 'source_configuration_json',
                            'source_configuration_original', 'source_configuration_sha256',
                            'measurement_provenance_json'):
                    self.assertEqual(report[key], case['report'][key], key)
                self.assertEqual([grant['words'] for grant in report['grants']],
                                 [grant['words'] for grant in case['report']['grants']])
                self.assertTrue(result.reference_recovered)
                self.assertGreater(report['compared_source_bits'], 0)
                self.assertEqual(report['bit_errors'], 0)
                self.assertEqual(report['options'], saved['contents']['options'])
                observations = result.receiver_observations()
                for key in ('policy', 'completed_frames', 'retained_frames',
                            'dropped_frames', 'used_scalar_slots'):
                    self.assertEqual(observations[key], case['receiver_observations'][key])
                self.assertEqual(project.original_document, original)
