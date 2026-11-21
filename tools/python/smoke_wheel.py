#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Install one native release wheel in isolation and execute the real API example."""
import argparse
from email.parser import Parser
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import time
import venv
import zipfile

ROOT = Path(__file__).resolve().parents[2]
EXAMPLE = ROOT / "examples/api/python/run_single_capture.py"
EXAMPLE_TIMEOUT_SECONDS = 360
EXPECTED_TOTAL_CODEWORDS = 82  # Existing single-channel fixture, 1 ms observation.
EXPECTED_CORRECTED_CODEWORDS = 0
EXPECTED_UNCORRECTABLE_CODEWORDS = 0


def inspect_wheel(wheel, version, python_tag, platform_tag):
    """Require the exact version/ABI/platform wheel and its native/legal payload.

    Args:
        wheel: One newly built native wheel, not a source archive.
        version: Canonical checkout version.
        python_tag: Expected CPython ABI, such as cp310.
        platform_tag: linux_x86_64 or win_amd64 for the native release host.
    Returns:
        Native extension archive member name.
    Raises:
        ValueError for wrong identity, ABI, native format or missing notices.
    """
    expected = f"merlin_ofdm-{version}-{python_tag}-{python_tag}-{platform_tag}.whl"
    if wheel.name != expected or wheel.is_symlink():
        raise ValueError(f"Expected native wheel {expected}, found {wheel.name}")
    with zipfile.ZipFile(wheel) as archive:
        names = archive.namelist()
        metadata = [name for name in names if name.endswith('.dist-info/METADATA')]
        records = [name for name in names if name.endswith('.dist-info/WHEEL')]
        if len(metadata) != 1 or len(records) != 1:
            raise ValueError("Wheel must contain one distribution metadata record")
        headers = Parser().parsestr(archive.read(metadata[0]).decode('utf-8'))
        if headers['Name'] != 'merlin-ofdm' or headers['Version'] != version:
            raise ValueError("Wheel metadata does not match canonical package identity")
        tags = Parser().parsestr(archive.read(records[0]).decode('utf-8'))
        if tags.get_all('Tag') != [f'{python_tag}-{python_tag}-{platform_tag}']:
            raise ValueError("Wheel metadata ABI/platform does not match its native runner")
        extension = '.so'
        if platform_tag == 'win_amd64':
            extension = '.pyd'
        binaries = [name for name in names if name.endswith(('.so', '.pyd'))]
        if len(binaries) != 1 or not binaries[0].endswith(extension):
            raise ValueError("Wheel is missing its expected single native extension")
        if not binaries[0].startswith('merlin/_native'):
            raise ValueError("Wheel must contain its single private native extension")
        for required in ('__init__.py', 'downstream.py', 'upstream.py', 'common.py', 'merlin.py',
                         '__init__.pyi', 'downstream.pyi', 'upstream.pyi', 'common.pyi', 'merlin.pyi', 'py.typed'):
            if f'merlin/{required}' not in names:
                raise ValueError(f"Wheel lacks namespace/typing payload: {required}")
        if any('__pycache__' in name or name.endswith('.pyc') for name in names):
            raise ValueError("Wheel contains generated Python bytecode")
        for required in ('MERLIN-LICENSE', 'MERLIN-NOTICE', 'manifest.json'):
            if f'target/wheel-notices/{required}' not in names:
                raise ValueError(f"Wheel lacks required notices: {required}")
    return binaries[0]


def validate_example_output(output):
    """Require actual completion and the known single-capture fixture outcomes.

    Args:
        output: stdout from independently executing the installed-wheel example.
    Raises:
        ValueError for missing/duplicate fields, failed/partial runs or FEC/BER drift.
    """
    expected = ['RunStatus.Complete', f'Total: {EXPECTED_TOTAL_CODEWORDS}',
                f'Corrected: {EXPECTED_CORRECTED_CODEWORDS}',
                f'Uncorrectable: {EXPECTED_UNCORRECTABLE_CODEWORDS}', 'BER: 0.000e+00']
    if output.splitlines() != expected:
        raise ValueError(f"Installed-wheel example result differs from fixture: {output}")


