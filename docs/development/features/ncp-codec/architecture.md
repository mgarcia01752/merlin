> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/ncp-codec/`
> Logs, captures and specification images are not published.

# NCP control encoding and receiver discovery

## Scope and ownership

fec-codec::ncp owns the (160,80) mother LDPC and NCP (48,24)
shortening/puncturing; fec-codec::crc owns CRC-24-D. transmission::ncp owns
3-byte messages, bounded chains, NCP cell mapping and discovery. Packer keeps
planning only. Feature-gate codec-backed integration under simulation so existing
analysis-only use stays independent of codec dependencies.

Pinned evidence CM-SP-PHYv3.1-I20-230419: §7.4.3.2 PDF56 (right shifts),
Table19 PDF91 (mother matrix), §7.5.14 PDF159–164 (byte order, CRC, puncturing,
label mapping and placement), §8.3.4 PDF180–184 (control fields and chain),
AnnexE PDF210 (polynomial and independent CRC vectors). Original PDF inspected
for Table19, Figure92 and Figures71–79. No external interoperability vector yet.

Implement frozen profiles: even data/NCP selection, U=0, no tagging. Generic
message representation preserves flags; adapter must reject unsupported dynamic
profile state. Zero reserved bits on transmission. CRC over MSB-first message
bytes before FEC. Receive complete blocks high-to-low, stop after L then CRC,
reject entire chain on decoder/CRC/structure error. Never return partial entries.
Receiver carries codeword boundaries from decoded pointers, not diagnostic IDs.

## Validation

Independent AnnexE CRC examples, hand-transcribed bit/byte mapping and mother
syndrome oracle; basis vectors, valid/invalid finite LLRs, correction/failure,
NULL/Z/last/control errors, CRC mismatch, truncated/oversized inputs. All three
NCP constellations. Integrate packing, shared symbol randomizers and real NCP
FEC/CRC; reassemble data using received pointer boundaries, compare actual
BCH/LDPC payloads with original. Bound blocks<=12 and decoder iterations<=100.

## Completed validation

Implemented separate control LDPC and CRC in fec-codec; message, mapping and
bounded chain discovery in transmission/ncp. NCP labels fill validated packer
reservations. The standalone ncp_field example emits decoded JSON and iterations.

Nine new tests passed: three low-level FEC/CRC tests, one message test, four
field/mapping/validation tests and one cross-symbol integration test. The latter
runs all three NCP modulations; each recovers four14232-bit payloads with zero
errors after actual pointer discovery across the128-symbol randomizer reset.
The pilot mask/NI are synthetic; physical interleavers and waveform remain next.
CRC examples caught a hexadecimal polynomial transcription error during initial
implementation; the corrected polynomial passes both published AnnexE vectors.

scripts/check-before-push.sh passed on local Linux, including workspace tests,
analysis-only tests, rustfmt, warnings-denied Clippy, helper tests, ShellCheck,
release CLI build/version and AWGN smoke. Changed Markdown links, example JSON,
git diff --check, and skill validation at canonical/.agents/.claude paths passed.
No global packages, commits, tags or pushes were performed. Windows/macOS runtime
and external DOCSIS interoperability remain unverified. Logs: checks.log;
example output: example.json. Local PDF diagram evidence retained in this folder.
