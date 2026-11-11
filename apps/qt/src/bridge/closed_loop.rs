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
        let prepare = || -> Result<model::Options, String> {
            if options.to_string().len() > 4096 || settings.to_string().len() > 4096 {
                return Err("Options too large".into());
            }
            let options = serde_json::from_str::<model::Options>(&options.to_string())
                .map_err(|e| e.to_string())?;
            let transmission =
                serde_json::from_str(&settings.to_string()).map_err(|e| e.to_string())?;
            model::prepare_sweep(
                &transmission,
                &crate::workbench::session::snapshot(),
                &options,
            )?;
            Ok(options)
        };
        match prepare() {
            Ok(options) => {
                self.as_mut().rust_mut().model.options = Some(options);
                self.as_mut()
                    .rust_mut()
                    .model
                    .configure(&settings.to_string());
                QString::from("")
            }
            Err(error) => QString::from(error),
        }
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
