#!/usr/bin/env python3
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
"""Exercise launcher display admission without compiling or starting Qt."""
import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[3]
COMMAND_TIMEOUT_SECONDS = 10


class QtDisplayTests(unittest.TestCase):
    def setUp(self):
        """Prepare fake host tools; none can install, build or open a display."""
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.directory = Path(self.temporary.name)
        self.bin = self.directory / 'bin'
        self.bin.mkdir()
        sdk = self.directory / 'sdk'
        (sdk / 'bin').mkdir(parents=True)
        self._executable(sdk / 'bin/qmake', '#!/bin/sh\necho 6.8.3\n')
        self._executable(self.bin / 'systemctl',
            '#!/bin/sh\nprintf "%s\\n" "$TEST_DESKTOP_ENVIRONMENT"\n')
        self._executable(self.bin / 'cargo', '''#!/usr/bin/python3
import json, os, sys
print(json.dumps({'arguments': sys.argv[1:], 'DISPLAY': os.environ.get('DISPLAY'),
                 'XAUTHORITY': os.environ.get('XAUTHORITY')}))
''')
        self.environment = os.environ.copy()
        for key in ('DISPLAY', 'XAUTHORITY', 'WAYLAND_DISPLAY', 'QT_QPA_PLATFORM'):
            self.environment.pop(key, None)
        self.environment.update(PATH=str(self.bin) + os.pathsep + '/usr/bin:/bin',
            QT_SDK_DIR=str(sdk), CARGO_HOME=str(self.directory / 'cargo'),
            TEST_DESKTOP_ENVIRONMENT='DISPLAY=:99\nXAUTHORITY=/test/desktop-auth')

    def _executable(self, path, content):
        """Write an isolated executable stub for the launcher under test."""
        path.write_text(content)
        path.chmod(0o755)

    def _run(self):
        """Run the actual helper with isolated tools and a finite time budget."""
        return subprocess.run(['bash', str(ROOT / 'scripts/qt-desktop.sh'), 'run'],
            cwd=ROOT, env=self.environment, text=True, capture_output=True,
            timeout=COMMAND_TIMEOUT_SECONDS)

    def test_xcb_discovers_display_even_when_wayland_is_present(self):
        """WAYLAND_DISPLAY alone does not satisfy the XCB display requirement."""
        self.environment['WAYLAND_DISPLAY'] = 'wayland-test'
        result = self._run()
        self.assertEqual(result.returncode, 0, result.stderr)
        observed = json.loads(result.stdout)
        self.assertEqual(observed['DISPLAY'], ':99')
        self.assertEqual(observed['XAUTHORITY'], '/test/desktop-auth')
        self.assertIn('--release', observed['arguments'])

    def test_explicit_forwarded_display_and_authorization_are_preserved(self):
        """Desktop discovery must not replace an explicitly forwarded X server."""
        self.environment.update(DISPLAY='localhost:10.0', XAUTHORITY='/test/forwarded-auth')
        result = self._run()
        self.assertEqual(result.returncode, 0, result.stderr)
        observed = json.loads(result.stdout)
        self.assertEqual(observed['DISPLAY'], 'localhost:10.0')
        self.assertEqual(observed['XAUTHORITY'], '/test/forwarded-auth')

    def test_unavailable_xcb_display_rejects_before_cargo(self):
        """A Wayland environment cannot bypass missing XCB display admission."""
        self.environment.update(WAYLAND_DISPLAY='wayland-test', TEST_DESKTOP_ENVIRONMENT='')
        result = self._run()
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('Run from the VM desktop terminal', result.stderr)
        self.assertEqual(result.stdout, '')

    def test_explicit_offscreen_execution_needs_no_display(self):
        """Existing headless test execution stays available without a desktop."""
        self.environment.update(QT_QPA_PLATFORM='offscreen', TEST_DESKTOP_ENVIRONMENT='')
        result = self._run()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIsNone(json.loads(result.stdout)['DISPLAY'])


if __name__ == '__main__':
    unittest.main()
