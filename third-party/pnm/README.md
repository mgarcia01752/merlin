# Local PNM decoder source

MERLIN builds its PNM adapter and Qt application against
`crates/core/Cargo.toml` in this directory. No other repository, checkout,
repository credential or Git source cache is required. Normal locked registry
dependencies and development toolchain downloads remain supported.

This is the reusable Apache-2.0 library from source revision
`5fbd5561f3e06b73486546b5ffa520279080a5eb`, package `pnm` 0.3.6.
`provenance.json` records the original repository, exact revision and SHA-256/Git
blob identity of every imported file. Scientific/parser source, resource data,
tests and the nine required licensed MAC-sanitized fixtures are unchanged.
Three readmes have local links/commands adapted, with their original bytes retained
as `README.upstream.txt` and independently hash-verified. The local workspace
includes the original logging and fixture development dependencies so the
original tests and examples remain independently usable. Its package versions
remain upstream versions and are excluded from MERLIN version synchronization.

Original copyright/SPDX headers, component licensing, full Apache license,
NOTICE and fixture/table provenance are preserved. No Qt/viewer application
code was imported. Original example/resource documentation is retained as source
reference material; MERLIN's maintained guides remain under `docs/`.

Run the original decoder tests from MERLIN's root:

```bash
scripts/cargo-local.sh test --manifest-path third-party/pnm/Cargo.toml --workspace --locked
```

The local source verifier and packaging notice collector include this dependency.
Fresh-checkout acceptance must use an empty Cargo source cache with Git repository
fetching unavailable; warm caches are not independence evidence.
