#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Build local Basic Rust API and Basic Python API HTML, without publication."""
import argparse
import hashlib
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

from api_reference_support import public_classes, public_members, public_operations

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
    # Mixed wheels use one private extension; older wheels used merlin.merlin.
    # Read actual native objects rather than the import-only compatibility shim.
    native = getattr(module, "_native", getattr(module, "merlin", module))
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
                        "module": value.__module__, "signature": signature, "constructible": "__new__" in vars(value),
                        "members": public_members(value), "operations": public_operations(value)})
    namespaces = {}
    for name in ("downstream", "upstream", "common"):
        direction = getattr(module, name, None)
        if direction is None:
            continue
        exported = []
        for public, value in public_classes(direction):
            try:
                signature = str(inspect.signature(value))
            except (TypeError, ValueError):
                signature = None
            exported.append({"name": public, "canonical_name": value.__name__,
                             "module": value.__module__, "signature": signature,
                             "constructible": "__new__" in vars(value),
                             "members": public_members(value), "operations": public_operations(value)})
        namespaces[name] = exported
    return {"version": expected_version, "native_artifact": str(artifact),
            "classes": classes, "namespaces": namespaces}


def reference_entries(inventory):
    """Resolve each exported import name to one canonical documented class.

    Args:
        inventory: Root and directional class metadata from the installed wheel.
    Returns:
        Entries with full import names, page paths and canonical member targets.
        Aliases retain their own class anchor and link to the owning class page.
    """
    groups = [("merlin", inventory["classes"])]
    groups.extend(("merlin." + name, items)
                  for name, items in inventory.get("namespaces", {}).items())
    entries = {}
    for module, items in groups:
        for item in items:
            name = item["name"]
            full_name = module + "." + name
            if full_name in entries:
                raise ValueError(f"Duplicate documented import: {full_name}")
            prefix = module.removeprefix("merlin").lstrip(".").replace(".", "/")
            page = (Path(prefix) / name).as_posix()
            entries[full_name] = {"full_name": full_name, "page": page,
                                  "item": item, "module": module}
    for entry in entries.values():
        item = entry["item"]
        canonical = item.get("module", entry["module"]) + "." + item.get("canonical_name", item["name"])
        target = entries.get(canonical, entry)
        entry["target_name"] = target["full_name"]
        entry["target_page"] = target["page"]
    return list(entries.values())


def generate_sources(inventory):
    """Stage searchable root and namespace pages from installed native exports.

    Args:
        inventory: Validated native class inventory, including exact aliases.
    Returns:
        Ignored generated source directory; tracked guides remain unchanged.
    """
    source = OUTPUT / "python-source"
    shutil.copytree(ROOT / "docs/reference/python-api", source, dirs_exist_ok=True)
    listings = {}
    for entry in reference_entries(inventory):
        item, name = entry["item"], entry["full_name"]
        listings.setdefault(entry["module"], []).append(item["name"])
        notice = ""
        if not item["constructible"]:
            notice = "Use the documented factory methods or returned results; no public constructor is provided.\n\n"
        text = (f"{name}\n{'=' * len(name)}\n\n{notice}"
                f".. autoclass:: {name}\n   :members:\n   :undoc-members:\n"
                "   :special-members: __iter__, __len__, __getitem__\n")
        if entry["target_name"] != name:
            text = (f"{name}\n{'=' * len(name)}\n\n"
                    f"Alias for :class:`{entry['target_name']}`; both imports name the same native class.\n\n"
                    f".. autoclass:: {name}\n\n"
                    f"See :doc:`/{entry['target_page']}` for all methods and properties.\n")
        path = source / (entry["page"] + ".rst")
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(text, encoding="utf-8")
    for module, names in listings.items():
        prefix = module.removeprefix("merlin").lstrip(".")
        title = module + (" compatibility imports" if not prefix else " namespace")
        listing = [title, "=" * len(title), "", ".. toctree::", "   :maxdepth: 1", ""]
        listing.extend("   " + name for name in names)
        path = source / prefix / "index.rst" if prefix else source / "classes.rst"
        path.write_text("\n".join(listing) + "\n", encoding="utf-8")
    return source


def validate_pages(inventory):
    """Require each import anchor, canonical member and searchable index.

    Args:
        inventory: Actual native export inventory used for this Sphinx build.
    Raises:
        ValueError for missing class/member anchors or leaked interpreter tokens.
        OSError for absent generated pages or search artifacts.
    """
    for entry in reference_entries(inventory):
        name = entry["full_name"]
        page = (OUTPUT / "python" / (entry["page"] + ".html")).read_text(encoding="utf-8")
        if f'id="{name}"' not in page:
            raise ValueError(f"Missing generated API class: {name}")
        member_page = (OUTPUT / "python" / (entry["target_page"] + ".html")).read_text(encoding="utf-8")
        target_name = entry["target_name"]
        for member in entry["item"]["members"]:
            if f'id="{target_name}.{member}"' not in member_page:
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
    inventory["native_sha256"] = hashlib.sha256(Path(inventory["native_artifact"]).read_bytes()).hexdigest()
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
                        "-p", "pnm-input", "-p", "merlin", "-p", "merlin-api",
                        "-p", "docsis-ofdma", "--features", "high-level-api", "--no-deps",
                        "--locked", "--target-dir", str(ROOT / "target")],
                       cwd=ROOT, env=environment, check=True)
        shutil.copytree(ROOT / "target/doc", OUTPUT / "rust", dirs_exist_ok=True)
    source = generate_sources(inventory)
    subprocess.run([sys.executable, "-m", "sphinx", "-b", "html", "-W", "--keep-going",
                    "-E", str(source), str(OUTPUT / "python")],
                   cwd=ROOT, env=environment, check=True)
    validate_pages(inventory)
    (OUTPUT / "inventory.json").write_text(json.dumps(inventory, indent=2) + "\n", encoding="utf-8")
    links = [("python/index.html", "Basic Python API"),
             ("rust/merlin/index.html", "Unified Rust API"),
             ("rust/docsis_ofdm_fec/api/index.html", "Legacy downstream Rust API"),
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
    print(f"Documented {len(reference_entries(inventory))} Python class imports across root and namespaces")
    print(f"Total API reference build time: {time.perf_counter() - started:.2f}s")


if __name__ == "__main__":
    main()
