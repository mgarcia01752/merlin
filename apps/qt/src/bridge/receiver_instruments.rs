/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded receiver instruments over one shared immutable closed-loop result.
use crate::instruments::receiver::views;
pub(crate) use crate::instruments::receiver::{diagnostics, selection};
use cxx_qt_lib::QString;
use docsis_ofdm_fec::simulation::test_points::TestPointId;
use serde_json::{json, Value};

const NO_RESULTS: &str = "Run the closed loop to retain receiver observations.";

#[cxx_qt::bridge]
pub mod bridge {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }
    extern "RustQt" {
        #[qobject]
        #[qml_element]
        type ReceiverInstruments = super::ReceiverInstrumentsRust;
        /// Describes a received-signal selection without running scientific code.
        /// # Arguments
        /// * `signal` - Data profile, NCP or PLC index.
        /// * `view` - Raw, equalized or paired index.
        /// * `part` - PLC data or preamble index.
        /// # Returns
        /// Actual bounded observation state with explicit admission reason and tap IDs.
        #[qinvokable]
        fn qam(self: &ReceiverInstruments, signal: i32, view: i32, part: i32) -> QString;
        /// Queries actual retained cells with profile/modulation/bin/symbol filters.
        /// # Arguments
        /// * `document` - Bounded selector JSON, never capture data.
        /// # Returns
        /// Bounded matched scatter points or a precise empty/error state.
        #[qinvokable]
        fn qam_selected(self: &ReceiverInstruments, document: &QString) -> QString;
        /// Queries supplied H or actual H-hat and its band-limited impulse diagnostic.
        /// # Arguments
        /// * `document` - Bounded source/snapshot/view/VF JSON.
        /// # Returns
        /// Bounded response/echo plot with explicit reference plane and units.
        #[qinvokable]
        fn response(self: &ReceiverInstruments, document: &QString) -> QString;
        /// Queries actual selected cell decisions/LLRs or the retained LDPC input.
        /// # Arguments
        /// * `document` - Bounded received-role/coordinate selection JSON.
        /// * `decoder_input` - Select the separate coding-order LDPC soft boundary.
        /// # Returns
        /// Finite actual decisions and likelihood statistics, never decoder input mutations.
        #[qinvokable]
        fn slicer(self: &ReceiverInstruments, document: &QString, decoder_input: bool) -> QString;
        /// Reads actual FEC statistics and bounded completed word records.
        /// # Returns
        /// Decoder/comparison counters or an explicit pre-run empty state.
        #[qinvokable]
        fn fec(self: &ReceiverInstruments) -> QString;
        /// Reads a retained FEC result independently of other instrument selections.
        /// # Arguments
        /// * `profile_id` - Captured identity, or -1 to follow the latest result.
        /// # Returns
        /// Bounded decoder charts and rows, including explicit unavailable states.
        #[qinvokable]
        fn fec_profile(self: &ReceiverInstruments, profile_id: i32) -> QString;

        /// Queries one independent profile's measured MER and FEC performance.
        /// # Arguments
        /// * `profile_id` - Captured identity, or -1 for the latest completed profile.
        /// * `start`, `count` - Carrier offsets; zero count restores Full Span.
        /// # Returns
        /// Bounded profile curve and scalar counters; no simulation is started.
        #[qinvokable]
        fn performance(
            self: &ReceiverInstruments,
            profile_id: i32,
            start: i32,
            count: i32,
        ) -> QString;
        /// Selects a registered RX spectrum reference plane without generating data.
        /// # Arguments
        /// * `tap` - RX input, framed, useful or FFT-bin-power index.
        /// # Returns
        /// Shared SA state with local generation disabled; main Start owns execution.
        #[qinvokable]
        fn spectrum(self: &ReceiverInstruments, tap: i32) -> QString;
        /// Queries retained RX time spectra or separately normalized FFT-bin power.
        /// # Arguments
        /// * `tap` - Registered receiver tap index.
        /// * `start` - Nonnegative first display bin.
        /// * `count` - Bin count, zero for full span.
        /// * `capture_axis` - Use captured frequency mapping without RF conversion.
        /// # Returns
        /// Bounded points or an explicit unavailable diagnostic.
        #[qinvokable]
        fn view_tap(
            self: &ReceiverInstruments,
            tap: i32,
            start: i32,
            count: i32,
            capture_axis: bool,
        ) -> QString;
        /// Preserves the shared SA cancellation interface; this instrument does not own the shared run worker.
        #[qinvokable]
        fn cancel(self: &ReceiverInstruments);
    }
}

