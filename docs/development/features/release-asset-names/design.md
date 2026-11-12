# RELEASE-ASSET-01: Consistent distributable filenames

Canonical pattern: merlin-<desktop|cli>_<MAJOR.MINOR.PATCH>_<target>_<arch>.<format>.
Use amd64 consistently in filenames (internal build/WiX x64 identifiers remain).
Ubuntu DEBs state ubuntu22.04; portable Linux archives state linux. Extend the
same component/version/platform/architecture order to prepared macOS/Windows
CLI builders without claiming those targets have been validated or released.

A shared Python name builder validates combinations and is used by CLI archive,
native installer and desktop builders. Preserve package IDs merlin/merlin-ofdm,
executable names and paths so installed upgrades keep ownership and identity.
Update docs, release skill and packaging tests. Do not modify public releases.

Compatibility: 0.1.4 and earlier updaters select only the old desktop filename.
New releases include an identical legacy desktop alias and checksum until those
clients are explicitly retired. New updater code prefers the canonical desktop
asset and permits an exact legacy fallback; retain digest/URL/size/duplicate and
package-identity checks. A malformed canonical candidate must not fall back to
legacy. CI smoke/install steps select only the canonical DEB, not both aliases.

Validate pure naming contracts across targets, a tiny native DEB without system
installation, alias byte/checksum equality, collection behavior, updater selection
and install URL checks, then the Qt checks/native build. Keep runtime/release
publication claims separate from local tests.

## Validation

14 packaging tests pass, including an actual tiny DEB build/metadata inspection
without installation, portable archive layout, alias bytes/checksums, no-clobber,
rendered release-version APT commands and generic local-file guide instructions.
The edited workflow YAML parses and its contract test passes. Full Qt check
passes rustfmt, warnings-denied Clippy, 45 adapter tests, native build and 46 QML
tests. Updater tests cover canonical preference, legacy-only fallback, invalid
canonical rejection and exact download identities. The release skill validates.

No full installer, installed upgrade, hosted workflow or new platform runtime
was executed in this task. Existing published release assets and notes were not
modified. Canonical filenames first appear in the next release after this change.
