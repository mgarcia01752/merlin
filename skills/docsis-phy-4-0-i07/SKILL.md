---
name: docsis-phy-4-0-i07
description: Look up and interpret CableLabs DOCSIS 4.0 PHY I07 (CM-SP-PHYv4.0-I07-250627). Use for CM, CMTS, node, RPD, or RMD physical-layer behavior, FDX, FDD, sounding, echo cancellation, RF limits, power, MER/CNR, spectrum, numerology, triggered RxMER, compliance reviews, and specification-derived tests. Do not use it as sole authority for delegated DOCSIS 3.1 PHY, MULPI, CM-OSSI, CCAP-OSSI, SEC, or R-PHY behavior.
---

# DOCSIS 4.0 PHY I07 Reference Skill

Use this skill when a task depends on the DOCSIS 4.0 **Physical Layer
Specification**, issue **I07**, document `CM-SP-PHYv4.0-I07-250627`.

This corpus is a layout-preserving text extraction for lookup. The original
CableLabs PDF remains authoritative, especially when a table, figure, formula,
spectrum plot, footnote, or page layout is ambiguous in the extraction. Do not
claim that a value was verified against the PDF unless the PDF was available
and inspected.

## Retrieval workflow

1. Read [the topic map](references/topic-map.md) to identify the likely area.
2. Use [the section index](references/section-index.md) to choose the smallest
   relevant file under `references/sections/`.
3. Search that section for the exact signal, band, interface, measurement,
   limit, equation, or technical term. Prefer `rg -n` so the result can be
   located again.
4. Read the complete surrounding paragraph, list, table, equation, and
   applicable notes. Follow any cross-reference that affects the conclusion.
5. Confirm the actual section heading around the text. Section files preserve
   complete PDF pages, so adjacent extracts can share a boundary page and the
   first or last page can contain neighboring section text.
6. Use [the normative-keyword index](references/normative-keyword-index.tsv)
   only to locate candidate pages. Never treat its shortened context as the
   complete requirement.
7. Search [the full extraction](references/phy-full.txt) only when a topic
   spans major sections, a cross-reference cannot be resolved in the selected
   section, or a page-boundary split lacks necessary context.

## Interpretation rules

- Prefer the specification text over model memory or generic DOCSIS knowledge.
- Preserve **MUST**, **MUST NOT**, **SHOULD**, **SHOULD NOT**, and **MAY**
  exactly.
- Identify whether each requirement applies to the CM, CMTS/node/RPD/RMD, or both.
- Separate normative text, informative text, and engineering inference.
- Never combine multiple normative statements into a synthesized requirement unless explicitly labeled engineering inference.
- Do not invent field values, RF limits, PSD values, power levels, CNR/MER thresholds, frequency limits, timers, defaults, numerology, spectrum behavior, or PHY behavior.
- Prefer concise paraphrase. Quote only the short portion needed to preserve
  exact normative meaning.

## DOCSIS 4.0 applicability and boundaries

- Treat FDX and FDD as distinct operational alternatives. Do not merge limits or assumptions between them.
- DOCSIS 4.0 PHY I07 explicitly reuses DOCSIS 3.1 PHY definitions for several base OFDM/OFDMA behaviors. When this specification delegates to DOCSIS 3.1 PHY, MULPI, CM-OSSI, CCAP-OSSI, SEC, R-PHY, or another specification, identify that source.
- Do not infer the delegated requirement unless that referenced specification is available. If its skill exists, use it separately and attribute each conclusion.
- Sections 8 and 9 explicitly defer PHY-MAC convergence and PNM to DOCSIS PHYv3.1; do not treat the one-line reference as a complete standalone requirement corpus.
- Appendices are informative unless the document explicitly states otherwise.

For a Puma8 DOCSIS 4.0 PHY review, load `$docsis-phy-3-1-i20` as well whenever
I07 delegates, augments, or applies exceptions to DOCSIS 3.1 PHY behavior. Read
the delegated section before combining it with the I07 additions or changes.

## Tables and equations

- Preserve table/figure/equation identifiers.
- Do not flatten a table in a way that makes row/column association ambiguous.
- Verify values, units, footnotes, superscripts/subscripts, and formulas in the
  PDF when available before calling them PDF-verified.
- If extraction is ambiguous and the PDF is unavailable, report the limitation
  instead of reconstructing the value.

## Results

- Cite substantive results as
  `CM-SP-PHYv4.0-I07-250627, Section <n>, PDF page <p>`.
- Include the table, figure, equation, band, or interface identifier when it
  materially supports the result.
- For a code, requirement, or Jira compliance review, report:
  `source -> normative force and actor/mode -> implementation mapping -> pass,
  gap, or unresolved dependency`.
- For specification-derived tests, cover only cases supported by cited
  behavior. Include positive, negative, boundary, tolerance,
  mode-applicability, or interoperability cases when they apply.
- When I07 delegates to PHY I20, cite the I07 delegation and the controlling
  PHY I20 requirement separately.
- State when the extraction is inconclusive and verification against the
  authoritative PDF is still required.

## Version and corpus discipline

All material in this skill is for **DOCSIS 4.0 PHY I07, issued 2025-06-27**.
Never silently mix it with DOCSIS 3.1 PHY or another DOCSIS 4.0 PHY issue.
When the requested issue is unclear and the difference could matter, state the
issue used. If the user requests another issue, use a separate versioned skill
or request that source.

Treat the extracted files as generated reference material. Do not casually edit
normative text, page markers, section extracts, or keyword indexes. Add another
DOCSIS document or revision as a separate skill so agents cannot silently mix
versions.

Source and extraction details are recorded in
[the specification metadata](references/spec-metadata.yaml).
