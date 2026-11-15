#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Install a verified MERLIN release wheel into a project virtual environment."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import shutil
import subprocess
import sys
import tempfile
import urllib.request

REPOSITORY = "mgarcia01752/merlin"
METADATA_LIMIT = 2 * 1024 * 1024
WHEEL_LIMIT = 256 * 1024 * 1024
NETWORK_TIMEOUT_SECONDS = 60
COMMAND_TIMEOUT_SECONDS = 600
PROBE = """import importlib.util,json,platform,struct,sys
print(json.dumps(dict(version=list(sys.version_info[:2]), implementation=platform.python_implementation(), bits=struct.calcsize('P')*8, machine=platform.machine(), prefix=sys.prefix, base_prefix=sys.base_prefix, venv=importlib.util.find_spec('venv') is not None, ensurepip=importlib.util.find_spec('ensurepip') is not None, pip=importlib.util.find_spec('pip') is not None, ssl=importlib.util.find_spec('ssl') is not None)))"""


def run(command, capture=False, timeout=COMMAND_TIMEOUT_SECONDS):
    """Run a bounded child process without a shell.

    Args:
        command: Executable and literal arguments.
        capture: Capture output for machine-readable prerequisite probes.
        timeout: Maximum execution time in seconds.
    Returns:
        Completed child process.
    Raises:
        Subprocess errors on failure or timeout; no errors are suppressed.
    """
    environment = os.environ.copy()
    environment.pop("PYTHONPATH", None)
    environment.pop("PYTHONHOME", None)
    return subprocess.run(command, check=True, text=True, capture_output=capture,
                          timeout=timeout, env=environment)


def release_identity(value):
    """Validate a stable numeric version or tag and derive both forms.

    Args:
        value: MAJOR.MINOR.PATCH, optionally prefixed with v.
    Returns:
        Canonical tag and package version.
    Raises:
        ValueError for paths, moving references or unsupported version syntax.
    """

    # Match a complete stable version, such as v0.3.1; capture its numeric portion.
    match = re.fullmatch(r"v?((?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*)\.(?:0|[1-9][0-9]*))", value)
    if match is None:
        raise ValueError("Use a stable version/tag, for example --tag v0.3.1.")
    return "v" + match[1], match[1]


def host_platform():
    """Check the supported OS and architecture before any installation.

    Returns:
        Wheel platform suffix for Ubuntu x64 or Windows x64.
    Raises:
        ValueError for unsupported systems; no external state is changed.
    """
    if platform.machine().lower() not in ("x86_64", "amd64"):
        raise ValueError("MERLIN release wheels require x64; ARM and 32-bit hosts are unsupported.")
    if sys.platform == "win32":
        return "win_amd64"
    if sys.platform == "linux":
        info = ubuntu_info()
        if info.get("ID") == "ubuntu" and info.get("VERSION_ID") in ("22.04", "24.04", "26.04"):
            return "linux_x86_64"
    raise ValueError("Supported targets: Ubuntu 22.04/24.04/26.04 x64 or Windows x64.")


def ubuntu_info():
    """Read OS identity without executing shell content.

    Returns:
        Parsed release fields, or an empty dictionary outside Linux.
    """
    path = Path("/etc/os-release")
    if not path.is_file():
        return {}
    fields = {}
    for line in path.read_text().splitlines():
        if "=" in line and not line.startswith("#"):
            key, value = line.split("=", 1)
            fields[key] = value.strip('"')
    return fields


def probe(command):
    """Inspect an interpreter using isolated Python execution.

    Args:
        command: Interpreter command prefix, possibly py -3.10.
    Returns:
        Native interpreter capability dictionary.
    Raises:
        ValueError or subprocess errors for an unusable interpreter.
    """
    data = json.loads(run(command + ["-I", "-c", PROBE], capture=True).stdout)
    if (data["version"] != [3, 10] or data["implementation"] != "CPython"
            or data["bits"] != 64 or data["machine"].lower() not in ("x86_64", "amd64")
            or not data["ssl"]):
        raise ValueError("Require x64 CPython 3.10 with SSL support; another Python ABI cannot load cp310 wheels.")
    return data


def find_python(explicit=None):
    """Locate a compatible interpreter without downloading or modifying it.

    Args:
        explicit: Optional path selected by the user.
    Returns:
        Validated interpreter command prefix and capability dictionary.
    Raises:
        ValueError with setup instructions when no compatible interpreter exists.
    """
    candidates = [[explicit]] if explicit else [[sys.executable], ["python3.10"]]
    if not explicit and sys.platform == "win32":
        candidates.insert(0, ["py", "-3.10"])
    for command in candidates:
        try:
            return command, probe(command)
        except (OSError, ValueError, subprocess.SubprocessError, KeyError):
            continue
    raise ValueError("CPython 3.10 x64 was not found. Install it and pass --python PATH. "
                     "Ubuntu 22.04 can use --install-prerequisites; on Windows install "
                     "64-bit CPython 3.10 with the Python launcher. Newer Ubuntu requires "
                     "a separately provisioned Python 3.10; do not replace system Python.")


