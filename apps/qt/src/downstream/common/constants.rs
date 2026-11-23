/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Named downstream presentation policies; backend admission remains authoritative.

/// Inclusive byte ceiling for the existing receiver/run-options editor document.
/// This application transport policy is independent of OCD configuration JSON,
/// even though both currently admit 4096 bytes. It is not a PHY or runtime bound.
pub(crate) const MAX_RECEIVER_OPTIONS_DOCUMENT_BYTES: usize = 4096;

/// Maximum received-word bars shown by the existing downstream verification view.
/// This display cap is independent of the backend's retained-codeword budget;
/// omitted comparisons remain explicitly counted when more details are retained.
pub(crate) const MAX_VERIFICATION_WORD_BARS: usize = 64;

/// Fixed adjacent-window graph padding preserving the existing full-view time scale.
/// This prefix allowance is a display policy, not a validator or a claimed PHY limit.
pub(crate) const TIMING_PREFIX_ALLOWANCE_SAMPLES: usize = 1024;
/// Independent roll padding for that same stable graph, not its sampling resolution.
pub(crate) const TIMING_ROLL_ALLOWANCE_SAMPLES: usize = 256;
/// Uniform overview/detail drawing intervals; independent of roll padding above.
pub(crate) const TIMING_DRAWING_INTERVALS: usize = 256;
/// Extra drawing intervals along each taper to keep its shape visible.
pub(crate) const TIMING_TAPER_DRAWING_INTERVALS: usize = 64;
/// Horizontal tick intervals on each window chart.
pub(crate) const TIMING_TICK_INTERVALS: usize = 4;
/// Round the full-view time axis upward in this many microseconds.
pub(crate) const TIMING_OVERVIEW_QUANTUM_MICROSECONDS: f64 = 10.0;

/// One completion envelope for the single idle transmitter-preview worker.
pub(crate) const IDLE_PREVIEW_COMPLETION_CAPACITY: usize = 1;

/// Backpressure permits one completed profile envelope before GUI collection.
/// Independent of the idle-preview channel, although both capacities are one.
pub(crate) const PROFILE_COMPLETION_CAPACITY: usize = 1;

/// Existing contiguous time-window display cap shared by TX and RX instruments.
/// This is presentation retention, separate from per-event backend query budgets.
pub(crate) const MAX_INSTRUMENT_TIME_SAMPLES: usize = 32768;
/// Legacy concatenated IFFT/prefix/window diagnostic prefix, in actual samples.
/// Independent of the equal-valued backend per-event retention limit and the
/// continuous-output budget above; its source frames may have physical gaps.
pub(crate) const MAX_TX_FRAME_SEQUENCE_SAMPLES: usize = 16384;

/// Whole transmitted PLC frames retained for the existing message viewer.
/// Independent of decoder-word and chart-bar limits that also happen to be 64.
pub(crate) const MAX_TRANSMITTER_MESSAGE_FRAMES: usize = 64;

/// Version of the bundled diagnostic registry, independent of saved projects.
pub(crate) const HELP_CATALOG_SCHEMA_VERSION: u32 = 1;
/// Maximum bundled help source bytes before parsing or line-ending normalization.
pub(crate) const MAX_HELP_DOCUMENT_BYTES: usize = 64 * 1024;
/// Maximum catalog entries parsed by the offline help adapter.
pub(crate) const MAX_HELP_ENTRIES: usize = 128;
/// Maximum lookup-code bytes; this is not a message or title length policy.
pub(crate) const MAX_DIAGNOSTIC_CODE_BYTES: usize = 96;

// PLC message/tree presentation policies; the public backend validates all meanings.
/// Packet records displayed at once, independent of the backend's own work cap.
pub(crate) const MAX_PLC_MESSAGE_RECORDS: usize = 32;
/// Original wire descriptor tag displayed as NCP; not an admitted data-profile ID.
pub(crate) const NCP_MESSAGE_PROFILE_ID: u8 = u8::MAX;
/// Conventional label for a backend-reported four-point constellation, not admission.
pub(crate) const QPSK_DISPLAY_ORDER: u16 = 4;
/// Each TLV begins with one type octet and one length octet; value bytes follow.
pub(crate) const DESCRIPTOR_TLV_HEADER_BYTES: usize = 2;
/// Assignment value starts with one backend-interpreted role/loading octet.
pub(crate) const ASSIGNMENT_CODE_BYTES: usize = 1;
/// Each original assignment endpoint is a two-octet bin field in the byte viewer.
pub(crate) const ASSIGNMENT_BIN_BYTES: usize = 2;
/// Raw first-endpoint offset within an assignment value, separate from packet offsets.
pub(crate) const ASSIGNMENT_FIRST_BIN_START: usize = ASSIGNMENT_CODE_BYTES;
/// Raw last-endpoint offset follows the complete first endpoint.
pub(crate) const ASSIGNMENT_LAST_BIN_START: usize =
    ASSIGNMENT_FIRST_BIN_START + ASSIGNMENT_BIN_BYTES;

