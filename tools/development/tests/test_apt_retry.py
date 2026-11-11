#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Exercise APT retry order without running APT, sudo or sleeps."""
from pathlib import Path
import subprocess
import unittest

ROOT = Path(__file__).resolve().parents[3]


class AptRetryTests(unittest.TestCase):
    def run_helper(self, update_failures, install_failures):
        """Run the sourced helper against shell fakes and return its transcript."""
        script = r'''
source scripts/apt-development.sh
updates=0; installs=0
sudo() {
  if [[ "$*" == *' update' ]]; then
    updates=$((updates+1)); echo "refresh $updates"
    [[ "$*" == *'APT::Update::Error-Mode=any'* ]] || return 9
    (( updates > UPDATE_FAILURES ))
  else
    installs=$((installs+1)); echo "install $installs"
    [[ "$*" == *'test-package'* ]] || return 9
    (( installs > INSTALL_FAILURES ))
  fi
}
sleep() { :; }
apt_development_packages test-package
'''
        return subprocess.run(['bash', '-c', f'UPDATE_FAILURES={update_failures}\nINSTALL_FAILURES={install_failures}\n'+script],
                              cwd=ROOT, capture_output=True, text=True)

    def test_failed_refresh_never_installs_stale_indexes(self):
        result = self.run_helper(9, 0)
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(result.stdout.splitlines(), ['refresh 1', 'refresh 2', 'refresh 3'])

    def test_failed_download_refreshes_before_retry(self):
        result = self.run_helper(0, 1)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(result.stdout.splitlines(), ['refresh 1', 'install 1', 'refresh 2', 'install 2'])

    def test_transient_refresh_recovers_and_persistent_install_stops(self):
        self.assertEqual(self.run_helper(1, 0).returncode, 0)
        result = self.run_helper(0, 9)
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(result.stdout.count('install '), 3)
