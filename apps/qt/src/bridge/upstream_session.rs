/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Thin QObject adapter; the direction model owns asynchronous public API calls.
use crate::shared::presentation::constants::MAX_VIEW_REQUEST_JSON_BYTES;
use crate::upstream::workbench::MAX_GRID_REQUEST_BYTES;
use crate::upstream::workbench::MAX_INSTRUMENT_KIND_CHARACTERS;
use crate::upstream::workbench::{CaptureFamily, MAX_CAPTURE_FAMILY_CHARACTERS};
use crate::upstream::workbench::{Model, MAX_CONFIGURATION_BYTES};
use crate::upstream::workbench::{MAX_DRAFT_CHANGE_BYTES, MAX_DRAFT_ID_CHARACTERS};
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
        type UpstreamSession = super::UpstreamSessionRust;

        /// Starts native upstream project Save or Save As without blocking on I/O.
        /// # Arguments
        /// * `save_as` - Select a new destination; first Save also uses the picker.
        /// # Returns
        /// Empty after dispatch/chooser Cancel; Poll supplies actual completion.
        #[qinvokable]
        fn save_project(self: Pin<&mut UpstreamSession>, save_as: bool) -> QString;

        /// Captures native upstream New identity before a dirty-state decision.
        /// # Returns
        /// Empty after capture; Poll's project decision contains only its action tag.
        #[qinvokable]
        fn request_new_project(self: Pin<&mut UpstreamSession>) -> QString;

        /// Selects a native upstream project before its target dirty decision.
        /// # Returns
        /// Empty after chooser Cancel/capture; no file is adopted until resolution.
        #[qinvokable]
        fn choose_open_project(self: Pin<&mut UpstreamSession>) -> QString;

        /// Resolves the previously captured native upstream project intent.
        /// # Arguments
        /// * `discard`, `cancel` - Explicit prompt outcome; Cancel preserves all data.
        /// # Returns
        /// Empty after dispatch/adoption, or the original stale/dirty diagnostic.
        #[qinvokable]
        fn resolve_project(self: Pin<&mut UpstreamSession>, discard: bool, cancel: bool)
            -> QString;

        /// Selects a real local upstream configuration using the OS-native picker.
        /// # Returns
        /// Empty on Cancel or queued preparation; the original error otherwise.
        /// Poll alone establishes eventual acceptance. No simulation is started.
        #[qinvokable]
        fn choose_configuration(self: Pin<&mut UpstreamSession>) -> QString;

        /// Opens a native capture picker for independent source inspection.
        /// # Arguments
        /// * `family` - Exact UCD, RxMER or Pre-Eq presentation destination.
        /// # Returns
        /// Empty on Cancel or queued admission; an actionable error otherwise.
        #[qinvokable]
        fn choose_capture(self: Pin<&mut UpstreamSession>, family: &QString) -> QString;

        /// Requests another bounded page of the same admitted original capture.
        /// # Arguments
        /// * `family` - Original capture family.
        /// * `first` - Nonnegative original record index, validated by the worker.
        /// # Returns
        /// Empty after dispatch; no accepted scientific configuration is changed.
        #[qinvokable]
        fn capture_page(self: Pin<&mut UpstreamSession>, family: &QString, first: i32) -> QString;

        /// Requests original measured RxMER coordinates for a bounded shared plot.
        /// # Arguments
        /// * `family` - Explicit RxMER source destination, never another capture.
        /// * `first`, `count` - Nonnegative original record viewport; zero span
        ///   selects the remaining bounded prefix through the native projection.
        /// # Returns
        /// Empty after dispatch; errors retain the original source and old view.
        #[qinvokable]
        fn capture_plot(
            self: Pin<&mut UpstreamSession>,
            family: &QString,
            first: i32,
            count: i32,
        ) -> QString;

        /// Requests the configured base grant's default public resource rectangle.
        /// # Returns
        /// Empty after background dispatch; Poll establishes actual acceptance.
        #[qinvokable]
        fn begin_resource_grid(self: Pin<&mut UpstreamSession>) -> QString;

        /// Requests an owned configured viewport with exact original context.
        /// # Arguments
        /// * `request`, `context` - Bounded JSON fields; no resource roles or samples.
        /// # Returns
        /// Empty after dispatch; errors retain previous scientific and display data.
        #[qinvokable]
        fn resource_grid(
            self: Pin<&mut UpstreamSession>,
            request: &QString,
            context: &QString,
        ) -> QString;

        /// Opens one finite public upstream observation without executing a run.
        /// # Arguments
        /// * `kind` - Existing QAM, spectrum, response or FEC presentation destination.
        /// # Returns
        /// Empty after background dispatch; the native diagnostic otherwise.
        #[qinvokable]
        fn begin_instrument(self: Pin<&mut UpstreamSession>, kind: &QString) -> QString;

        /// Forwards an exact typed selector and captured observation context.
        /// # Arguments
        /// * `kind`, `selection`, `context` - Bounded application transport only.
        /// # Returns
        /// Empty after dispatch; errors preserve previously accepted observations.
        #[qinvokable]
        fn instrument(
            self: Pin<&mut UpstreamSession>,
            kind: &QString,
            selection: &QString,
            context: &QString,
        ) -> QString;

        /// Opens one isolated native configuration draft with exact current values.
        /// # Arguments
        /// * `section` - Existing upstream configuration section tag.
        /// # Returns
        /// Empty after staging the draft; Poll supplies its opaque token and base.
        #[qinvokable]
        fn begin_draft(self: Pin<&mut UpstreamSession>, section: &QString) -> QString;

        /// Queues changed fields for an exact existing native draft.
        /// # Arguments
        /// * `section`, `token` - Original unchanged identifiers.
        /// * `changes` - Bounded JSON object containing changed strings/selections only.
        /// # Returns
        /// Empty means queued; accepted is reported only after validated adoption.
        #[qinvokable]
        fn apply_draft(
            self: Pin<&mut UpstreamSession>,
            section: &QString,
            token: &QString,
            changes: &QString,
        ) -> QString;

        /// Discards only the exact configuration draft identified by its token.
        /// # Arguments
        /// * `token` - Original native draft token; foreign tokens reject.
        /// # Returns
        /// Empty after cancellation request; this never stops an active simulation.
        #[qinvokable]
        fn cancel_draft(self: Pin<&mut UpstreamSession>, token: &QString) -> QString;

        /// Starts off-GUI validation of an existing public configuration document.
        /// # Arguments
        /// * `document` - Original bounded upstream configuration JSON.
        /// # Returns
        /// Empty when staged; immediate error otherwise. Poll reports final acceptance.
        #[qinvokable]
        fn apply(self: Pin<&mut UpstreamSession>, document: &QString) -> QString;
        /// Requests off-GUI target preflight followed by shared run admission.
        /// # Returns
        /// Empty when requested; error preserves prior configuration and report.
        #[qinvokable]
        fn start(self: Pin<&mut UpstreamSession>) -> QString;
        /// Toggles this upstream run's public pause/resume control.
        /// # Returns
        /// Empty after request; error cannot control a different session.
        #[qinvokable]
        fn pause(self: &UpstreamSession) -> QString;
        /// Cancels preparation or requests exact-token cooperative upstream stop.
        /// # Returns
        /// Empty after request; execution admission remains until actual termination.
        #[qinvokable]
        fn stop(self: Pin<&mut UpstreamSession>) -> QString;
        /// Adopts bounded current-generation completions without waiting for workers.
        /// # Returns
        /// Scalar readiness, progress, recovery results and lossless application IDs.
        #[qinvokable]
        fn poll(self: Pin<&mut UpstreamSession>) -> QString;
        /// Reads accepted state without polling or copying scientific arrays.
        /// # Returns
        /// Same bounded scalar presentation shape as Poll.
        #[qinvokable]
        fn state(self: &UpstreamSession) -> QString;
    }
}

