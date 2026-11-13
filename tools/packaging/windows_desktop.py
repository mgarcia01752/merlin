#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Build the Windows x64 desktop MSI; never install, sign or publish it."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import shutil
import subprocess
import uuid
import xml.etree.ElementTree as ET
from asset_names import asset_name
from common import ROOT, checksums, version
from desktop_payload import populate

NS = 'http://schemas.microsoft.com/wix/2006/wi'
# Desktop identity is intentionally separate from the prepared CLI-only MSI.
UPGRADE_CODE = 'C1858994-9ED6-43A5-82F8-B998E317B37C'
ARTWORK = ROOT / 'packaging/windows'


def identifier(relative):
    """Return a stable WiX identifier for a relative payload path.

    The SHA-256 digest avoids ordering-dependent component identities and keeps
    identifiers within WiX length limits. Path spelling is otherwise preserved.
    """
    return 'P' + hashlib.sha256(relative.as_posix().encode('utf-8')).hexdigest()[:40]


def author(payload, destination, number):
    """Author a desktop-only WiX3 installer with stable upgrade/component IDs.

    Args:
        payload: Fresh staged runtime, public resources and notices.
        destination: New XML path; existing files are rejected.
        number: Numeric three-component version fitting MSI bounds.
    Returns:
        Path to the new WiX source. No tool runs and no installation occurs.
    Raises:
        ValueError for unsafe payloads, versions or missing desktop resources.
        FileExistsError for an existing source destination.
    """
    asset_name('desktop', number, 'windows', 'amd64', 'msi')
    major, minor, patch = map(int, number.split('.'))
    if major > 255 or minor > 255 or patch > 65535:
        raise ValueError('VERSION exceeds MSI three-component limits')
    paths = sorted(payload.rglob('*'))
    seen = set()
    for path in [payload, *paths]:
        if path.is_symlink() or not (path.is_file() or path.is_dir()):
            raise ValueError(f'Unsupported payload path: {path}')
        relative = path.relative_to(payload).as_posix()
        if relative.casefold() in seen:
            raise ValueError('Windows payload paths collide ignoring case')
        seen.add(relative.casefold())
    if not (payload / 'merlin-qt.exe').is_file():
        raise ValueError('Missing desktop executable')
    ET.register_namespace('', NS)
    root = ET.Element(f'{{{NS}}}Wix')

    def element(parent, tag, **attributes):
        return ET.SubElement(parent, f'{{{NS}}}{tag}', attributes)

    product = element(root, 'Product', Id='*', Name='MERLIN', Language='1033',
                      Version=number, Manufacturer='MERLIN contributors', UpgradeCode=UPGRADE_CODE)
    element(product, 'Package', InstallerVersion='500', Compressed='yes',
            InstallScope='perMachine', Platform='x64', Description='MERLIN DOCSIS OFDM Simulation Toolkit')
    element(product, 'MajorUpgrade', Schedule='afterInstallInitialize',
            DowngradeErrorMessage='A newer MERLIN version is already installed.')
    element(product, 'MediaTemplate', EmbedCab='yes', CompressionLevel='high')
    element(product, 'Property', Id='ARPPRODUCTICON', Value='ApplicationIcon.ico')
    element(product, 'Property', Id='ARPURLINFOABOUT', Value='https://github.com/mgarcia01752/merlin')
    element(product, 'Icon', Id='ApplicationIcon.ico', SourceFile=str(ARTWORK / 'merlin.ico'))
    element(product, 'Property', Id='WIXUI_INSTALLDIR', Value='INSTALLDIR')
    element(product, 'Property', Id='DESKTOP_SHORTCUT', Secure='yes')
    element(product, 'UIRef', Id='Merlin_InstallDir')
    for name, path in [('WixUIDialogBmp', ARTWORK / 'dialog.bmp'),
                       ('WixUIBannerBmp', ARTWORK / 'banner.bmp'),
                       ('WixUILicenseRtf', destination.with_suffix('.rtf'))]:
        element(product, 'WixVariable', Id=name, Value=str(path))
    target = element(product, 'Directory', Id='TARGETDIR', Name='SourceDir')
    program = element(target, 'Directory', Id='ProgramFiles64Folder')
    install = element(program, 'Directory', Id='INSTALLDIR', Name='MERLIN')
    programs = element(target, 'Directory', Id='ProgramMenuFolder')
    menu = element(programs, 'Directory', Id='ApplicationMenu', Name='MERLIN')
    element(target, 'Directory', Id='DesktopFolder')
    feature = element(product, 'Feature', Id='Complete', Title='MERLIN desktop', Level='1')
    directories = {Path('.'): install}
    for path in paths:
        relative = path.relative_to(payload)
        identity = identifier(relative)
        if path.is_dir():
            directories[relative] = element(directories[relative.parent], 'Directory', Id='D' + identity, Name=path.name)
            continue
        component = element(directories[relative.parent], 'Component', Id=identity, Win64='yes',
                            Guid=str(uuid.uuid5(uuid.UUID(UPGRADE_CODE), relative.as_posix())))
        element(component, 'File', Id='F' + identity, Source=str(path), KeyPath='yes')
        element(feature, 'ComponentRef', Id=identity)
    # HKCU keypaths are required for non-advertised user shell shortcuts (ICE43).
    for name, directory in [('Menu', 'ApplicationMenu'), ('Desktop', 'DesktopFolder')]:
        component = element(menu if name == 'Menu' else product.find(".//{" + NS + "}Directory[@Id='DesktopFolder']"), 'Component', Id=name + 'Shortcut', Win64='yes',
                            Guid=str(uuid.uuid5(uuid.UUID(UPGRADE_CODE), 'shortcut/' + name)))
        if name == 'Desktop':
            element(component, 'Condition').text = 'DESKTOP_SHORTCUT = "1"'
        element(component, 'Shortcut', Id=name + 'Link', Directory=directory, Name='MERLIN',
                Target='[INSTALLDIR]merlin-qt.exe', WorkingDirectory='INSTALLDIR', Icon='ApplicationIcon.ico')
        if name == 'Menu':
            element(component, 'RemoveFolder', Id='RemoveApplicationMenu', Directory='ApplicationMenu', On='uninstall')
        element(component, 'RegistryValue', Root='HKCU', Key=r'Software\MERLIN\Installer',
                Name=name + 'Shortcut', Type='integer', Value='1', KeyPath='yes')
        element(feature, 'ComponentRef', Id=name + 'Shortcut')
    # Unset means unchecked. Do not initialize the property to "0": MSI checkbox
    # properties are present/absent. This choice occurs before Install executes.
    ui = element(product, 'UI')
    dialog = element(ui, 'Dialog', Id='ShortcutDlg', Width='370', Height='270', Title='MERLIN Setup')
    element(dialog, 'Control', Id='Title', Type='Text', X='15', Y='15', Width='340', Height='30',
            Text='Choose shortcuts')
    element(dialog, 'Control', Id='DesktopChoice', Type='CheckBox', X='20', Y='65', Width='330', Height='18',
            Property='DESKTOP_SHORTCUT', CheckBoxValue='1', Text='Create a desktop shortcut')
    element(dialog, 'Control', Id='Description', Type='Text', X='20', Y='95', Width='330', Height='40',
            Text='MERLIN is always available from the Start menu.')
    for name, x in [('Back', '180'), ('Next', '236'), ('Cancel', '304')]:
        control = element(dialog, 'Control', Id=name, Type='PushButton', X=x, Y='243', Width='56', Height='17',
                          Text=name, **({'Default': 'yes'} if name == 'Next' else {'Cancel': 'yes'} if name == 'Cancel' else {}))
        if name == 'Cancel':
            element(control, 'Publish', Event='SpawnDialog', Value='CancelDlg').text = '1'
        else:
            element(control, 'Publish', Event='NewDialog', Value='InstallDirDlg' if name == 'Back' else 'VerifyReadyDlg').text = '1'
    with destination.open('xb') as output:
        ET.ElementTree(root).write(output, encoding='utf-8', xml_declaration=True)
    license_text = (ROOT / 'apps/qt/LICENSE').read_text(encoding='utf-8')
    text = license_text.replace('\\', '\\\\').replace('{', r'\{').replace('}', r'\}').replace('\n', '\\par\n')
    with destination.with_suffix('.rtf').open('x', encoding='ascii') as output:
        output.write('{\\rtf1\\ansi\n' + text + '}')
    return destination


