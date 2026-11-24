/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! PRBS-TX source editing; generators and scientific validation remain in the core.
use crate::downstream::inputs::source_input::model;
use crate::shell::{Retained, RetentionReservation};
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
    pub(super) context: crate::downstream::workbench::session::Context,
    model: SourceModel,
    // Drop model data before releasing its actual retained-owner reservation.
    model_reservation: Option<RetentionReservation>,
}

impl SourceInputRust {
    fn replace_model(&mut self, candidate: Retained<SourceModel>) -> Result<(), String> {
        if !self.context.application.budget().owns(&candidate) {
            return Err("Prepared source editor belongs to another application host.".into());
        }
        let (model, reservation) = candidate.into_parts();
        self.model = model;
        drop(self.model_reservation.replace(reservation));
        Ok(())
    }

    fn begin_admitted(&mut self) -> Result<serde_json::Value, String> {
        let (candidate, metadata) = self
            .model
            .begin_admitted(self.context.application.budget())?;
        self.replace_model(candidate)?;
        Ok(metadata)
    }
}

impl bridge::SourceInput {
    /// Transfers an admitted project source or admits a reconstructed fallback.
    ///
    /// # Returns
    /// Unit after same-host model/guard replacement without scientific copies.
    /// Rejection preserves the previous editor and logs its existing-style diagnostic.
    pub fn reload(mut self: core::pin::Pin<&mut Self>) {
        let context = self.rust().context.clone();
        let candidate = crate::downstream::workbench::project_load::take(&context, |prepared| {
            prepared.source.take().map(|model| *model)
        })
        .map(Ok)
        .unwrap_or_else(|| {
            model::SourceModel::from_saved(&crate::downstream::workbench::session::saved(
                &context, "source",
            ))
            .and_then(|model| model.admit(context.application.budget()))
        });
        if let Err(error) =
            candidate.and_then(|model| self.as_mut().rust_mut().replace_model(model))
        {
            log::warn!("Cannot reload source editor: {error}");
        }
    }

    /// Copies accepted file/options into the edit session, without generating bits.
    /// # Returns
    /// Initial options/file metadata after admission, or a JSON error preserving the draft.
    pub fn begin(mut self: core::pin::Pin<&mut Self>) -> QString {
        let result = self.as_mut().rust_mut().begin_admitted();
        QString::from(match result {
            Ok(value) => value.to_string(),
            Err(error) => serde_json::json!({"error":error}).to_string(),
        })
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
                            merlin::downstream::SourceInput::file_limits().max_original_bytes,
                        )?;
                        let candidate = self.rust().model.stage_admitted(
                            self.rust().context.application.budget(),
                            name,
                            bytes,
                        )?;
                        self.as_mut().rust_mut().replace_model(candidate)?;
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
        let context = self.rust().context.clone();
        let result = self
            .as_mut()
            .rust_mut()
            .model
            .apply_transactional(&context, &document.to_string());
        match result {
            Ok(reservation) => {
                drop(
                    self.as_mut()
                        .rust_mut()
                        .model_reservation
                        .replace(reservation),
                );
                QString::from("")
            }
            Err(error) => QString::from(error),
        }
    }

    /// Restores an admitted accepted-file candidate when QML closes the editor.
    ///
    /// # Returns
    /// Unit after model/guard replacement. Admission rejection preserves the staged
    /// model and prior guard and logs a diagnostic; accepted Inputs stay unchanged.
    pub fn cancel(mut self: core::pin::Pin<&mut Self>) {
        if let Err(error) = self.as_mut().rust_mut().begin_admitted() {
            log::warn!("Cannot restore source draft: {error}");
        }
    }

    /// Reports explicit acceptance without requiring a simulation result.
    /// # Returns
    /// Status and accessible reason for the PRBS-TX block.
    pub fn state(&self) -> QString {
        QString::from(self.rust().model.state().to_string())
    }
}

#[cfg(test)]
mod admission_tests {
    use super::*;
    use crate::downstream::workbench::session::Context;
    use crate::shell::{ApplicationSessions, RetainedResources};

    // Explicit fixture ceilings; saturation below uses actual retained charges.
    const FIXTURE_LIMIT: RetainedResources = RetainedResources {
        payload_bytes: usize::MAX,
        ordered_entries: usize::MAX,
        shared_roots: usize::MAX,
    };

    fn context() -> Context {
        Context::for_application(ApplicationSessions::new(FIXTURE_LIMIT).unwrap()).unwrap()
    }

    fn candidate(context: &Context, name: &str) -> Retained<SourceModel> {
        let mut model = SourceModel::default();
        // Exact null/newline original payload; staging alone never accepts it.
        model.stage(name.into(), b"\0original\n".to_vec()).unwrap();
        model.admit(context.application.budget()).unwrap()
    }

    #[test]
    fn prepared_transfer_checks_host_and_moves_existing_charge_under_full_pressure() {
        let target = context();
        let foreign = context();
        let budget = target.application.budget();
        let baseline = budget.usage();
        let mut editor = SourceInputRust {
            context: target.clone(),
            ..Default::default()
        };
        let original = candidate(&target, "old.bin");
        let original_charge = original.fixture_reserved_resources();
        editor.replace_model(original).unwrap();
        let current = editor.model.file_summary();
        let before = budget.usage();
        let foreign_baseline = foreign.application.budget().usage();
        assert!(editor
            .replace_model(candidate(&foreign, "foreign.bin"))
            .is_err());
        assert_eq!(editor.model.file_summary(), current);
        assert!(editor.model_reservation.is_some());
        assert_eq!(budget.usage(), before);
        assert_eq!(foreign.application.budget().usage(), foreign_baseline);

        let prepared = candidate(&target, "prepared.bin");
        let prepared_charge = prepared.fixture_reserved_resources();
        let usage = budget.usage();
        let pressure = budget
            .reserve_resources(RetainedResources {
                payload_bytes: usize::MAX - usage.payload_bytes,
                ordered_entries: usize::MAX - usage.ordered_entries,
                shared_roots: usize::MAX - usage.shared_roots,
            })
            .unwrap();
        editor.replace_model(prepared).unwrap();
        assert_eq!(editor.model.file_summary()["name"], "prepared.bin");
        assert_eq!(
            budget.usage(),
            RetainedResources {
                payload_bytes: usize::MAX - original_charge.payload_bytes,
                ordered_entries: usize::MAX - original_charge.ordered_entries,
                shared_roots: usize::MAX - original_charge.shared_roots,
            }
        );
        drop(pressure);
        assert_eq!(
            budget.usage(),
            RetainedResources {
                payload_bytes: baseline.payload_bytes + prepared_charge.payload_bytes,
                ordered_entries: baseline.ordered_entries + prepared_charge.ordered_entries,
                shared_roots: baseline.shared_roots + prepared_charge.shared_roots,
            }
        );
        drop(editor);
        assert_eq!(budget.usage(), baseline);
    }
}