/// Fixed-direction Qt owner; tab selection never changes its scientific session.
pub struct UpstreamSessionRust {
    application: crate::shell::ApplicationSessions,
    model: Option<Box<Model>>,
}

impl Default for UpstreamSessionRust {
    /// Binds upstream controls to the canonical host without a panic fallback.
    /// # Returns
    /// Independent admitted state, or minimal references while fixed storage rejects.
    fn default() -> Self {
        Self::for_application(crate::downstream::workbench::session::Context::default().application)
    }
}

impl UpstreamSessionRust {
    /// Preflights the actual fixed GUI model against this exact application.
    /// # Arguments
    /// * `application` - Canonical shared lifecycle and retention host.
    /// # Returns
    /// Only host references on rejection; no full uncharged model is constructed.
    fn for_application(application: crate::shell::ApplicationSessions) -> Self {
        let model = Model::new(application.clone()).ok();
        Self { application, model }
    }

    /// Retries a rejected constructor without changing accepted readiness or data.
    /// # Returns
    /// Existing/admitted model or None while the same host remains at its limit.
    pub(super) fn ensure_model(&mut self) -> Option<&mut Model> {
        if self.model.is_none() {
            self.model = Model::new(self.application.clone()).ok();
        }
        self.model.as_deref_mut()
    }

