> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/cell-randomizer/`
> Logs, captures and specification images are not published.

# Data/NCP cell-word randomizer

## Scope

PHY I20 section7.5.5.3, Figure52 and page124 example. Implement persistent
GF(2^12) two-register state, MSB-label mask adaptation, data4/6..14bits,
NCP2/4/6bits, zero-loaded BPSK and SP placeholders. Receiver uses identical
mask stream to undo labels or flip LLR signs. No changes to existing CLI mode.

## Source evidence

Local PDF123–124 rendered and inspected. GF polynomial alpha^12+alpha^6+
alpha^4+alpha+1; D0'=alpha^11*D1, D1'=D0+D1, old values used simultaneously.
SeedsD0=555,D1=AAA. Source sequenceAAA555,FFFADF,FFFADF,520799,2B9828
for data,SP,data,zero-loaded,data. y0 usesD0[0], so reverse register-bit order
when forming integer labels. 13/14bit extensions useD1[0] andD1[1].

## Lifecycle

Explicit begin_symbol(index): first index0 is first pre-interleaving symbol after
PLC preamble; later indices consecutive, no codeword reset. Reinitialize at128
boundaries even if first cell is SP. Require an active symbol before processing.
Validate operation before any state mutation. Snapshots read-only copies.

## Boundaries

PLC randomizer7.5.13.8 has separate seeds and extra symbol-boundary clocking;
not included under data/NCP APIs. Pilots/exclusions/PLC are not clocked as data.
Future packer supplies correct pre-interleaving order and synchronization.
No actual NCP content generation, bit-packing or whole-channel path in this step.

## Control-table correction discovered by normative test

Data sequence state vectors matched, but its zero-loaded BPSK point test exposed
imported control tables in geometry order, not bit-label order. Verified local
PDF197 Figures99/100. Build-time permutations BPSK[1,0],QPSK[1,3,0,2]; original
resources/hashes preserved. Independent all-control-label tests added. Resource
metadata/docs now state the exception. Shared PLC_FRAME_SYMBOLS lives in types;
DSP no longer depends on a transmission/layout module for phase length.
