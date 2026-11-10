/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Qt adapter for transmitter readiness and bounded worker spectrum queries.
mod model;
use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;

#[cxx_qt::bridge]
pub mod bridge {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }
    extern "RustQt" {
        #[qobject]
        #[qml_element]
        type TransmitterAnalyzer = super::TransmitterAnalyzerRust;
        /// Freezes accepted Rust inputs and OCD JSON, invalidating old results.
        /// # Arguments
        /// * `settings` - Accepted OCD JSON, bounded to 4096 bytes.
        /// # Returns
        /// Scalar readiness/error JSON; no source/profile arrays cross QML.
        #[qinvokable]
        fn configure(self: Pin<&mut TransmitterAnalyzer>, settings: &QString) -> QString;
        /// Starts one bounded Rust worker without blocking Qt.
        #[qinvokable]
        fn generate(self: Pin<&mut TransmitterAnalyzer>);
        /// Requests cooperative cancellation, retaining prior complete results.
        #[qinvokable]
        fn cancel(self: Pin<&mut TransmitterAnalyzer>);
        /// Collects worker completion and reports scalar progress/result identity.
        /// # Returns
        /// Bounded status JSON, excluding samples and full FFT arrays.
        #[qinvokable]
        fn poll(self: Pin<&mut TransmitterAnalyzer>) -> QString;
        /// Queries a bounded min/max frequency viewport.
        /// # Arguments
        /// * `start` - First bin inside retained results.
        /// * `count` - Positive bin count; zero requests full span.
        /// * `capture_axis` - False uses centered baseband; true uses capture metadata.
        /// # Returns
        /// Display points/labels or a structured diagnostic document.
        #[qinvokable]
        fn view(self: &TransmitterAnalyzer, start: i32, count: i32, capture_axis: bool) -> QString;
        /// Queries completed-snapshot window timing and optional cable estimates.
        /// # Arguments
        /// * `velocity` - Cable VF in (0,1]; NaN explicitly means unknown.
        /// # Returns
        /// Bounded geometry/labels, with unavailable distances kept explicit.
        #[qinvokable]
        fn timing(self: &TransmitterAnalyzer, velocity: f64) -> QString;
    }
}

/// GUI-thread owner; scientific work runs on an independent bounded Rust worker.
#[derive(Default)]
pub struct TransmitterAnalyzerRust {
    model: model::Analyzer,
}

impl bridge::TransmitterAnalyzer {
    /// Resolves accepted Rust snapshots and validates supported transmitter readiness.
    /// # Arguments
    /// * `settings` - Bounded accepted OCD settings document.
    /// # Returns
    /// Status JSON; input changes cancel old jobs and invalidate stale samples.
    pub fn configure(mut self: core::pin::Pin<&mut Self>, settings: &QString) -> QString {
        let inputs = crate::workbench_session::snapshot();
        let model = &mut self.as_mut().rust_mut().model;
        model.configure(&settings.to_string(), inputs);
        QString::from(model.status().to_string())
    }
    /// Starts at most one bounded preview job, preserving GUI responsiveness.
    pub fn generate(mut self: core::pin::Pin<&mut Self>) {
        self.as_mut().rust_mut().model.generate();
    }
    /// Requests cancellation without waiting on scientific computation.
    pub fn cancel(mut self: core::pin::Pin<&mut Self>) {
        self.as_mut().rust_mut().model.cancel();
    }
    /// Drains bounded completion and exposes scalar progress/readiness only.
    /// # Returns
    /// Status JSON with a token identifying any new completed result.
    pub fn poll(mut self: core::pin::Pin<&mut Self>) -> QString {
        QString::from(self.as_mut().rust_mut().model.poll().to_string())
    }
    /// Formats retained spectrum data as a bounded min/max viewport.
    /// # Arguments
    /// * `start` - First FFT bin, nonnegative.
    /// * `count` - Bin count, or zero for full span.
    /// * `capture_axis` - True labels bins with captured RF mapping; samples stay baseband.
    /// # Returns
    /// At most 2048 display points with scientific units and Rust-derived labels.
    pub fn view(&self, start: i32, count: i32, capture_axis: bool) -> QString {
        QString::from(
            self.rust()
                .model
                .view(start, count, capture_axis)
                .to_string(),
        )
    }
    /// Adapts immutable result geometry into the shared timing display document.
    /// # Arguments
    /// * `velocity` - Nominal cable VF; NaN means unknown and gives no distance.
    /// # Returns
    /// Bounded timing JSON or validation/unavailable diagnostic.
    pub fn timing(&self, velocity: f64) -> QString {
        QString::from(self.rust().model.timing(velocity).to_string())
    }
}
