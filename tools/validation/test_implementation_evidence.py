#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Independent temporary contracts for accepted-chain and historical-identity gates."""
from copy import deepcopy
import hashlib
import json
from pathlib import Path
import subprocess
import tempfile
import unittest

from implementation_evidence import FIELDS, audit, candidate_revision, compiled_cases, fingerprint, rust_case


class ImplementationEvidenceTests(unittest.TestCase):
    """Synthetic records test the validator; they are never MERLIN science evidence."""

    def setUp(self):
        """Create one isolated Git base and a complete declared working candidate."""
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        self.write('README', 'Independent fixture.\n')
        self.write('docs/development/checklist.md', '- [ ] **INT-TOOLS-01** Not accepted yet.\n')
        self.git('init', '-q')
        self.git('add', '.')
        self.git('commit', '-qm', 'Fixture base')
        base = self.git('rev-parse', 'HEAD').strip()
        for path, body in {
            'docs/architecture.md': '## Owner\nCurrent owner.\n',
            'docs/requirements.md': '#### REQ-COM-TRACE-02\nComplete evidence.\n',
            'docs/development/checklist.md': '- [x] **INT-TOOLS-01** Verify chains.\n',
            'docs/design.md': '# Design\nExact owners.\n',
            'tools/engine.py': 'def actual():\n    return 1\n',
            'tests/native.py': 'class Accepted:\n    def test_actual(self):\n        pass\n',
        }.items():
            self.write(path, body)
        files = [{'path': name, 'sha256': hashlib.sha256((self.root / name).read_bytes()).hexdigest()}
                 for name in ['tools/engine.py', 'tests/native.py']]
        self.candidate = {'base_revision': base, 'files': files, 'sha256': fingerprint(files)}
        self.write_json('candidate.json', self.candidate)
        self.record = {'requirement_id': 'REQ-COM-TRACE-02', 'architecture_owner': 'docs/architecture.md#owner',
            'feature_design': 'docs/design.md', 'checklist_task': 'INT-TOOLS-01',
            'implementing_crate': 'development validation tools', 'source_file': 'tools/engine.py',
            'symbol': 'actual', 'test_file': 'tests/native.py', 'qualified_test_case': 'Accepted.test_actual',
            'validation_command': 'python -m unittest tests/native.py', 'observed_result': 'pass',
            'tested_candidate': {'record': 'candidate.json', 'sha256': self.candidate['sha256']},
            'status': 'integrated-validated fixture', 'limitations': 'Synthetic validator input only.'}
        self.evidence = {'completion_authority': 'docs/development/checklist.md', 'records': [self.record]}
        self.run = {'command': self.record['validation_command'], 'exit_code': 0, 'passed': 1,
            'candidate_record': 'candidate.json', 'candidate_sha256': self.candidate['sha256'],
            'cases': [{'test_file': 'tests/native.py', 'qualified_test_case': 'Accepted.test_actual'}]}
        self.inventory = {'integration_tasks': [{'id': 'INT-TOOLS-01', 'architecture': 'owner',
                                               'requirements': ['REQ-COM-TRACE-02']}],
            'implementation_acceptance': {'completion_authority': 'docs/development/checklist.md',
                                         'historical_tasks': [], 'evidence_files': ['evidence.json'],
                                         'execution_files': ['run.json'], 'operation_matrix': 'matrix.json'}}
        self.write_json('matrix.json', {'type_exports': {}})
        self.save()

    def git(self, *arguments):
        """Use Git only in this disposable fixture, never the user's checkout."""
        return subprocess.run(['git', '-C', str(self.root), '-c', 'user.name=Fixture',
            '-c', 'user.email=fixture@example.invalid', '-c', 'commit.gpgsign=false',
            '-c', f'core.hooksPath={self.root / "no-hooks"}', *arguments],
            check=True, capture_output=True, text=True).stdout

    def write(self, name, body):
        """Write one independently authored fixture file."""
        path = self.root / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(body)

    def write_json(self, name, data):
        """Persist one synthetic contract without sharing production generators."""
        self.write(name, json.dumps(data))

    def save(self):
        """Persist the current evidence and execution inputs for this fixture."""
        self.write_json('evidence.json', self.evidence)
        self.write_json('run.json', self.run)

    def test_complete_working_candidate_and_exact_case(self):
        """Accept a fully linked, internally consistent synthetic contract."""
        self.assertEqual(audit(self.root, self.inventory), [])

    def test_every_field_is_required_and_unknown_owners_fail(self):
        """Nonempty records still require exact identities, owners and source symbols."""
        for field in FIELDS:
            data = deepcopy(self.evidence)
            del data['records'][0][field]
            self.write_json('evidence.json', data)
            self.assertTrue(audit(self.root, self.inventory), field)
        for value in (False, 0, [], '   '):
            data = deepcopy(self.evidence)
            data['records'][0]['limitations'] = value
            self.write_json('evidence.json', data)
            self.assertTrue(audit(self.root, self.inventory), repr(value))
        for field, value in [('symbol', 'actual_more'), ('implementing_crate', 'invented'),
                             ('architecture_owner', 'docs/architecture.md#missing'),
                             ('qualified_test_case', 'WrongClass.test_actual'),
                             ('requirement_id', 'REQ-COM-TRACE-99')]:
            data = deepcopy(self.evidence)
            data['records'][0][field] = value
            self.write_json('evidence.json', data)
            self.assertTrue(audit(self.root, self.inventory), field)

    def test_failed_unexecuted_and_candidate_mismatched_runs_fail(self):
        """A test declaration or successful unrelated run cannot establish acceptance."""
        for field, value in [('exit_code', 1), ('passed', 2), ('command', ''),
                             ('candidate_sha256', '0' * 64)]:
            run = deepcopy(self.run)
            run[field] = value
            self.write_json('run.json', run)
            self.assertTrue(audit(self.root, self.inventory), field)
        run = deepcopy(self.run)
        run.update(passed=0, cases=[])
        self.write_json('run.json', run)
        self.assertTrue(any('no successful execution' in error for error in audit(self.root, self.inventory)))

    def test_tampered_candidates_and_duplicate_chains_fail(self):
        """Do not cache mutable acceptance or accept duplicated identity rows."""
        self.assertEqual(audit(self.root, self.inventory), [])
        altered = deepcopy(self.candidate)
        altered['files'][0]['sha256'] = '0' * 64
        self.write_json('candidate.json', altered)
        self.assertTrue(audit(self.root, self.inventory))
        self.write_json('candidate.json', self.candidate)
        self.evidence['records'].append(deepcopy(self.record))
        self.save()
        self.assertTrue(any('duplicate implementation' in error for error in audit(self.root, self.inventory)))

    def test_accepted_task_cannot_drop_its_requirement_chain(self):
        """Check the official state, independently of a nonempty supporting catalog."""
        self.evidence['records'] = []
        self.save()
        self.assertTrue(any('accepted task lacks' in error for error in audit(self.root, self.inventory)))

    def test_historical_candidate_survives_later_source_changes(self):
        """Verify actual committed bytes; later edits do not invalidate valid history."""
        self.git('add', '.')
        self.git('commit', '-qm', 'Fixture accepted candidate')
        accepted = self.git('rev-parse', 'HEAD').strip()
        self.write('tools/engine.py', 'def actual():\n    return 2\n')
        self.assertEqual(candidate_revision(str(self.root), 'candidate.json',
                                           self.candidate['sha256'], accepted), accepted)
        self.assertEqual(audit(self.root, self.inventory), [])

    def test_historical_exceptions_require_prior_accepted_state(self):
        """Retain valid old formats while rejecting a new task disguised as history."""
        self.git('add', '.')
        self.git('commit', '-qm', 'Fixture old acceptance')
        revision = self.git('rev-parse', 'HEAD').strip()
        entry = {'task': 'INT-TOOLS-01', 'revision': revision, 'evidence': 'docs/design.md',
                 'reason': 'Previously accepted fixture format.'}
        self.inventory['implementation_acceptance']['historical_tasks'] = [entry]
        self.evidence['records'] = []
        self.save()
        self.assertEqual(audit(self.root, self.inventory), [])
        entry['revision'] = self.candidate['base_revision']
        self.assertTrue(any('historical exception' in error.lower() for error in audit(self.root, self.inventory)))

    def test_qualified_rust_discovery_rejects_wrong_modules_and_comment_functions(self):
        """Root targets resolve exact inline/external module paths, never suffixes."""
        self.write('src/lib.rs', 'mod science;\n')
        self.write('src/science.rs', 'mod tests { #[test] fn actual() {} }\n// fn fake() {}\n')
        self.assertTrue(rust_case(self.root, 'src/lib.rs', 'science::tests::actual'))
        self.assertFalse(rust_case(self.root, 'src/lib.rs', 'wrong::tests::actual'))
        self.assertFalse(rust_case(self.root, 'src/lib.rs', 'science::fake'))
        self.assertFalse(rust_case(self.root, 'src/lib.rs', 'science::tests::actual_more'))

    def test_compiled_discovery_requires_the_exact_cargo_target_and_active_case(self):
        """Reject renamed binaries and ignore unexecuted disabled tests."""
        metadata = {'workspace_root': str(self.root), 'packages': [{'id': 'id', 'name': 'crate',
            'targets': [{'name': 'acceptance', 'kind': ['test'], 'src_path': str(self.root / 'tests/a.rs')}]}]}
        suite = {'package-id': 'id', 'package-name': 'crate', 'binary-name': 'acceptance', 'kind': 'test',
                 'testcases': {'actual': {'ignored': False, 'filter-match': {'status': 'matches'}},
                               'ignored': {'ignored': True, 'filter-match': {'status': 'matches'}}}}
        self.assertEqual(compiled_cases(self.root, metadata, {'rust-suites': {'id': suite}}),
                         {('tests/a.rs', 'actual')})
        suite['binary-name'] = 'wrong'
        with self.assertRaises(ValueError):
            compiled_cases(self.root, metadata, {'rust-suites': {'id': suite}})


if __name__ == '__main__':
    unittest.main()
