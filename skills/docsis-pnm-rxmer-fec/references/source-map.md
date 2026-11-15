# Sources, downloads and topic map

## Source identity and retrieval

Primary: **CM-GL-PNM-3.1-V05-230927**, September 27, 2023,
*PNM Current Methods and Practices in HFC Networks (DOCSIS 3.1)*, 252 PDF pages.
[Official download/version selection](https://www.cablelabs.com/specifications/CM-GL-PNM-3.1).
Select V05 when reproducing this evidence. An immutable V05 direct-download URL
was not verified; do not invent one or assume the portal still defaults to V05.

Historical: **CM-GL-PNM-3.1-V02-210114**, January 14, 2021,
*Primer for PNM Best Practices in HFC Networks (DOCSIS 3.1)*, 175 PDF pages.
[Direct CableLabs PDF](https://account.cablelabs.com/server/alfresco/c021877d-cce7-434a-b3ac-9cc466d6c601).

Original user-supplied files reside under repository-relative
`docs/development/local/specs/pnm-best-practices/`, using document IDs as filenames.
This directory is ignored. [Source manifest](sources.json) pins PDF and extracted
text SHA-256 values. Preserve CableLabs copyright, notices and third-party
attributions; the text is an uncorrected layout extraction with page markers.
No original PDFs or rendered pages are included in Git.

For reconstruction, use `pdftotext -layout INPUT.pdf OUTPUT.txt`; split on form
feeds, discard the terminal empty page and prefix each page with
`<!-- PDF_PAGE: N -->` (one-based), retaining each page's text unchanged.
Compare hashes before relying on the extraction. `pdftoppm -f N -l N -png
-singlefile INPUT.pdf OUTPUT_PREFIX` renders a page for layout verification.

## V05 topic map

| Question | Section | PDF/printed pages |
| --- | --- | --- |
| Source identity, revision history and notices | Front matter | 1–3 |
| RxMER measurement, interpretation and limitations | 5.3 | 72–85 |
| Percentile, mean, standard deviation, reliability table | 5.3.5, Table 6 | 82–84 |
| Downstream FEC counters and high correctables | 5.4 | 85–91 |
| Channel-estimate relation to RxMER | 5.5 | 92–98 |
| Candidate-profile threshold counts and SNR margin | 5.12 | 120–121 |
| Calibration and complex channel measurements | 8.3–8.4 | 173–176 |
| Network health and averaging context | 9.1 | 177–181 |
| Coding rate, Shannon comparison and FEC meanings | 9.3 | 183–187 |
| Controlled AWGN/LTE experiments and conclusions | Appendix II | 203–215 |

This map covers relevant downstream topics; full text includes other subjects,
which were not comprehensively reviewed for this skill.

## Revision comparison and visual evidence

| Topic | V02 | V05 |
| --- | --- | --- |
| Reliability thresholds | 5.3.5.6, Table 6, p.74 | 5.3.5.6, Table 6, pp.83–84 |
| Controlled margin experiment | Appendix I, pp.151–166 | Appendix II, pp.203–215 |
| LTE bandwidth observation | I.4, p.166 | II.4, p.215 |

Reviewed on 2026-11-15: rendered V02 p.74 and V05 pp.84,121,208.
Verified table row/column association, candidate-profile inequality and the
AWGN whale-plot axes/legend. This is selective visual review, not full-PDF visual
validation or curve digitization. The threshold table and qualitative 5% LTE
conclusion remain in V05; do not infer that every intervening paragraph is unchanged.
The two supplied PDFs are PNM revisions, not SCTE 285.
