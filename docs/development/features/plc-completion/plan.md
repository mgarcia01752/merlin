> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/plc-completion/`
> Logs, captures and specification images are not published.

# PLC completion

Complete NEXT07 and NEXT11–15, with remaining NEXT01 preamble evidence.
- descriptors: derive fixed OCD/ProfileA/NCP DPD from validated channel/layout;
  mandatory excluded edge ranges, all CP and PLC assignments; bounded range TLVs.
- mac_packets: complete management MAC frame, CRC16/X25 HCS, Ethernet CRC32;
  strict bounded laboratory parser for generated single-part OCD/DPD subset.
- scheduling: fixed repeats within200ms(default)/250ms(max), bounded serialized
  packets, segmentation across MC frames and receiver reassembly by pointer/length.
- preamble: Tables48/49, eight BPSK columns, no interleaver/randomizer.
- mapping: shared16-QAM label table, unit mean energy; soft inverse uses shared demapper.
- frame assembly: immutable frequency-bin outputs; excluded0, PLC and pilots
  protected, explicit optional reduced cells preserve startup/drain markers.
- simulation/plc: known-phase full PLC TX/RX, per-frame state, stage snapshots,
  bounded observation of bytes/bits/labels/IQ and full physical grid.
- Tests: literal independent preamble columns, AnnexA mapping vectors, CRC check
  strings, descriptor bytes/ranges, segmentation/pointers/repetition deadlines,
  CRC rejection; both-spacings multi-frame actual FEC→IQ→soft recovery, all128
  phases assembled without protected-bin collision; observation invariance.
- Guides/math/example, official architecture/checklist, full pre-push gate.

Authority: PHY3.1 I20 §7.5.13/PDF148–158; Tables48/49 PDF151–152,
Figure68 PDF157, AnnexA16-QAM. MULPI3.1 I25 §6.2.3.2/PDF88–89,
§6.4.1/PDF99–103, §6.4.40–41/PDF197–203, §6.5.4/PDF218,
AnnexA/PDF586 and AnnexB/PDF592. Fixed descriptor subset uses single-part
MMMs; MC segmentation remains distinct from multipart-MMM fragmentation.
No EM/TR modes, live CMTS, blind acquisition or IFFT/FFT yet.
