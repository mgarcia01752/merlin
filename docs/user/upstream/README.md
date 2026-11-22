# Upstream OFDMA

The independent upstream Rust engine has local synthetic backend acceptance:

- [Configuration and continuous source](configuration.md).
- [Transmitter](transmitter.md).
- [Configured resource grid](resource-grid.md).
- [Channel and reference planes](channel.md).
- [Configured response and periodic impulse](response.md).
- [Received-only receiver](receiver.md).
- [Actual received-pilot estimate retention](receiver-estimates.md).
- [Bounded sessions and reports](session.md).
- [Recovery and decoder views](recovery.md).
- [Measurement admission](measurements.md).
- [Directional CLI](cli.md).

Native PNM capture-backed runs, the public Rust facade,
[native Python namespaces](../python/upstream.md) and directional CLI have local
acceptance, including complete unified Rust regressions and existing downstream Qt
compatibility. Current documentation/backend gates and retained limits follow the
[official checklist](../../development/checklist.md#unified-backend-integration).
