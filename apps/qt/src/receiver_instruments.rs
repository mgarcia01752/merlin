/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Receiver instrument admission until bounded full-loop results are connected.
use cxx_qt_lib::QString;
use docsis_ofdm_fec::simulation::test_points::TestPointId;
use serde_json::{json, Value};

const NO_RESULTS: &str =
    "No retained receiver results. Full-loop UI run dispatch is not connected yet.";

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
        QString::from(qam_state(signal, view, part).to_string())
    }

    /// Maps a presentation choice to the core's registered test-point identity.
    /// # Arguments
    /// * `tap` - Time-sample or separately labeled FFT-bin selection.
    /// # Returns
    /// Shared SA state; all selections currently require full-loop UI results.
    pub fn spectrum(&self, tap: i32) -> QString {
        QString::from(spectrum_state(tap).to_string())
    }

    /// Completes the shared SA close operation without changing scientific state.
    /// No receiver job exists to cancel in this navigation implementation.
    pub fn cancel(&self) {}
}
