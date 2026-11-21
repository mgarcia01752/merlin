#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Verify documentation-only discovery, native identity and formatting contracts."""
import importlib.machinery
from pathlib import Path
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch

from api_reference_support import public_classes, public_members, public_operations, sphinx_docstring
from build_api_reference import native_inventory, validate_pages


class ApiReferenceTests(unittest.TestCase):
    """Use metadata-only fixtures; never load captures or run an engine job."""

    def test_discovery_preserves_aliases_without_constructing_objects(self):
        """Expose every declared class and fail explicitly for new export kinds."""
        class Controller:
            def __init__(self):
                raise AssertionError("Documentation must not construct controllers")
        module = SimpleNamespace(Merlin=Controller, Alias=Controller, __version__="test")
        self.assertEqual([name for name, _ in public_classes(module)], ["Alias", "Merlin"])
        module.unhandled_function = lambda: None
        with self.assertRaisesRegex(ValueError, "unhandled_function"):
            public_classes(module)

    def test_member_discovery_does_not_evaluate_getters(self):
        """Read descriptor names statically, including the exposed collection protocol."""
        class Report:
            @property
            def fec(self):
                raise AssertionError("Getter evaluated during discovery")
            def __iter__(self):
                raise AssertionError("Report iterated during discovery")
        self.assertEqual(public_members(Report), ["__iter__", "fec"])

    def test_markup_conversion_preserves_semantics_and_original_lines(self):
        """Format source-owned text and inline code without altering native docs."""
        lines = ["Run explicitly.", "# Arguments", "* `control` - Shared token.",
                 "# Returns", "Owned partial results."]
        result = sphinx_docstring(lines)
        self.assertIn(".. rubric:: Parameters", result)
        self.assertIn("* ``control`` - Shared token.", result)
        self.assertIn("# Arguments", lines)
        self.assertIn("Owned partial results.", result)
        self.assertEqual(sphinx_docstring(["``existing``"]), ["``existing``"])

    def test_exact_operation_inventory_does_not_construct_or_read_properties(self):
        """Preserve constructor defaults and descriptor kinds without evaluation."""
        class Value:
            def __new__(cls, count=7):
                raise AssertionError('Constructor evaluated')
            @property
            def quantity(self):
                raise AssertionError('Getter evaluated')
            def run(self, *, control=None):
                raise AssertionError('Operation evaluated')
            ZERO = 0
        operations = {item['name']: item for item in public_operations(Value)}
        self.assertEqual(operations['__init__']['signature'], '(count=7)')
        self.assertEqual(operations['run']['signature'], '(self, *, control=None)')
        self.assertEqual(operations['quantity']['kind'], 'property')
        self.assertEqual(operations['ZERO']['kind'], 'constant')

    def test_native_identity_and_version_are_required(self):
        """Reject a pure Python replacement and a mismatched installed version."""
        module = SimpleNamespace(__file__="fake.py", __version__="1.0.0")
        with self.assertRaisesRegex(ValueError, "real native"):
            native_inventory(module, "1.0.0")
        module.__file__ = "fake" + importlib.machinery.EXTENSION_SUFFIXES[0]
        with self.assertRaisesRegex(ValueError, "version differs"):
            native_inventory(module, "2.0.0")

    def test_private_extension_is_selected_before_the_legacy_import_shim(self):
        """A real package's compatibility .py module is never mistaken for native."""
        native = SimpleNamespace(__file__="fake" + importlib.machinery.EXTENSION_SUFFIXES[0],
                                 __version__="1.0.0")
        wrapper = SimpleNamespace(__version__="1.0.0", _native=native,
                                  merlin=SimpleNamespace(__file__="merlin.py"))
        self.assertEqual(native_inventory(wrapper, "1.0.0")["native_artifact"], str(Path(native.__file__).resolve()))

    def test_maturin_wrapper_uses_native_exports_without_documenting_wrapper_module(self):
        """Accept the wheel's normal package wrapper and enumerate only native classes."""
        class Controller:
            pass
        native = SimpleNamespace(__file__="fake" + importlib.machinery.EXTENSION_SUFFIXES[0],
                                 Merlin=Controller)
        wrapper = SimpleNamespace(__file__="__init__.py", __version__="1.0.0", merlin=native)
        inventory = native_inventory(wrapper, "1.0.0")
        self.assertEqual([item["name"] for item in inventory["classes"]], ["Merlin"])
        self.assertFalse(inventory["classes"][0]["constructible"])

    def test_coverage_check_rejects_omitted_members_and_hidden_arguments(self):
        """A successfully rendered page must still contain every actual API anchor."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "python").mkdir()
            page = root / "python/Merlin.html"
            page.write_text('<h1>Merlin</h1>')
            inventory = {"classes": [{"name": "Merlin", "members": ["run"]}]}
            with patch("build_api_reference.OUTPUT", root):
                with self.assertRaisesRegex(ValueError, "Missing generated API member"):
                    validate_pages(inventory)
                page.write_text('<a id="merlin.Merlin.run">Interpreter token</a>')
                with self.assertRaisesRegex(ValueError, "Hidden interpreter"):
                    validate_pages(inventory)

    def test_alias_links_to_verified_canonical_members(self):
        """Keep compatibility names discoverable without losing member coverage."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            (root / "python").mkdir()
            (root / "python/Merlin.html").write_text('<a id="merlin.Merlin"></a>')
            canonical = root / "python/Simulation.html"
            canonical.write_text('<a id="merlin.Simulation.run"></a>')
            for name in ("search.html", "searchindex.js", "genindex.html"):
                (root / "python" / name).write_text("")
            inventory = {"classes": [
                {"name": "Merlin", "canonical_name": "Simulation", "members": ["run"]},
                {"name": "Simulation", "canonical_name": "Simulation", "members": ["run"]}]}
            with patch("build_api_reference.OUTPUT", root):
                validate_pages(inventory)
                canonical.write_text("")
                with self.assertRaisesRegex(ValueError, "Missing generated API member"):
                    validate_pages(inventory)


if __name__ == "__main__":
    unittest.main()
