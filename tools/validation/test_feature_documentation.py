#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Independent positive/negative contracts for functional documentation checks."""
import hashlib
from pathlib import Path
import tempfile
import unittest

from feature_documentation import (
    FEC_AUDIT, ROOT, STALE_FEC_AUDIT, anchors, audit,
    checkbox_history_errors, dependency_errors, feature_errors,
    fec_owner_errors, markdown_audit, owner_errors,
    navigation_preservation_errors,
)


class DocumentationValidationTests(unittest.TestCase):
    """Reject broken navigation and evidence changes using small literal fixtures."""

    def setUp(self):
        """Create a disposable repository-shaped documentation tree."""
        directory = tempfile.TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        self.root = Path(directory.name)

    def write(self, path, text):
        """Create only the requested literal test fixture."""
        destination = self.root / path
        destination.parent.mkdir(parents=True, exist_ok=True)
        destination.write_text(text, encoding='utf-8')

    def test_relative_links_spaces_images_and_duplicate_headings(self):
        """Resolve paths/fragments relative to each file and skip remote fetches."""
        self.write('docs/source.md', '[first](<part/with space.md#repeat-1>)\n'
                   '![image](part/picture.svg)\n[remote](https://example.invalid/x)\n')
        self.write('docs/part/with space.md', '# Repeat\n# Repeat\n')
        self.write('docs/part/picture.svg', '<svg/>')
        errors, edges, count = markdown_audit(self.root, ['docs/source.md'])
        self.assertEqual(errors, [])
        self.assertEqual(count, 2)
        self.assertIn('docs/part/with space.md', edges['docs/source.md'])

    def test_missing_local_file(self):
        """A well-formed link cannot hide an absent document."""
        self.write('source.md', '[missing](nowhere.md)')
        errors, _, _ = markdown_audit(self.root, ['source.md'])
        self.assertTrue(any('missing link' in error for error in errors))

    def test_missing_fragment(self):
        """Existing files do not satisfy nonexistent heading anchors."""
        self.write('source.md', '[wrong](target.md#absent)')
        self.write('target.md', '# Present\n')
        errors, _, _ = markdown_audit(self.root, ['source.md'])
        self.assertTrue(any('missing fragment' in error for error in errors))

    def test_reorganization_preserves_exact_prior_navigation(self):
        """A replacement index must retain direct routes, including fragments."""
        before = '[API](../reference.md#contract)'
        self.assertTrue(navigation_preservation_errors(self.root, Path('docs/index.md'), before, '[API](../reference.md)'))
        self.assertEqual(navigation_preservation_errors(self.root, Path('docs/index.md'), before, before + '\n[more](new.md)'), [])

    def test_fenced_examples_are_not_navigation(self):
        """Literal code links/headings do not create false failures or anchors."""
        self.write('source.md', '```md\n[example](missing.md)\n# Hidden\n```\n'
                   '~~~text\n[example](also-missing.md)\n~~~\n# Actual\n')
        errors, _, count = markdown_audit(self.root, ['source.md'])
        self.assertEqual((errors, count), ([], 0))
        self.assertEqual(anchors((self.root / 'source.md').read_text())[0], {'actual'})

    def test_explicit_anchor_collision(self):
        """Duplicate explicit IDs remain distinguishable from repeated headings."""
        _, duplicates = anchors('<a id="same"></a>\n<a id="same"></a>\n')
        self.assertEqual(duplicates, ['same'])

    def test_named_aliases_and_punctuation(self):
        """Keep distinct legacy anchors and deterministic heading punctuation."""
        result, duplicates = anchors('<a id="task-terms"></a>\n<a id="task-png"></a>\n'
                                    '## A: [linked](x.md) heading\n')
        self.assertEqual(duplicates, [])
        self.assertTrue({'task-terms', 'task-png', 'a-linked-heading'} <= result)

    def test_planned_owner_requires_an_explicit_marker(self):
        """Absent future files are valid only when their local role is planned."""
        owners = [{'path': 'crates/future/src/lib.rs', 'state': 'planned'}]
        self.assertEqual(owner_errors(self.root, owners, '`crates/future/src/lib.rs` (planned)'), [])
        self.assertTrue(owner_errors(self.root, owners, '`crates/future/src/lib.rs`'))

    def test_existing_owner_cannot_be_a_missing_plan(self):
        """Do not report a nonexistent module as current implementation."""
        owners = [{'path': 'crates/missing.rs', 'state': 'existing'}]
        self.assertTrue(any('Missing existing' in e for e in owner_errors(self.root, owners, 'crates/missing.rs')))

    def test_existing_owner_requires_a_feature_reference(self):
        """An inventory locator alone is insufficient feature ownership."""
        self.write('crates/real.rs', '// fixture')
        owners = [{'path': 'crates/real.rs', 'state': 'existing'}]
        self.assertTrue(owner_errors(self.root, owners, 'No module named here.'))
        self.assertEqual(owner_errors(self.root, owners, 'crates/real.rs'), [])

    def test_history_allows_only_the_new_documentation_task(self):
        """Keep historical checkmarks/text exact while adding DOC-ORG-01."""
        original = '- [x] **OLD-01** Accepted.\n- [ ] **OLD-02** Pending.'
        snapshot = {'checkbox_count': 2, 'checkbox_lines_sha256': hashlib.sha256(original.encode()).hexdigest()}
        extra = original + '\n- [x] **DOC-ORG-01** Documentation.'
        self.assertEqual(checkbox_history_errors(extra, snapshot, True), [])
        self.assertTrue(checkbox_history_errors(extra.replace('[x] **OLD-01**', '[ ] **OLD-01**'), snapshot, True))
        self.assertTrue(checkbox_history_errors(extra + '\n- [x] **OTHER-01** Extra.', snapshot, True))

    def test_unknown_dependency(self):
        """Reject a prerequisite that cannot resolve to a declared task."""
        self.assertEqual(dependency_errors({'A': []}), [])
        self.assertTrue(any('Unknown prerequisite' in e for e in dependency_errors({'A': ['missing']})))

    def test_dependency_cycle(self):
        """A valid task ID does not make a cyclic graph acceptable."""
        self.assertTrue(any('cycle' in e for e in dependency_errors({'A': ['B'], 'B': ['A']})))

    def test_current_fec_locator_and_stale_negative_case(self):
        """Exercise the exact stale-owner defect while keeping source history separate."""
        self.write(FEC_AUDIT, '// source-audit fixture')
        self.assertEqual(fec_owner_errors(self.root, {'test': FEC_AUDIT}, FEC_AUDIT), [])
        self.assertTrue(fec_owner_errors(self.root, {'test': STALE_FEC_AUDIT}, FEC_AUDIT))
        self.assertTrue(fec_owner_errors(self.root, {'test': FEC_AUDIT}, STALE_FEC_AUDIT))

    def test_all_canonical_return_paths(self):
        """Each feature must return through requirement, architecture and task owners."""
        feature = 'docs/development/features/fec/design.md'
        texts = {
            'docs/architecture.md': '## FEC\n[feature](development/features/fec/design.md)\n',
            'docs/requirements.md': '#### REQ-X\n[feature](development/features/fec/design.md)\n',
            'docs/development/checklist.md': '- [ ] **INT-FEC-01** Work.\n'
                '  Design: [feature](features/fec/design.md)\n',
            feature: '[architecture](../../../architecture.md#fec)\n'
                '[requirement](../../../requirements.md#req-x)\n'
                '[task](../../checklist.md#int-fec-01)\n',
        }
        record = {'proposed_path': feature, 'architecture_references': ['docs/architecture.md#fec'],
                  'requirement_ids': ['REQ-X'], 'checklist_ids': ['INT-FEC-01'],
                  'implementation_and_test_owners': []}
        for path, text in texts.items():
            self.write(path, text)
        self.assertEqual(feature_errors(self.root, record, {'REQ-X': 'req-x'}), [])
        for path in ['docs/architecture.md', 'docs/requirements.md', 'docs/development/checklist.md']:
            with self.subTest(path=path):
                self.write(path, texts[path].replace('features/fec/design.md', 'features/fec/absent.md'))
                self.assertTrue(any('return link' in e for e in feature_errors(self.root, record, {'REQ-X': 'req-x'})))
                self.write(path, texts[path])

    def test_real_checkout(self):
        """Audit actual canonical/functional records as part of existing tool discovery."""
        self.assertEqual(audit(ROOT)['errors'], [])


if __name__ == '__main__':
    unittest.main()