def environment_python(destination):
    """Resolve the platform's interpreter path inside a virtual environment.

    Args:
        destination: Environment directory.
    Returns:
        Interpreter path without executing it.
    """
    return destination / ("Scripts/python.exe" if sys.platform == "win32" else "bin/python")


def check_destination(destination):
    """Reject unsafe or unrelated existing destinations before pip can mutate them.

    Args:
        destination: User-selected virtual environment path.
    Returns:
        True for an existing compatible environment, false for a new destination.
    Raises:
        ValueError for redirected, nonempty non-venv or incompatible destinations.
    """
    if destination.is_symlink():
        raise ValueError("The environment directory must not be a symlink.")
    if not destination.exists():
        return False
    if not destination.is_dir():
        raise ValueError("Environment destination is not a directory.")
    if not any(destination.iterdir()):
        return False
    if not (destination / "pyvenv.cfg").is_file():
        raise ValueError("Destination is nonempty and is not a virtual environment; choose --venv PATH.")
    data = probe([str(environment_python(destination))])
    if (Path(data["prefix"]).resolve() != destination.resolve()
            or data["prefix"] == data["base_prefix"] or not data["pip"]):
        raise ValueError("Existing destination must be an isolated Python 3.10 venv with pip.")
    return True


def install_prerequisites():
    """Install Ubuntu 22.04 Python prerequisites only after the explicit flag.

    Returns:
        None after apt completes; does not install MERLIN globally.
    Raises:
        ValueError on other OS releases; subprocess errors on apt failure.
    """
    info = ubuntu_info()
    if sys.platform != "linux" or info.get("ID") != "ubuntu" or info.get("VERSION_ID") != "22.04":
        raise ValueError("Automatic apt prerequisites are supported only on Ubuntu 22.04.")
    run(["sudo", "apt-get", "update"])
    run(["sudo", "apt-get", "install", "-y", "python3.10", "python3.10-venv"])


def download(url, destination, limit):
    """Download a bounded HTTPS response to a temporary file.

    Args:
        url: Fixed-repository HTTPS release URL.
        destination: Temporary output path, never a source capture.
        limit: Maximum accepted bytes.
    Returns:
        None when all bytes are stored.
    Raises:
        ValueError on size limits; I/O/network errors on download failure.
    """
    request = urllib.request.Request(url, headers={"User-Agent": "MERLIN-Python-Installer"})
    with urllib.request.urlopen(request, timeout=NETWORK_TIMEOUT_SECONDS) as response:
        with destination.open("xb") as output:
            total = 0
            while True:
                chunk = response.read(64 * 1024)
                if not chunk:
                    break
                total += len(chunk)
                if total > limit:
                    raise ValueError("Release download exceeds the installer size limit.")
                output.write(chunk)


def verified_wheel(tag, version, suffix, directory):
    """Resolve published assets and verify a wheel before environment creation.

    Args:
        tag: Validated release tag.
        version: Matching numeric package version.
        suffix: Validated native wheel platform.
        directory: Temporary download directory.
    Returns:
        Local wheel path after exact SHA-256 verification.
    Raises:
        ValueError for absent assets, draft releases or invalid checksums.
    """
    name = "merlin_ofdm-{}-cp310-cp310-{}.whl".format(version, suffix)
    metadata = directory / "release.json"
    download("https://api.github.com/repos/{}/releases/tags/{}".format(REPOSITORY, tag), metadata, METADATA_LIMIT)
    release = json.loads(metadata.read_text())
    if not isinstance(release, dict) or release.get("tag_name") != tag or release.get("draft") or release.get("prerelease"):
        raise ValueError("Select a published stable release with the requested tag.")
    assets = release.get("assets", [])
    if not isinstance(assets, list) or any(not isinstance(asset, dict) for asset in assets):
        raise ValueError("Invalid release asset metadata.")
    assets = {asset.get("name") for asset in assets}
    if not {name, "SHA256SUMS"}.issubset(assets):
        raise ValueError("Release does not contain the matching wheel and SHA256SUMS.")
    base = "https://github.com/{}/releases/download/{}/".format(REPOSITORY, tag)
    checksum_file = directory / "SHA256SUMS"
    download(base + "SHA256SUMS", checksum_file, METADATA_LIMIT)
    expected = []
    for line in checksum_file.read_text(encoding="ascii").splitlines():
        fields = line.split()
        if len(fields) == 2 and fields[1].lstrip("*") == name:
            expected.append(fields[0])
    if len(expected) != 1 or re.fullmatch(r"[a-fA-F0-9]{64}", expected[0]) is None:
        raise ValueError("Release checksum must identify this wheel exactly once.")
    wheel = directory / name
    download(base + name, wheel, WHEEL_LIMIT)
    digest = hashlib.sha256()
    with wheel.open("rb") as data:
        for chunk in iter(lambda: data.read(64 * 1024), b""):
            digest.update(chunk)
    if digest.hexdigest() != expected[0].lower():
        raise ValueError("Wheel SHA-256 mismatch; installation was not started.")
    return wheel


