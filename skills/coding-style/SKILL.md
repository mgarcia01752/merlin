---
name: coding-style
description: Apply DOCSIS OFDM FEC Toolkit coding conventions when writing, modifying, or reviewing project-owned code, including SPDX headers, full public-function documentation banners, multiline branches and focused Rust scientific modules.
---

# DOCSIS OFDM FEC Toolkit coding style

Ported from PNM Viewer Toolkit coding-style on 2026-11-08.
Originally adapted from OSCAR-UI on 2026-11-03.
Source checkout HEAD: `4dfe461eab736a71a7dd35e71491fd83f74c4eb3` (working-tree skill reviewed).
Retain its documentation and readability conventions; Rust/native-codec ownership and
Local PHY evidence and native-codec ownership replace the desktop-specific rules.

## Member and module order

Place constants and type declarations before implementation blocks. Within an
inherent Rust impl, place constructors first, then public operations and private
helpers, keeping related operations together. Keep trait implementations cohesive
and each banner/attribute with its declaration. Do not alter visibility or behavior
just to satisfy ordering. Preserve initialization and execution order.

## Public function banners

Every explicitly declared public function, associated function, constructor-like
function and method must have a multiline documentation banner immediately above
its attributes or declaration. Include public trait methods and trait implementations,
accessors, overrides and entry points. Document public types and their fields where
needed; do not generate accessors solely to attach documentation.

For Rust, use `///` rustdoc containing:

- A clear summary and purpose.
- Caller-visible behavior: preconditions, side effects, state changes, threading,
  ownership and borrowing lifetimes where material.
- `# Arguments` describing every parameter/type parameter, units and constraints.
- `# Returns` describing results, empty values, None and borrowed data.
- `# Errors` describing intentional Result failures and triggers.
- `# Panics` only when caller-visible panics are possible; never invent guarantees.

Omit sections that do not apply. Use equivalent multiline JSDoc/TSDoc for exported
TypeScript functions, Python docstrings for public functions, and comment banners
for script entry points/reusable shell functions. A one-line comment, repeated
method name or bare inherited-doc reference is not a complete banner.

```rust
/// Decodes an RxMER capture while borrowing the supplied bytes.
///
/// # Arguments
/// * `bytes` - Complete capture bytes, retained by the caller for the result's lifetime.
/// * `limits` - Application sample-count policy, not a protocol limit.
///
/// # Returns
/// A typed capture exposing unchanged samples and shared header access.
///
/// # Errors
/// Returns a field/offset diagnostic for unsupported versions, truncated input,
/// inconsistent lengths or exceeded resource limits.
```

Keep banners consistent with actual signatures and behavior. In particular,
missing samples are not zeros and file-format versions are not DOCSIS versions.

## Regular-expression documentation

When adding/changing a regex, comment directly above its declaration or inline
use. Explain extraction/validation intent, a synthetic input and expected output,
consumed capture groups, whole-input versus substring behavior, flags,
preprocessing, empty values and excluded delimiters where relevant. Distinguish
extraction from validation. Update the example when behavior changes; comment-only
changes do not require new tests.

## Conditional statements

Put if/else bodies on separate lines with normal language block syntax, including
guards, returns, errors and branches inside closures. Use braces in Rust and
TypeScript. Do not replace branches with ternaries or short-circuit expressions
just to evade this rule. Keep match arms readable; do not change control flow
merely for formatting.

## Primitive representation and readable domain types

Use native Rust primitives as the underlying representation. As features are
added or APIs are modified, define meaningful domain types for values whose
purpose would otherwise be unclear, especially public fields and parameters.
Examples include MacAddress, ChannelId, FrequencyHz, SubcarrierIdx
and SimulationSeed. Type aliases, structs and enums use UpperCamelCase
(PascalCase), such as `MacAddress`, not `macAddress` or `MAC_ADDRESS`. Fields
and variables remain snake_case, such as `mac_address`. Reuse `MacAddress` for
six-octet MAC values; do not replace unrelated byte arrays merely because they
have the same length.
Do not rename primitives solely by width (such as UInt8); name their meaning.

Before adding shared measurement fields, search `crates/ofdm/src/types` and
reuse the existing domain vocabulary (CaptureTime, ChannelId, FrequencyHz,
SubcarrierIdx). Preserve consumer metadata widths without introducing a PNM
binary parser dependency into the toolkit.
FrequencyHz is u32 for stored and calculated Hz; use checked arithmetic and
report overflow rather than silently wrapping or introducing a wider public type.
Python NewType gives static identity separation; Rust aliases do not. Use a Rust
newtype if that separation is required, and document the distinction.

Choose the lightest appropriate representation:

- A type alias such as `pub type ChannelId = u8;` improves readability when
  interchangeability is intentional. It does not enforce identity or validation.
- A newtype such as `pub struct FrequencyHz(u32);` is appropriate when mixing
  units/identifiers must be prevented or construction must enforce invariants.
- A struct such as FrequencyGrid groups related metadata with named fields.

Introduce shared domain types in `crates/ofdm/src/types/` when needed; keep types owned
by a specific component near that component (for example SimulationSettings in
the simulation module). Reuse existing definitions rather than creating parser-local aliases
for the same concept. Document units, ranges and whether validation is enforced.
Do not change encoded widths, accepted input or conversions merely to add a name.

