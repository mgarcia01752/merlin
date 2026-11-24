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

/// Current native saved-workbench format written by New, Save and recovery.
/// Legacy readers and independent compatibility vectors retain their older versions.
pub(crate) const WORKBENCH_SCHEMA_VERSION: u32 = 2;

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

/// One generated Designer batch may await GUI collection for its single worker.
/// Independent of export-product and idle-preview completion queue policies.
pub(crate) const DESIGNER_CAPTURE_QUEUE_CAPACITY: usize = 1;

/// Shared generation/export fallback when an owned error cannot fit admission.
/// This static transport message has no retained String or invented help data.
pub(crate) const DESIGNER_WORKER_ERROR_ADMISSION_MESSAGE: &str =
    "Designer completion exceeds retained-data limits.";

/// Publication succeeded, but its queued path/summary could not fit admission.
pub(crate) const DESIGNER_SAVED_STATUS_ADMISSION_MESSAGE: &str =
    "Export saved; its result details exceed retained-data limits.";
/// Independent cancellation outcome when detailed completion cannot be retained.
pub(crate) const DESIGNER_CANCELLED_STATUS_ADMISSION_MESSAGE: &str =
    "Export cancelled; its result details exceed retained-data limits.";
/// Publication failed; this fallback does not incorrectly claim a successful save.
pub(crate) const DESIGNER_FAILED_STATUS_ADMISSION_MESSAGE: &str =
    "Export failed; its error details exceed retained-data limits.";

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

/// Existing aggregate original-byte ceiling for measured-input editor collections.
/// This UI retention policy is shared by import and Designer batches; it remains
/// independent of the public measurement sequence's 96-MiB original-byte budget.
pub(crate) const MAX_MEASUREMENT_COLLECTION_ORIGINAL_BYTES: usize = 64 * 1024 * 1024;

/// Maximum independent receiver profiles retained for performance and QAM history.
/// This UI history cap is separate from source-profile identity and engine admission.
pub(crate) const MAX_RETAINED_RECEIVER_PROFILES: usize = 32;
/// Initial upper performance-axis edge in dB; not a receiver operating threshold.
pub(crate) const PERFORMANCE_INITIAL_MAXIMUM_DB: f64 = 45.0;
/// Initial lower performance-axis edge, independent of missing/zero-valued MER.
pub(crate) const PERFORMANCE_INITIAL_MINIMUM_DB: f64 = 0.0;
/// Round observed MER drawing bounds to this grid; never round scientific results.
pub(crate) const PERFORMANCE_AXIS_QUANTUM_DB: f64 = 5.0;
/// Maximum curve groups in this viewer, independent of source observation retention.
pub(crate) const PERFORMANCE_CURVE_BUCKETS: usize = 1024;
/// Vertical tick intervals, independent of the horizontal frequency tick policy.
pub(crate) const PERFORMANCE_METRIC_TICK_INTERVALS: u32 = 4;
/// Existing ideal-capacity curve identifier in the presentation protocol.
pub(crate) const PERFORMANCE_IDEAL_SERIES_ID: u8 = 0;
/// Existing observed-MER curve identifier; unrelated to source profile identities.
pub(crate) const PERFORMANCE_OBSERVED_SERIES_ID: u8 = 1;

/// Tiny drawing floor for channel/post-amplifier scatter, independent of TX's unit floor.
pub(crate) const CHANNEL_SCATTER_MIN_EXTENT: f64 = 1e-9;
/// Existing receiver-input level warning target, not a channel admission constraint.
pub(crate) const CHANNEL_LEVEL_TARGET_MIN_DBMV_PER_6MHZ: f64 = -12.0;
/// Upper edge of that same displayed band-average operating target.
pub(crate) const CHANNEL_LEVEL_TARGET_MAX_DBMV_PER_6MHZ: f64 = 12.0;

