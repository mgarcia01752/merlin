# Python Guide

Python bindings are **planned, not implemented**. Development is deferred until
Qt UI tasks UI-01 through UI-04 are complete. There is currently no MERLIN Python
package to install or import.

The proposed `MerlinSimulation` class will provide settings, source selection,
measurement inputs, run limits and results through the existing Rust engine.
Signal processing, modulation and FEC will remain in Rust. Package naming,
supported Python versions and installation instructions are not finalized.

See the [adapter design](../../development/features/python-adapter/design.md)
for the proposed interface. Its code sketch is a design example, not a usable
Python API. Available interfaces are the [Qt workbench](../workbench.md),
[Rust API](../../reference/public-api.md) and [CLI](../channel-runner/cli.md).

Return to the [documentation index](../../README.md).
