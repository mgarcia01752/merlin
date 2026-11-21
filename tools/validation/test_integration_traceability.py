#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Reject missing integration references using independent synthetic documents."""
import json
from pathlib import Path
import tempfile
import unittest

from integration_traceability import ARCHITECTURE, CHECKLIST, INVENTORY, REQUIREMENTS, ROOT, audit


class IntegrationTraceabilityTests(unittest.TestCase):
    """Exercise the reported empty-reference defect and each navigation direction."""

    def setUp(self):
        """Create a temporary, minimal contract with exactly one integration task."""
        self.directory = tempfile.TemporaryDirectory()
        self.addCleanup(self.directory.cleanup)
        self.root = Path(self.directory.name)
        documents = {
            CHECKLIST: '<a id="int-pnm-01"></a>\n- [ ] **INT-PNM-01** Decode inputs.\n'
                       '  Requirements: [REQ-US-PNM-01](../requirements.md#req-us-pnm-01).\n'
                       '  Architecture: [owner](../architecture.md#capture-owner).\n',
            ARCHITECTURE: '## Capture owner\n[INT-PNM-01](development/checklist.md#int-pnm-01)\n',
            REQUIREMENTS: '#### REQ-US-PNM-01\nPreserve bytes.\n'
                          '[INT-PNM-01](development/checklist.md#int-pnm-01)\n',
            INVENTORY: json.dumps({'integration_tasks': [{'id': 'INT-PNM-01',
                'requirements': ['REQ-US-PNM-01'], 'architecture': 'capture-owner'}]}),
        }
        for path, text in documents.items():
            destination = self.root / path
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_text(text, encoding="utf-8")

    def _replace(self, path, old, new):
        """Change only the requested synthetic contract fragment."""
        destination = self.root / path
        destination.write_text(destination.read_text(encoding="utf-8").replace(old, new), encoding="utf-8")

    def test_valid_contract_and_real_repository(self):
        """Accept complete synthetic navigation and audit the current real checkout."""
        self.assertEqual(audit(self.root), [])
        self.assertEqual(audit(ROOT), [])

    def test_empty_requirement_field_and_inventory(self):
        """Reject the original Requirements: . defect even when both owners omit IDs."""
        self._replace(CHECKLIST, '[REQ-US-PNM-01](../requirements.md#req-us-pnm-01)', '')
        self._replace(INVENTORY, '["REQ-US-PNM-01"]', '[]')
        self.assertTrue(any('empty requirement' in message for message in audit(self.root)))

    def test_unknown_or_wrong_fragment(self):
        """Reject a label whose link fragment does not resolve to its definition."""
        self._replace(CHECKLIST, '#req-us-pnm-01)', '#req-us-pnm-99)')
        self.assertTrue(any('invalid requirement target' in message for message in audit(self.root)))

    def test_missing_requirement_backlink(self):
        """Reject a valid forward reference with no requirement-to-task return path."""
        self._replace(REQUIREMENTS, 'development/checklist.md#int-pnm-01', 'development/checklist.md#int-pnm-02')
        self.assertTrue(any('lacks task backlink' in message for message in audit(self.root)))

    def test_missing_architecture_backlink(self):
        """Reject a valid architecture owner that cannot navigate back to its task."""
        self._replace(ARCHITECTURE, '#int-pnm-01', '#int-pnm-02')
        self.assertTrue(any('architecture lacks task backlink' in message for message in audit(self.root)))

    def test_wrong_architecture_forward_reference(self):
        """Reject a checklist owner fragment that differs from the declared owner."""
        self._replace(CHECKLIST, '#capture-owner', '#other-owner')
        self.assertTrue(any('missing architecture owner/link' in message for message in audit(self.root)))

    def test_task_inventory_and_requirement_mismatch(self):
        """Reject missing tasks and requirements inconsistent with the inventory."""
        self._replace(INVENTORY, 'REQ-US-PNM-01', 'REQ-US-PNM-02')
        self.assertTrue(any('requirements differ' in message for message in audit(self.root)))
        self._replace(INVENTORY, 'INT-PNM-01', 'INT-PNM-02')
        self.assertTrue(any('task IDs differ' in message for message in audit(self.root)))

    def test_requirement_cannot_borrow_the_next_task_references(self):
        """Reject a missing field even if a later downstream task has valid links."""
        self._replace(CHECKLIST, '  Requirements: [REQ-US-PNM-01](../requirements.md#req-us-pnm-01).', '')
        with (self.root / CHECKLIST).open('a', encoding='utf-8') as output:
            output.write('- [x] **INT-01** Existing downstream stage.\n'
                         '  Requirements: [REQ-US-PNM-01](../requirements.md#req-us-pnm-01).\n')
        self.assertTrue(any('empty requirement' in message for message in audit(self.root)))


if __name__ == "__main__":
    unittest.main()
