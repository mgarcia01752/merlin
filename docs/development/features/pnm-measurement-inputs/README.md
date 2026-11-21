> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/pnm-measurement-inputs/`
> Logs, captures and specification images are not published.

# PNM measurement input preparation

Original sanitized binary captures and their upstream license, NOTICE and provenance
are retained under originals/. Shared decoded JSON and generation instructions
are under examples/measurements/single-channel/. Source timestamps differ; these
files do not establish contemporaneous channel conditions.

## Upstream adapter ownership

The [active upstream input design](upstream.md) separates bounded decode,
association and measured playback from the existing downstream input contracts.
Upstream local acceptance remains INT-PNM-01/02; source format evidence is historical.
