#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: GPL-3.0-only
#
"""Run actual Qt-adapter project exports through an isolated installed native wheel."""
import argparse
import hashlib
import json
from pathlib import Path
import sys

import merlin
import merlin._native as native
from merlin import common, downstream


def main():
    """Compare real installed Rust execution with separate Qt/Rust owner references.

    Arguments are ignored-local reference/output directories and the installed
    module's expected archive hash. Input references are never changed; output
    must be a new directory. Python performs no scientific calculations.
    """
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--references', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--native-sha256', required=True)
    args = parser.parse_args()
    artifact = Path(native.__file__).resolve()
    assert artifact.is_relative_to(Path(sys.prefix).resolve())
    assert hashlib.sha256(artifact.read_bytes()).hexdigest() == args.native_sha256
    assert merlin.Simulation is downstream.Simulation
    sources = sorted(args.references.glob('*.json'))
    assert sources, 'No actual project reference records supplied'
    args.output.mkdir(parents=True, exist_ok=False)
    cases = []
    for source in sources:
        record = json.loads(source.read_text())
        wire = json.loads(record['project'])
        expected = record['reference']
        project = common.Project.from_bytes(record['project'].encode())
        assert project.direction == 'upstream'
        session = project.upstream_simulation()
        saved_policy = wire['contents']['receiver_observations']
        actual_policy = session.receiver_observation_policy().info()
        assert actual_policy['maximum_frames'] == saved_policy['maximum_frames']
        assert actual_policy['maximum_scalar_slots'] == saved_policy['maximum_scalar_slots']
        assert actual_policy.get('soft_metrics') == saved_policy['soft']
        del project
        actual = json.loads(session.run().to_json())
        for key in ['configuration', 'options', 'planned', 'status',
                    'generated_source_bits', 'compared_source_bits', 'bit_errors',
                    'ber', 'reference_recovered', 'not_started_grants',
                    'source_configuration_original', 'source_configuration_sha256',
                    'measurement_provenance_json']:
            assert actual[key] == expected[key], (source.name, key)
        assert [grant['words'] for grant in actual['grants']] == [
            grant['words'] for grant in expected['grants']]
        assert actual['options'] == wire['contents']['options']
        (args.output / source.name).write_text(json.dumps(actual) + '\n')
        cases.append({'case': source.name, 'source_sha256': hashlib.sha256(source.read_bytes()).hexdigest(),
                      'status': actual['status'], 'bits': actual['compared_source_bits'],
                      'bit_errors': actual['bit_errors'], 'recovered': actual['reference_recovered']})
    (args.output / 'results.json').write_text(json.dumps({
        'exit_code': 0, 'native_sha256': args.native_sha256, 'passed': len(cases), 'cases': cases,
        'scope': 'Qt adapter exports through installed native Rust; no external interoperability claim.'
    }, indent=2) + '\n')
    print(f'Installed native project parity: {len(cases)} actual cases pass')


if __name__ == '__main__':
    main()
