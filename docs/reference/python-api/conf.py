#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Strict native-extension API reference configuration; no mocked imports."""
import os
from pathlib import Path
import sys

ROOT = Path(os.environ["MERLIN_DOC_ROOT"])
sys.path.insert(0, str(ROOT / "tools/python"))
from api_reference_support import sphinx_docstring

project = "MERLIN Python API"
author = "MERLIN contributors"
copyright = "2026, MERLIN contributors"
release = (ROOT / "VERSION").read_text().strip()
extensions = ["sphinx.ext.autodoc"]
html_theme = "alabaster"
html_title = f"MERLIN {release} Python API"
html_sidebars = {"**": ["about.html", "navigation.html", "searchbox.html"]}
autodoc_member_order = "alphabetical"
autodoc_class_signature = "mixed"
autodoc_typehints = "none"
add_module_names = False
html_show_sourcelink = False


def format_docstring(app, what, name, obj, options, lines):
    """Render source-owned Rust docstrings as Sphinx sections and literals.

    Sphinx supplies the documented object metadata and mutable line list.
    Only presentation markup changes; scientific text and signatures are retained.
    """
    lines[:] = sphinx_docstring(lines)


def setup(app):
    """Register the presentation-only autodoc hook on the active Sphinx build."""
    app.connect("autodoc-process-docstring", format_docstring)