def source_requirements():
    """Validate checkout build tools without installing prerequisites.

    Returns:
        Checkout root after checking Cargo and a C linker.
    Raises:
        ValueError when the source tree or native build tools are missing.
    """
    root = Path(__file__).resolve().parents[2]
    if not (root / "tools/python/build_wheel.py").is_file():
        raise ValueError("Source mode requires a complete MERLIN checkout.")
    local_cargo = root / "local/cargo/bin"
    if (local_cargo / "cargo").is_file():
        os.environ.setdefault("CARGO_HOME", str(root / "local/cargo"))
        os.environ.setdefault("RUSTUP_HOME", str(root / "local/rustup"))
        os.environ["PATH"] = str(local_cargo) + os.pathsep + os.environ.get("PATH", "")
    if shutil.which("cargo") is None or shutil.which("cc") is None:
        raise ValueError("Source builds need pinned Rust 1.85.1 and a C linker; see docs/development/build.md.")
    run(["cargo", "+1.85.1", "--version"], capture=True)
    return root


def build_source(root, python, directory):
    """Build the native wheel using the repository's licensed packaging helper.

    Args:
        root: Validated source checkout.
        python: Private environment interpreter.
        directory: Temporary wheel output folder.
    Returns:
        The single newly built wheel; failures stop before installation.
    """
    run([python, "-I", "-m", "pip", "--isolated", "install", "maturin==1.8.3"])
    run([python, str(root / "tools/python/build_wheel.py"), "--out", str(directory)], timeout=3600)
    wheels = list(directory.glob("merlin_ofdm-*.whl"))
    if len(wheels) != 1:
        raise ValueError("Expected exactly one locally built MERLIN wheel.")
    return wheels[0]


def main(argv=None):
    """Check requirements and install one requested release into a project environment.

    Args:
        argv: Optional command-line arguments for testing.
    Returns:
        Zero on success; failures propagate to the CLI diagnostic handler.
    Raises:
        ValueError, OS or subprocess errors before reporting incomplete installation.
    """
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source", action="store_true", help="Build only the Python API from this checkout")
    parser.add_argument("--tag", help="Release tag or version, e.g. v0.3.1")
    parser.add_argument("--venv", type=Path, default=Path(".venv-merlin"), help="Destination (default: .venv-merlin)")
    parser.add_argument("--python", help="Path to an existing CPython 3.10 x64 interpreter")
    parser.add_argument("--check", "--dry-run", dest="check", action="store_true", help="Check local requirements only; no network or installation")
    parser.add_argument("--install-prerequisites", action="store_true", help="Explicitly permit apt Python prerequisites on Ubuntu 22.04")
    args = parser.parse_args(argv)
    if args.source and args.tag:
        parser.error("Choose --source or --tag, not both.")
    root = None
    if args.source:
        root = source_requirements()
        tag, version = release_identity((root / "VERSION").read_text().strip())
    elif args.tag:
        tag, version = release_identity(args.tag)
    else:
        parser.error("Select --tag VERSION or --source.")
    suffix = host_platform()
    if hasattr(os, "geteuid") and os.geteuid() == 0:
        raise ValueError("Run as your normal user; --install-prerequisites invokes sudo only for apt.")
    destination = args.venv.absolute()
    existing = check_destination(destination)
    if args.install_prerequisites and not args.check:
        install_prerequisites()
    command, capabilities = find_python(str(environment_python(destination)) if existing else args.python)
    if not existing and not (capabilities["venv"] and capabilities["ensurepip"]):
        raise ValueError("Python 3.10 venv/pip bootstrap is missing. Ubuntu 22.04: use --install-prerequisites.")
    print("Requirements passed: {} / CPython 3.10 x64".format(suffix), flush=True)
    print("Release: {}; environment: {}".format(tag, destination), flush=True)
    if args.check:
        print("Check only: no changes made; remote release availability has not been checked.")
        return 0
    with tempfile.TemporaryDirectory(prefix="merlin-python-install-") as temporary:
        wheel = None
        if not args.source:
            wheel = verified_wheel(tag, version, suffix, Path(temporary))
        if not existing:
            run(command + ["-I", "-m", "venv", str(destination)])
        python = str(environment_python(destination))
        if args.source:
            wheel = build_source(root, python, Path(temporary))
        run([python, "-I", "-m", "pip", "--isolated", "install", "--force-reinstall", "--no-index", "--only-binary=:all:", str(wheel)])
        check = "from merlin import Simulation; import merlin; assert merlin.__version__ == {!r}; print('MERLIN', merlin.__version__, 'ready')".format(version)
        run([python, "-I", "-c", check])
    print("Use this interpreter in your project: {}".format(python))
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError, subprocess.SubprocessError) as error:
        print("MERLIN installation failed: {}".format(error), file=sys.stderr)
        sys.exit(1)
