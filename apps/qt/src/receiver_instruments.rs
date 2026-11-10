/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Receiver instrument admission until bounded full-loop results are connected.
mod views;
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
        /// Empty observation state with an explicit admission reason and tap IDs.
        #[qinvokable]
        fn qam(self: &ReceiverInstruments, signal: i32, view: i32, part: i32) -> QString;
        /// Selects a registered RX spectrum reference plane without generating data.
        /// # Arguments
        /// * `tap` - RX input, framed, useful or FFT-bin-power index.
        /// # Returns
        /// Shared SA state with generation disabled until retained results exist.
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
        /// Preserves the shared SA cancellation interface; there is no RX worker yet.
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
        1 => "Matched received NCP I/Q classification and collection are pending. NCP LLRs and accepted messages cannot be plotted as received I/Q.",
        2 => "PLC data and preamble are separate observation groups. Their matched raw/equalized collector is not connected to Qt yet.",
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
    /// Admits only known selector values, preserving empty results until RX wiring.
    /// # Arguments
    /// * `signal` - Data profile / NCP / PLC index.
    /// * `view` - Raw / equalized / paired index.
    /// * `part` - PLC data / preamble index.
    /// # Returns
    /// Bounded metadata, no source bytes or synthesized I/Q.
    pub fn qam(&self, signal: i32, view: i32, part: i32) -> QString {
        let mut state =
            views::qam(signal, view, part).unwrap_or_else(|| qam_state(signal, view, part));
        if signal == 1 && crate::closed_loop::model::latest().is_some() {
            state["reason"] = json!("Matched NCP constellation collection is not implemented yet.");
        }
        QString::from(state.to_string())
    }

    /// Maps a presentation choice to the core's registered test-point identity.
    /// # Arguments
    /// * `tap` - Time-sample or separately labeled FFT-bin selection.
    /// # Returns
    /// Shared SA state; all selections currently require full-loop UI results.
    pub fn spectrum(&self, tap: i32) -> QString {
        let mut state = spectrum_state(tap);
        if let Some(result) = crate::closed_loop::model::latest() {
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
    /// No receiver job exists to cancel in this navigation implementation.
    pub fn cancel(&self) {}
}
