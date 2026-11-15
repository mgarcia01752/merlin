# MERLIN native Python adapter

Optional PyO3/maturin workspace consuming the canonical high-level Rust API and
`pnm-input`. No Qt, CLI subprocess, Python scientific calculations or second
engine. Distribution name `merlin-ofdm`, import name `merlin`.

See [setup and usage](../../docs/user/python/README.md),
[runnable examples](../../examples/api/README.md),
[shared Rust workflow](../../docs/user/api/README.md) and
[feature evidence](../../docs/development/features/high-level-api/validation.md).
Build with the repository's `tools/python/build_wheel.py` helper so complete
locked dependency notices and embedded QAM/BCH notices accompany the wheel.

The supported development verification target is Ubuntu 22.04 x86_64 / CPython
3.10. Other targets are unverified. Artifacts are local and unpublished.
