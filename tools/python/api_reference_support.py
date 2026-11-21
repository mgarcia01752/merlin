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


def public_operations(value):
    """Inspect exact native signatures and descriptors without running a session.

    Args:
        value: Exported native class; aliases retain their own public paths.
    Returns:
        Constructor, methods, properties and constants with explicit kinds.
        Properties are never evaluated and constants are never constructed.
    Raises:
        ValueError if a callable lacks a discoverable native signature.
    """
    result = []
    members = public_members(value)
    if '__new__' in vars(value):
        members = ['__init__', *members]
    for name in members:
        member = value if name == '__init__' else getattr(value, name)
        if name == '__init__' or callable(member) and not isinstance(member, type(value)):
            kind = 'method'
            if name == '__init__':
                kind = 'constructor'
            try:
                signature = str(inspect.signature(member))
            except (TypeError, ValueError) as cause:
                raise ValueError(f'Native signature unavailable: {value.__name__}.{name}') from cause
        elif inspect.isdatadescriptor(member):
            kind, signature = 'property', None
        else:
            kind, signature = 'constant', None
        result.append({'name': name, 'kind': kind, 'signature': signature})
    return result


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
