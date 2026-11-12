#!/usr/bin/env python3
#
# Copyright 2026 DOCSIS OFDM Channel Sim contributors
# SPDX-License-Identifier: Apache-2.0
#
"""WiX3 native CLI installer with stable component identities and PATH cleanup."""
import os
import subprocess
import uuid
import xml.etree.ElementTree as ET
from pathlib import Path
from common import architecture, version
from asset_names import asset_name

NS = "http://schemas.microsoft.com/wix/2006/wi"
UPGRADE_CODE = "590DB065-595A-4E02-8F9A-83C9525B50D1"


def build(payload, output, scratch):
    """Build a canonically named x64 CLI MSI with stable upgrade identity.

    payload is the staged CLI tree; output is an owned artifact directory;
    scratch is caller-owned temporary storage. Native packaging tools must be
    available on this host. Return None; propagate tool and filesystem failures.
    No package is installed or published.
    """
    if architecture() != "x64":
        raise ValueError("The prepared Windows MSI supports x64 only")
    major, minor, patch = map(int, version().split("."))
    if major > 255 or minor > 255 or patch > 65535:
        raise ValueError("VERSION exceeds MSI three-component limits")
    ET.register_namespace("", NS)
    def element(parent, tag, **attributes):
        return ET.SubElement(parent, f"{{{NS}}}{tag}", attributes)
    root = ET.Element(f"{{{NS}}}Wix")
    product = element(root, "Product", Id="*", Name="MERLIN OFDM CLI",
                      Language="1033", Version=version(),
                      Manufacturer="DOCSIS OFDM Channel Sim contributors", UpgradeCode=UPGRADE_CODE)
    element(product, "Package", InstallerVersion="500", Compressed="yes", InstallScope="perMachine", Platform="x64")
    element(product, "MajorUpgrade", DowngradeErrorMessage="A newer MERLIN version is installed.")
    element(product, "MediaTemplate", EmbedCab="yes")
    target = element(product, "Directory", Id="TARGETDIR", Name="SourceDir")
    program = element(target, "Directory", Id="ProgramFiles64Folder")
    install = element(program, "Directory", Id="INSTALLDIR", Name="MERLIN OFDM CLI")
    feature = element(product, "Feature", Id="Complete", Title="MERLIN CLI", Level="1")
    directories = {Path("."): install}
    for index, path in enumerate(sorted(payload.rglob("*"))):
        relative = path.relative_to(payload)
        if path.is_dir():
            directories[relative] = element(directories[relative.parent], "Directory", Id=f"Dir{index}", Name=path.name)
            continue
        identity = f"File{index}"
        guid = str(uuid.uuid5(uuid.UUID(UPGRADE_CODE), relative.as_posix())).upper()
        component = element(directories[relative.parent], "Component", Id=identity, Guid=guid, Win64="yes")
        element(component, "File", Id=f"Payload{index}", Source=str(path), KeyPath="yes")
        if relative.as_posix() == "bin/ofdm-fec.exe":
            element(component, "Environment", Id="CliPath", Name="PATH", Value="[INSTALLDIR]bin",
                    Action="set", Part="last", System="yes", Permanent="no")
        element(feature, "ComponentRef", Id=identity)
    source = scratch / "installer.wxs"
    ET.ElementTree(root).write(source, encoding="utf-8", xml_declaration=True)
    wix = Path(os.environ.get("WIX_BIN", r"C:\Program Files (x86)\WiX Toolset v3.14\bin"))
    obj = scratch / "installer.wixobj"
    subprocess.run([str(wix / "candle.exe"), "-nologo", "-arch", "x64", "-out", str(obj), str(source)], check=True)
    subprocess.run([str(wix / "light.exe"), "-nologo", "-out",
                    str(output / asset_name("cli", version(), "windows", "x64", "msi")), str(obj)], check=True)
    # WiX's private debug database is build evidence, not a release asset.
    for path in output.glob("*.wixpdb"):
        path.unlink()
