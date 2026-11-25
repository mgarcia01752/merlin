# Public parser API examples

Run these examples from the repository root. Each concrete example calls the
public parser directly and writes formatted JSON to stdout; diagnostics use
stderr. The generic `pnm_json` example selects the parser by file identity.

| Example | Public API | Default input |
| --- | --- | --- |
| `rxmer_json` | `CmDsOfdmRxMer` | `rxmer.bin` |
| `channel_estimate_json` | `CmDsOfdmChanEstimateCoef` | `channel_estimation.bin` |
| `constellation_json` | `CmDsConstDispMeas` | `const_display.bin` |
| `histogram_json` | `CmDsHist` | `histogram.bin` |
| `upstream_pre_eq_json` | `CmUsOfdmaPreEq` | `us_pre_equalizer_coef.bin` |
| `upstream_pre_eq_last_json` | `CmUsOfdmaPreEq` | `us_pre_equalizer_coef_last.bin` |
| `fec_summary_json` | `CmDsOfdmFecSummary` | `fec_summary.bin` |
| `spectrum_json` | `CmSpectrumAnalysis` | `spectrum_analyzer.bin` |
| `modulation_profile_json` | `CmDsOfdmModulationProfile` | `modulation_profile.bin` |
| `latency_json` | `CmLatencyRpt` | File path required; synthetic tests only |
| `symbol_capture_header_json` | `CmSymbolCapture` | File path required; header only |
| `pnm_json` | `PnmFile` | File path required; automatic dispatch |

Default inputs are embedded from the preserved
[PyPNM fixtures](../../../tests/fixtures/pypnm/README.md).
There is no real latency or symbol-capture fixture in this repository.
Symbol payload decoding remains unsupported.

```bash
# Run with the bundled RxMER fixture.
scripts/cargo-local.sh run --quiet --locked -p pnm --example rxmer_json

# Choose another example and save its JSON.
mkdir -p local/pnm-json
scripts/cargo-local.sh run --quiet --locked -p pnm \
  --example channel_estimate_json > local/pnm-json/channel-estimate.json

# Override the default with your own file of the matching type.
scripts/cargo-local.sh run --quiet --locked -p pnm \
  --example rxmer_json -- /path/to/rxmer.bin

# Automatically choose the decoder for a supplied file.
scripts/cargo-local.sh run --quiet --locked -p pnm \
  --example pnm_json -- tests/fixtures/pypnm/tests/files/histogram.bin
```

Examples accept at most one path and cap external input at 16 MiB. They use
`ParseLimits::default()` and the bounded 16 MiB JSON writer. These are application
budgets, not DOCSIS limits. Parse errors terminate the example with a nonzero
exit status. See [format support](../../../docs/reference/formats.md) for limitations.

## Use typed data before exporting

The parser object holds typed metadata and borrowed sample bytes. JSON is an
optional export; callers can access decoded samples directly. For example,
inside a function returning a compatible `Result`:

```rust
use pnm::{
    common::json::DEFAULT_MAX_JSON_BYTES,
    pnm::parser::{
        cm_ds_ofdm_chan_estimate_coef::CmDsOfdmChanEstimateCoef,
        parse_limits::ParseLimits,
        pnm_header::HasPnmHeader,
    },
};

let capture = CmDsOfdmChanEstimateCoef::parse(&capture_bytes, ParseLimits::default())?;
let header = capture.pnm_header();
let original_iq_bytes = capture.raw_iq();
for sample in capture.iq() {
    // Each sample is Some([in_phase, quadrature]) or None if excluded.
}
let json = capture.to_json(DEFAULT_MAX_JSON_BYTES)?;
```

Keep `capture_bytes` alive while using the capture. To reuse an already parsed
header, call the concrete parser's `from_header(header, limits)` method.
The shared `common/mod.rs` handles example CLI input only; the parser library
performs no file I/O and does not initialize global logging.
