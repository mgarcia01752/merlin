#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Build exact local nextest exclusions; full/hosted tests remain unchanged."""
import json
from pathlib import Path
import re
import sys

CATALOG = Path(__file__).with_name('quick-tests.json')


def exclusions():
    """Read the reviewed bounded manifest and validate exact test identities.

    Return package/binary/test/reason tuples. Reject malformed or duplicate
    identities before emitting a filter; no captures or network are accessed.
    """
    if CATALOG.stat().st_size > 65536:
        raise ValueError('Quick-test catalog exceeds 64 KiB')
    document = json.loads(CATALOG.read_text(encoding='utf-8'))
    if document['schema_version'] != 1:
        raise ValueError('Unsupported quick-test catalog schema')
    result = []
    seen = set()
    for group in document['groups']:
        for name in group['tests']:
            identity = (group['package'], group['binary'], name)
            if identity in seen or not all(re.fullmatch(r'[A-Za-z0-9_:-]+', item) for item in identity):
                raise ValueError('Invalid or duplicate exact test identity')
            seen.add(identity)
            result.append((*identity, group['reason']))
    if not 1 <= len(result) <= 128:
        raise ValueError('Quick-test catalog must contain 1–128 exclusions')
    return result


def filter_expression(cases):
    """Return a nextest filter excluding only the supplied exact identities.

    cases contains validated package/binary/test/reason tuples. The expression
    includes all other tests, so newly added tests run by default.
    """
    return 'not (' + ' or '.join(
        f'(package(={package}) and binary(={binary}) and test(={name}))'
        for package, binary, name, _ in cases) + ')'


def main():
    """Print the selection to stdout and explicit omission report to stderr.

    No arguments are accepted. Invalid catalogs stop the launcher with a nonzero
    status before tests run; generated selection never evaluates external code.
    """
    if len(sys.argv) != 1:
        raise ValueError('quick_tests.py accepts no arguments')
    cases = exclusions()
    print(f'Quick local tests: omit {len(cases)} named sweep cases; full coverage was NOT run.', file=sys.stderr)
    for package, binary, name, reason in cases:
        print(f'  Omit {package}::{binary}::{name}: {reason}', file=sys.stderr)
    print(filter_expression(cases))


if __name__ == '__main__':
    main()
