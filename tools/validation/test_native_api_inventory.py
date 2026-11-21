#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Protect legacy native metadata against permissive namespace acceptance."""
from copy import deepcopy
from pathlib import Path
from types import SimpleNamespace
import sys
import tempfile
import unittest
from unittest.mock import patch

from native_api_inventory import main, verify_legacy


class LegacyNativeInventoryTests(unittest.TestCase):
    def test_native_inventory_rejects_python_stubs_before_writing_evidence(self):
        """A .so-looking path cannot replace a real extension import loader."""
        with tempfile.TemporaryDirectory() as directory:
            output = Path(directory) / 'inventory.json'
            fake = SimpleNamespace(_native=SimpleNamespace(
                __file__='pretend.so', __spec__=SimpleNamespace(loader=object())))
            with patch.dict(sys.modules, {'merlin': fake}), patch.object(sys, 'argv',
                    ['native_api_inventory.py', '--output', str(output)]):
                with self.assertRaisesRegex(ValueError, 'real extension loader'):
                    main()
            self.assertFalse(output.exists())

    def test_all_old_defaults_aliases_and_descriptors_survive_additive_changes(self):
        """Extra members are valid; changed signatures and property kinds are not."""
        old = {'name': 'Alias', 'canonical_name': 'Value', 'signature': '(count=7)',
               'constructible': True, 'members': ['read'],
               'details': {'read': {'signature': '(self, /)', 'property': False}}}
        current = {key: value for key, value in old.items() if key not in ('details', 'members')}
        current['operations'] = [{'name': 'read', 'signature': '(self, /)', 'kind': 'method'},
                                 {'name': 'added', 'signature': None, 'kind': 'property'}]
        self.assertEqual(verify_legacy({'classes': [current]}, {'classes': [old]}), 1)
        for field, value in [('signature', '(count=8)'), ('canonical_name', 'Other')]:
            changed = deepcopy(current)
            changed[field] = value
            with self.assertRaises(ValueError):
                verify_legacy({'classes': [changed]}, {'classes': [old]})
        for field, value in [('signature', '(self, /, input)'), ('kind', 'property')]:
            changed = deepcopy(current)
            changed['operations'][0][field] = value
            with self.assertRaises(ValueError):
                verify_legacy({'classes': [changed]}, {'classes': [old]})
        with self.assertRaises(ValueError):
            verify_legacy({'classes': []}, {'classes': [old]})


if __name__ == '__main__':
    unittest.main()
