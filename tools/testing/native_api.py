#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Validate an installed wheel against fresh separate Rust API reference processes."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import time
import venv

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'tools/python'))
from smoke_wheel import inspect_wheel

REFERENCES = (
    ('PROJECT', 'pnm-input', 'run_project', ()),
    ('SAVED_FULL_LOOP', 'pnm-input', 'run_saved_full_loop', ()),
    ('REFERENCE', 'docsis-ofdma', 'upstream_loop', ()),
    ('RESOURCE_GRID', 'docsis-ofdma', 'upstream_resource_grid', ()),
    ('RECOVERY', 'docsis-ofdma', 'upstream_recovery_views', ()),
    ('CONFIGURED_RESPONSE', 'docsis-ofdma', 'upstream_configured_response', ()),
    ('RECEIVER_ESTIMATES', 'docsis-ofdma', 'upstream_receiver_estimates', ()),
    ('SOFT_OBSERVATIONS', 'docsis-ofdma', 'upstream_soft_observations', ()),
    ('QAM_VIEWS', 'docsis-ofdma', 'upstream_qam_views', ()),
    ('MEASUREMENTS', 'docsis-ofdma', 'upstream_measurements', ()),
    ('DOCUMENT', 'docsis-ofdma', 'upstream_document', ()),
    ('SYNTHETIC', 'docsis-ofdma', 'upstream_document', ('synthetic',)),
    ('UCD', 'pnm-input', 'inspect_ucd', ()),
    ('RXMER', 'pnm-input', 'inspect_rxmer', ()),
    ('CAPTURES', 'pnm-input', 'upstream_captures', ()),
    ('CAPTURES25', 'pnm-input', 'upstream_captures', ('25',)),
)

# Focused optional and required control references are mandatory in complete acceptance.
DOWNSTREAM_REFERENCES = (
    ('MERLIN_SOURCE_FILE_INFO_REFERENCE', 'docsis-ofdm-fec', 'api_source_file_info'),
    ('MERLIN_SPECTRUM_BUDGET_REFERENCE', 'docsis-ofdm-fec', 'api_spectrum_average_capacity'),
    ('MERLIN_CALIBRATED_SEQUENCE_REFERENCE', 'docsis-ofdm-fec', 'api_calibrated_sequence'),
    ('MERLIN_PROFILE_PERFORMANCE_REFERENCE', 'pnm-input', 'profile_performance'),
    ('MERLIN_MEASURED_INPUT_VIEW_REFERENCE', 'pnm-input', 'measured_input_views'),
    ('MERLIN_RETAINED_FOOTPRINT_REFERENCE', 'docsis-ofdma', 'upstream_retained_footprint'),
    ('MERLIN_BOUNDED_SESSION_REFERENCE', 'pnm-input', 'bounded_session_outputs'),
    ('MERLIN_INPUT_RETAINED_REFERENCE', 'pnm-input', 'input_retained_footprint'),
    ('MERLIN_BOUNDED_ARCHIVE_REFERENCE', 'pnm-input', 'bounded_receiver_archive'),
    ('MERLIN_DOWNSTREAM_RETAINED_REFERENCE', 'pnm-input', 'downstream_retained_footprint'),
    ('MERLIN_CONTROL_FOOTPRINT_REFERENCE', 'ofdm-fec-cli', 'control_footprints'),
)


def passed_cases(log):
    """Read exact unittest identities only after a wholly successful native run.

    Args:
        log: Actual verbose unittest output, including its terminal summary.
    Returns:
        Named source-file/class/method records for every passed case.
    Raises:
        ValueError for missing/count-mismatched, failed or skipped cases.
    """
    # Runtime identifier: test_run (test_api.Suite); capture method/module/class.
    found = re.findall(r'^(test_\w+) \((test_\w+)\.(\w+)\)', log, re.MULTILINE)
    # Whole summary, e.g. Ran 48 tests in 29.254s; no inferred case count.
    summary = re.search(r'^Ran (\d+) tests in [\d.]+s$', log, re.MULTILINE)
    if summary is None or int(summary[1]) != len(found) or not log.endswith('\nOK\n'):
        raise ValueError('Native run is incomplete, failed, skipped or count-mismatched')
    cases = [{'test_file': f'bindings/python/tests/{module}.py',
              'qualified_test_case': cls + '.' + method} for method, module, cls in found]
    if len({(case['test_file'], case['qualified_test_case']) for case in cases}) != len(cases):
        raise ValueError('Native run duplicates an acceptance case')
    return cases


