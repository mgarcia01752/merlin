#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Build local high-level Rust and native Python API HTML, without publication."""
import argparse
import html
import importlib.machinery
import inspect
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import time

from api_reference_support import public_classes, public_members

ROOT = Path(__file__).resolve().parents[2]
OUTPUT = ROOT / "local/api-reference"


def native_inventory(module, expected_version):
    """Validate a real native extension and enumerate its exported API metadata.

    Args:
        module: Imported merlin extension; no object construction is performed.
        expected_version: Checkout VERSION string required for matching references.
    Returns:
        JSON-compatible class/member/signature inventory.
    Raises:
        ValueError for a mocked/non-native module, version drift or export omission.
    """
    # Maturin wheels expose a thin package re-exporting merlin.merlin; source
    # development installs can expose the extension directly. Validate either.
    native = getattr(module, "merlin", module)
    artifact = Path(native.__file__).resolve()
    if not any(str(artifact).endswith(suffix)
               for suffix in importlib.machinery.EXTENSION_SUFFIXES):
        raise ValueError("Documentation requires the real native merlin extension")
    if module.__version__ != expected_version:
        raise ValueError("Installed merlin version differs from checkout VERSION; rebuild/install the wheel")
    classes = []
    for name, value in public_classes(native):
        try:
            signature = str(inspect.signature(value))
        except (TypeError, ValueError):
            signature = None
        classes.append({"name": name, "canonical_name": value.__name__,
                        "signature": signature, "constructible": "__new__" in vars(value),
                        "members": public_members(value)})
    return {"version": expected_version, "native_artifact": str(artifact), "classes": classes}


def generate_sources(inventory):
    """Stage Sphinx sources and one autodoc page per exported native class.

    Args:
        inventory: Validated native class inventory, including aliases.
    Returns:
        Ignored generated source directory, with no modifications to tracked guides.
    """
    source = OUTPUT / "python-source"
    shutil.copytree(ROOT / "docs/reference/python-api", source, dirs_exist_ok=True)
    listing = ["Classes and domain types", "========================", "",
               ".. toctree::", "   :maxdepth: 1", ""]
    for item in inventory["classes"]:
        name = item["name"]
        listing.append(f"   {name}")
        notice = ""
        if not item["constructible"]:
            notice = "Use the documented factory methods or returned results; no public constructor is provided.\n\n"
        text = (f"{name}\n{'=' * len(name)}\n\n{notice}.. currentmodule:: merlin\n\n"
                f".. autoclass:: {name}\n   :members:\n   :undoc-members:\n"
                "   :special-members: __iter__, __len__, __getitem__\n")
        (source / f"{name}.rst").write_text(text, encoding="utf-8")
    (source / "classes.rst").write_text("\n".join(listing) + "\n", encoding="utf-8")
    return source


def validate_pages(inventory):
    """Require searchable output and every declared public native member anchor.

    Args:
        inventory: Native export inventory used to generate the pages.
    Raises:
        ValueError if Sphinx omits a class/member, search page or readable docstring.
    """
    for item in inventory["classes"]:
        name = item["name"]
        page = (OUTPUT / "python" / f"{name}.html").read_text(encoding="utf-8")
        for member in item["members"]:
            if f'id="merlin.{name}.{member}"' not in page:
                raise ValueError(f"Missing generated API member: {name}.{member}")
        if "Interpreter token" in page or "* `py`" in page:
            raise ValueError(f"Hidden interpreter argument leaked into {name} docs")
    for name in ("search.html", "searchindex.js", "genindex.html"):
        if not (OUTPUT / "python" / name).is_file():
            raise ValueError(f"Missing searchable reference output: {name}")


def main():
    """Build strict Rust/Python references with the current interpreter and checkout.

    Args:
        CLI --python-only skips rustdoc; it does not refresh the Rust output.
    Returns:
        None after printing the local landing path and total wall time.
    Raises:
        Import/validation/subprocess errors; no publishing or installation occurs.
    """
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--python-only", action="store_true", help="Skip rustdoc generation")
    args = parser.parse_args()
    started = time.perf_counter()
    import merlin
    import sphinx
    if sphinx.__version__ != "8.1.3":
        raise ValueError("Install tools/python/api-docs-requirements.txt in this local environment")
    inventory = native_inventory(merlin, (ROOT / "VERSION").read_text().strip())
    # Output ownership is fixed to ignored local/: reject redirected staging.
    for directory in (ROOT / "local", OUTPUT):
        if directory.is_symlink():
            raise ValueError("Refuse redirected API documentation output")
    if OUTPUT.exists() and any(path.is_symlink() for path in OUTPUT.rglob("*")):
        raise ValueError("Refuse redirected generated API documentation")
    marker = OUTPUT / ".merlin-api-reference"
    if OUTPUT.exists() and any(OUTPUT.iterdir()) and not marker.is_file():
        raise ValueError("Refuse unrecognized documentation output; choose a clean local/api-reference directory")
    OUTPUT.mkdir(parents=True, exist_ok=True)
    marker.write_text("MERLIN generated API documentation\n", encoding="utf-8")
    # Remove only this helper's generated documentation, never Cargo outputs.
    directories = ["python-source", "python"]
    if not args.python_only:
        directories.append("rust")
    for name in directories:
        if (OUTPUT / name).exists():
            shutil.rmtree(OUTPUT / name)
    environment = os.environ.copy()
    environment.pop("PYTHONPATH", None)
    environment["MERLIN_DOC_ROOT"] = str(ROOT)
    if not args.python_only:
        environment["RUSTDOCFLAGS"] = environment.get("RUSTDOCFLAGS", "") + " -D warnings"
        subprocess.run([str(ROOT / "scripts/cargo-local.sh"), "doc", "-p", "docsis-ofdm-fec",
                        "-p", "pnm-input", "--features", "high-level-api", "--no-deps",
                        "--locked", "--target-dir", str(ROOT / "target")],
                       cwd=ROOT, env=environment, check=True)
        shutil.copytree(ROOT / "target/doc", OUTPUT / "rust", dirs_exist_ok=True)
    source = generate_sources(inventory)
    subprocess.run([sys.executable, "-m", "sphinx", "-b", "html", "-W", "--keep-going",
                    "-E", str(source), str(OUTPUT / "python")],
                   cwd=ROOT, env=environment, check=True)
    validate_pages(inventory)
    (OUTPUT / "inventory.json").write_text(json.dumps(inventory, indent=2) + "\n", encoding="utf-8")
    links = [("python/index.html", "Python high-level API"),
             ("rust/docsis_ofdm_fec/api/index.html", "Rust high-level API"),
             ("rust/pnm_input/index.html", "Rust PNM inputs and Workbench"),
             ("../../docs/reference/api-reference.md", "Build instructions and scope"),
             ("../../examples/api/README.md", "Matching Rust and Python examples")]
    body = "".join(f'<li><a href="{url}">{label}</a></li>' for url, label in links
                   if (OUTPUT / url).is_file())
    (OUTPUT / "index.html").write_text(
        '<!doctype html><html lang="en"><meta charset="utf-8">'
        '<title>MERLIN API references</title><h1>MERLIN API references</h1>'
        f'<p>Version {html.escape(inventory["version"])}; generated locally from Rust and the native extension.</p>'
        f'<ul>{body}</ul></html>\n', encoding="utf-8")
    print(f"API reference: {OUTPUT / 'index.html'}")
    print(f"Documented {len(inventory['classes'])} exported Python classes")
    print(f"Total API reference build time: {time.perf_counter() - started:.2f}s")


if __name__ == "__main__":
    main()
