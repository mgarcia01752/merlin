# Upstream OFDMA scientific contracts

[Skill](../SKILL.md) · [Architecture](../../../docs/architecture.md) ·
[Integration planning guide](../../../docs/development/ofdm-ofdma-integration-guide.md)

These rules guide explicitly scoped upstream work. Their presence does not mean
MERLIN contains an upstream implementation or that a source backend's completion
claims have been reproduced locally. Keep downstream contracts intact and follow
the reviewed integration architecture before moving scientific implementations.

## Evidence and stage ownership

Identify the CM transmitter/CMTS receiver endpoint, code/profile, numerology and
FDD/FDX applicability before selecting pinned evidence. Use
[PHY 3.1 I20](../../docsis-phy-3-1-i20/SKILL.md) and
[PHY 4.0 I07](../../docsis-phy-4-0-i07/SKILL.md) with exact sections, tables and
PDF pages. Read delegated specifications separately. Do not treat a PHY statement
as the complete UCD wire format or a PNM binary-file definition.

Verify layout-sensitive matrix/table orientation against available authoritative
evidence. Missing evidence stays a gap. Private PDFs/extractions/page images remain
under ignored local evidence storage; do not copy source corpora into a skill.
No PDF was verified merely by adapting these instructions.

Give each stage typed inputs/outputs and explicit state/reset/continuity contracts.
All encoding, mapping, channel, estimation, demapping and orchestration algorithms,
including dependencies, remain safe Rust in Qt/Python-independent core owners.
CLI/Python/Qt adapters do not calculate a second scientific result.

Before sharing FFT, QAM, PRBS or channel kernels with downstream, compare exact
conventions, normalization, indexing, random draws and numerical bounds through
the [semantic reuse audit](../../development-workflow/SKILL.md#reuse-decisions).
Keep CP/window/burst and pilot/interleaving/resource state in the appropriate PHY
owner. Do not build a universal receiver around per-sample direction switches.

## Upstream FEC and waveform composition

Audit each declared QC-LDPC family independently: matrix dimensions and cells,
right-shift orientation, systematic information/parity order, shortening coordinates,
codeword selection, LLR sign/scale and bounded decoder iterations. Matching N/K
does not make upstream/downstream matrices interchangeable. Ordinary scheduled
upstream data must not acquire downstream BCH or ranging-only puncturing rules.
The codec must remain usable independently of QAM, waveforms and channel models.

Verify the normative upstream randomization, joint interleaving, cell loading,
pilot/preamble placement, resource allocation, transform and prefix/window ordering
before composing stages. IUC alone does not establish a QAM order. Recognized
orders, source-verified definitions, mapper support and executable coding/waveform
support are distinct states; unsupported cases return explicit typed errors.

## Receiver inputs and reference planes

Keep transmitter Pre-Eq C, physical channel H and received observation Y distinct:

```text
Y[k] = X[k] C[k] H[k] + N[k]
```

Normal receiver inputs contain received samples and declared configuration, not
injected H/C, original payload bits, TX codeword IDs or private scheduler truth.
Received pilots estimate the effective C×H response. Supplied-truth correction
must be an explicitly labeled reference experiment with its own input contract.
Independent observers may compare estimates/recovered bits with simulator truth;
they must not feed that truth back into receiver decisions.

Measured Pre-Eq is a corrective transmitter response, not an exact physical
channel estimate or a CMTS receive-noise observation. Any inferred inverse-channel
model needs documented regularization and gain/delay/reference assumptions.
Do not count the same measured distortion/noise twice at different reference planes.

## Measurement admission and acceptance

Preserve original bytes, hashes, identities, full grids, nulls, frequency mappings,
units, capture timestamps and update order. Use one capture-admission/parser owner;
reuse the pinned decoder where it actually supports the required format. Do not
guess association metadata or invent upstream formats from downstream similarities.

Pre-Eq set/update/application semantics require explicit source-backed context;
the existence of coefficients does not prove how or when they were applied.
Keep raw interpretation, accepted associations, analytical screening and simulated
decoder outcomes separate. Partial coverage and unknown coefficients remain visible.

Follow the [upstream test procedure](../../scientific-testing/references/upstream.md)
and [public API contract](../../public-api/SKILL.md). Bound files, retained samples,
allocations, iterations and runtime. Cancellation preserves completed work, and
observation must not alter RNG consumption or receiver outcomes. Closed-loop
acceptance is MERLIN source → TX → channel → RX → independent original-bit
comparison; source provenance checks remain mandatory and external hardware
interoperability requires separate evidence.
