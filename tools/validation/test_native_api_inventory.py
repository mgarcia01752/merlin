#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Protect legacy native metadata against permissive namespace acceptance."""
from copy import deepcopy
import unittest

from native_api_inventory import verify_legacy


class LegacyNativeInventoryTests(unittest.TestCase):
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
