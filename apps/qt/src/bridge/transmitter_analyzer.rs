/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Qt adapter for transmitter readiness and bounded worker spectrum queries.
use crate::instruments::transmitter::preview as model;
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
        /// Queries a bounded page of an actual completed-run transmitter boundary.
        /// # Arguments
        /// * `stage` - Registered presentation stage index0..8.
        /// * `offset` - Nonnegative first retained element.
        /// # Returns
        /// At most64 rows and2048 scatter points, or explicit unavailability.
        #[qinvokable]
        fn stage(self: &TransmitterAnalyzer, stage: i32, offset: i32) -> QString;
        /// Queries six bounded graphical test-point cards from retained results.
        /// # Returns
        /// Graph data and explicit missing states; no simulation is started.
        #[qinvokable]
        fn stage_previews(self: &TransmitterAnalyzer) -> QString;
        /// Selects actual physical TX bins before channel application.
        /// # Arguments
        /// * `first` - Inclusive physical bin, -1 for no lower bound.
        /// * `last` - Inclusive physical bin, -1 for no upper bound.
        /// # Returns
        /// Bounded actual I/Q and coordinates, or explicit unavailability.
        #[qinvokable]
        fn channel_qam(self: &TransmitterAnalyzer, first: i32, last: i32) -> QString;
        /// Reads actual TX PLC packet headers, TLVs and bounded byte rows.
        /// # Returns
        /// At most32 reassembled packets with completed-frame provenance.
        #[qinvokable]
        fn messages(self: &TransmitterAnalyzer) -> QString;
        /// Describes a retained-run waveform test point without running a job.
        /// # Arguments
        /// * `tap` - IFFT, prefix, window or overlap-add index0..3.
        /// # Returns
        /// SA availability and immutable reference-plane identity.
        #[qinvokable]
        fn spectrum_tap(self: &TransmitterAnalyzer, tap: i32) -> QString;
        /// Formats the selected retained TX waveform spectrum.
        /// # Arguments
        /// * `tap` - Waveform boundary0..3.
        /// * `start` - First bin, nonnegative.
        /// * `count` - Bin count, or zero for full span.
        /// * `capture_axis` - Display capture-mapped frequency coordinates.
        /// # Returns
        /// Bounded shared spectrum viewport or an unavailable/error document.
        #[qinvokable]
        fn view_tap(
            self: &TransmitterAnalyzer,
            tap: i32,
            start: i32,
            count: i32,
            capture_axis: bool,
        ) -> QString;
    }
}

/// GUI-thread owner; scientific work runs on an independent bounded Rust worker.
#[derive(Default)]
pub struct TransmitterAnalyzerRust {
    model: model::Analyzer,
}

impl bridge::TransmitterAnalyzer {
    /// Queries immutable TX stage data without starting or altering a simulation.
    /// # Arguments
    /// * `stage` - Presentation stage0..8.
    /// * `offset` - First retained element in the bounded page.
    /// # Returns
    /// Small JSON page with original metadata and finite scatter points.
    pub fn stage(&self, stage: i32, offset: i32) -> QString {
        QString::from(crate::transmitter_instruments::stage(stage, offset).to_string())
    }
    /// Returns actual retained graphical previews without scientific recomputation.
    /// # Returns
    /// Six bounded graphs with indices/provenance and no exact-value tables.
    pub fn stage_previews(&self) -> QString {
        QString::from(crate::transmitter_instruments::stage_previews().to_string())
    }
    /// Selects an immutable physical-bin window without running a simulation.
    /// # Arguments
    /// * `first` - Inclusive physical bin or -1 for all.
    /// * `last` - Inclusive physical bin or -1 for all.
    /// # Returns
    /// At most2048 real TX points, without inferred data/control labels.
    pub fn channel_qam(&self, first: i32, last: i32) -> QString {
        QString::from(crate::transmitter_instruments::channel_input::qam(first, last).to_string())
    }
    /// Reads actual retained TX management packets through the existing decoders.
    /// # Returns
    /// Bounded header/TLV/byte records with transmission status.
    pub fn messages(&self) -> QString {
        QString::from(crate::transmitter_instruments::messages().to_string())
    }
    /// Reports the availability of one genuine TX waveform plane.
    /// # Arguments
    /// * `tap` - IFFT, prefix, window or overlap-add index.
    /// # Returns
    /// Scalar shared SA state; no sample arrays cross this method.
    pub fn spectrum_tap(&self, tap: i32) -> QString {
        QString::from(crate::transmitter_instruments::spectrum_state(tap).to_string())
    }
    /// Adapts a retained TX time-sample spectrum to a bounded viewport.
    /// # Arguments
    /// * `tap` - Waveform plane0..3.
    /// * `start` - First FFT display bin.
    /// * `count` - Bin count, or zero for full span.
    /// * `capture_axis` - Use mapped frequency labels without RF upconversion.
    /// # Returns
    /// Normalized viewport points or an explicit diagnostic.
    pub fn view_tap(&self, tap: i32, start: i32, count: i32, capture_axis: bool) -> QString {
        QString::from(
            crate::transmitter_instruments::spectrum(tap, start, count, capture_axis).to_string(),
        )
    }
    /// Resolves accepted Rust snapshots and validates supported transmitter readiness.
    /// # Arguments
    /// * `settings` - Bounded accepted OCD settings document.
    /// # Returns
    /// Status JSON; input changes cancel old jobs and invalidate stale samples.
    pub fn configure(mut self: core::pin::Pin<&mut Self>, settings: &QString) -> QString {
        if let Some(model) =
            crate::workbench::project_load::take(|prepared| prepared.transmitter.take())
        {
            self.as_mut().rust_mut().model.cancel();
            self.as_mut().rust_mut().model = model;
            return QString::from(self.rust().model.status().to_string());
        }
        let inputs = crate::workbench::session::snapshot();
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
