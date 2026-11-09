# Public API

DOCSIS OFDM Channel Sim's scientific entry points are public Rust APIs. The standalone CLI calls
those same functions. Future Python bindings and an optional Qt presentation
component will adapt inputs and results without duplicating scientific logic.

| Public module | Responsibility |
| --- | --- |
| `transmission::{TransmissionSettings, SubcarrierSpacing}` | Fixed single-channel numerology and PHY validation |
| `transmission::TransmissionTiming` | Whole-symbol accounting for one simulated second |
| `transmission::layout::{CarrierLayout, CarrierLayoutSettings}` | Immutable physical bin roles and per-symbol scattered pilots |
| `transmission::layout::ncp::select_ncp_cells` | Eligible NCP cells in the pre-interleaving domain |
| `transmission::packing::{SymbolPacker, PackingSettings, PackedSymbol}` | Stateful256-QAM queue, symbol plans and NCP capacity reservations |
| `transmission::ncp::{NcpMessage, NcpFieldCodec}` | Control serialization, encoded fields and CRC-accepted receiver pointers |
| `fec_codec::{ncp::NcpFecCodec, crc::crc24d}` | Separate NCP control LDPC and CRC-24-D |
| `signal_processing::interleaving::{InterleaverSettings, SymbolInterleaver}` | Bounded cell time/frequency TX/RX streams and explicit startup/drain |
| `signal_processing::interleaving::{FrequencyInterleaver, TimeInterleaver}` | Independent permutation and generic convolutional delay operations |
| `transmission::layout::PilotReference` | Physical-bin association and pre-interleaving pilot masks |
| `scenario::{grid, observation}` | Exact identity, source provenance and full-resolution observations |
| `scenario::association` | Explicit-tolerance timestamp association |
| `scenario::document` | Versioned decoded scenario inputs |
| `profile::candidate` | Captured assignments and editable hypotheses |
| `profile::{evaluation, document}` | Temporal screening, candidate documents and role verification |
| `signal_processing::{modulation, noise}` | QAM, soft demapping and residual-noise models |
| `signal_processing::modulation::{map_symbol, qam_llrs}` | Shared label-ordered mapping and reference soft demapping through16K-QAM |
| `signal_processing::modulation::randomization::CellRandomizer` | Stateful data/NCP cell randomization and soft-bit inverse through16K-QAM |
| `signal_processing::modulation::interleaving::BitInterleaver` | Short8/9 parity/twist/demux and soft inverse for16and256-QAM |
| `simulation::coding_loop::NoiselessQam256Codec` | Independent noiseless codeword trials with per-word decoder iterations |
| `simulation::{config, runner, results, source}` | Bounded trial orchestration and reproducible source state |
| `types::ofdm_modulation` | Normative constellation registry and implementation status |
| `fec_codec::{parameters, bch, ldpc}` | Mode dimensions and individual codecs |
| `fec_codec::concatenated` | Encode, reverse-decode and compare without a channel model |

Canonical modules are shown above. Existing imports under `math`, `profile::model`,
`profile::association`, `profile::simulation` and
`profile::document::ScenarioDocument` remain supported by re-exports. Existing
root `fec_codec::{encode_bch, decode_bch, LdpcCodec, ...}` imports also remain valid.
No duplicate scientific implementations are kept for compatibility.

New APIs should use documented units, typed input/results, structured recoverable
errors and explicit ownership/lifetime contracts. Library functions must not exit
the process, depend on GUI objects, require Python objects or emit results through
console text. Consumers preserve missing values and original scientific results.

Progress callbacks and cancellation are Rust job interfaces. A future binding
will manage Python interpreter locking, callback delivery and buffer lifetimes.
No Python dependency, binding package or stable C ABI is introduced now.

See the [public-api skill](../../skills/public-api/SKILL.md) for API development
requirements and [JSON contracts](documents.md) for the existing serialization.

Existing typed operation outputs can be consumed directly. The planned common
stage-observer/test-point contract is specified in the
[architecture](../architecture.md#test-points-and-data-access), with remaining work
tracked in the [implementation checklist](../development/checklist.md).
It is not an implemented export/observer API yet.