Apply incrementally as we develop and review each parser; do not migrate every
primitive in advance. Raw bytes can remain u8 and ordinary lengths/indexing can
remain usize when their meaning is clear. Keep Rust ownership, Result/Option
and traits; C ABI layout is unrelated to these readability choices.

## Descriptive identifiers and lifetime names

Use meaningful names for variables, parameters, generic parameters and Rust
lifetimes. Avoid single-letter names unless they are obvious in a tiny local
operation, such as a simple loop counter `i`. The exception is not a default
for parser fields, buffers, closures, public APIs or lifetimes.

Name lifetimes for the borrow they describe: `'input` for capture bytes and
`'session` for session-owned borrows, rather than `'a` or `'b`. Keep anonymous
`'_` where no explicit relationship needs a name; it is lifetime inference,
not an opaque variable name. Prefer `BYTE_COUNT` to const generic `N`,
`formatter` to `f`, and `capture_bytes` to `b`. Keep docs and examples aligned
with renamed parameters. Do not rename identifiers in preserved upstream
fixtures/reference sources or generated files.

## Descriptive names and domain types

Name values for domain meaning, including units and bounds where relevant:
`data_length_bytes`, `subcarrier_spacing_khz`, `source_end_exclusive`, and
`captures_by_channel`. Distinguish byte offsets, subcarrier indices, capture IDs,
transaction IDs, protocol applicability and file-format versions.
Use domain types when they prevent mixing unrelated identities. Do not wrap every
counter or text string. Do not create closed enums for extensible domains without
an explicit unknown/unsupported path.

## Scientific ownership and named constants

Verify protocol constants against the applicable pinned PHY or OSSI source.
Do not promote PyPNM behavior to normative requirements or change its checkout.
Keep reusable unit conversions in types::units. Format-specific offsets and
signatures belong to the consuming PNM parser, not this toolkit.

Name mode dimensions, generator parameters, permutations, decoder settings and
application limits. Keep application safety limits distinct from protocol limits.
Centralize definitions only when semantics actually match. Mathematical zeroes
and tiny obvious loop origins need no artificial constants.

Use focused snake_case Rust modules and UpperCamelCase types. Scientific code
must not depend on Qt, a transport or application state. Keep unsafe/native code
inside the isolated codec boundary and preserve upstream notices.

## JSON contracts and calculation ownership

Implement measurement JSON changes in the Rust parser serializer or a shared
Rust serialization/analysis helper. CLI and desktop exports must use that same
contract; do not add, rename or calculate measurement fields in Qt/QML
or CLI-specific post-processing. Reuse the existing analysis before adding a
second calculation. All measurement math, derived rates, unit conversions and
scientific aggregation belong in Rust; UI code presents the supplied values.
Transient selection and layout arithmetic remain UI concerns.

For JSON changes, verify both core serialization and actual CLI output, keep
unavailable values explicit, and regenerate the linked JSON examples. Expose
Rust-derived metadata to UI components rather than reading values back from
exported JSON or recomputing them for chips/tables.

## Consumer boundaries

The viewer owns Qt presentation and raw PNM source files. This toolkit owns
scientific computation and typed results. Keep library inputs independent of
QML, GUI controls, filesystem paths and HTTP. Preserve bounded work, cancellation
and partial-result semantics when refactoring.

## SPDX headers

Add the full copyright/SPDX file banner below to new project-owned source and
test files. A single-line SPDX comment alone does not satisfy the banner style. When modifying an existing source file without one, add the appropriate
header after checking its ownership and license context.

| Scope | Identifier |
| --- | --- |
| Reusable Rust libraries under crates/ and their tests | Apache-2.0 |
| Project-owned development tooling under scripts/ and tools/ | Apache-2.0 |

Rust, JavaScript and TypeScript:

```rust
/*
 * Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
 * SPDX-License-Identifier: Apache-2.0
 */
```

Preserve copied PNM Viewer copyrights and all third-party notices. Use the actual
creation year for new files. Place the banner before module docs,
attributes or imports. Use the file language's valid comment syntax (for example,
CSS supports the same multiline block banner).

Shell/Python executable scripts retain the shebang on the first line:

```bash
#!/usr/bin/env bash
#
# Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
# SPDX-License-Identifier: Apache-2.0
#
```

Preserve existing copyright statements; do not invent authors, dates or ownership.
SPDX identifiers must agree with package licenses and applicable full license texts.
A header does not replace required LICENSE/NOTICE files.

Do not relabel third-party code, copied PyPNM fixtures/tests, extracted OSSI text,
or preserved bootstrap files. Inspect their original licensing first and retain
applicable upstream headers/notices. Do not insert comments into binary files,
JSON, generated files or lockfiles; use package metadata or provenance for those.

## Applying and checking the style

Apply these conventions to new and modified code. Installing this skill alone
does not authorize repository-wide formatting or API migrations. During reviews,
report violations in the requested scope. During migrations, preserve behavior
and run appropriate existing checks.

Review full changed methods, signatures and comments, not regex matches alone.
Check banners against errors, ownership and missing-value semantics; confirm
branch reformatting preserves execution order. Run rustfmt, Clippy with warnings
denied and relevant tests for behavior changes. For header/comment-only edits,
formatting and syntax checks suffice. Follow AGENTS.md for project commands.

Verify SPDX scope and position without relabeling upstream material. Validate
this skill and both discovery links after edits. Keep one canonical copy.

