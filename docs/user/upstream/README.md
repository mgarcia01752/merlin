# Upstream OFDMA

The independent upstream Rust engine has local synthetic backend acceptance:

- [Configuration and continuous source](configuration.md).
- [Transmitter](transmitter.md).
- [Channel and reference planes](channel.md).
- [Received-only receiver](receiver.md).
- [Bounded sessions and reports](session.md).
- [Measurement admission](measurements.md).
- [Directional CLI](cli.md).

Native PNM capture-backed runs, the public Rust facade,
[native Python namespaces](../python/upstream.md) and directional CLI have local
acceptance. Final regression acceptance follows the
[official checklist](../../development/checklist.md#unified-backend-integration).