def run(command, directory, environment, label, report, cwd=ROOT):
    """Run one bounded reference/install/test command and retain failure evidence.

    Args:
        command: Exact argv, never shell text.
        directory: New ignored output directory owned by this invocation.
        environment: Explicit child environment; no system installation is used.
        label: Unique output stem.
        report: Mutable list receiving actual command/status/time observations.
        cwd: Execution directory; installed tests run outside the source package.
    Returns:
        Stdout path; both streams and timings survive failure.
    Raises:
        Subprocess errors after recording the observed command exit.
    """
    started = time.perf_counter()
    entry = {'command': [str(value) for value in command], 'working_directory': str(cwd),
             'exit_code': None, 'label': label}
    output = directory / (label + '.stdout')
    try:
        with output.open('xb') as stdout, (directory / (label + '.stderr')).open('xb') as stderr:
            result = subprocess.run(command, cwd=cwd, env=environment, stdout=stdout,
                                    stderr=stderr, timeout=900)
        entry['exit_code'] = result.returncode
        result.check_returncode()
        return output
    finally:
        entry['elapsed_seconds'] = time.perf_counter() - started
        report.append(entry)
        print(f"Native API {label}: {entry['elapsed_seconds']:.2f}s (exit {entry['exit_code']})", flush=True)


