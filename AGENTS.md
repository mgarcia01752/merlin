# DOCSIS OFDM Channel Sim

DOCSIS OFDM Channel Sim is a standalone DOCSIS simulation
library and CLI. Work exclusively in this repository unless scope is changed.
For Qt framework reuse, read-only inspection of PNM-Viewer-Toolkit is authorized;
all modifications remain in MERLIN. Follow coding-style for reuse and licensing.

Focus on downstream OFDM first. OFDMA is deferred. Two simulation modes are the
ideal/reference experiment and measurement-driven analysis using RxMER, downstream
channel estimates and an applied modulation profile. Support is developed against
pinned DOCSIS3.1/4.0 evidence up to16384-QAM; do not equate a recognized order with
an implemented full coding chain or assume an optional modem capability.

The minimum engine is a MERLIN-only closed loop: source → MERLIN TX → channel
model → MERLIN RX → compare recovered bits against the original source. Another
implementation or external interoperability dataset is not an acceptance
prerequisite. Preserve normative/source-table checks and honest decoder outcomes;
do not describe closed-loop results as external interoperability evidence.

The CLI functional MVP is complete within its documented supported scope.
Native Qt UI development is the next phase; Qt is presentation only. Follow
the UI section in docs/architecture.md and docs/development/ui-checklist.md,
starting left to right with OFDM Channel Descriptor (OCD). The Qt workbench
shell, Rust-validated timing/interleaving editor, PRBS-TX source editor and DPD
binary/JSON capture importer are implemented. Source and OCD/profile readiness
are connected. A shared Spectrum Analyzer now shows actual complex-baseband TX
output with derived Transmitter readiness. Channel Estimate/RxMER now share
native input viewers with exact DPD validation, flat/unity response and Rust
combination/playback previews. Channel readiness reflects accepted configuration.
Receiver workbench navigation, NCP/PLC/profile QAM selectors and RX SA test-point
routing are implemented. A bounded Rust closed-loop worker drives Start/Stop/
Pause/Resume, accepted Receiver/PRBS-RX readiness and actual payload/PLC QAM, RX SA
and BER/FEC results. Native Open/Save/Save As persist self-contained workbench JSON.
Matched NCP/pilot I/Q and retained-data selectors, response/echo, slicer/LLR and
detailed FEC instruments are implemented for the supported transport;
Python bindings are deferred until after UI-01 through UI-04.
Dependencies point from CLI/future adapters to docsis-ofdm-fec to fec-codec.

Apply skills/coding-style, skills/scientific-testing, and the applicable versioned
PHY skill. Apply skills/public-api to public models and operations and
skills/docsis-signal-processing to signal-processing changes. Keep canonical skills under skills/ with relative discovery links
under .agents/skills/ and .claude/skills/.

- All signal processing, channel models, modulation/demapping, FEC and simulation
  orchestration are 100% Rust, including dependencies providing those algorithms.
  Do not add C/C++ DSP/codec implementations, native FFI or Python calculations.
  Keep simulation crates free of unsafe code. CXX-Qt/native icon interop is confined
  to the separate GPL Qt presentation adapter. Development tooling may use Python.
- Reusable code and tools are Apache-2.0. Preserve copied copyrights and third
  party licenses, including the Rust-adapted MIT BCH implementation and ldpc-toolbox.
- Follow meaningful domain aliases, descriptive variables/lifetimes and multiline
  public API documentation. New implementations belong in focused modules.
- Preserve exact source grids, nulls, excluded carriers, hashes and timestamps.
  Applications retain original capture bytes; decoded input contracts preserve provenance.
- Keep measured observations, analytical screening and simulated FEC outcomes
  distinct. Do not claim a DOCSIS receiver from a generic coding experiment.
- Record pinned normative evidence. Never infer missing shortening, interleaving,
  scrambling or waveform behavior from generic DVB support.
- Bound files, trials, iterations, runtime and memory. Preserve completed FEC
  trials on cancellation. Never overwrite user inputs or export destinations.
- Keep private PDFs and source captures under ignored docs/development/local/. Do not ship
  migration prototypes, private evidence, Cargo caches or build outputs.
- Plan ongoing implementation under docs/development/features/<feature>/; keep
  public plans, design notes and validation summaries tracked and linked from
  docs/development/README.md. Keep specs, captures and generated artifacts under
  ignored docs/development/local/<feature>/. Maintain user guides under docs/user/.
  Update the architecture when ownership or input contracts change.
- No SNMP/TFTP, device capture/control, network analysis or Python sidecars.
- No commits, pushes, publication or system-wide installation without explicit
  authorization. Supply commands when asked for commit/push commands.