    /// Reads bounded control state, including canonical global admission.
    /// # Returns
    /// Normal model state or explicit unavailability without fabricated results.
    fn current_state(&self) -> serde_json::Value {
        self.model.as_deref().map_or_else(
            || {
                let active = self.application.active();
                serde_json::json!({"configured":false,"preparing":false,"busy":false,
                    "application_busy":active.is_some(),
                    "active_direction":active.map(|run|match run.token.key.direction {
                        crate::shell::Direction::Downstream=>"downstream",
                        crate::shell::Direction::Upstream=>"upstream",
                    }),
                    "can_start":false,"session":null,"generation":null,"progress":null,
                    "has_result":false,"result":null,
                    "error":crate::shared::presentation::constants::RUNTIME_MODEL_ADMISSION_MESSAGE})
            },
            Model::state,
        )
    }
}

impl bridge::UpstreamSession {
    /// Queues one direction's real public project through its existing worker.
    /// # Arguments
    /// * `save_as` - Native destination selection, without changing the old path.
    /// # Returns
    /// Empty on queued work or chooser Cancel; actual errors otherwise. Successful
    /// write and exact captured-revision acknowledgement are observed through Poll.
    pub fn save_project(mut self: core::pin::Pin<&mut Self>, save_as: bool) -> QString {
        let result = (|| {
            let needs_path = {
                let mut owner = self.as_mut().rust_mut();
                let model = owner.ensure_model().ok_or(
                    crate::shared::presentation::constants::RUNTIME_MODEL_ADMISSION_MESSAGE,
                )?;
                if model.state()["project"]["can_save"] != true {
                    return Err(
                        "Apply a configuration and finish upstream work before saving.".into(),
                    );
                }
                save_as || model.project_path().is_none()
            };
            let destination = if needs_path {
                let Some(uri) =
                    crate::platform::native_file_picker::save("Save MERLIN Upstream Workbench")?
                else {
                    return Ok(());
                };
                Some(
                    cxx_qt_lib::QUrl::from(&QString::from(uri))
                        .to_local_file()
                        .map(|path| std::path::PathBuf::from(path.to_string()))
                        .ok_or("Choose a local project destination.")?,
                )
            } else {
                None
            };
            self.as_mut()
                .rust_mut()
                .ensure_model()
                .ok_or(crate::shared::presentation::constants::RUNTIME_MODEL_ADMISSION_MESSAGE)?
                .save_project(destination)
        })();
        QString::from(result.err().unwrap_or_default())
    }

    /// Captures the native exact target without exposing revisions to QML.
    /// # Returns
    /// Empty after capture; accepted science/path remains untouched until resolution.
    pub fn request_new_project(mut self: core::pin::Pin<&mut Self>) -> QString {
        let mut owner = self.as_mut().rust_mut();
        let result = owner
            .ensure_model()
            .ok_or(
                crate::shared::presentation::constants::RUNTIME_MODEL_ADMISSION_MESSAGE.to_owned(),
            )
            .and_then(Model::request_project_new);
        QString::from(result.err().unwrap_or_default())
    }

