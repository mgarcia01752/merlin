# Generated Rust and Basic Python API reference

[Documentation index](../README.md) · [Workflow guide](../user/api/README.md) · [Examples](../../examples/api/README.md)

MERLIN provides generated HTML for the Rust API and Basic Python API:

| Reference | Source of truth | Main entry points |
| --- | --- | --- |
| Rust | Rust public items and rustdoc comments | `docsis_ofdm_fec::api` and `pnm_input`, including `Workbench` |
| Basic Python API | Signatures, classes, methods, properties and docstrings from the installed native extension | `merlin.Simulation` (legacy alias `Merlin`), input/selection/options/report types and `merlin.Workbench` |

These are searchable class/method references, complementing the written guides.
Python uses pinned [Sphinx autodoc](https://www.sphinx-doc.org/en/master/usage/extensions/autodoc.html)
with the real extension, without mocked types or a second Python implementation.
The builder discovers every exported native class, generates its own page and
requires an HTML anchor for every declared public method/property/enum member.
Rustdoc and Sphinx warnings fail the build. Neither documentation build runs a
simulation or needs Qt, a modem or a server.

## Build both references

From the checkout root, prepare a separate local documentation environment:

```bash
cd /home/dev01/Projects/merlin &&
python3 -m venv local/api-docs &&
PYTHONPATH= local/api-docs/bin/python -m pip install -r tools/python/api-docs-requirements.txt
```

Build the current wheel using the [Python setup guide](../user/python/README.md).
Install that wheel into the documentation environment, then build:

```bash
cd /home/dev01/Projects/merlin &&
PYTHONPATH= local/api-docs/bin/python -m pip install --no-index --force-reinstall \
  local/python-wheels/merlin_ofdm-0.2.9-cp310-cp310-linux_x86_64.whl &&
PYTHONPATH= local/api-docs/bin/python tools/python/build_api_reference.py
```

The wheel filename shown is the tested development target; use the matching
current version/interpreter artifact. The helper rejects an installed version
that differs from checkout `VERSION`. Rebuild and reinstall after binding or
Rust API changes, even when the version number is unchanged: the Python reference
documents the installed binary, not uncompiled source edits. `inventory.json`
records its actual path, version and all documented class/member identities.

Clearing `PYTHONPATH` keeps unrelated source checkouts out of reference generation.
The pinned dependency file is development-only and includes Sphinx dependencies;
it adds no dependencies to the Rust library, CLI, Qt app or Python runtime wheel.
Initial dependency/Cargo setup may need network access. Once dependencies and the
matching native wheel are installed/cached, references can be built offline.

## View the result

Open `local/api-reference/index.html` in a browser. It links to:

- `python/index.html`: Python reference, class index and full-text search.
- `python/Simulation.html`: session loading, controller methods and properties.
- `python/Workbench.html`: saved-workbench loading and execution.
- `rust/docsis_ofdm_fec/api/index.html`: Basic Rust API types and operations.
- `rust/pnm_input/index.html`: native binary/JSON/file loading and WB adapter.

The landing page also links to the written guides and runnable examples.
Generated HTML remains under ignored `local/`; do not commit or publish it.
The helper reuses the normal Rust target cache, stages rustdoc output beside the
Python pages and replaces only its own generated documentation directories.
`--python-only` refreshes Python without rebuilding Rust; any existing Rust pages
retain their previous content. A standalone Rust rebuild is also available:

```bash
cd /home/dev01/Projects/merlin &&
RUSTDOCFLAGS="-D warnings" scripts/cargo-local.sh doc \
  -p docsis-ofdm-fec -p pnm-input --features high-level-api --no-deps --locked
```

## Maintain the reference

Edit documentation alongside the owning public Rust item or PyO3 binding.
Python-facing docstrings must describe Python parameters; hidden interpreter
tokens are not public arguments. Describe units, defaults, validation, ownership,
side effects, structured errors and partial results where relevant. Keep source
code expressions in backticks so rustdoc does not interpret brackets as links.
The Sphinx hook only converts existing Rust section/inline markup for rendering;
it does not calculate scientific values or invent missing parameter semantics.
The Python workflow page links the generated types and explains configuration,
selection, defaults, cancellation, results and errors. Input quantities document
Rust-validated units/ranges; result quantities identify their units and ownership.

Run tooling checks with:

```bash
cd /home/dev01/Projects/merlin &&
PYTHONPATH= python3 -m unittest discover -s tools/python -p test_api_reference.py -v
```

The first validated reference build target is Ubuntu x86_64 with CPython 3.10.
Other Python versions/operating systems and hosted publication remain unverified.