/// Existing response-selection transport ceiling, independent of capture/model admission.
pub(crate) const MAX_RESPONSE_SELECTOR_BYTES: usize = 4096;
/// Configured-transfer source selector in the response-view protocol.
pub(crate) const RESPONSE_CONFIGURED_SOURCE: usize = 0;
/// Actual receiver-estimate source selector; absence never selects configured H.
pub(crate) const RESPONSE_ESTIMATED_SOURCE: usize = 1;
/// Presentation comparison combines the two independent public response queries.
pub(crate) const RESPONSE_COMPARISON_SOURCE: usize = 2;
/// Existing last response snapshot offered by the bounded UI selector (four entries).
pub(crate) const RESPONSE_MAX_SNAPSHOT_INDEX: usize = 3;
/// Last response view selects the backend's band-limited impulse observation.
pub(crate) const RESPONSE_IMPULSE_VIEW: usize = 4;
/// Gain view selector, independent of the equal-valued configured-source selector.
pub(crate) const RESPONSE_GAIN_VIEW: usize = 0;
/// Wrapped phase view selector in the response-view protocol.
pub(crate) const RESPONSE_PHASE_VIEW: usize = 1;
/// Real coefficient view selector, unrelated to source-comparison selection.
pub(crate) const RESPONSE_REAL_VIEW: usize = 2;
/// Initial uniform-cable diagnostic ruler percentage; not an inferred measurement.
pub(crate) const INITIAL_RESPONSE_VELOCITY_PERCENT: u32 = 87;
/// Minimum integer percentage offered by the response ruler editor.
pub(crate) const MIN_RESPONSE_VELOCITY_PERCENT: u32 = 1;
/// Maximum integer percentage; the public backend independently validates (0, 1].
pub(crate) const MAX_RESPONSE_VELOCITY_PERCENT: u32 = 100;

/// Initial editor replay interval as unsigned physical-OFDM-symbol decimal text.
/// Borrow static text for untouched editors; parsed scientific units stay unchanged.
pub(crate) const DEFAULT_REPLAY_INTERVAL_TEXT: &str = "1000";
/// Existing RxMER/profile original-epoch association default, independent of H.
pub(crate) const DEFAULT_RXMER_ASSOCIATION_SECONDS: u32 = 90000;
/// Existing H/profile original-epoch association default; independently editable.
pub(crate) const DEFAULT_RESPONSE_ASSOCIATION_SECONDS: u32 = 90000;
/// Persisted channel-model section version, independent of power/project versions.
pub(crate) const CHANNEL_MODEL_SECTION_VERSION: u32 = 1;
/// Persisted scalar power section version, independent of channel-model version.
pub(crate) const CHANNEL_POWER_SECTION_VERSION: u32 = 1;
/// Drawing margin around additional/effective gain curves in dB.
pub(crate) const CHANNEL_GAIN_AXIS_PADDING_DB: f64 = 1.0;
/// Vertical label intervals for the additional/effective gain editor chart.
pub(crate) const CHANNEL_GAIN_TICK_INTERVALS: u32 = 4;
/// Minimum extra PLC context in original source offsets; not a PHY carrier count.
pub(crate) const MIN_PLC_PREVIEW_CONTEXT: usize = 8;
/// Display padding relative to PLC span, independent of carrier allocation.
pub(crate) const PLC_PREVIEW_CONTEXT_MULTIPLIER: usize = 2;

/// Measurement-dialog gain selector; independent of receiver instrument protocols.
pub(crate) const MEASUREMENT_GAIN_PROJECTION: i32 = 0;
/// Measurement-dialog wrapped-phase selector in the existing serialized UI protocol.
pub(crate) const MEASUREMENT_PHASE_PROJECTION: i32 = 1;
/// Measurement-dialog real-component selector, not a carrier role or count.
pub(crate) const MEASUREMENT_REAL_PROJECTION: i32 = 2;
/// Measurement-dialog imaginary-component selector in the existing UI protocol.
pub(crate) const MEASUREMENT_IMAGINARY_PROJECTION: i32 = 3;

/// Existing initial flat Designer MER shown before user edits; backend validates the domain.
pub(crate) const DEFAULT_DESIGN_BASELINE_DB: f64 = 48.0;

/// Minimum symmetric empty Designer drawing extent; not an admitted MER bound.
pub(crate) const DESIGNER_EMPTY_AXIS_EXTENT_DB: f64 = 1.0;
/// Fractional margin around each Designer trace before sharing its drawing axis.
pub(crate) const DESIGNER_AXIS_PADDING_FRACTION: f64 = 0.1;
/// Independent minimum drawing margin; not the empty-axis extent or a MER limit.
pub(crate) const DESIGNER_MIN_AXIS_PADDING_DB: f64 = 1.0;
/// Vertical drawing tick intervals; independent of capture count and recipe scope.
pub(crate) const DESIGNER_METRIC_TICK_INTERVALS: u32 = 4;

