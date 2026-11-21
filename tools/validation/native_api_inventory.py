#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Discover actual installed native PHY exports without executing scientific work."""
import argparse
import hashlib
import json
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'tools/python'))
from build_api_reference import native_inventory


def verify_legacy(current, legacy):
    """Preserve every recorded root export, constructor default and member kind.

    Args:
        current: Fresh actual installed metadata.
        legacy: Previously observed native root metadata, including member details.
    Returns:
        Number of preserved class exports; new additive members remain permitted.
    Raises:
        ValueError for any lost export, alias, default, descriptor or signature.
    """
    classes = {item['name']: item for item in current['classes']}
    for old in legacy['classes']:
        new = classes.get(old['name'])
        keys = ('canonical_name', 'signature', 'constructible')
        if new is None or any(new[key] != old[key] for key in keys):
            raise ValueError(f"Legacy native export/constructor changed: {old['name']}")
        members = {item['name']: item for item in new['operations']}
        for name in old['members']:
            item = members.get(name)
            detail = old['details'][name]
            if (item is None or item['signature'] != detail['signature']
                    or (item['kind'] == 'property') != detail['property']):
                raise ValueError(f"Legacy native member changed: {old['name']}.{name}")
    return len(legacy['classes'])


def main():
    """Write real installed class, alias, descriptor and signature metadata.

    CLI --output selects an evidence file. The interpreter must import the
    extension from its own environment; a source fallback cannot pass this gate.
    Raises import/identity errors before recording any native acceptance claim.
    """
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--legacy-inventory', type=Path)
    args = parser.parse_args()
    import merlin
    inventory = native_inventory(merlin, (ROOT / 'VERSION').read_text().strip())
    artifact = Path(inventory['native_artifact'])
    if not artifact.is_relative_to(Path(sys.prefix).resolve()):
        raise ValueError('Native inventory requires an installed environment, not source fallback')
    if set(inventory['namespaces']) != {'downstream', 'upstream', 'common'}:
        raise ValueError('Native inventory requires all three real namespaces')
    inventory['native_sha256'] = hashlib.sha256(artifact.read_bytes()).hexdigest()
    inventory['scope'] = 'Actual installed native metadata; scientific execution is recorded separately.'
    if args.legacy_inventory:
        original = args.legacy_inventory.read_bytes()
        count = verify_legacy(inventory, json.loads(original))
        inventory['legacy_compatibility'] = {
            'inventory_sha256': hashlib.sha256(original).hexdigest(),
            'preserved_class_exports': count,
            'scope': 'Every old root constructor/alias/member kind/signature preserved; additive members allowed.'}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(inventory, indent=2) + '\n', encoding='utf-8')
    count = sum(len(item['operations']) for items in inventory['namespaces'].values() for item in items)
    print(f'Installed native inventory: {count} constructors/methods/properties/constants')


if __name__ == '__main__':
    main()