def main():
    """Build fresh reference results, install one local wheel and run native cases.

    CLI --wheel-dir selects the already built ABI-matching native wheel;
    --output must be a new ignored local directory. Nothing is overwritten or
    published. Development Python orchestrates processes; all science stays Rust.
    Raises validation/subprocess errors, preserving all completed evidence.
    """
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--wheel-dir', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    if not (sys.platform.startswith('linux') or sys.platform == 'win32'):
        parser.error('Native API validation supports Linux/Windows hosts only')
    output = args.output.absolute()
    if not any(output.resolve().is_relative_to(ROOT / name) for name in ('local', 'docs/development/local')):
        parser.error('--output must be an ignored project-local directory')
    if any(path.is_symlink() for path in [output, *output.parents] if path.is_relative_to(ROOT)):
        parser.error('Refuse redirected native-test output')
    wheels = list(args.wheel_dir.glob('*.whl'))
    if len(wheels) != 1:
        parser.error('--wheel-dir must contain exactly one newly built wheel')
    wheel = wheels[0].resolve()
    platform = 'win_amd64' if sys.platform == 'win32' else 'linux_x86_64'
    abi = f'cp{sys.version_info.major}{sys.version_info.minor}'
    inspect_wheel(wheels[0], (ROOT / 'VERSION').read_text().strip(), abi, platform)
    output.mkdir(parents=True, exist_ok=False)
    started = time.perf_counter()
    report = {'status': 'failed', 'commands': [], 'wheel': wheel.name,
              'wheel_sha256': hashlib.sha256(wheel.read_bytes()).hexdigest(),
              'host': sys.platform, 'python': sys.version,
              'scope': 'Installed native MERLIN/direct Rust process parity; no external interoperability claim.'}
    environment = os.environ.copy()
    environment.pop('PYTHONPATH', None)
    environment.pop('PYTHONHOME', None)
    environment['PYTHONNOUSERSITE'] = '1'
    cargo = ['bash', str(ROOT / 'scripts/cargo-local.sh')]
    if sys.platform == 'win32':
        cargo = ['cargo', '+1.85.1']
    try:
        for key, package, example, arguments in REFERENCES:
            command = [*cargo, 'run', '-p', package, '--example', example, '--locked']
            if arguments:
                command += ['--', *arguments]
            path = run(command, output, environment, key.lower(), report['commands'])
            json.loads(path.read_text())
            environment['MERLIN_UPSTREAM_RUST_' + key + '_JSON'] = str(path)
        generated = output / 'synthetic-rxmer'
        generated.mkdir()
        for name in ('us-25khz', 'us-50khz'):
            reference = run([*cargo, 'run', '-p', 'pnm-input', '--example',
                'generate_upstream_rxmer', '--locked', '--',
                str(ROOT / 'examples/measurements/upstream/synthetic-rxmer' / (name + '.json')),
                str(generated / name)], output, environment, 'synthetic-rxmer-' + name,
                report['commands'])
            json.loads(reference.read_text())
        environment['MERLIN_SYNTHETIC_RXMER_RUST_OUTPUT'] = str(generated)
        for key, package, example in DOWNSTREAM_REFERENCES:
            command = [*cargo, 'run', '-p', package, '--example', example, '--locked']
            if package == 'docsis-ofdm-fec':
                command += ['--features', 'high-level-api']
            path = run(command, output, environment, example, report['commands'])
            json.loads(path.read_text())
            environment[key] = str(path)
        transmitter = run([*cargo, 'run', '-p', 'docsis-ofdm-fec', '--example',
                           'api_transmitter_preview', '--features', 'high-level-api', '--locked'],
                          output, environment, 'transmitter', report['commands'])
        json.loads(transmitter.read_text())
        environment['MERLIN_TRANSMITTER_PARITY'] = str(transmitter)
        full_loop = run([*cargo, 'run', '-p', 'docsis-ofdm-fec', '--example',
                         'api_full_loop', '--features', 'high-level-api', '--locked'],
                        output, environment, 'full-loop', report['commands'])
        json.loads(full_loop.read_text())
        environment['MERLIN_FULL_LOOP_PARITY'] = str(full_loop)
        policies = output / 'policies'
        run([*cargo, 'run', '-p', 'pnm-input', '--example', 'run_rxmer_selection_parity',
             '--locked', '--', str(policies)], output, environment, 'policies', report['commands'])
        environment['MERLIN_API_PARITY'] = str(policies)
        workbench = run([*cargo, 'run', '-p', 'pnm-input', '--example', 'run_multi_rxmer_cmce_workbench',
                         '--locked', '--', str(ROOT / 'examples/workbench/downstream/multi-rxmer-2/workbench.merlin.json')],
                        output, environment, 'workbench', report['commands'])
        json.loads(workbench.read_text())
        environment['MERLIN_WB_PARITY'] = str(workbench)
        designer = output / 'designer.json'
        environment['MERLIN_DESIGNER_PARITY_OUT'] = str(designer)
        run([*cargo, 'test', '-p', 'pnm-input', '--test', 'designer_workbench', '--locked',
             'designer_run_matches_materialized_capture_and_exports_python_parity', '--', '--nocapture'],
            output, environment, 'designer', report['commands'])
        environment.pop('MERLIN_DESIGNER_PARITY_OUT')
        environment['MERLIN_DESIGNER_PARITY'] = str(designer)
        options = output / 'accepted-options.json'
        environment['MERLIN_OPTIONS_RUST_REFERENCE_JSON'] = str(options)
        run([*cargo, 'test', '-p', 'docsis-ofdma', '--test', 'api_options', '--locked'],
            output, environment, 'accepted-options', report['commands'])
        json.loads(options.read_text())
        edited_project = output / 'edited-configuration-projects.json'
        environment['MERLIN_PROJECT_EDITS_RUST_REFERENCE_JSON'] = str(edited_project)
        run([*cargo, 'test', '-p', 'pnm-input', '--test', 'project_configuration_edits', '--locked'],
            output, environment, 'edited-configuration-projects', report['commands'])
        json.loads(edited_project.read_text())
        workbench_examples = output / 'directional-workbench-examples.json'
        environment['MERLIN_WORKBENCH_EXAMPLES_RUST_REFERENCE_JSON'] = str(workbench_examples)
        run([*cargo, 'test', '-p', 'pnm-input', '--test', 'directional_workbench_examples', '--locked'],
            output, environment, 'directional-workbench-examples', report['commands'])
        json.loads(workbench_examples.read_text())
        venv.EnvBuilder(with_pip=True).create(output / 'installed')
        executable = output / ('installed/Scripts/python.exe' if sys.platform == 'win32' else 'installed/bin/python')
        run([str(executable), '-I', '-B', '-m', 'pip', 'install', '--no-index', '--no-deps', str(wheel)],
            output, environment, 'install', report['commands'], cwd=output)
        tests = run([str(executable), '-I', '-B', '-m', 'unittest', 'discover', '-s',
                     str(ROOT / 'bindings/python/tests'), '-v'], output, environment,
                    'native-tests', report['commands'], cwd=output)
        # unittest writes verbose identities/summary to stderr, not stdout.
        report['cases'] = passed_cases(tests.with_suffix('.stderr').read_text())
        report['passed'] = len(report['cases'])
        run([str(executable), '-I', '-B', str(ROOT / 'tools/validation/native_api_inventory.py'),
             '--output', str(output / 'native-exports.json')], output, environment,
            'native-inventory', report['commands'], cwd=output)
        report['reference_environment'] = {key: value for key, value in environment.items()
            if key.startswith('MERLIN_UPSTREAM_RUST_') or key in {row[0] for row in DOWNSTREAM_REFERENCES} or key in
            ('MERLIN_API_PARITY', 'MERLIN_WB_PARITY', 'MERLIN_DESIGNER_PARITY', 'MERLIN_TRANSMITTER_PARITY', 'MERLIN_FULL_LOOP_PARITY', 'MERLIN_OPTIONS_RUST_REFERENCE_JSON', 'MERLIN_PROJECT_EDITS_RUST_REFERENCE_JSON', 'MERLIN_SYNTHETIC_RXMER_RUST_OUTPUT', 'MERLIN_WORKBENCH_EXAMPLES_RUST_REFERENCE_JSON')}
        report['status'] = 'passed'
        print(f"Installed native API: {report['passed']} cases pass", flush=True)
    finally:
        report['elapsed_seconds'] = time.perf_counter() - started
        (output / 'results.json').write_text(json.dumps(report, indent=2) + '\n')


if __name__ == '__main__':
    main()