    /// Selects a real local project without copying its document through QML.
    /// # Returns
    /// Empty after Cancel/captured native intent; resolution prepares public
    /// scientific admission off the GUI, preserving old state on wrong direction.
    pub fn choose_open_project(mut self: core::pin::Pin<&mut Self>) -> QString {
        let result = (|| {
            let Some(uri) =
                crate::platform::native_file_picker::choose_workbench("Open MERLIN Workbench")?
            else {
                return Ok(());
            };
            let path = cxx_qt_lib::QUrl::from(&QString::from(uri))
                .to_local_file()
                .map(|path| std::path::PathBuf::from(path.to_string()))
                .ok_or("Choose a local project path.")?;
            self.as_mut()
                .rust_mut()
                .ensure_model()
                .ok_or(crate::shared::presentation::constants::RUNTIME_MODEL_ADMISSION_MESSAGE)?
                .request_project_open(path)
        })();
        QString::from(result.err().unwrap_or_default())
    }

    /// Forwards a prompt outcome to its original native target.
    /// # Arguments
    /// * `discard`, `cancel` - Explicit decision; no destination/document is supplied.
    /// # Returns
    /// Empty after resolution, or the unchanged native stale/dirty diagnostic.
    pub fn resolve_project(
        mut self: core::pin::Pin<&mut Self>,
        discard: bool,
        cancel: bool,
    ) -> QString {
        let mut owner = self.as_mut().rust_mut();
        let result = owner
            .ensure_model()
            .ok_or(
                crate::shared::presentation::constants::RUNTIME_MODEL_ADMISSION_MESSAGE.to_owned(),
            )
            .and_then(|model| model.resolve_project(discard, cancel));
        QString::from(result.err().unwrap_or_default())
    }

    /// Dispatches one finite public observation from a compiled instrument tag.
    /// # Arguments
    /// * `kind` - Existing destination, bounded before copying Qt text.
    /// # Returns
    /// Empty after dispatch; actionable native feedback on rejection.
    pub fn begin_instrument(mut self: core::pin::Pin<&mut Self>, kind: &QString) -> QString {
        let mut owner = self.as_mut().rust_mut();
        let Some(model) = owner.ensure_model() else {
            return QString::from(
                crate::shared::presentation::constants::RUNTIME_MODEL_ADMISSION_MESSAGE,
            );
        };
        if kind.len() > MAX_INSTRUMENT_KIND_CHARACTERS {
            const MESSAGE: &str = "Choose a supported upstream instrument.";
            model.reject_instrument_request(MESSAGE);
            return QString::from(MESSAGE);
        }
        QString::from(
            model
                .begin_instrument(&kind.to_string())
                .err()
                .unwrap_or_default(),
        )
    }

    /// Routes only validated application selector/context transport to its owner.
    /// # Arguments
    /// * `kind`, `selection`, `context` - Existing destination and exact original context.
    /// # Returns
    /// Empty after dispatch; native error state settles rejected pending requests.
    pub fn instrument(
        mut self: core::pin::Pin<&mut Self>,
        kind: &QString,
        selection: &QString,
        context: &QString,
    ) -> QString {
        let mut owner = self.as_mut().rust_mut();
        let Some(model) = owner.ensure_model() else {
            return QString::from(
                crate::shared::presentation::constants::RUNTIME_MODEL_ADMISSION_MESSAGE,
            );
        };
        if kind.len() > MAX_INSTRUMENT_KIND_CHARACTERS
            || selection.len() > MAX_VIEW_REQUEST_JSON_BYTES as isize
            || context.len() > MAX_VIEW_REQUEST_JSON_BYTES as isize
        {
            const MESSAGE: &str = "Instrument request exceeds its transport limit.";
            model.reject_instrument_request(MESSAGE);
            return QString::from(MESSAGE);
        }
        QString::from(
            model
                .request_instrument(
                    &kind.to_string(),
                    &selection.to_string(),
                    &context.to_string(),
                )
                .err()
                .unwrap_or_default(),
        )
    }

    /// Starts a passive configured-resource query without advancing the source.
    /// # Returns
    /// Empty after finite background dispatch; an actionable native error otherwise.
    pub fn begin_resource_grid(mut self: core::pin::Pin<&mut Self>) -> QString {
        let mut owner = self.as_mut().rust_mut();
        let Some(model) = owner.ensure_model() else {
            return QString::from(
                crate::shared::presentation::constants::RUNTIME_MODEL_ADMISSION_MESSAGE,
            );
        };
        QString::from(model.begin_resource_grid().err().unwrap_or_default())
    }