def main():
    """Install into a fresh temporary venv, run the API example, then stage checksums.

    CLI arguments select wheel input, new release output and diagnostic report.
    No system environment is installed, no source fallback or CLI is invoked and
    no release assets are staged until the real example passes.
    """
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--wheel-dir', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--report', type=Path, required=True)
    args = parser.parse_args()
    started = time.perf_counter()
    args.report.parent.mkdir(parents=True, exist_ok=True)
    report = {'status': 'failed', 'platform': sys.platform, 'python': sys.version}
    try:
        version = (ROOT / 'VERSION').read_text().strip()
        wheels = list(args.wheel_dir.glob('*.whl'))
        if len(wheels) != 1:
            raise ValueError('Release test requires exactly one newly built wheel')
        if wheels[0].is_symlink():
            raise ValueError('Release wheel input cannot be redirected')
        wheel = wheels[0].resolve()
        if sys.platform != 'win32' and not sys.platform.startswith('linux'):
            raise ValueError('Only native Windows/Ubuntu x64 wheels are in release scope')
        platform_tag = 'linux_x86_64'
        if sys.platform == 'win32':
            platform_tag = 'win_amd64'
        python_tag = f'cp{sys.version_info.major}{sys.version_info.minor}'
        report['extension'] = inspect_wheel(wheel, version, python_tag, platform_tag)
        report['wheel'] = wheel.name
        environment = os.environ.copy()
        environment.pop('PYTHONPATH', None)
        environment.pop('PYTHONHOME', None)
        environment['PYTHONNOUSERSITE'] = '1'
        with tempfile.TemporaryDirectory(prefix='merlin-wheel-test-') as temporary:
            isolated = Path(temporary)
            venv.EnvBuilder(with_pip=True).create(isolated / 'venv')
            executable = isolated / 'venv/bin/python'
            if sys.platform == 'win32':
                executable = isolated / 'venv/Scripts/python.exe'
            subprocess.run([str(executable), '-I', '-m', 'pip', 'install', '--no-index',
                            '--no-deps', str(wheel)], cwd=isolated, env=environment, check=True)
            # Verify import resolves inside the fresh environment and has the source version.
            probe = ("import merlin,sys; from pathlib import Path; "
                     f"assert merlin.__version__ == {version!r}; "
                     "native=getattr(merlin,'_native',getattr(merlin,'merlin',merlin)); "
                     "assert Path(native.__file__).resolve().is_relative_to(Path(sys.prefix).resolve()); "
                     "print(native.__file__)")
            imported = subprocess.run([str(executable), '-I', '-c', probe], cwd=isolated,
                                      env=environment, check=True, text=True, capture_output=True)
            report['imported_native'] = imported.stdout.strip()
            report['example_elapsed_seconds'] = 0.0
            report['examples'] = []
            for example in (EXAMPLE, EXAMPLE.with_name('run_single_capture_cm_chan_est.py')):
                example_started = time.perf_counter()
                try:
                    result = subprocess.run([str(executable), '-I', str(example)], cwd=isolated,
                                            env=environment, check=True, text=True, capture_output=True,
                                            timeout=EXAMPLE_TIMEOUT_SECONDS)
                finally:
                    elapsed = time.perf_counter() - example_started
                    report['example_elapsed_seconds'] += elapsed
                    report['examples'].append({'name': example.name, 'elapsed_seconds': elapsed})
                    print(f"Python API example {example.name}: {elapsed:.2f}s", flush=True)
                report['stdout'] = result.stdout
                report['stderr'] = result.stderr
                print(result.stdout, end='')
                validate_example_output(result.stdout)

        args.output.mkdir(parents=True, exist_ok=False)
        shutil.copy2(wheel, args.output / wheel.name)
        digest = hashlib.sha256(wheel.read_bytes()).hexdigest()
        (args.output / 'SHA256SUMS').write_text(f'{digest}  {wheel.name}\n', encoding='ascii')
        report['sha256'] = digest
        report['status'] = 'passed'
    except Exception as error:
        report['error'] = str(error)
        if isinstance(error, subprocess.CalledProcessError):
            report['stdout'] = error.stdout
            report['stderr'] = error.stderr
        raise
    finally:
        report['elapsed_seconds'] = time.perf_counter() - started
        args.report.write_text(json.dumps(report, indent=2) + '\n', encoding='utf-8')
        print(f"Wheel validation time: {report['elapsed_seconds']:.2f}s ({report['status']})")


if __name__ == '__main__':
    main()
