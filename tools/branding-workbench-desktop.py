#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Prepare a checkout-local desktop launcher; never install it into the desktop."""
import os
from pathlib import Path
import shlex


def main():
    """Write a launcher and desktop entry under local/workbench for this checkout.

    Uses QT_SDK_DIR or the project-local Qt SDK. No system files are changed.
    Raises OSError for inaccessible files and ValueError for unsafe desktop paths.
    """
    root = Path(__file__).resolve().parents[1]
    sdk = Path(os.environ.get('QT_SDK_DIR', root / 'local/qt/6.8.3/gcc_64')).resolve()
    destination = root / 'local/workbench'
    destination.mkdir(parents=True, exist_ok=True)
    launcher = destination / 'launch.sh'
    icon = root / 'docs/images/logo/merlin-icon-dark.png'
    for value in (str(launcher), str(icon)):
        if any(character in value for character in ('\n', '\r', '"', '`', '$', '\\', '%')):
            raise ValueError('Unsupported special character in desktop-entry path')
    launcher.write_text('#!/usr/bin/env bash\nset -euo pipefail\n'
                        + 'export QT_SDK_DIR=' + shlex.quote(str(sdk)) + '\n'
                        + 'exec ' + shlex.quote(str(root / 'scripts/qt-desktop.sh')) + ' run\n')
    launcher.chmod(0o755)
    template = (root / 'apps/qt/merlin.desktop.in').read_text()
    entry = destination / 'merlin.desktop'
    entry.write_text(template.replace('@LAUNCHER@', str(launcher)).replace('@ICON@', str(icon)))
    entry.chmod(0o755)
    print(entry)


if __name__ == '__main__':
    main()