    /// Forwards only bounded viewport/context transport to the native direction owner.
    /// # Arguments
    /// * `request`, `context` - Original QML request and captured view identity.
    /// # Returns
    /// Empty after dispatch. Native error status settles rejected pending actions.
    pub fn resource_grid(
        mut self: core::pin::Pin<&mut Self>,
        request: &QString,
        context: &QString,
    ) -> QString {
        let mut owner = self.as_mut().rust_mut();
        let Some(model) = owner.ensure_model() else {
            return QString::from(
                crate::shared::presentation::constants::RUNTIME_MODEL_ADMISSION_MESSAGE,
            );
        };
        if request.len() > MAX_GRID_REQUEST_BYTES as isize
            || context.len() > MAX_GRID_REQUEST_BYTES as isize
        {
            const MESSAGE: &str = "Resource grid request exceeds its transport byte limit.";
            model.reject_grid_request(MESSAGE);
            return QString::from(MESSAGE);
        }
        QString::from(
            model
                .request_resource_grid(&request.to_string(), &context.to_string())
                .err()
                .unwrap_or_default(),
        )
    }

    /// Selects exact capture bytes without creating an associated experiment.
    /// # Arguments
    /// * `family` - Compiled destination tag, bounded before its UTF-8 copy.
    /// # Returns
    /// Empty after Cancel or dispatch; Poll reports eventual admission.
    pub fn choose_capture(mut self: core::pin::Pin<&mut Self>, family: &QString) -> QString {
        let result = (|| {
            if family.len() > MAX_CAPTURE_FAMILY_CHARACTERS {
                return Err("Choose a supported upstream capture family.".into());
            }
            let family = CaptureFamily::parse(&family.to_string())?;
            {
                let mut owner = self.as_mut().rust_mut();
                let model = owner.ensure_model().ok_or(
                    crate::shared::presentation::constants::RUNTIME_MODEL_ADMISSION_MESSAGE,
                )?;
                model.ensure_configuration_idle()?;
            }
            let uri = crate::platform::native_file_picker::choose(family.picker_title())?;
            let mut owner = self.as_mut().rust_mut();
            let model = owner
                .ensure_model()
                .ok_or(crate::shared::presentation::constants::RUNTIME_MODEL_ADMISSION_MESSAGE)?;
            model.load_capture_uri(family, uri.as_deref())?;
            Ok::<_, String>(())
        })();
        QString::from(result.err().unwrap_or_default())
    }

    /// Pages an existing admitted source through the ordinary bounded worker.
    /// # Arguments
    /// * `family` - Original compiled capture destination.
    /// * `first` - Original nonnegative record index, never a reconstructed grid.
    /// # Returns
    /// Empty after dispatch; rejection preserves the prior source and page.
    pub fn capture_page(
        mut self: core::pin::Pin<&mut Self>,
        family: &QString,
        first: i32,
    ) -> QString {
        let result = (|| {
            if family.len() > MAX_CAPTURE_FAMILY_CHARACTERS {
                return Err("Choose a supported upstream capture family.".into());
            }
            let family = CaptureFamily::parse(&family.to_string())?;
            let first =
                usize::try_from(first).map_err(|_| "Capture page origin must be nonnegative.")?;
            let mut owner = self.as_mut().rust_mut();
            let model = owner
                .ensure_model()
                .ok_or(crate::shared::presentation::constants::RUNTIME_MODEL_ADMISSION_MESSAGE)?;
            model.capture_page(family, first)
        })();
        QString::from(result.err().unwrap_or_default())
    }