/// Existing open physical-bin selector; explicit indices use public SubcarrierIndex.
pub(crate) const OPEN_PHYSICAL_BIN_SELECTOR: i32 = -1;
/// Minimum absolute extent of the pre-channel TX physical-bin drawing only.
/// This preserves its unit-sized empty/small-signal axis and is not an RX scale,
/// power calibration, sample modification or backend numerical acceptance limit.
pub(crate) const TX_PHYSICAL_SCATTER_MIN_EXTENT: f64 = 1.0;

/// Bit characters per inspector text row, independent of raster graph columns.
pub(crate) const TX_INSPECTOR_BITS_PER_ROW: usize = 32;
/// Maximum inspector rows per ordinary page, independent of bit raster width.
pub(crate) const TX_INSPECTOR_ROWS: usize = 64;
/// Source/interleaved-bit preview page length; not a source or FEC work limit.
pub(crate) const TX_BIT_PREVIEW_PAGE_BITS: usize = 128;
/// Horizontal cells in the coded-bit raster; separate from inspector row count.
pub(crate) const TX_CODED_RASTER_COLUMNS: usize = 64;
/// Lower trace position for a displayed zero bit on the normalized drawing axis.
pub(crate) const TX_ZERO_BIT_Y: f64 = 0.8;
/// Upper trace position for a displayed one bit, independent of label row placement.
pub(crate) const TX_ONE_BIT_Y: f64 = 0.2;
/// Fixed label-strip row, separate from the equal-valued one-bit trace position.
pub(crate) const TX_LABEL_STRIP_Y: f64 = 0.2;
/// Unit-sized minimum I/Q drawing extent shared by TX cell scatter/sequence views.
/// This is independent of physical-bin scatter, power calibration and RX scaling.
pub(crate) const TX_CELL_VIEW_MIN_EXTENT: f64 = 1.0;
/// Maximum physical-power min/max groups; each emits at most two display points.
pub(crate) const TX_PHYSICAL_POWER_BUCKETS: usize = 1024;
/// Minimum physical-power graph vertical scale, distinct from any amplitude extent.
pub(crate) const TX_PHYSICAL_POWER_MIN_SCALE: f64 = 1.0;

/// Existing single-profile grouping columns, separate from the public overlay budget.
pub(crate) const PROFILE_SINGLE_TRACE_COLUMNS: usize = 512;
/// Maximum bytes in the ephemeral profile-view selection request, not capture admission.
pub(crate) const MAX_PROFILE_VIEW_REQUEST_BYTES: usize = 8192;
/// Existing single-view bit-axis extent; a display choice, not source/run validation.
pub(crate) const PROFILE_SINGLE_BIT_AXIS_MAX: u32 = 14;
/// Bit spacing of that same single-view axis; its interval count derives from the extent.
pub(crate) const PROFILE_SINGLE_BIT_TICK_STEP: u32 = 2;
/// Existing overlay bit-axis headroom, independent of the single-view extent above.
pub(crate) const PROFILE_OVERLAY_BIT_AXIS_MAX: f64 = 15.0;
/// Existing ideal-Shannon overlay axis extent in dB; not a receiver admission limit.
pub(crate) const PROFILE_OVERLAY_SHANNON_AXIS_MAX_DB: f64 = 45.0;
/// Horizontal tick density shared by the two profile-view formats.
pub(crate) const PROFILE_FREQUENCY_TICK_INTERVALS: u32 = 4;
/// Overlay vertical tick density; independent of equal-valued horizontal density.
pub(crate) const PROFILE_OVERLAY_METRIC_TICK_INTERVALS: u32 = 4;
/// Existing ephemeral viewer metric selector; scientific meaning comes from ProfileMetric.
pub(crate) const PROFILE_VIEW_IDEAL_DB_SELECTOR: u8 = 0;
/// Bit-loading selector in that same viewer message, unrelated to source bit values.
pub(crate) const PROFILE_VIEW_BITS_SELECTOR: u8 = 1;
/// One-Hz denominator floor for drawing a single-carrier viewport; not a grid limit.
pub(crate) const MIN_PROFILE_FREQUENCY_AXIS_SPAN_HZ: u32 = 1;

/// Initial continuous-source widget choice: all 15 register bits set (0x7fff).
/// This explicit editor default is separate from seed validation and sequence period.
pub(crate) const INITIAL_CONTINUOUS_SOURCE_SEED: u16 = 32_767;
/// Independent initial file-fill widget seed, also all ones; not the continuous policy.
pub(crate) const INITIAL_FILE_FILL_SEED: u16 = 32_767;
/// Maximum source-options message bytes, independent of file content and OCD limits.
pub(crate) const MAX_SOURCE_OPTIONS_DOCUMENT_BYTES: usize = 2048;

/// Root group indentation in the profile metadata tree; unrelated to carrier indices.
pub(crate) const PROFILE_TREE_ROOT_DEPTH: u8 = 0;
/// Immediate field/profile indentation beneath a root metadata group.
pub(crate) const PROFILE_TREE_CHILD_DEPTH: u8 = 1;
/// Compact range/detail indentation beneath an individual profile.
pub(crate) const PROFILE_TREE_DETAIL_DEPTH: u8 = 2;

/// Initial UTF-8 byte-order marker (U+FEFF), only for a validated JSON display view.
/// These three encoding octets are transport metadata, unrelated to PHY/source bytes.
pub(crate) const UTF8_JSON_BYTE_ORDER_MARK: &[u8] = b"\xef\xbb\xbf";
