---
name: docsis-phy-3-1-i20
description: Look up and interpret CableLabs DOCSIS 3.1 PHY I20 (CM-SP-PHYv3.1-I20-230419). Use for CM or CMTS physical-layer behavior, RF requirements, SC-QAM, OFDM/OFDMA, FEC, modulation, power and fidelity limits, ranging, PLC/NCP, PHY-MAC convergence, PNM, compliance reviews, and specification-derived tests. Do not use it as authority for MULPI, OSSI, SEC, DRFI, or another referenced specification.
---

# DOCSIS 3.1 PHY I20 Reference Skill

Use this skill when a task depends on the DOCSIS 3.1 **Physical Layer
Specification**, issue **I20**, document `CM-SP-PHYv3.1-I20-230419`.

This corpus is a layout-preserving text extraction for lookup. The original
CableLabs PDF remains authoritative, especially when a table, figure, formula,
footnote, or page layout is ambiguous in the extraction. Do not claim that a
value was verified against the PDF unless the PDF was available and inspected.

## Retrieval workflow

1. Read [the topic map](references/topic-map.md) to identify the likely area.
2. Use [the section index](references/section-index.md) to choose the smallest
   relevant file under `references/sections/`.
3. Search that section for the exact channel, signal, profile, field, state,
   measurement, or technical term. Prefer `rg -n` so the result can be located
   again.
4. Read the complete surrounding paragraph, list, table, and applicable notes.
   Follow any cross-reference that affects the conclusion.
5. Confirm the actual section heading around the text. Section files preserve
   complete PDF pages, so the first or last page can contain carryover from an
   adjacent section.
6. Use [the normative-keyword index](references/normative-keyword-index.tsv)
   only to locate candidate pages. Never treat its shortened context as the
   complete requirement.
7. Search [the full extraction](references/phy-full.txt) only when a topic spans
   major sections, a cross-reference cannot be resolved in the selected
   section, or a page-boundary split lacks necessary context.

## Interpretation rules

- Prefer the specification text over model memory or generic DOCSIS knowledge.
- Preserve normative force exactly. Never weaken or strengthen **MUST**,
  **MUST NOT**, **SHOULD**, **SHOULD NOT**, or **MAY**.
- Identify whether a requirement applies to the CM, CMTS, or both. If the subject is not clear from the complete source context, say so rather than guessing.
- Distinguish clearly among normative text, informative text, and engineering inference.
- Do not combine separate normative statements into a synthesized requirement unless the synthesis is explicitly labeled engineering inference.
- Do not invent field values, RF limits, defaults, timers, modulation/FEC behavior, power levels, tolerances, equations, or PHY procedures.
- For numeric values from tables, verify row/column association against the PDF when there is any extraction ambiguity.
- For equations, preserve operators, units, variables, subscripts, superscripts, and equation numbers; do not reconstruct unclear notation from memory.
- Prefer concise paraphrase. Quote only the short portion needed to preserve
  exact normative meaning.

## Cross-specification boundaries

When PHY delegates or references behavior in MULPI, OSSI, SEC, DRFI, or another specification:

- Identify the referenced specification.
- State what PHY itself establishes.
- Do not infer the external specification's normative behavior unless that specification is available in the repository.
- If the referenced skill is available, use it separately and identify which conclusion comes from which specification.
- Never silently mix text or requirements from different PHY revisions.

## Results

- Cite substantive results as
  `CM-SP-PHYv3.1-I20-230419, Section <n>, PDF page <p>`.
- Include the table, figure, equation, or annex identifier when it materially
  supports the result.
- For a code, requirement, or Jira compliance review, report:
  `source -> normative force and endpoint -> implementation mapping -> pass,
  gap, or unresolved dependency`.
- For specification-derived tests, cover only cases supported by cited
  behavior. Include positive, negative, boundary, tolerance, state-transition,
  or interoperability cases when they apply.
- State when the extraction is inconclusive and verification against the
  authoritative PDF is still required.

## Tables and figures

- Preserve table and figure titles and numbers when discussing them.
- Treat extracted table text as potentially layout-sensitive.
- If a row/column relationship is unclear, do not choose a value. Verify the
  PDF when available; otherwise report the limitation.
- Informative appendices must not be promoted to normative requirements.

## Version and corpus discipline

All material in this skill is for **DOCSIS 3.1 PHY I20, issued 2023-04-19**.
Do not silently apply requirements from another PHY issue. When the requested
issue is unclear and the difference could matter, state the issue used. If the
user requests another issue, use a separate versioned skill or request that
source.

Treat the extracted files as generated reference material. Do not casually edit
normative text, page markers, section extracts, or keyword indexes. Add another
DOCSIS document or revision as a separate skill so agents cannot silently mix
versions.

Source and extraction details are recorded in
[the specification metadata](references/spec-metadata.yaml).