Development setup uses `./install.sh --development` on the initial Ubuntu x64
targets, with read-only `--check` and `--dry-run` modes. The PNM Rust decoder is
an exact Git/version dependency fetched into ignored Cargo caches; do not vendor
PNM Viewer source. Keep source pins, developer prerequisites and setup docs current.
No setup invocation or system package installation is implied by ordinary UI work.

Validation scope:

For UI-only work, use scripts/qt-desktop.sh build/check and targeted Qt tests.
Do not rebuild or retest the non-Qt workspace on every UI iteration. Broaden
validation when simulation/shared code, dependency contracts, or integration
changes make it necessary. Release/push gates retain their required checks.
Preserve Cargo target artifacts and the stable build profile so unchanged code
is ready for reuse/linking; do not routinely clean or delete build outputs.
Keep the Qt executable built for the latest UI changes before handing off.

Simulation/shared-code validation when applicable:

```bash
scripts/cargo-local.sh fmt --all -- --check
scripts/cargo-local.sh test --workspace --locked
scripts/cargo-local.sh clippy --workspace --all-targets --locked -- -D warnings
scripts/cargo-local.sh test -p docsis-ofdm-fec --no-default-features --locked
```

Local full Rust validation uses `scripts/test-local.sh all`: checksum-pinned
nextest runs independent cases in separate processes, four concurrently by
default. Set `MERLIN_TEST_THREADS` to override the 1..32 process budget. Preserve
doctests, examples and analysis-only coverage and the serialized pause case.
Qt/QML checks remain separate. Hosted workflows retain their own commands.
See docs/development/features/parallel-local-tests/design.md for measurement.

The initial toolkit version is 0.1.0. Source packages and VERSION must agree.
Native CLI package builders/workflows are prepared locally. Cross-platform
runtime, native installer lifecycles and remote publication remain unverified.

## Git and releases

Apply skills/git-workflow and skills/toolkit-release for these operations.
Use tools/git/git-save.sh --commit-msg and tools/git/git-push.sh. Inspect all
tracked/staged/untracked changes before writing a message. Format Feature:,
BugFix: or Documentation:, followed by - detail lines and - Tests:, at most60
characters per message line. Supply commands unless asked to execute.
Begin Git command blocks with cd /home/dev01/Projects/merlin &&.

VERSION is canonical. tools/bump-version.sh synchronizes every local package and
Cargo.lock. Combined --tag-patch/--tag-minor/--tag-major modes require clean master,
validate the bumped candidate, commit version files, annotate and atomically push.
Ordinary branch pushes may automatically skip local builds/tests only for
verified documentation-only outgoing commits; see skills/git-workflow. Initial
pushes and tag/release modes default to full validation. Explicit `--skip-test`
omits local Rust tests and simulation smokes only; version, tooling, style,
Clippy, release build and Git guards remain. Hosted CI remains mandatory. No Git force-push. --force permits a fresh release only
from an already published clean HEAD. --tag retries without another bump.
Prepared native validation/publication workflows live under .github/workflows/.
Keep them local until the user requests pushing; file presence is not runtime
evidence. Native CLI packaging lives under tools/packaging/.

## Source module ownership

Keep fec-codec split into parameters, BCH, LDPC and concatenated codec modules.
The concatenated path must be usable without modulation, noise or capture models.
In ofdm, scenario owns observations/grids/association; profile owns hypotheses
and evaluation; signal_processing owns numerical algorithms; simulation owns
source generation, job orchestration and results. Keep CLI main focused on
startup, with arguments, commands and bounded JSON I/O in separate modules.
Preserve existing public imports with re-exports rather than duplicate logic.
Use docs/architecture.md as the canonical design and
docs/development/checklist.md as the official progress/TODO list. Update them
when stage ownership, test points or implementation status changes. At feature
completion and each commit-msg request, review both documents, update them where
needed before returning the command, and keep partial work explicitly unchecked.
Do not edit them solely to record that a review occurred.
PRBS15 is a separate continuous test source, not a label for the existing
seeded random generator or a substitute for PHY scrambling. Preserve its
explicit recurrence, seed/bit order and independent post-decode verification.

When the user says "next", implement the next unfinished item in the official
checklist/architecture in dependency order. Keep its status accurate and report
any prerequisite gaps rather than substituting unrelated work.


## Current desktop delivery scope

Ubuntu 22.04 amd64 is the current native desktop package target. Follow
skills/toolkit-release and the Ubuntu desktop design for DEB/workflow work.
Keep update HTTP, metadata checks and global preferences in the application
adapter, separate from simulation and WB JSON. Use a quiet bounded worker and
OS-browser release-page handoff. Desktop delivery on other platforms is deferred;
development support and existing CLI validation are separate. Audit both locks
without suppressions; only report installed/hosted behavior with actual evidence.
