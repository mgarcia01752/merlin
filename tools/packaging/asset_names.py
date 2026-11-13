#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Canonical component/version/platform/architecture release filenames."""
import re
import shutil

FORMATS = {
    ('desktop', 'ubuntu22.04'): {'deb'},
    ('desktop', 'windows'): {'msi'},
    ('cli', 'ubuntu22.04'): {'deb'},
    ('cli', 'linux'): {'tar.gz'},
    ('cli', 'macos'): {'tar.gz', 'pkg'},
    ('cli', 'windows'): {'zip', 'msi'},
}


def asset_name(component, version, target, machine, extension):
    """Return a validated canonical filename without changing package identity.

    component is desktop/cli; version is numeric MAJOR.MINOR.PATCH; target and
    extension must be a prepared delivery combination. machine accepts build
    x64 or canonical amd64/arm64. Return a filename only, never a path.
    Raise ValueError for unsupported or unsafe fields; no files are accessed.
    """
    if not re.fullmatch(r'(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)\.(0|[1-9][0-9]*)', version):
        raise ValueError('Asset version must be MAJOR.MINOR.PATCH')
    arch = {'x64': 'amd64', 'amd64': 'amd64', 'arm64': 'arm64'}.get(machine)
    if arch is None or extension not in FORMATS.get((component, target), set()):
        raise ValueError('Unsupported release asset identity')
    return f'merlin-{component}_{version}_{target}_{arch}.{extension}'


def legacy_desktop_name(version):
    """Return the exact Ubuntu amd64 asset name required by old updaters.

    version must pass canonical identity validation. Return the compatibility
    filename only; it is not a second package or an alternate payload.
    """
    asset_name('desktop', version, 'ubuntu22.04', 'amd64', 'deb')
    return f'merlin_{version}_ubuntu22.04_amd64.deb'


def desktop_alias(artifact, version):
    """Copy a canonical desktop DEB for installed legacy updater compatibility.

    artifact is a regular canonical file beside its new alias. version selects
    its exact identity. Return the new alias; reject redirects and collisions
    before writing. Never replace an existing distributable or create a hardlink.
    """
    if artifact.name != asset_name('desktop', version, 'ubuntu22.04', 'amd64', 'deb'):
        raise ValueError('Desktop compatibility source must have its canonical name')
    if artifact.is_symlink() or not artifact.is_file():
        raise ValueError('Desktop compatibility source must be a regular file')
    destination = artifact.with_name(legacy_desktop_name(version))
    with destination.open('xb') as output, artifact.open('rb') as source:
        shutil.copyfileobj(source, output)
    return destination
