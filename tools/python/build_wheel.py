#!/usr/bin/env python3
#
# Copyright 2026 DOCSIS OFDM Channel Sim contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Build a local native Linux/Windows wheel with complete locked dependency notices."""
import argparse
import hashlib
import os
import platform
from pathlib import Path
import shutil
import subprocess
import sys
import zipfile

ROOT = Path(__file__).resolve().parents[2]
BINDING = ROOT / "bindings/python"
sys.path.insert(0, str(ROOT / "tools/packaging"))
from notices import collect_notices


def build_command(interpreter, binding, output, host):
    """Select native maturin flags without changing runtime/scientific behavior.

    Args:
        interpreter: Python used to build the matching ABI extension.
        binding: Binding Cargo manifest path.
        output: Generated wheel directory.
        host: sys.platform value; only Linux and Windows x64 are in this milestone.
    Returns:
        Argument vector for a locked optimized native wheel build.
    Raises:
        ValueError for an unsupported host platform.
    """
    if not (host.startswith("linux") or host == "win32"):
        raise ValueError("Native wheel packaging supports Linux and Windows only")
    compatibility = []
    if host.startswith("linux"):
        compatibility = ["--compatibility", "linux"]
    return [interpreter, "-m", "maturin", "build", "--manifest-path", str(binding),
            "--release", "--locked", *compatibility, "--interpreter", interpreter,
            "--out", str(output)]


def prepare_notices(root, destination, manifest):
    """Fetch locked host sources before offline license collection on fresh hosts.

    Args:
        root: Source checkout containing the pinned Cargo helper.
        destination: Fresh notice staging directory.
        manifest: Python binding Cargo manifest, whose lockfile stays unchanged.
    Returns:
        None after the existing collector validates and copies all notices.
    Raises:
        ValueError for missing host metadata; subprocess and notice failures
        propagate before wheel construction or installation.
    """
    if platform.system() == "Windows":
        cargo = ["cargo", "+1.85.1"]
    else:
        cargo = ["bash", (root / "scripts/cargo-local.sh").as_posix()]
    information = subprocess.run([*cargo, "-vV"], cwd=root, check=True,
                                 capture_output=True, text=True).stdout
    hosts = [line.removeprefix("host: ") for line in information.splitlines()
             if line.startswith("host: ")]
    if len(hosts) != 1 or not hosts[0]:
        raise ValueError("Cargo did not report one native host target")
    subprocess.run([*cargo, "fetch", "--locked", "--manifest-path", str(manifest),
                    "--target", hosts[0]], cwd=root, check=True)
    collect_notices(root, destination, manifest)


def windows_build_environment(environment):
    """Select the active MSVC linker even when Git Bash leads the search path.

    Args:
        environment: Inherited build environment; the input is never modified.
    Returns:
        A copied environment with the compiler's sibling linker selected for
        the native x64 MSVC target and its tools first on the child PATH.
    Raises:
        ValueError when the active developer shell lacks cl.exe or link.exe.
    """
    result = environment.copy()
    compiler = shutil.which("cl.exe", path=result.get("PATH", ""))
    if compiler is None:
        raise ValueError("Windows wheel builds require an x64 MSVC developer shell (cl.exe).")
    tools = Path(compiler).resolve().parent
    linker = tools / "link.exe"
    if not linker.is_file():
        raise ValueError(f"MSVC linker is missing beside the compiler: {linker}")
    result["CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER"] = str(linker)
    result["PATH"] = str(tools) + os.pathsep + result.get("PATH", "")
    return result


def validate_notices(wheel, destination):
    """Require every generated legal file, with unchanged bytes, in the wheel.

    Args:
        wheel: Newly built native archive.
        destination: Reviewed staging directory containing the complete notices.
    Raises:
        ValueError for omitted or changed legal files before writing a checksum.
    """
    with zipfile.ZipFile(wheel) as archive:
        for source in destination.rglob('*'):
            if not source.is_file():
                continue
            member = 'target/wheel-notices/' + source.relative_to(destination).as_posix()
            if member not in archive.namelist() or archive.read(member) != source.read_bytes():
                raise ValueError(f"Wheel lacks unchanged generated notice: {member}")


def main():
    """Collect reproducible legal assets, then invoke pinned local maturin.

    Arguments come from argparse: --out selects generated wheel storage.
    Returns no scientific result and never installs or publishes anything.
    Raises subprocess/filesystem errors for invalid sources or failed builds.
    Only generated notice staging and wheel/checksum artifacts may be replaced.
    """
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path, default=ROOT / "local/python-wheels")
    args = parser.parse_args()
    if platform.machine().lower() not in ("x86_64", "amd64"):
        parser.error("Native wheel release targets are Linux/Windows x64")
    if not (sys.platform.startswith("linux") or sys.platform == "win32"):
        parser.error("Native wheel release targets are Linux/Windows x64")
    # Pinned maturin 1.8.3 resolves wheel include globs from python-source.
    # Keep the archive's existing target/wheel-notices paths and ignored staging.
    destination = BINDING / "python/target/wheel-notices"
    if any(parent.is_symlink() for parent in
           [BINDING, BINDING / "python", BINDING / "python/target", destination]):
        raise ValueError("Refuse redirected notice staging")
    if destination.exists():
        if not (destination / "manifest.json").is_file():
            raise ValueError("Refuse unrecognized notice staging directory")
        if any(path.is_symlink() for path in destination.rglob("*")):
            raise ValueError("Refuse redirected generated notices")
        shutil.rmtree(destination)
    prepare_notices(ROOT, destination, BINDING / "Cargo.toml")
    for source, name in (
        ("LICENSE", "MERLIN-LICENSE"), ("NOTICE", "MERLIN-NOTICE"),
        ("crates/fec-codec/LICENSE-MIT", "fec-codec-LICENSE-MIT"),
        ("crates/ofdm/resources/qam-table/LICENSE", "qam-table-LICENSE"),
        ("crates/ofdm/resources/qam-table/NOTICE", "qam-table-NOTICE"),
    ):
        shutil.copyfile(ROOT / source, destination / name)
    environment = os.environ.copy()
    if (ROOT / "local/cargo/bin/cargo").is_file():
        environment.setdefault("CARGO_HOME", str(ROOT / "local/cargo"))
        environment.setdefault("RUSTUP_HOME", str(ROOT / "local/rustup"))
        environment["PATH"] = str(ROOT / "local/cargo/bin") + os.pathsep + environment.get("PATH", "")
    if sys.platform == "win32":
        environment = windows_build_environment(environment)
    args.out.mkdir(parents=True, exist_ok=True)
    subprocess.run(build_command(sys.executable, BINDING / "Cargo.toml",
                                 args.out.resolve(), sys.platform),
                   cwd=ROOT, env=environment, check=True)
    for wheel in args.out.glob("merlin_ofdm-*.whl"):
        validate_notices(wheel, destination)
        digest = hashlib.sha256(wheel.read_bytes()).hexdigest()
        wheel.with_suffix(wheel.suffix + ".sha256").write_text(
            f"{digest}  {wheel.name}\n", encoding="utf-8")


if __name__ == "__main__":
    main()