    /// Queues a bounded original-source measurement viewport off the GUI thread.
    /// # Arguments
    /// * `family` - Existing explicit capture family tag.
    /// * `first`, `count` - Original nonnegative source range, without interpolation.
    /// # Returns
    /// Empty after dispatch; rejected requests leave accepted source/configuration.
    pub fn capture_plot(
        mut self: core::pin::Pin<&mut Self>,
        family: &QString,
        first: i32,
        count: i32,
    ) -> QString {
        let result = (|| {
            if family.len() > MAX_CAPTURE_FAMILY_CHARACTERS {
                return Err("Choose a supported upstream capture family.".into());
            }
            let family = CaptureFamily::parse(&family.to_string())?;
            let first =
                usize::try_from(first).map_err(|_| "Capture plot origin must be nonnegative.")?;
            let count =
                usize::try_from(count).map_err(|_| "Capture plot span must be nonnegative.")?;
            let mut owner = self.as_mut().rust_mut();
            let model = owner
                .ensure_model()
                .ok_or(crate::shared::presentation::constants::RUNTIME_MODEL_ADMISSION_MESSAGE)?;
            model.capture_plot(family, first, count)
        })();
        QString::from(result.err().unwrap_or_default())
    }

    /// Loads actual accepted values into an independent configuration draft.
    /// # Arguments
    /// * `section` - A bounded native section identifier; unknown values reject.
    /// # Returns
    /// Empty on success; accepted source, results and downstream remain unchanged.
    pub fn begin_draft(mut self: core::pin::Pin<&mut Self>, section: &QString) -> QString {
        if section.len() > MAX_DRAFT_ID_CHARACTERS {
            return QString::from("Unknown upstream configuration section.");
        }
        let mut owner = self.as_mut().rust_mut();
        let Some(model) = owner.ensure_model() else {
            return QString::from(
                crate::shared::presentation::constants::RUNTIME_MODEL_ADMISSION_MESSAGE,
            );
        };
        QString::from(
            model
                .begin_draft(&section.to_string())
                .err()
                .unwrap_or_default(),
        )
    }

    /// Queues only changed values against their original captured draft identity.
    /// # Arguments
    /// * `section`, `token` - Opaque native identifiers, checked before merge.
    /// * `changes` - JSON limited before UTF-8 copy and again by exact byte count.
    /// # Returns
    /// Empty means queued; Poll returns its eventual accepted/rejected outcome.
    pub fn apply_draft(
        mut self: core::pin::Pin<&mut Self>,
        section: &QString,
        token: &QString,
        changes: &QString,
    ) -> QString {
        if section.len() > MAX_DRAFT_ID_CHARACTERS || token.len() > MAX_DRAFT_ID_CHARACTERS {
            return QString::from("Upstream draft request exceeds its transport limit.");
        }
        let mut owner = self.as_mut().rust_mut();
        let Some(model) = owner.ensure_model() else {
            return QString::from(
                crate::shared::presentation::constants::RUNTIME_MODEL_ADMISSION_MESSAGE,
            );
        };
        if changes.len() > MAX_DRAFT_CHANGE_BYTES as isize {
            const MESSAGE: &str = "Upstream draft changes exceed the input byte limit.";
            model.reject_draft_request(&section.to_string(), &token.to_string(), MESSAGE);
            return QString::from(MESSAGE);
        }
        QString::from(
            model
                .apply_draft(
                    &section.to_string(),
                    &token.to_string(),
                    &changes.to_string(),
                )
                .err()
                .unwrap_or_default(),
        )
    }

    /// Cancels this configuration draft without dispatching any simulation control.
    /// # Arguments
    /// * `token` - Bounded original token, compared unchanged.
    /// # Returns
    /// Empty after targeted cancellation; foreign tokens leave all owners unchanged.
    pub fn cancel_draft(mut self: core::pin::Pin<&mut Self>, token: &QString) -> QString {
        if token.len() > MAX_DRAFT_ID_CHARACTERS {
            return QString::from("Unknown upstream configuration draft.");
        }
        let mut owner = self.as_mut().rust_mut();
        let Some(model) = owner.ensure_model() else {
            return QString::from(
                crate::shared::presentation::constants::RUNTIME_MODEL_ADMISSION_MESSAGE,
            );
        };
        QString::from(
            model
                .cancel_draft(&token.to_string())
                .err()
                .unwrap_or_default(),
        )
    }