def package(binary, sdk, crt, output):
    """Stage/deploy a native desktop and compile the branded MSI/checksums.

    Args:
        binary: Version-matched release merlin-qt.exe built on Windows x64.
        sdk: Existing Qt6.8.3 MSVC2022 x64 SDK, used read-only.
        crt: MSVC x64 redistributable CRT directory from the build installation.
        output: New output directory; no existing artifact is overwritten.
    Returns:
        MSI path. Stage/report siblings are retained as local build evidence.
    Raises:
        ValueError for incompatible host/SDK/version or incomplete deployment.
        FileExistsError for output/stage collisions; tool failures propagate.
    """
    stage = output.with_name(output.name + '-stage')
    report = output.with_name(output.name + '-report.json')
    scratch = output.with_name(output.name + '-wix')
    for path in (output, stage, report, scratch):
        if path.exists() or path.is_symlink():
            raise FileExistsError(path)
    if platform.system() != 'Windows' or platform.machine().lower() not in ('amd64', 'x86_64'):
        raise ValueError('Build the desktop MSI on native Windows x64')
    number = version()
    environment = os.environ.copy()
    environment['PATH'] = str(sdk / 'bin') + os.pathsep + environment['PATH']
    actual = subprocess.check_output([str(binary), '--version'], env=environment, text=True).strip()
    qt_version = subprocess.check_output([str(sdk / 'bin/qmake.exe'), '-query', 'QT_VERSION'], text=True).strip()
    if actual != f'MERLIN {number}' or qt_version != '6.8.3':
        raise ValueError('Binary or Qt SDK version mismatch')
    stage.mkdir(parents=True)
    shutil.copy2(binary, stage / 'merlin-qt.exe')
    subprocess.run([str(sdk / 'bin/windeployqt.exe'), '--release', '--no-translations',
                    '--no-compiler-runtime', '--qmldir', str(ROOT / 'apps/qt/qml'),
                    str(stage / 'merlin-qt.exe')], env=environment, check=True)
    for path in sorted(crt.glob('*.dll')):
        shutil.copy2(path, stage / path.name)
    for name in ('Qt6Core.dll', 'Qt6Quick.dll', 'platforms/qwindows.dll',
                 'msvcp140.dll', 'vcruntime140.dll', 'vcruntime140_1.dll'):
        if not (stage / name).is_file():
            raise ValueError(f'Missing deployed runtime: {name}')
    (stage / 'qt.conf').write_text('[Paths]\nPrefix=.\nPlugins=.\nQmlImports=qml\n', encoding='ascii')
    populate(stage)
    for name in ('runtime-NOTICE.txt', 'installer-ui.wxs', 'wix-ui-LICENSE.txt', 'wix-ui-provenance.json'):
        shutil.copy2(ARTWORK / name, stage / 'notices' / name)
    # Exercise a relocated runtime with developer SDKs absent from PATH.
    smoke(stage)
    output.mkdir(parents=True)
    scratch.mkdir()
    source = author(stage, scratch / 'installer.wxs', number)
    wix = Path(os.environ.get('WIX_BIN', r'C:\Program Files (x86)\WiX Toolset v3.14\bin'))
    obj = scratch / 'installer.wixobj'
    msi = output / asset_name('desktop', number, 'windows', 'amd64', 'msi')
    subprocess.run([str(wix / 'candle.exe'), '-nologo', '-arch', 'x64', '-out', str(obj), str(source)], check=True)
    ui_obj = scratch / 'installer-ui.wixobj'
    subprocess.run([str(wix / 'candle.exe'), '-nologo', '-arch', 'x64', '-out', str(ui_obj), str(ARTWORK / 'installer-ui.wxs')], check=True)
    subprocess.run([str(wix / 'light.exe'), '-nologo', '-ext', 'WixUIExtension', '-out', str(msi), str(obj), str(ui_obj)], check=True)
    for path in output.glob('*.wixpdb'):
        path.unlink()
    checksums(output)
    report.write_text(json.dumps({'msi': msi.name, 'installer_bytes': msi.stat().st_size,
        'installed_bytes': sum(p.stat().st_size for p in stage.rglob('*') if p.is_file()),
        'external_prerequisites': 'Windows 10/11 x64 system libraries; Qt and MSVC CRT bundled',
        'signed': False}, indent=2) + '\n', encoding='utf-8')
    return msi


