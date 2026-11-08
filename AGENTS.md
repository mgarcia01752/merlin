# DOCSIS OFDM FEC Toolkit

This repository owns offline OFDM profile science and real-codec experiments.
PNM Viewer Toolkit is a consumer. Keep dependencies one-way:
PNM Viewer -> docsis-ofdm-fec -> fec-codec. No Qt or PNM parser dependency here.

Apply skills/coding-style, skills/scientific-testing, and the applicable versioned
PHY skill. Keep canonical skills under skills/ with relative discovery links
under .agents/skills/ and .claude/skills/.

- Rust owns analysis and orchestration. Isolate native C++ in fec-codec's CXX
  boundary. Keep the scientific Rust library free of unsafe code.
- Reusable code and tools are Apache-2.0. Preserve copied copyrights and third
  party licenses, including the pinned MIT AFF3CT BCH kernel and ldpc-toolbox.
- Follow meaningful domain aliases, descriptive variables/lifetimes and multiline
  public API documentation. New implementations belong in focused modules.
- Preserve exact source grids, nulls, excluded carriers, hashes and timestamps.
  Consumers retain original capture bytes; do not duplicate PNM parsers here.
- Keep measured observations, analytical screening and simulated FEC outcomes
  distinct. Do not claim a DOCSIS receiver from a generic coding experiment.
- Record pinned normative evidence. Never infer missing shortening, interleaving,
  scrambling or waveform behavior from generic DVB support.
- Bound files, trials, iterations, runtime and memory. Preserve completed FEC
  trials on cancellation. Never overwrite user inputs or export destinations.
- Keep private PDFs and source captures under ignored development/. Do not ship
  migration prototypes, private evidence, Cargo caches or build outputs.
- Plan implementation under development/<feature>/ and maintain public docs in
  docs/. Update the architecture when ownership or input contracts change.
- No SNMP/TFTP, device capture/control, network analysis or Python sidecars.
- No commits, pushes, publication or system-wide installation without explicit
  authorization. Supply commands when asked for commit/push commands.

Validation:

```bash
scripts/cargo-local.sh fmt --all -- --check
scripts/cargo-local.sh test --workspace --locked
scripts/cargo-local.sh clippy --workspace --all-targets --locked -- -D warnings
scripts/cargo-local.sh test -p docsis-ofdm-fec --no-default-features --locked
```

The initial toolkit version is 0.1.0. Source packages and VERSION must agree.
No native installer or cross-platform runtime is validated yet.

## Git and releases

Apply skills/git-workflow and skills/toolkit-release for these operations.
Use tools/git/git-save.sh --commit-msg and tools/git/git-push.sh. Inspect all
tracked/staged/untracked changes before writing a message. Format Feature:,
BugFix: or Documentation:, followed by - detail lines and - Tests:, at most60
characters per message line. Supply commands unless asked to execute.
Begin Git command blocks with cd /home/dev01/Projects/DOCSIS-OFDM-FEC-Toolkit &&.

VERSION is canonical. tools/bump-version.sh synchronizes every local package and
Cargo.lock. Combined --tag-patch/--tag-minor/--tag-major modes require clean master,
validate the bumped candidate, commit version files, annotate and atomically push.
No validation bypass or Git force-push. --force permits a fresh release only
from an already published clean HEAD. --tag retries without another bump.
No automatic GitHub publication or installer workflow is configured yet.