/// Existing serialized Designer recipe/source-control input ceiling in UTF-8 bytes.
pub(crate) const MAX_DESIGNER_RECIPE_DOCUMENT_BYTES: usize = 24 * 1024;
/// Existing serialized parent playback/selection input ceiling in UTF-8 bytes.
/// Equal to the recipe ceiling today, with an independent protocol purpose.
pub(crate) const MAX_DESIGNER_PLAYBACK_DOCUMENT_BYTES: usize = 24 * 1024;
/// Existing export capture-range/clamping/collection input ceiling in UTF-8 bytes.
pub(crate) const MAX_DESIGNER_EXPORT_SELECTION_BYTES: usize = 1024;
/// One terminal product per export worker; progress uses its separate atomic count.
pub(crate) const EXPORT_RESULT_QUEUE_CAPACITY: usize = 1;

/// Existing receiver selector message ceiling; independent of response/editor documents.
pub(crate) const MAX_RECEIVER_SELECTION_DOCUMENT_BYTES: usize = 4096;
/// Existing UI run selector sentinel; not an open physical-bin or source-profile filter.
pub(crate) const LATEST_RECEIVER_RUN_SELECTOR: i32 = -1;
/// Highest source-catalog byte identity exposed by the history selector, not run admission.
pub(crate) const MAX_RECEIVER_SOURCE_PROFILE_SELECTOR: i32 = u8::MAX as i32;
/// Independent wire-profile-filter sentinel; it does not select the latest captured run.
pub(crate) const ANY_RECEIVER_WIRE_PROFILE_SELECTOR: i32 = -1;
/// Existing wire-byte filter envelope; recognized filters need not be implemented transports.
pub(crate) const MAX_RECEIVER_WIRE_PROFILE_SELECTOR: i32 = u8::MAX as i32;
/// Existing open temporal bound; independent of the equal-valued physical-bin sentinel.
pub(crate) const OPEN_RECEIVER_SYMBOL_SELECTOR: i64 = -1;
/// Existing inclusive UI channel-symbol selector ceiling, not an API/runtime domain limit.
pub(crate) const MAX_RECEIVER_SELECTION_SYMBOL_INDEX: i64 = 1_000_000;
/// Payload signal selector in the existing receiver widget protocol.
pub(crate) const RECEIVER_PAYLOAD_SIGNAL: usize = 0;
/// NCP selector; unrelated to payload profile IDs or sample values.
pub(crate) const RECEIVER_NCP_SIGNAL: usize = 1;
/// PLC selector; its part field distinguishes data from preamble.
pub(crate) const RECEIVER_PLC_SIGNAL: usize = 2;
/// Continuous-pilot selector in the receiver widget protocol.
pub(crate) const RECEIVER_CONTINUOUS_PILOT_SIGNAL: usize = 3;
/// Scattered-pilot selector and final admitted signal index.
pub(crate) const RECEIVER_SCATTERED_PILOT_SIGNAL: usize = 4;
/// Existing equalized-view widget default; not a scientific correction mode.
pub(crate) const RECEIVER_EQUALIZED_VIEW: usize = 1;
/// Final existing paired-view selector; both raw/corrected data always come from Rust.
pub(crate) const RECEIVER_PAIRED_VIEW: usize = 2;
/// PLC data subview, unrelated to the equal-valued payload signal selector.
pub(crate) const RECEIVER_PLC_DATA_PART: usize = 0;
/// PLC preamble subview and final admitted part index.
pub(crate) const RECEIVER_PLC_PREAMBLE_PART: usize = 1;
/// Existing unit-sized receiver scatter drawing floor, independent of channel/TX floors.
pub(crate) const RECEIVER_SCATTER_MIN_EXTENT: f64 = 1.0;
/// Legacy likelihood histogram label midpoint within an existing backend-reported interval.
pub(crate) const RECEIVER_LLR_BIN_CENTRE_FRACTION: f64 = 0.5;

/// Legacy modulation-order sentinel selecting the first backend-reported available order.
pub(crate) const AUTO_RECEIVER_ORDER_SELECTOR: u16 = 0;
