#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Verify exact sweep exclusions and retained representative Rust cases."""
import json
from pathlib import Path
import re
import tempfile
import unittest
from unittest.mock import patch

import quick_tests

ROOT = Path(__file__).resolve().parents[2]


class QuickSelectionTests(unittest.TestCase):
    """Keep the reviewed omission manifest tied to real tests and bounded input."""

    def test_catalog_names_real_cases_and_preserves_representatives(self):
        """Every exclusion names a real case; representative boundaries remain."""
        cases = quick_tests.exclusions()
        self.assertEqual(len(cases), 21)
        omitted = {(package, binary, name) for package, binary, name, _ in cases}
        for package, binary, name, reason in cases:
            crate = 'cli' if package == 'ofdm-fec-cli' else 'ofdm'
            source = (ROOT / f'crates/{crate}/tests/{binary}.rs').read_text()
            self.assertRegex(source, r'#\[test\]\s*fn ' + re.escape(name) + r'\(')
            self.assertTrue(reason)
        for spacing in ('khz25', 'khz50'):
            for order in (64, 16384):
                self.assertNotIn(('docsis-ofdm-fec', 'uniform_profile_orders',
                                  f'{spacing}_qam{order}_recovers_through_live_plc_ncp_and_waveform'), omitted)
        for width, profile in ((24, 0), (90, 3)):
            self.assertNotIn(('ofdm-fec-cli', 'pnm_import',
                              f'synthetic_{width}mhz_profile_{profile}_loads_through_import_and_channel'), omitted)
        expression = quick_tests.filter_expression(cases)
        self.assertTrue(expression.startswith('not ('))
        self.assertEqual(expression.count('package(='), len(cases))
        self.assertEqual(expression.count('binary(='), len(cases))
        self.assertEqual(expression.count('test(='), len(cases))

    def test_catalog_rejects_duplicate_injected_and_oversized_selection(self):
        """Malformed entries cannot widen the test subset or hide test failures."""
        document = json.loads(quick_tests.CATALOG.read_text())
        with tempfile.TemporaryDirectory() as directory:
            catalog = Path(directory) / 'quick.json'
            with patch.object(quick_tests, 'CATALOG', catalog):
                group = document['groups'][0]
                group['tests'].append(group['tests'][0])
                catalog.write_text(json.dumps(document))
                with self.assertRaisesRegex(ValueError, 'duplicate'):
                    quick_tests.exclusions()
                group['tests'][-1] = 'case) or all('
                catalog.write_text(json.dumps(document))
                with self.assertRaisesRegex(ValueError, 'Invalid'):
                    quick_tests.exclusions()
                catalog.write_text(' ' * 65537)
                with self.assertRaisesRegex(ValueError, '64 KiB'):
                    quick_tests.exclusions()
