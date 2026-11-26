# Designed RxMER example

[Measurement examples](../README.md) · [API walkthroughs](../../api/README.md#custom-designed-rxmer)

This collection is synthetic MERLIN RxMER Designer output, not a modem
measurement. The binary and export sidecar are preserved unchanged from the
user-supplied example.

| File | Purpose |
| --- | --- |
| [merlin_rxmer_designed.bin](merlin_rxmer_designed.bin) | Quantized designed RxMER in the supported PNM binary format. |
| [merlin_rxmer_designed.bin.merlin.json](merlin_rxmer_designed.bin.merlin.json) | Synthetic-origin declaration, recipe, original/export hashes and conversion details. |

The recipe records a 48 dB baseline, -10 dB tilt, 2 dB peak-to-peak ripple at
20 MHz spacing and a 6 dB notch centered at 766.4875 MHz. The binary already
contains the result; do not reapply the recipe. Export rounded 7,479 samples,
clipped none and changed samples by at most approximately 0.125 dB.

Reuse [the compatible modulation profile](../../workbench/downstream/single/modulation-profile.bin)
without copying it. Its exact grid matches all 7,480 samples on channel 34, and
its decoded hash matches the sidecar's profile hash. The Rust/Python
`run_designed_rxmer` examples discover all captured profiles and explicitly
simulate each using this original RxMER. See the API walkthroughs for commands,
settings and validated results. Native binary loading preserves original bytes
but does not automatically ingest the sidecar; keep both files together.

Binary SHA256: `fb30aa5e96d9e44791718950b732b6748a62c5a5e59a398f1b8783371ec1101c`.
The sidecar's embedded `file` remains the unchanged sibling basename.
