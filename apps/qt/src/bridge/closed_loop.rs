/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Qt lifecycle adapter for the existing Rust closed-loop engine.
use crate::simulation::closed_loop as model;
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
        type ClosedLoop = super::ClosedLoopRust;
        /// Validates accepted input changes, cancelling stale work.
        /// # Arguments
        /// * `settings` - Accepted OCD JSON, at most 4096 bytes.
        /// # Returns
        /// Bounded next-run readiness and progress state.
        #[qinvokable]
        fn configure(self: Pin<&mut ClosedLoop>, settings: &QString) -> QString;
        /// Accepts receiver/run options atomically through shared Rust validation.
        /// # Arguments
        /// * `options` - Finite receiver/iteration/duration/runtime JSON.
        /// * `settings` - Accepted OCD JSON.
        /// # Returns
        /// Empty on success; an error preserves accepted receiver options.
        #[qinvokable]
        fn apply(self: Pin<&mut ClosedLoop>, options: &QString, settings: &QString) -> QString;
        /// Returns accepted channel gain on the OCD grid without starting simulation.
        /// # Arguments
        /// * `settings` - Accepted OCD JSON, at most4096 bytes.
        /// # Returns
        /// At most1024 chart groups or an empty chart with optional known-code help.
        #[qinvokable]
        fn channel_preview(self: &ClosedLoop, settings: &QString) -> QString;
        /// Returns known guidance from the last Apply failure, without changing settings.
        /// # Returns
        /// Bounded diagnostic JSON or null for an unclassified error.
        #[qinvokable]
        fn admission_help(self: &ClosedLoop) -> QString;
        /// Restores already validated project receiver/comparison choices.
        /// # Arguments
        /// * `options` - Saved receiver options or null.
        /// * `verified` - Saved independent comparison acceptance.
        /// * `settings` - Restored OCD JSON.
        #[qinvokable]
        fn restore(
            self: Pin<&mut ClosedLoop>,
            options: &QString,
            verified: bool,
            settings: &QString,
        );
        /// Enables independent recovered-source comparison for the next run.
        #[qinvokable]
        fn verify(self: Pin<&mut ClosedLoop>);
        /// Starts one frozen closed-loop worker when the full request is ready.
        #[qinvokable]
        fn start(self: Pin<&mut ClosedLoop>);
        /// Requests cooperative cancellation, preserving actual partial results.
        #[qinvokable]
        fn stop(self: &ClosedLoop);
        /// Pauses/resumes the same scientific session at step boundaries.
        #[qinvokable]
        fn pause(self: &ClosedLoop);
        /// Drains completion and returns bounded progress/results metadata.
        /// # Returns
        /// Status JSON excluding full sample/constellation arrays.
        #[qinvokable]
        fn poll(self: Pin<&mut ClosedLoop>) -> QString;
    }
}
/// GUI-thread lifecycle owner; worker jobs use frozen scientific snapshots.
#[derive(Default)]
pub struct ClosedLoopRust {
    pub(crate) model: model::Model,
    admission_diagnostic: Option<Box<crate::help::Diagnostic>>,
}
impl bridge::ClosedLoop {
    /// Resolves accepted inputs and invalidates prior results.
    /// # Arguments
    /// * `settings` - Bounded OCD JSON.
    /// # Returns
    /// Next-run status, never a fabricated scientific result.
    pub fn configure(mut self: core::pin::Pin<&mut Self>, settings: &QString) -> QString {
        if settings.to_string().len() <= 4096 {
            self.as_mut()
                .rust_mut()
                .model
                .configure(&settings.to_string());
        }
        QString::from(self.rust().model.state().to_string())
    }
    /// Projects the current accepted channel model independently of retained runs.
    /// # Arguments
    /// * `settings` - Bounded OCD JSON; no capture arrays cross this boundary.
    /// # Returns
    /// Bounded gain viewport, or an original error with known-code contextual help.
    pub fn channel_preview(&self, settings: &QString) -> QString {
        let result = (|| {
            if settings.to_string().len() > 4096 {
                return Err(crate::help::AdmissionError::from(
                    "Settings too large".to_string(),
                ));
            }
            let transmission = serde_json::from_str(&settings.to_string())
                .map_err(|error| crate::help::AdmissionError::from(error.to_string()))?;
            let options = self.rust().model.options.clone().unwrap_or_default();
            crate::inputs::channel_model::combined_preview(
                &transmission,
                &crate::workbench::session::snapshot(),
                &options.channel_filter,
            )
        })();
        QString::from(result.unwrap_or_else(|error|serde_json::json!({"available":false,"points":[],"x_ticks":[],"y_ticks":[],"error":error.message,"diagnostic":error.diagnostic})).to_string())
    }
    /// Applies supported receiver/run options only after validating the entire request.
    /// # Arguments
    /// * `options` - Bounded JSON options.
    /// * `settings` - Current OCD JSON.
    /// # Returns
    /// Empty on success, otherwise an admission reason.
    pub fn apply(
        mut self: core::pin::Pin<&mut Self>,
        options: &QString,
        settings: &QString,
    ) -> QString {
        let prepare = || -> Result<model::Options, crate::help::AdmissionError> {
            if self.rust().model.state()["busy"].as_bool() == Some(true) {
                return Err(
                    "Stop the simulation before changing receiver or channel settings.".into(),
                );
            }
            if options.to_string().len() > 4096 || settings.to_string().len() > 4096 {
                return Err("Options too large".into());
            }
            let options = serde_json::from_str::<model::Options>(&options.to_string())
                .map_err(|e| e.to_string())?;
            let transmission: docsis_ofdm_fec::transmission::TransmissionSettings =
                serde_json::from_str(&settings.to_string()).map_err(|e| e.to_string())?;
            transmission.validate().map_err(|error| error.to_string())?;
            options
                .mce
                .validate()
                .map_err(crate::help::AdmissionError::classified)?;
            options.validate(&crate::workbench::session::snapshot())?;
            let inputs = crate::workbench::session::snapshot();
            if inputs.source.is_some() && inputs.profile.is_some() && inputs.playback.is_some() {
                model::prepare(&transmission, &inputs, &options)?;
            }
            Ok(options)
        };
        match prepare() {
            Ok(options) => {
                self.as_mut().rust_mut().admission_diagnostic = None;
                self.as_mut().rust_mut().model.options = Some(options);
                self.as_mut()
                    .rust_mut()
                    .model
                    .configure(&settings.to_string());
                QString::from("")
            }
            Err(error) => {
                let message = error.to_string();
                self.as_mut().rust_mut().admission_diagnostic = error.diagnostic;
                QString::from(message)
            }
        }
    }
    /// Reads transient Apply help while preserving the accepted simulation state.
    /// # Returns
    /// Registered catalog metadata or null; never parses an error message for identity.
    pub fn admission_help(&self) -> QString {
        QString::from(serde_json::json!(self.rust().admission_diagnostic).to_string())
    }
    /// Restores saved receiver choices, deriving readiness from restored accepted inputs.
    /// # Arguments
    /// * `options` - Validated project options/null.
    /// * `verified` - Comparison setup.
    /// * `settings` - Restored transmission geometry.
    pub fn restore(
        mut self: core::pin::Pin<&mut Self>,
        options: &QString,
        verified: bool,
        settings: &QString,
    ) {
        if let Some(model) =
            crate::workbench::project_load::take(|prepared| prepared.receiver.take())
        {
            self.as_mut().rust_mut().model.invalidate();
            self.as_mut().rust_mut().model = model;
            return;
        }
        if options.to_string().len() > 4096 {
            return;
        }
        if let Ok(options) = serde_json::from_str::<Option<model::Options>>(&options.to_string()) {
            self.as_mut().rust_mut().model.options = options;
            self.as_mut().rust_mut().model.verify = verified;
            self.as_mut()
                .rust_mut()
                .model
                .configure(&settings.to_string());
        }
    }
    /// Accepts independent source comparison without generating data.
    pub fn verify(mut self: core::pin::Pin<&mut Self>) {
        self.as_mut().rust_mut().model.verify = true;
    }
    /// Starts at most one bounded Rust worker.
    pub fn start(mut self: core::pin::Pin<&mut Self>) {
        self.as_mut().rust_mut().model.start();
    }
    /// Stops active work cooperatively, including while paused.
    pub fn stop(&self) {
        self.rust().model.stop();
    }
    /// Toggles a step-boundary pause gate without resetting the engine.
    pub fn pause(&self) {
        self.rust().model.pause();
    }
    /// Collects bounded completion and progress metadata.
    /// # Returns
    /// JSON status and genuine decoder/comparison totals.
    pub fn poll(mut self: core::pin::Pin<&mut Self>) -> QString {
        QString::from(self.as_mut().rust_mut().model.poll().to_string())
    }
}
