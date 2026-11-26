# Mandatory High-Level API Integration Contract

Instruction source: the user's addition to the unified Qt master guide. It takes
precedence over the guide's permission to use underlying scientific owners.

The Qt Workbench MUST use MERLIN's accepted high-level Rust API as its primary
and authoritative interface to both PHY simulation engines:

```rust
use merlin::downstream;
use merlin::upstream;
use merlin::common;
```

## Architecture rules

1. All new Qt simulation operations MUST use the high-level Rust API.
2. The Qt application MUST NOT directly invoke internal OFDM/OFDMA simulation
   engines, DSP kernels, FEC implementations, or private scientific modules.
3. CXX-Qt QObject adapters remain thin: translate QML requests into validated
   public API calls, manage UI session references, dispatch long-running work
   without blocking Qt, map API errors/progress/results into bounded UI data,
   and manage ownership/cancellation.
4. Scientific configuration, validation, execution and result generation stay
   in the Rust backend.
5. Use separate direction adapters for downstream and upstream.
6. Reuse common contracts only where compatible.
7. Shared QAM, spectrum, PRBS, FEC and channel views consume public API results.
8. Do not duplicate the high-level API with a Qt-specific simulation interface.

## Missing functionality

Identify the missing operation/observation and scientific owner. Extend the
appropriate high-level API with a small typed validated operation, add
architecture/requirements/checklist traceability and scientific/API tests,
validate existing consumers, then commit the focused API unit before exposing
it through Qt. Never access private structures as a workaround. In particular,
the OFDMA pilot/resource grid must come from the upstream public API rather
than allocation/pilot calculations in Qt/QML.

## Downstream migration and enforcement

Audit all direct `docsis-ofdm-fec` dependencies. Preserve working downstream
presentation and behavior; provide a compatible public API extension before
removing a legacy scientific dependency. Final Qt simulation dependencies use
the `merlin` facade. Add an automated dependency/source boundary check; document
narrow non-simulation infrastructure, build and presentation exceptions.

Acceptance requires both workbenches on the unified public Rust API, no new
Qt-specific scientific algorithms/private scientific access, independently
validated API additions, compatible downstream behavior, authoritative scientific
instruments, separate direction lifecycle/configuration and passing public API
compatibility plus architectural checks.