    /// Acquires exact source bytes through the existing native file adapter.
    ///
    /// # Returns
    /// Empty for cancellation or queued validation; an actionable error otherwise.
    /// The modal chooser is separate from the finite off-GUI scientific preparation.
    /// Accepted configuration and results are unchanged until native adoption.
    pub fn choose_configuration(mut self: core::pin::Pin<&mut Self>) -> QString {
        let result = (|| {
            {
                let mut owner = self.as_mut().rust_mut();
                let model = owner.ensure_model().ok_or(
                    crate::shared::presentation::constants::RUNTIME_MODEL_ADMISSION_MESSAGE,
                )?;
                model.ensure_configuration_idle()?;
            }
            let uri = crate::platform::native_file_picker::choose("Load Upstream Configuration")?;
            let mut owner = self.as_mut().rust_mut();
            let model = owner
                .ensure_model()
                .ok_or(crate::shared::presentation::constants::RUNTIME_MODEL_ADMISSION_MESSAGE)?;
            model.load_configuration_uri(uri.as_deref())?;
            Ok::<_, String>(())
        })();
        QString::from(result.err().unwrap_or_default())
    }

    /// Stages existing public JSON for background scientific validation.
    /// # Arguments
    /// * `document` - Input limited before UTF-8 copy and again by exact byte count.
    /// # Returns
    /// Empty after staging; errors preserve accepted state.
    pub fn apply(mut self: core::pin::Pin<&mut Self>, document: &QString) -> QString {
        if document.len() > MAX_CONFIGURATION_BYTES as isize {
            return QString::from("Upstream configuration exceeds the input byte limit.");
        }
        let mut owner = self.as_mut().rust_mut();
        let Some(model) = owner.ensure_model() else {
            return QString::from(
                crate::shared::presentation::constants::RUNTIME_MODEL_ADMISSION_MESSAGE,
            );
        };
        QString::from(
            model
                .apply(document.to_string().as_bytes())
                .err()
                .unwrap_or_default(),
        )
    }

    /// Requests a bounded upstream run through the application model.
    /// # Returns
    /// Empty after request; failure leaves previous results intact.
    pub fn start(mut self: core::pin::Pin<&mut Self>) -> QString {
        let mut owner = self.as_mut().rust_mut();
        let Some(model) = owner.ensure_model() else {
            return QString::from(
                crate::shared::presentation::constants::RUNTIME_MODEL_ADMISSION_MESSAGE,
            );
        };
        QString::from(model.start().err().unwrap_or_default())
    }

    /// Toggles the existing public upstream pause/resume control.
    /// # Returns
    /// Empty after request, or the original ownership/control error.
    pub fn pause(&self) -> QString {
        let Some(model) = self.rust().model.as_deref() else {
            return QString::from(
                crate::shared::presentation::constants::RUNTIME_MODEL_ADMISSION_MESSAGE,
            );
        };
        QString::from(model.pause().err().unwrap_or_default())
    }

    /// Cancels owned preparation or active upstream work without joining.
    /// # Returns
    /// Empty after request; the actual terminal still owns active admission.
    pub fn stop(mut self: core::pin::Pin<&mut Self>) -> QString {
        let mut owner = self.as_mut().rust_mut();
        let Some(model) = owner.model.as_deref_mut() else {
            return QString::from(
                crate::shared::presentation::constants::RUNTIME_MODEL_ADMISSION_MESSAGE,
            );
        };
        QString::from(model.stop().err().unwrap_or_default())
    }

    /// Polls application events and projects bounded authoritative metadata.
    /// # Returns
    /// Scalar JSON, excluding captures, sample arrays and full scientific reports.
    pub fn poll(mut self: core::pin::Pin<&mut Self>) -> QString {
        let state = if let Some(model) = self.as_mut().rust_mut().ensure_model() {
            model.poll()
        } else {
            self.rust().current_state()
        };
        QString::from(state.to_string())
    }

    /// Reads accepted scalar state independently of the selected tab.
    /// # Returns
    /// No simulated result is fabricated before an actual report exists.
    pub fn state(&self) -> QString {
        QString::from(self.rust().current_state().to_string())
    }
}

#[cfg(test)]
#[path = "upstream_session/runtime_tests.rs"]
mod runtime_tests;
