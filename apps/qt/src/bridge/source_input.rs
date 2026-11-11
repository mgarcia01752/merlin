/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! PRBS-TX source editing; generators and scientific validation remain in the core.
use crate::inputs::source_input::model;
use cxx_qt::CxxQtType;
use cxx_qt_lib::QString;
use model::SourceModel;

#[cxx_qt::bridge]
pub mod bridge {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }
    extern "RustQt" {
        #[qobject]
        #[qml_element]
        type SourceInput = super::SourceInputRust;

        /// Starts editing from accepted source or unaccepted PRBS defaults.
        /// # Returns
        /// Bounded metadata/options JSON; no source bits are generated.
        #[qinvokable]
        fn begin(self: Pin<&mut SourceInput>) -> QString;
        /// Opens the OS picker and stages exact bytes; Cancel preserves the draft.
        /// # Returns
        /// File summary JSON or a diagnostic; no automatic acceptance.
        #[qinvokable]
        fn choose_file(self: Pin<&mut SourceInput>) -> QString;
        /// Validates proposed options against the current Rust-owned draft file.
        /// # Arguments
        /// * `document` - Bounded options JSON, excluding payload bytes/paths.
        /// # Returns
        /// Rust-derived bit counts or a validation diagnostic.
        #[qinvokable]
        fn describe(self: &SourceInput, document: &QString) -> QString;
        /// Commits a validated source for the next run without starting a job.
        /// # Arguments
        /// * `document` - Draft options JSON.
        /// # Returns
        /// Empty on success; diagnostic on failure preserving accepted state.
        #[qinvokable]
        fn apply(self: Pin<&mut SourceInput>, document: &QString) -> QString;
        /// Discards staged file changes without altering accepted configuration.
        #[qinvokable]
        fn cancel(self: Pin<&mut SourceInput>);
        /// Reloads accepted source state after atomic workbench Open.
        #[qinvokable]
        fn reload(self: Pin<&mut SourceInput>);
        /// Reports readiness from explicit accepted source configuration.
        /// # Returns
        /// Compact status/reason JSON; defaults alone never imply acceptance.
        #[qinvokable]
        fn state(self: &SourceInput) -> QString;
    }
}

/// GUI-thread adapter retaining immutable files and a separate accepted snapshot.
#[derive(Default)]
pub struct SourceInputRust {
    model: SourceModel,
}

impl bridge::SourceInput {
    /// Adopts already validated project source state on the GUI thread.
    pub fn reload(mut self: core::pin::Pin<&mut Self>) {
        if let Ok(model) =
            model::SourceModel::from_saved(&crate::workbench::session::saved("source"))
        {
            self.as_mut().rust_mut().model = model;
        }
    }

    /// Copies accepted file/options into the edit session, without generating bits.
    /// # Returns
    /// Initial options and compact original-file metadata.
    pub fn begin(mut self: core::pin::Pin<&mut Self>) -> QString {
        let model = &mut self.as_mut().rust_mut().model;
        QString::from(model.begin().to_string())
    }

    /// Uses GTK's native chooser; successful selection stages a bounded local file.
    /// # Returns
    /// Metadata JSON or an error; picker cancellation preserves the staged file.
    pub fn choose_file(mut self: core::pin::Pin<&mut Self>) -> QString {
        let result =
            crate::platform::native_file_picker::choose("Open Transmit Payload (Text or Binary)")
                .and_then(|url| {
                    if let Some(url) = url {
                        let (name, bytes) = crate::platform::input_file::read_local(
                            &url,
                            docsis_ofdm_fec::simulation::source::file::MAX_FILE_BYTES,
                        )?;
                        self.as_mut().rust_mut().model.stage(name, bytes)?;
                    }
                    Ok(self.rust().model.file_summary())
                });
        QString::from(match result {
            Ok(value) => value.to_string(),
            Err(error) => serde_json::json!({"error":error}).to_string(),
        })
    }

    /// Describes counts with the same core validators used on Apply.
    /// # Arguments
    /// * `document` - Options JSON limited to 2048 bytes.
    /// # Returns
    /// Summary JSON; invalid drafts report a diagnostic without modifying acceptance.
    pub fn describe(&self, document: &QString) -> QString {
        QString::from(
            self.rust()
                .model
                .describe(&document.to_string())
                .to_string(),
        )
    }

    /// Accepts validated PRBS/file options atomically for a future job.
    /// # Arguments
    /// * `document` - Bounded options JSON, with explicit mode and fill policy.
    /// # Returns
    /// Empty on success; error text otherwise.
    pub fn apply(mut self: core::pin::Pin<&mut Self>, document: &QString) -> QString {
        let result = self.as_mut().rust_mut().model.accept(&document.to_string());
        if result.is_ok() {
            crate::workbench::session::set_source(self.rust().model.snapshot());
            crate::workbench::session::put_saved("source", self.rust().model.export());
        }
        QString::from(result.err().unwrap_or_default())
    }

    /// Restores the accepted file; QML discards its transient options on close.
    pub fn cancel(mut self: core::pin::Pin<&mut Self>) {
        self.as_mut().rust_mut().model.begin();
    }

    /// Reports explicit acceptance without requiring a simulation result.
    /// # Returns
    /// Status and accessible reason for the PRBS-TX block.
    pub fn state(&self) -> QString {
        QString::from(self.rust().model.state().to_string())
    }
}
