#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Exercise wheel identity/ABI/legal gates and real-example result acceptance."""
from pathlib import Path
import tempfile
import unittest
import zipfile
import subprocess
from unittest.mock import patch, Mock, call

from build_wheel import build_command, prepare_notices
from smoke_wheel import inspect_wheel, validate_example_output


class WheelToolTests(unittest.TestCase):
    """Use synthetic archive headers only; engine smoke execution is separate."""

    def wheel(self, root, platform='linux_x86_64', native='merlin/merlin.so', notice=True):
        """Write a small archive fixture for metadata validation, never installation."""
        path = root / f'merlin_ofdm-1.2.3-cp310-cp310-{platform}.whl'
        with zipfile.ZipFile(path, 'w') as archive:
            archive.writestr('merlin_ofdm-1.2.3.dist-info/METADATA', 'Name: merlin-ofdm\nVersion: 1.2.3\n')
            archive.writestr('merlin_ofdm-1.2.3.dist-info/WHEEL', f'Tag: cp310-cp310-{platform}\n')
            archive.writestr(native, b'archive fixture only')
            if notice:
                for name in ('MERLIN-LICENSE', 'MERLIN-NOTICE', 'manifest.json'):
                    archive.writestr(f'target/wheel-notices/{name}', '{}')
        return path

    def test_native_linux_and_windows_build_commands(self):
        """Keep Linux's honest platform tag without passing it to Windows maturin."""
        linux = build_command('python', Path('Cargo.toml'), Path('out'), 'linux')
        windows = build_command('python.exe', Path('Cargo.toml'), Path('out'), 'win32')
        self.assertIn('--locked', windows)
        self.assertEqual(linux[linux.index('--compatibility') + 1], 'linux')
        self.assertNotIn('--compatibility', windows)
        with self.assertRaises(ValueError):
            build_command('python', Path('Cargo.toml'), Path('out'), 'darwin')

    def test_cold_cache_fetch_precedes_offline_notices_on_each_host(self):
        """Download only locked native sources before collecting their licenses."""
        root = Path("checkout")
        manifest = root / "bindings/python/Cargo.toml"
        destination = root / "notices"
        for system, target, prefix in [
            ("Linux", "x86_64-unknown-linux-gnu", ["bash", "checkout/scripts/cargo-local.sh"]),
            ("Windows", "x86_64-pc-windows-msvc", ["cargo", "+1.85.1"]),
        ]:
            with self.subTest(system=system), patch("build_wheel.platform.system", return_value=system), \
                    patch("build_wheel.subprocess.run") as run, patch("build_wheel.collect_notices") as collect:
                order = Mock()
                order.attach_mock(run, "run")
                order.attach_mock(collect, "collect")
                run.return_value.stdout = f"cargo 1.85.1\nhost: {target}\n"
                prepare_notices(root, destination, manifest)
                self.assertEqual(order.mock_calls, [
                    call.run([*prefix, "-vV"], cwd=root, check=True, capture_output=True, text=True),
                    call.run([*prefix, "fetch", "--locked", "--manifest-path", str(manifest),
                              "--target", target], cwd=root, check=True),
                    call.collect(root, destination, manifest),
                ])
                collect.reset_mock()
                run.side_effect = [Mock(stdout=f"host: {target}\n"),
                                   subprocess.CalledProcessError(101, "cargo fetch")]
                with self.assertRaises(subprocess.CalledProcessError):
                    prepare_notices(root, destination, manifest)
                collect.assert_not_called()

    def test_version_abi_platform_and_notices_are_required(self):
        """Reject mismatched consumer wheels and incomplete legal packaging."""
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            linux = self.wheel(root)
            self.assertEqual(inspect_wheel(linux, '1.2.3', 'cp310', 'linux_x86_64'), 'merlin/merlin.so')
            for version, abi, platform in [('2.0.0', 'cp310', 'linux_x86_64'),
                                           ('1.2.3', 'cp312', 'linux_x86_64'),
                                           ('1.2.3', 'cp310', 'win_amd64')]:
                with self.assertRaises(ValueError):
                    inspect_wheel(linux, version, abi, platform)
            windows = self.wheel(root, platform='win_amd64', native='merlin/merlin.pyd')
            self.assertTrue(inspect_wheel(windows, '1.2.3', 'cp310', 'win_amd64').endswith('.pyd'))
            windows = self.wheel(root, platform='win_amd64')
            with self.assertRaisesRegex(ValueError, 'native extension'):
                inspect_wheel(windows, '1.2.3', 'cp310', 'win_amd64')
            self.wheel(root, notice=False)
            with self.assertRaisesRegex(ValueError, 'notices'):
                inspect_wheel(linux, '1.2.3', 'cp310', 'linux_x86_64')

    def test_failed_partial_or_incorrect_fec_results_block_staging(self):
        """Require the existing fixture's actual completion and decoder outcomes."""
        complete = 'RunStatus.Complete\nTotal: 82\nCorrected: 0\nUncorrectable: 0\nBER: 0.000e+00\n'
        validate_example_output(complete)
        for incorrect in ['', complete.replace('Complete', 'Cancelled'),
                          complete.replace('Total: 82', 'Total: 0'),
                          complete.replace('Uncorrectable: 0', 'Uncorrectable: 1'),
                          complete.replace('0.000e+00', '1.000e-03'), complete + complete]:
            with self.assertRaises(ValueError):
                validate_example_output(incorrect)


if __name__ == '__main__':
    unittest.main()
