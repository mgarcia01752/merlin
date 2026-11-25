/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Common downstream policy definitions grouped by meaning; no mutable configuration.
pub use merlin_dsp::common::constants::{
    METRES_PER_FOOT, MILLISECONDS_PER_SECOND, NANOSECONDS_PER_SECOND, POWER_RATIO_DECIBEL_FACTOR,
    PRBS15_MAX_SEED, SECONDS_PER_MICROSECOND, SPEED_OF_LIGHT_METRES_PER_SECOND,
};

/// Existing continuous-source default: every PRBS15 register bit set (0x7fff/32767).
/// This intentional all-ones policy derives from the seed domain, not the sequence period.
/// It does not choose a file-fill seed or a channel/noise seed.
pub const DEFAULT_CONTINUOUS_PRBS15_SEED: u16 = PRBS15_MAX_SEED;

/// Downstream samples per microsecond, derived from its authoritative waveform sample clock.
/// This is not the upstream sample rate and does not select another PHY's timing.
pub const DOWNSTREAM_SAMPLES_PER_MICROSECOND: f64 = crate::transmission::settings::SAMPLE_RATE_HZ
    as f64
    / merlin_dsp::common::constants::MICROSECONDS_PER_SECOND;

/// Lower inclusive synthetic Designer value in dB: existing application policy, not a PNM limit.
pub const MIN_DESIGNED_RXMER_DB: f64 = -30.0;
/// Upper inclusive synthetic Designer value in dB, shared by flat input, replacements and output.
pub const MAX_DESIGNED_RXMER_DB: f64 = 120.0;

/// Maximum synthetic random-variation standard deviation in dB; independent of edit depth.
pub const MAX_DESIGN_RANDOM_STANDARD_DEVIATION_DB: f64 = 20.0;
/// Maximum absolute low-to-high synthetic tilt in dB; an independent editing policy.
pub const MAX_DESIGN_ABSOLUTE_TILT_DB: f64 = 60.0;
/// Maximum synthetic ripple peak-to-peak depth in dB; independent of tilt/notch policy.
pub const MAX_DESIGN_RIPPLE_PEAK_TO_PEAK_DB: f64 = 60.0;
/// Maximum synthetic notch depth in dB; independent of ripple/tilt policy.
pub const MAX_DESIGN_NOTCH_DEPTH_DB: f64 = 60.0;
/// Minimum admitted echo-distance helper input in metres; existing application policy.
pub const MIN_DESIGN_ECHO_DISTANCE_METRES: f64 = 0.001;
/// Maximum admitted echo-distance helper input in metres; separate from any waveform range.
pub const MAX_DESIGN_ECHO_DISTANCE_METRES: f64 = 10_000.0;

// Detached receiver-history application policy; independent of live capture/stage budgets.
/// Maximum/default payload cells in one detached history; original positions are sampled uniformly.
pub const MAX_RECEIVER_ARCHIVE_PAYLOAD_CELLS: usize = 8192;
/// Independent maximum/default NCP history cells, not a protected-role or event limit.
pub const MAX_RECEIVER_ARCHIVE_NCP_CELLS: usize = 1024;
/// Maximum/default per protected history role, shared deliberately by the four protected streams.
pub const MAX_RECEIVER_ARCHIVE_PROTECTED_CELLS: usize = 1024;
/// Maximum/default original coding-order LLR prefix retained for one historical payload word.
pub const MAX_RECEIVER_ARCHIVE_DECODER_LLRS: usize = 16200;
/// Per-history copy budget ceiling/default, eight MiB; 32 histories cannot exceed 256 MiB
/// of owner-accounted bytes. Aggregate application retention still needs its own budget.
pub const MAX_RECEIVER_ARCHIVE_BYTES: usize = 8 * 1024 * 1024;
/// Independent strict archive-policy transport ceiling; unrelated to stage/query row counts.
pub const MAX_RECEIVER_ARCHIVE_POLICY_BYTES: usize = 4096;

/// Shared live/archive QAM page and selected-cell soft-population ceiling; not a history cap.
pub const MAX_RECEIVER_CELL_VIEW_ROWS: usize = 2048;
/// Independent live/archive physical decision row ceiling; not a likelihood count.
pub const MAX_RECEIVER_DECISION_VIEW_ROWS: usize = 128;

/// Strict receiver-cell query JSON byte ceiling; independent of returned row counts.
#[cfg(feature = "high-level-api")]
pub(crate) const MAX_RECEIVER_CELL_QUERY_BYTES: usize = 4096;

/// Maximum consecutive retained PLC frames inspected by one passive message query.
/// This bounds reassembly work independently of the observation retention policy.
#[cfg(feature = "high-level-api")]
pub(crate) const MAX_PLC_MESSAGE_INSPECTION_FRAMES: usize = 64;
/// Maximum complete packets copied by one passive PLC message query.
/// This is a backend inspection policy, separate from any application's display limit.
#[cfg(feature = "high-level-api")]
pub(crate) const MAX_PLC_MESSAGE_INSPECTION_PACKETS: usize = 32;

/// Maximum actual active-carrier samples in the additional-response preview.
/// Uniform index selection includes both active-band endpoints without changing the full response.
#[cfg(feature = "high-level-api")]
pub(crate) const MAX_CHANNEL_PREVIEW_RESPONSE_POINTS: usize = 128;
