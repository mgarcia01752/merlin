# DOCSIS OFDM Channel Sim

DOCSIS OFDM Channel Sim is a standalone DOCSIS scientific
library and CLI. Work exclusively in this repository unless scope is changed.
For Qt framework reuse, read-only inspection of PNM-Viewer-Toolkit is authorized;
all modifications remain in MERLIN. Follow coding-style for reuse and licensing.

Focus on downstream OFDM first. OFDMA is deferred. Two scientific modes are the
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
starting left to right with OFDM Settings. The initial Qt workbench shell exists; editors and run integration remain pending;
Python bindings are deferred until after UI-01 through UI-04.
Dependencies point from CLI/future adapters to docsis-ofdm-fec to fec-codec.

Apply skills/coding-style, skills/scientific-testing, and the applicable versioned
PHY skill. Apply skills/public-api to public models and operations and
skills/docsis-signal-processing to scientific changes. Keep canonical skills under skills/ with relative discovery links
under .agents/skills/ and .claude/skills/.

- All signal processing, channel models, modulation/demapping, FEC and scientific
  orchestration are 100% Rust, including dependencies providing those algorithms.
  Do not add C/C++ DSP/codec implementations, native FFI or Python calculations.
  Keep scientific crates free of unsafe code. CXX-Qt/native icon interop is confined
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

Validation scope:

For UI-only work, use scripts/qt-desktop.sh build/check and targeted Qt tests.
Do not rebuild or retest the non-Qt workspace on every UI iteration. Broaden
validation when scientific/shared code, dependency contracts, or integration
changes make it necessary. Release/push gates retain their required checks.
Preserve Cargo target artifacts and the stable build profile so unchanged code
is ready for reuse/linking; do not routinely clean or delete build outputs.
Keep the Qt executable built for the latest UI changes before handing off.

Scientific/shared-code validation when applicable:

```bash
scripts/cargo-local.sh fmt --all -- --check
scripts/cargo-local.sh test --workspace --locked
scripts/cargo-local.sh clippy --workspace --all-targets --locked -- -D warnings
scripts/cargo-local.sh test -p docsis-ofdm-fec --no-default-features --locked
```

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
pushes and tag/release modes always require full validation. No unrestricted
validation bypass or Git force-push. --force permits a fresh release only
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