/// Stateless presentation admission; owns no simulation, RNG or fabricated samples.
#[derive(Default)]
pub struct ReceiverInstrumentsRust;

fn qam_state(signal: i32, view: i32, part: i32) -> Value {
    let valid = (0..3).contains(&signal) && (0..3).contains(&view) && (0..2).contains(&part);
    let detail = match signal {
        0 => "Data-profile selection requires received profile identity and matched carrier/time observations. The current payload collector supports its documented profile only.",
        1 => "NCP requires actual CRC-classified matched receive cells; soft hypotheses alone are not received-role admission.",
        2 => "PLC data and preamble are separate observation groups. Their matched raw/equalized streams retain separate modulation and timing.",
        _ => "Unsupported receiver signal selection.",
    };
    json!({"available":false,"reason":if valid {NO_RESULTS} else {"Invalid receiver instrument selection."},"detail":detail,"raw_tap":TestPointId::RxFft,"equalized_tap":TestPointId::RxEqualized,"points":[],"profiles":[]})
}

fn spectrum_state(tap: i32) -> Value {
    let selected = match tap {
        0 => Some((
            "RX Input / Channel Output",
            TestPointId::ChannelOutput,
            "Complex baseband time samples after channel impairments",
        )),
        1 => Some((
            "RX Framed Samples",
            TestPointId::RxFramed,
            "Framed time samples including cyclic prefix",
        )),
        2 => Some((
            "RX Useful Samples",
            TestPointId::RxUseful,
            "After prefix removal; symbol gaps must remain explicit",
        )),
        3 => Some((
            "RX FFT Bin Power",
            TestPointId::RxFft,
            "Frequency-domain bin power; not a time-sample PSD or a second FFT",
        )),
        _ => None,
    };
    match selected {
        Some((name, id, description)) => {
            json!({"tap_name":name,"tap_id":id,"tap_description":description,"axis_label":if tap == 3 {"FFT Bin Power · no retained observations"} else {"Time-sample Spectrum · no retained observations"},"reason":NO_RESULTS,"message":NO_RESULTS,"has_result":false,"result_token":"","can_generate":false,"busy":false,"points":[]})
        }
        None => {
            json!({"tap_name":"Unavailable","tap_id":"","tap_description":"Invalid receiver test point","reason":"Invalid receiver test point.","message":"Invalid receiver test point.","has_result":false,"result_token":"","can_generate":false,"busy":false,"points":[]})
        }
    }
}

impl bridge::ReceiverInstruments {
    /// Reads a profile performance snapshot independently of the latest detailed taps.
    /// # Arguments
    /// * `profile_id` - Captured identity or -1 for latest; never an array index.
    /// * `start`, `count` - Nonnegative source offsets, zero count for Full Span.
    /// # Returns
    /// Bounded JSON, preserving accepted inputs and other instrument selections.
    pub fn performance(&self, profile_id: i32, start: i32, count: i32) -> QString {
        QString::from(
            crate::instruments::receiver::performance::view(profile_id, start, count).to_string(),
        )
    }

