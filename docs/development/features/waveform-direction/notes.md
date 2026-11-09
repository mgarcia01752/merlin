> Historical implementation record. For current status, see the
> [checklist](../../checklist.md) and [architecture](../../../architecture.md).
> Local artifacts: `docs/development/local/waveform-direction/`
> Logs, captures and specification images are not published.

# Waveform direction clarification

Documentation-only clarification before implementation. Explicit user-requested
renaming splits TX and RX transform/prefix work into directional IDs. WAVE-CH
owns reference sample transfer; WAVE-QA owns shared probes/integration. Nonideal
RX and production payload recovery remain later items. No algorithm status was
changed to complete. Prefix is selectable before a run and held constant during
it; decoded OCD supplies RX settings after the explicit bootstrap.

Validation: changed Markdown links/anchors and balanced code fences, unique IDs,
no old WAVE-01..06 or RX-01..04 references in maintained docs/skills, skill validator
through canonical and both discovery paths, git diff --check. No code changed.
