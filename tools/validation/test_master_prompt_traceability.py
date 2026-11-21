#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Independent prompt-coverage and exact-case discovery regressions."""
import hashlib
from pathlib import Path
import tempfile
import unittest

from integration_traceability import master_prompt_errors, matrix_evidence_errors


class MasterPromptTraceabilityTests(unittest.TestCase):
    """Verify retained prompt coverage without running or importing backend code."""

    def setUp(self):
        """Build an independent single-clause review without backend dependencies."""
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.root = Path(self.directory.name)
        source = '# 1. Preserve scope\nDo not overwrite accepted work.\n'
        digest = hashlib.sha256(source.encode()).hexdigest()
        self.review = {
            'source': 'docs/prompt.md', 'review': 'docs/review.md',
            'api_matrix': 'docs/matrix.md', 'sha256': digest,
            'clauses': [{'id': 'MASTER-1', 'heading': '1. Preserve scope',
                         'line': 1, 'sha256': digest,
                         'requirements': ['REQ-COM-TRACE-01'],
                         'tasks': ['INT-PLAN-01']}],
        }
        self.inventory = {'master_prompt_review': self.review, 'integration_tasks': [
            {'id': 'INT-PLAN-01', 'architecture': 'scope',
             'requirements': ['REQ-COM-TRACE-01']}]}
        self.write('docs/prompt.md', source)
        self.write('docs/review.md', '| <a id="master-1"></a>MASTER-1 | '
                   '[requirement](requirements.md#req-com-trace-01) | '
                   '[task](development/checklist.md#int-plan-01) | '
                   '[architecture](architecture.md#scope) |\n')
        self.write('docs/requirements.md', '#### REQ-COM-TRACE-01\nPreserve scope.\n')
        self.write('docs/matrix.md', '`tests/a.rs::accepts_scope`\n'
                   '`tests/a.py::NativeTests.test_scope`\n')
        self.write('tests/a.rs', '#[test]\nfn accepts_scope() {}\n')
        self.write('tests/a.py', 'class NativeTests:\n    def test_scope(self): pass\n')

    def write(self, path, content):
        """Write one independent fixture; never mutate the actual repository."""
        destination = self.root / path
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_text(content, encoding='utf-8')

    def test_complete_prompt_and_exact_cases(self):
        """Accept a full local delivery chain and qualified test declarations."""
        self.assertEqual(master_prompt_errors(self.root, self.inventory), [])

    def test_review_cannot_be_removed_while_preflight_task_remains(self):
        """Deleting supporting metadata cannot bypass the declared preflight gate."""
        del self.inventory['master_prompt_review']
        self.assertTrue(any('missing master prompt review inventory' in error
                            for error in master_prompt_errors(self.root, self.inventory)))

    def test_deleted_mandatory_bullet_changes_source_and_section_hash(self):
        """A retained title cannot conceal a changed clause body."""
        self.write('docs/prompt.md', '# 1. Preserve scope\n')
        errors = master_prompt_errors(self.root, self.inventory)
        self.assertTrue(any('retained source hash' in error for error in errors))
        self.assertTrue(any('source clause/line/hash' in error for error in errors))

    def test_unmapped_subheading(self):
        """Coverage includes subordinate clauses and numbered parent titles."""
        self.write('docs/prompt.md', '# 1. Preserve scope\n## Acceptance\nKeep evidence.\n')
        self.assertTrue(any('clause coverage' in error for error in
                            master_prompt_errors(self.root, self.inventory)))

    def test_duplicate_clause_identity(self):
        """Two rows cannot reuse one stable clause identity."""
        self.review['clauses'].append(dict(self.review['clauses'][0]))
        self.assertTrue(any('duplicate clause' in error for error in
                            master_prompt_errors(self.root, self.inventory)))

    def test_empty_mapping(self):
        """No clause is covered by an empty requirement or task list."""
        self.review['clauses'][0]['requirements'] = []
        self.assertTrue(any('empty requirement/task' in error for error in
                            master_prompt_errors(self.root, self.inventory)))

    def test_unknown_delivery_or_undelivered_requirement(self):
        """Each requirement must belong to a clause's actual delivery task."""
        self.inventory['integration_tasks'][0]['requirements'] = []
        self.assertTrue(any('invalid requirement delivery' in error for error in
                            master_prompt_errors(self.root, self.inventory)))
        self.review['clauses'][0]['tasks'] = ['INT-UNKNOWN-01']
        self.assertTrue(any('unknown delivery task' in error for error in
                            master_prompt_errors(self.root, self.inventory)))

    def test_missing_architecture_and_requirement_review_links(self):
        """Index metadata cannot substitute for navigable clause references."""
        self.write('docs/review.md', '| <a id="master-1"></a>MASTER-1 |\n')
        errors = master_prompt_errors(self.root, self.inventory)
        self.assertTrue(any('missing architecture link' in error for error in errors))
        self.assertTrue(any('invalid requirement delivery' in error for error in errors))

    def test_rust_case_is_a_test_not_a_same_named_helper(self):
        """A function name alone is insufficient acceptance-case discovery."""
        self.write('tests/a.rs', 'fn accepts_scope() {}\n')
        self.assertTrue(any('missing exact test tests/a.rs' in error for error in
                            matrix_evidence_errors(self.root, Path('docs/matrix.md'))))

    def test_python_case_qualifies_actual_class(self):
        """Catch a correct method name paired with the wrong test class."""
        self.write('tests/a.py', 'class OtherTests:\n    def test_scope(self): pass\n')
        self.assertTrue(any('missing exact test tests/a.py' in error for error in
                            matrix_evidence_errors(self.root, Path('docs/matrix.md'))))


if __name__ == '__main__':
    unittest.main()