    /// Queries known received-role selectors over the immutable retained result.
    /// # Arguments
    /// * `signal` - Data profile / NCP / PLC index.
    /// * `view` - Raw / equalized / paired index.
    /// * `part` - PLC data / preamble index.
    /// # Returns
    /// Bounded metadata, no source bytes or synthesized I/Q.
    pub fn qam(&self, signal: i32, view: i32, part: i32) -> QString {
        let state = if (0..5).contains(&signal) && (0..3).contains(&view) && (0..2).contains(&part)
        {
            selection::qam(&selection::Query {
                signal: signal as usize,
                view: view as usize,
                part: part as usize,
                ..Default::default()
            })
        } else {
            qam_state(signal, view, part)
        };
        QString::from(state.to_string())
    }
    /// Selects actual retained matched cells without rerunning science.
    /// # Arguments
    /// * `document` - At most4096 bytes of selector JSON.
    /// # Returns
    /// Bounded points/selector metadata or an empty error state.
    pub fn qam_selected(&self, document: &QString) -> QString {
        QString::from(
            selection::Query::parse(&document.to_string())
                .map(|q| selection::qam(&q))
                .unwrap_or_else(
                    |e| json!({"available":false,"points":[],"profiles":[],"orders":[],"reason":e}),
                )
                .to_string(),
        )
    }
    /// Queries response/echo curves with explicit reference and delay coordinates.
    /// # Arguments
    /// * `document` - Bounded response selector JSON.
    /// # Returns
    /// Curve/metadata or an empty diagnostic, never stale observations.
    pub fn response(&self, document: &QString) -> QString {
        QString::from(diagnostics::response(&document.to_string()).unwrap_or_else(|e|json!({"available":false,"points":[],"x_ticks":[],"y_ticks":[],"error":e,"reason":e})).to_string())
    }
    /// Queries received-cell slicing and actual retained soft values.
    /// # Arguments
    /// * `document` - Bounded signal/coordinate selector JSON.
    /// * `decoder_input` - Read the separate payload decoder-input window.
    /// # Returns
    /// Bounded diagnostic rows/histogram or an explicit empty state.
    pub fn slicer(&self, document: &QString, decoder_input: bool) -> QString {
        QString::from(diagnostics::slicer(&document.to_string(),decoder_input).unwrap_or_else(|e|json!({"available":false,"rows":[],"reason":e,"detail":e,"histogram":{"points":[],"x_ticks":[],"y_ticks":[]}})).to_string())
    }
    /// Reads the shared job's genuine decoder and comparison counters.
    /// # Returns
    /// Bounded counters and completed-word rows; no engine state changes.
    pub fn fec(&self) -> QString {
        QString::from(diagnostics::fec().to_string())
    }

    /// Reads one retained profile's decoder summary without running the simulation.
    /// # Arguments
    /// * `profile_id` - Captured identity, or -1 for the latest finished result.
    /// # Returns
    /// Bounded FEC counters, graph values, records and profile catalog JSON.
    pub fn fec_profile(&self, profile_id: i32) -> QString {
        QString::from(crate::instruments::receiver::performance::fec_view(profile_id).to_string())
    }

    /// Maps a presentation choice to the core's registered test-point identity.
    /// # Arguments
    /// * `tap` - Time-sample or separately labeled FFT-bin selection.
    /// # Returns
    /// Shared SA state with retained tap availability and explicit reference planes.
    pub fn spectrum(&self, tap: i32) -> QString {
        let mut state = spectrum_state(tap);
        if let Some(result) = crate::simulation::closed_loop::latest() {
            let available = usize::try_from(tap)
                .ok()
                .and_then(|i| result.observations.taps.get(i))
                .is_some_and(|t| !t.values.is_empty());
            state["has_result"] = json!(available);
            state["result_token"] = json!(format!("{}-{tap}", result.job.elapsed_ms));
            state["reason"] = json!(if available {
                "Retained receiver observations"
            } else {
                "No samples retained for this tap"
            });
        }
        QString::from(state.to_string())
    }

    /// Returns a bounded real-data spectrum viewport without running the simulation.
    /// # Arguments
    /// * `tap` - Receiver sample/bin reference plane.
    /// * `start` - First bin.
    /// * `count` - Requested count; zero uses full span.
    /// * `capture_axis` - True for mapped captured frequency coordinates.
    /// # Returns
    /// Rust-derived display coordinates or a diagnostic with no stale samples.
    pub fn view_tap(&self, tap: i32, start: i32, count: i32, capture_axis: bool) -> QString {
        QString::from(
            views::spectrum(tap, start, count, capture_axis)
                .unwrap_or_else(|e| json!({"error":e,"points":[],"x_ticks":[],"y_ticks":[]}))
                .to_string(),
        )
    }
    /// Completes the shared SA close operation without changing scientific state.
    /// The main workbench owns cancellation; closing this instrument preserves its run.
    pub fn cancel(&self) {}
}
