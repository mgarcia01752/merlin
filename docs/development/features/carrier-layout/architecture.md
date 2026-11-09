> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/carrier-layout/`
> Logs, captures and specification images are not published.

# Physical carrier-layout implementation

## Scope and ownership

transmission/layout/physical.rs owns validated physical FFT roles, PLC/predefined
pilots and128-symbol scattered-pilot progression. Immutable map; inputs unchanged.
transmission/layout/ncp.rs selects descending eligible pre-interleaving cell
indices, explicitly separate from physical bins. No new CLI or JSON contract.

## Evidence

Local CM-SP-PHYv3.1-I20-230419:7.5.13.1–2 PDF148–149,7.5.15.1.1–2 PDF164–168,
Table50 PDF169,7.5.15.2.2 PDF170–171 and7.5.14.4–5 PDF163–164. PDF168–169
rendered and inspected for8K half-cycle offsets and predefined pilot distances.

## Confirmed implemented behavior

FFT4K/8K; explicit active envelope/internal exclusions; frequency width overflow
checks; PLC8/16 bins with clear6MHz neighborhood starting on1MHz grid; predefined
pilot distances; additional caller-supplied pilot collision/8..120 count validation.
Continuous pilot/PLC/exclusion precedence. SP shift4K=s and8K=2*(s%64)+s/64.
Every eligible physical bin visited once per128-symbol cycle; eligible domain
size remains constant. u64symbol index wraps safely. NCP helper skips placeholders
and zero-loaded cells without mutation/partial results. Maxdomain8192 cells.

## Pending boundaries

Additional-pilot generator, density/merged spectrum/edge adjustment/island coverage
and periodicity compliance are not implemented. Count and geometry validation do
not certify full CMTS placement or spectral compliance. Synthetic fixtures are
API examples, not deployment profiles.
NCP actual count/content/encoding and inverse physical-to-preinterleaving pilot
mapping require upcoming interleavers/packer. Physical Data is candidate space,
not user payload capacity. No waveform or256QAM chain yet. No complete one-second
recovery claim. Existing16QAM CLI behavior unchanged.

## Validation

Independent predefined pilot vectors and phases0/1/63/64/65/127; bothFFT modes;
full128-symbol coverage/domain invariants; out-of-grid PLC, protected neighborhood,
duplicate/overflow/out-of-envelope inputs; NCP ordering/ineligible skips.
Initial test fixture accidentally assigned an additional pilot to excluded bin6000;
constructor rejected it. Fixture corrected, explicit collision rejection retained.