def smoke(payload):
    """Run version, all installed example validations and the rendered Qt probe.

    Args:
        payload: Native Windows runtime directory (staged or installed).
    Returns:
        None after bounded probes with a clean environment and unrelated cwd.
    Raises:
        ValueError for a bad version; subprocess errors/timeouts propagate.
    """
    environment = os.environ.copy()
    for key in ('QT_PLUGIN_PATH', 'QML_IMPORT_PATH', 'QML2_IMPORT_PATH', 'QMAKE', 'QT_SDK_DIR'):
        environment.pop(key, None)
    environment['PATH'] = os.pathsep.join((str(Path(environment['SYSTEMROOT']) / 'System32'), environment['SYSTEMROOT']))
    environment['QT_QPA_PLATFORM'] = 'windows'
    environment['QT_QUICK_BACKEND'] = 'software'
    binary = payload / 'merlin-qt.exe'
    actual = subprocess.check_output([str(binary), '--version'], env=environment, cwd=payload.parent, text=True, timeout=30).strip()
    if actual != f'MERLIN {version()}':
        raise ValueError('Deployed version mismatch')
    manifest = payload / 'files/examples/workbench/manifest.json'
    data = json.loads(manifest.read_text(encoding='utf-8'))
    for bundle in data['bundles']:
        workbench = manifest.parent / bundle['workbench']
        subprocess.run([str(binary), '--check-workbench', str(workbench)], env=environment, cwd=payload.parent, check=True, timeout=60)
    result = subprocess.run([str(binary), '--smoke-test'], env=environment, cwd=payload.parent,
                            check=True, timeout=45, text=True, capture_output=True)
    if any(term in result.stderr for term in ('failed to load', 'is not installed', 'ReferenceError', 'TypeError', 'Cannot assign')):
        raise ValueError('Packaged QML startup failed: ' + result.stderr)



if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--smoke', type=Path)
    parser.add_argument('--binary', type=Path)
    parser.add_argument('--qt-sdk', type=Path)
    parser.add_argument('--crt-dir', type=Path)
    parser.add_argument('--output', type=Path)
    args = parser.parse_args()
    if args.smoke:
        smoke(args.smoke.resolve())
    else:
        if not all((args.binary, args.qt_sdk, args.crt_dir, args.output)):
            parser.error('Packaging requires --binary, --qt-sdk, --crt-dir and --output')
        package(args.binary.resolve(), args.qt_sdk.resolve(), args.crt_dir.resolve(), args.output.resolve())
