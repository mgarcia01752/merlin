# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
"""Explicit exhaustive saved-scenario parity using an installed native wheel."""
import json
import os
from pathlib import Path
import unittest

from merlin.common import Project

ROOT = Path(__file__).resolve().parents[3] / 'examples/workbench'
REFERENCE = os.environ.get('MERLIN_DOWNSTREAM_EXAMPLES_RUST_REFERENCE_JSON')


@unittest.skipUnless(REFERENCE, 'Requires explicit exhaustive Rust scenario acceptance reference')
class WorkbenchScenarioAcceptanceTests(unittest.TestCase):
    def test_every_downstream_profile_matches_independent_rust_execution(self):
        """Saved settings retain actual successful and impaired decoder outcomes."""
        cases = json.loads(Path(REFERENCE).read_bytes())['cases']
        grouped = {}
        for case in cases:
            grouped.setdefault(case['workbench'], []).append(case)
        manifest = json.loads((ROOT / 'manifest.json').read_bytes())
        advertised = {path for bundle in manifest['bundles']
                      if bundle['direction'] == 'downstream'
                      for path in [bundle['workbench'], *bundle.get('companions', [])]}
        self.assertEqual(set(grouped), advertised)
        for relative, expected in grouped.items():
            with self.subTest(workbench=relative):
                path = ROOT / relative
                original = path.read_bytes()
                project = Project.from_file(str(path))
                plan = project.downstream_plan()
                self.assertEqual(plan.info['selected_profile_ids'],
                                 [case['profile'] for case in expected])
                for index, case in enumerate(expected):
                    with self.subTest(profile=case['profile']):
                        report = plan.request(index).run().report()
                        for key in ('status', 'summary', 'error',
                                    'unrecovered_generated_payloads'):
                            self.assertEqual(report[key], case[key], key)
                self.assertEqual(project.original_document, original)
                self.assertEqual(path.read_bytes(), original)


if __name__ == '__main__':
    unittest.main()
