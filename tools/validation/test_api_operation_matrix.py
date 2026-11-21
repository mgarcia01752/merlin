#!/usr/bin/env python3
#
# Copyright 2026 DOCSIS OFDM Channel Sim contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Reject incomplete API contracts using independent source/export/run fixtures."""
from copy import deepcopy
import json
from pathlib import Path
import tempfile
import unittest

from api_operation_matrix import COLUMNS, executed_cases, validate


class OperationMatrixTests(unittest.TestCase):
    """Verify exact signatures, test names and successful execution boundaries."""

    def setUp(self):
        """Create one complete contract independent of project matrix generation."""
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.root = Path(temporary.name)
        for name, body in {
            'docs/requirements.md': '#### REQ-US-API-01\nSupported operation.\n',
            'docs/development/checklist.md': '- [ ] **INT-RUST-01** Deliver API.\n',
            'src/api.rs': 'pub fn new() -> Value { Value }',
            'tests/api.rs': '#[test]\nfn exact_case() {}\nfn exact_case_more() {}',
            'tests/native.py': 'class NativeTests:\n    def test_real(self): pass\n',
        }.items():
            path = self.root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(body)
        self.method = {'operation': 'merlin::upstream::Value::new',
                       'signature': 'pub fn new() -> Value', 'source_file': 'src/api.rs'}
        row = dict.fromkeys(COLUMNS, 'contract')
        row.update(direction='upstream', operation=self.method['operation'],
                   rust_namespace='merlin::upstream', rust_type_method=deepcopy(self.method),
                   python_namespace='merlin.upstream', python_type_method={'declaration': 'Value()'},
                   report_contract={'return': 'Value'}, requirement=['REQ-US-API-01'],
                   checklist=['INT-RUST-01'], tests={'rust': ['tests/api.rs::exact_case'],
                       'python': {'status': 'existing root case', 'cases': ['tests/native.py::NativeTests.test_real']}})
        self.data = {'contracts': {'contract': 'An explicit contract.'}, 'operations': [row]}
        self.exports = {'upstream': [{'export':'merlin::upstream::Value', 'methods': [self.method]}]}
        self.executions = {'tests/api.rs::exact_case'}

    def test_complete_contract_and_exact_export_execution(self):
        """Accept complete explicit cells against a real named source/pass tuple."""
        self.assertEqual(validate(self.root, self.data, self.exports, self.executions), [])

    def test_missing_empty_duplicate_and_unknown_references(self):
        """Reject structural gaps before an accepted task can conceal them."""
        for field in COLUMNS:
            data = deepcopy(self.data)
            del data['operations'][0][field]
            self.assertTrue(validate(self.root, data))
        data = deepcopy(self.data)
        data['operations'][0]['units'] = ''
        self.assertTrue(validate(self.root, data))
        data = deepcopy(self.data)
        data['operations'].append(deepcopy(data['operations'][0]))
        self.assertTrue(any('duplicate' in error for error in validate(self.root, data)))
        for field, value in [('units', 'unknown'), ('requirement', ['REQ-US-API-99']), ('checklist', ['INT-UNKNOWN-01'])]:
            data = deepcopy(self.data)
            data['operations'][0][field] = value
            self.assertTrue(validate(self.root, data))

    def test_stale_signature_missing_export_and_unexecuted_case(self):
        """Compilation/source existence cannot substitute for the exact contract."""
        data = deepcopy(self.data)
        data['operations'][0]['rust_type_method']['signature'] = 'pub fn new() -> Other'
        self.assertTrue(any('stale' in error for error in validate(self.root, data, self.exports)))
        self.assertTrue(any('no matching' in error for error in validate(self.root, self.data, {})))
        data = deepcopy(self.data)
        data['operations'] = []
        self.assertTrue(any('lacks a matrix row' in error for error in validate(self.root, data, self.exports)))
        self.assertTrue(any('no successful execution' in error
                            for error in validate(self.root, self.data, self.exports, set())))
        data = deepcopy(self.data)
        data['operations'][0]['tests']['rust'] = ['tests/api.rs::exact_case_mor']
        self.assertTrue(any('missing exact Rust' in error for error in validate(self.root, data)))
        data = deepcopy(self.data)
        data['operations'][0]['tests']['python']['cases'] = ['tests/native.py::Other.test_real']
        self.assertTrue(any('missing exact Python' in error for error in validate(self.root, data)))

    def test_wrong_namespace_direction_and_duplicate_execution_are_rejected(self):
        """Nonempty cells still need exact export scope and unique named passes."""
        for field, value in [('direction', 'both'), ('rust_namespace', 'other')]:
            data = deepcopy(self.data)
            data['operations'][0][field] = value
            self.assertTrue(validate(self.root, data, self.exports))
        row = {'test_file': 'tests/api.rs', 'qualified_test_case': 'exact_case'}
        (self.root / 'run.json').write_text(json.dumps({'exit_code': 0, 'passed': 2, 'cases': [row, row]}))
        with self.assertRaises(ValueError):
            executed_cases(self.root, ['run.json'])

    def test_failed_or_mismatched_run_records_do_not_become_evidence(self):
        """Only exit-zero runs with complete named-case counts are admitted."""
        record = {'exit_code': 0, 'passed': 1, 'cases': [
            {'test_file': 'tests/api.rs', 'qualified_test_case': 'exact_case'}]}
        path = self.root / 'run.json'
        path.write_text(json.dumps(record))
        self.assertEqual(executed_cases(self.root, ['run.json']), self.executions)
        for field, value in [('exit_code', 1), ('passed', 2)]:
            changed = deepcopy(record)
            changed[field] = value
            path.write_text(json.dumps(changed))
            with self.assertRaises(ValueError):
                executed_cases(self.root, ['run.json'])


if __name__ == '__main__':
    unittest.main()
