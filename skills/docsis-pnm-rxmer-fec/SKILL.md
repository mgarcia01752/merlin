---
name: docsis-pnm-rxmer-fec
description: Interpret DOCSIS downstream field RxMER, modulation-profile margins, subcarrier summary metrics and FEC outcomes using pinned CableLabs PNM V05 guidance and V02 comparison. Use for MERLIN impairment validation and below-target metrics; not as a binary-format or normative PHY specification.
---

# DOCSIS PNM RxMER and FEC guidance

Use V05 (2023) by default; V02 (2021) is historical. These are PNM guidelines,
not substitutes for the applicable PHY, OSSI or MULPI specifications.

1. Read [sources and topic map](references/source-map.md) for the relevant
   revision, section and pages. Read only the relevant text pages initially.
2. Use [review findings](references/rxmer-fec-guidance.md) to distinguish empirical
   thresholds, candidate-profile screening and actual decoder outcomes.
3. Search the page-marked [V05 text](references/v05-full.txt); use
   [V02 text](references/v02-full.txt) only for historical comparisons.
4. Cite document ID, section, table/figure and printed page. Do not silently
   transfer numbering between revisions. PDF page indices are one-based here.
5. Retrieve the original from the official links in the source map when tables,
   equations or graph interpretation depend on layout. Verify identity/hash
   against [sources.json](references/sources.json). A newer download is a new
   source revision, not the pinned V05. Inspect the relevant rendered page before
   claiming visual verification. Text cannot recover plot samples reliably.

For MERLIN work, preserve field input RxMER versus simulated receiver MER,
profile/carrier roles, missing samples, reference threshold policy and denominator.
A below-target count is a screening metric; never synthesize UFEC from a fixed
percentage. Actual FEC outcomes require the existing Rust decoder and independent
source-bit comparison. Python calls the Basic Rust API; no second engine.

Do not use the 2nd-percentile guidance or approximately 5% LTE-bandwidth observation
as a universal decoder failure rule. Do not use modulation-only Shannon values
as calibrated receiver thresholds. Do not infer temporal ingress statistics from
an RxMER magnitude capture. Keep field measurement limitations explicit.

Original PDFs and renders are local-only and absent from fresh clones. Text
extracts retain CableLabs notices and are reference material, not relicensed
project code. No capture initiation, device control or network access is required
for normal text lookup. This skill does not authorize those operations.
