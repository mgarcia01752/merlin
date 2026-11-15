#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Inspect native public API metadata and adapt Rust docstrings for Sphinx."""
import inspect
import re

# Convert Rust inline code to reStructuredText literals, matching a single
# backtick-delimited nonempty span anywhere in a doc line. For example `path`
# becomes ``path``. Group 1 is its contents; existing double backticks are excluded.
INLINE_CODE = re.compile(r"(?<!`)`([^`]+)`(?!`)")
SECTIONS = {"# Arguments": "Parameters", "# Returns": "Returns",
            "# Errors": "Raises", "# Panics": "Panics"}


def public_classes(module):
    """Read exported class identities without constructing instances or running jobs.

    Args:
        module: Imported native extension, exposing its real public namespace.
    Returns:
        Sorted (export name, class) pairs; aliases remain visible as exports.
    Raises:
        ValueError if a public export cannot be documented as a class.
    """
    classes = []
    for name, value in vars(module).items():
        if name.startswith("_"):
            continue
        if not inspect.isclass(value):
            raise ValueError(f"Undocumented public export kind: {name}")
        classes.append((name, value))
    return sorted(classes)


def public_members(value):
    """Read explicit class descriptors without evaluating instance getters.

    Args:
        value: Native API class.
    Returns:
        Sorted declared public names plus supported collection protocol methods.
    """
    protocols = {"__iter__", "__len__", "__getitem__"}
    return sorted(name for name in vars(value)
                  if not name.startswith("_") or name in protocols)


def sphinx_docstring(lines):
    """Convert Rust section/inline markup without inventing parameter semantics.

    Args:
        lines: Docstring split into individual lines.
    Returns:
        New reStructuredText lines; the source/native docstring is not mutated.
    """
    result = []
    for line in lines:
        if line.strip() in SECTIONS:
            result.extend(["", f".. rubric:: {SECTIONS[line.strip()]}", ""])
        else:
            result.append(INLINE_CODE.sub(r"``\1``", line))
    return result
