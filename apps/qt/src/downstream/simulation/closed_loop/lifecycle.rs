/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Transactional receiver configuration and canonical application generations.
use super::{
    application_error, metadata, prepare_sweep, publish, request_retention, AdmissionError,
    Context, Inputs, Model, Options, Sections, SessionKey, TransmissionSettings,
};
use crate::{
    shared::{
        presentation::constants::WORKER_FAILURE_ADMISSION_MESSAGE, worker_failure::WorkerFailure,
    },
    shell::RetentionBudget,
};

type PreparedConfiguration<Error = AdmissionError> = Option<
    Result<
        (
            request_retention::SharedPlan,
            request_retention::SharedRequest,
        ),
        Error,
    >,
>;

impl Model {
    /// Enables persisted independent comparison after native dirty admission.
    /// # Returns
    /// Unit without regenerating data; an already enabled flag is unchanged.
    /// Closing/exhausted host preserves the prior flag and records its diagnostic.
    pub(crate) fn enable_verification(&mut self) {
        if self.verify {
            return;
        }
        match self
            .context
            .application
            .mark_project_dirty(crate::shell::Direction::Downstream)
        {
            Ok(()) => self.verify = true,
            Err(error) => self.set_message(application_error(error)),
        }
    }
    /// Cancels active work after the new unconfigured generation is admitted.
    /// # Returns
    /// True after admission; rejection preserves the accepted request and report.
    pub(crate) fn invalidate(&mut self) -> bool {
        let key = match self.context.application.configure_downstream(false) {
            Ok(key) => key,
            Err(error) => {
                self.set_message(application_error(error));
                return false;
            }
        };
        self.accept_generation(key);
        true
    }
    /// Revalidates current accepted inputs and options through their existing API.
    /// # Arguments
    /// * `settings` - Current OCD JSON; failure remains visible in model state.
    pub(crate) fn configure(&mut self, settings: &str) {
        if self.options.is_none() {
            // Initial view refresh has no accepted receiver generation to
            // invalidate. Actual editor transactions already admit dirty state.
            if self.generation.is_some() {
                self.invalidate();
            }
            return;
        }
        let _ = self.accept_options(self.options.clone(), self.verify, settings);
    }
    /// Adopts prevalidated editor choices only after host generation admission.
    /// # Arguments
    /// * `options` - Accepted receiver options or an explicitly unconfigured state.
    /// * `verify` - Independent comparison acceptance for that configuration.
    /// * `settings` - Current OCD JSON; incomplete inputs retain legacy readiness errors.
    /// # Returns
    /// Success after options, request, generation and invalidation commit together.
    /// # Errors
    /// Host admission failure preserves prior options, verification, request and report.
    pub(crate) fn accept_options(
        &mut self,
        options: Option<Options>,
        verify: bool,
        settings: &str,
    ) -> Result<(), String> {
        let inputs = crate::downstream::workbench::session::snapshot(&self.context);
        let result = self.configure_owned(settings, &inputs, inputs.sections(), options, verify);
        if let Err(message) = &result {
            self.set_message(message.clone());
        }
        result
    }
    /// Prepares requests from independent inputs without adopting into another context.
    /// # Arguments
    /// * `settings` - Accepted OCD JSON.
    /// * `inputs` - Frozen snapshot owned by the caller.
    /// * `sections` - Matching original-byte editor sections.
    /// # Returns
    /// Readiness in this model; no simulation starts or foreign context changes.
    #[cfg(test)]
    pub(crate) fn prepare_inputs(&mut self, settings: &str, inputs: &Inputs, sections: &Sections) {
        if let Err(message) = self.configure_owned(
            settings,
            inputs,
            sections,
            self.options.clone(),
            self.verify,
        ) {
            self.set_message(message);
        }
    }

    /// Prepares an independent receiver candidate against its accepting host budget.
    ///
    /// # Arguments
    /// * `settings` - Accepted OCD JSON, interpreted by the existing scientific builder.
    /// * `inputs`, `sections` - Matching frozen inputs and original-byte documents.
    /// * `budget` - Target counter covering old, queued and candidate scientific owners.
    ///
    /// # Returns
    /// Candidate readiness and existing incomplete-input diagnostics in this model.
    /// Its Context stays independent and its generation remains unset until host adoption.
    /// No target readiness, results, histories or active control are changed.
    ///
    /// # Errors
    /// Rejects tracked execution or target admission before changing this candidate.
    pub(crate) fn prepare_inputs_for_budget(
        &mut self,
        settings: &str,
        inputs: &Inputs,
        sections: &Sections,
        budget: &RetentionBudget,
    ) -> Result<(), String> {
        self.check_prepared_idle()?;
        let fixed_guard = self.fixed_target(budget).map_err(fixed_error)?;
        let replacement_guard = if options_owned(self, budget) {
            None
        } else {
            Some(metadata::reserve_options(self.options.as_ref(), budget)?)
        };
        let prepared = prepare_admitted(settings, inputs, sections, self.options.as_ref(), budget)?;
        let prepared = try_admit_error(prepared, budget)?;
        self.request = None;
        self.plan = None;
        self.generation = None;
        self.error = WorkerFailure::default();
        if let Some(guard) = replacement_guard {
            self.options_guard = guard;
        }
        self.accept_prepared(prepared);
        if let Some(guard) = fixed_guard {
            self.fixed_guard = guard;
        }
        Ok(())
    }

    /// Retains a schema-3 project's actual public plan without rebuilding its policy.
    /// # Arguments
    /// * `plan` - Independently admitted public request/profile/observation settings.
    /// * `budget` - Accepting host covering old and prepared scientific owners.
    /// # Returns
    /// Complete idle candidate roots with their existing actual-capacity charges.
    /// No run, target generation or accepted editor state changes here.
    /// # Errors
    /// Busy, missing staged options, a policy unsupported by native instruments
    /// or root/fixed metadata rejection preserves this candidate unchanged.
    pub(crate) fn prepare_public_plan_for_budget(
        &mut self,
        plan: merlin::downstream::FullLoopSweepPlan,
        budget: &RetentionBudget,
    ) -> Result<(), String> {
        self.check_prepared_idle()?;
        if self.options.is_none() || self.generation.is_some() {
            return Err("Stage the saved receiver options before its public project plan.".into());
        }
        self.check_metadata_target(budget)?;
        let fixed_guard = self.fixed_target(budget).map_err(fixed_error)?;
        let request = plan.request(0).map_err(|error| error.to_string())?;

        // Native instruments currently require their complete accepted default
        // observation set. API-only policies remain valid for public consumers;
        // reject unsupported UI imports explicitly rather than replacing them.
        if serde_json::to_value(request.observation_policy().map(|policy| policy.settings()))
            .map_err(|error| error.to_string())?
            != serde_json::to_value(Some(merlin::downstream::FullLoopObservationInput::default()))
                .map_err(|error| error.to_string())?
        {
            return Err("This downstream project's observation policy is not supported by the native instruments. Open it through the public Rust or Python Project API.".into());
        }
        let request = request_retention::admit_request(request, budget).map_err(owner_error)?;
        let plan = request_retention::admit_plan(plan, budget).map_err(owner_error)?;
        self.request = Some(request);
        self.plan = Some(plan);
        self.error = Default::default();
        if let Some(guard) = fixed_guard {
            self.fixed_guard = guard;
        }
        Ok(())
    }

    /// Admits inline options for a fresh detached project/fixture candidate.
    /// # Arguments
    /// * `options`, `verify` - Existing saved receiver/comparison choices.
    /// * `budget` - Actual accepting host, before scientific target preparation.
    /// # Returns
    /// Unit after moving options then their child-only guard. Every Some has a
    /// witness, even with zero children; fixed witness/Options bytes stay in Model.
    /// # Errors
    /// Rejects accepted/executing models or target capacity before any field changes.
    pub(crate) fn prepare_options_for_budget(
        &mut self,
        options: Option<Options>,
        verify: bool,
        budget: &RetentionBudget,
    ) -> Result<(), String> {
        self.check_prepared_idle()?;
        if self.generation.is_some() || self.request.is_some() || self.plan.is_some() {
            return Err("Only a fresh receiver candidate can stage project options.".into());
        }
        let fixed_guard = self.fixed_target(budget).map_err(fixed_error)?;
        let guard = metadata::reserve_options(options.as_ref(), budget)?;
        self.options = options;
        self.options_guard = guard;
        if let Some(guard) = fixed_guard {
            self.fixed_guard = guard;
        }
        self.verify = verify;
        Ok(())
    }

    /// Checks an independent candidate before any target project state is replaced.
    ///
    /// # Arguments
    /// * `budget` - Exact accepting counter; equal limits alone do not establish ownership.
    ///
    /// # Returns
    /// Whether both admitted scientific roots provide receiver readiness.
    /// Missing prerequisites remain a valid unconfigured project candidate.
    ///
    /// # Errors
    /// Rejects tracked execution, inconsistent root presence or any foreign reservation.
    pub(crate) fn check_prepared_target(&self, budget: &RetentionBudget) -> Result<bool, String> {
        self.check_prepared_idle()?;
        if !budget.owns(&self.fixed_guard) {
            return Err(
                "Prepared receiver fixed storage belongs to another application host.".into(),
            );
        }
        self.check_metadata_target(budget)?;
        if self.request.is_some() != self.plan.is_some() {
            return Err("Prepared receiver request and plan are inconsistent.".into());
        }
        if self
            .request
            .as_ref()
            .is_some_and(|request| !budget.owns(request.as_ref()))
            || self
                .plan
                .as_ref()
                .is_some_and(|plan| !budget.owns(plan.as_ref()))
        {
            return Err("Prepared receiver belongs to another application host.".into());
        }
        Ok(self.request.is_some())
    }

    /// Records the genuine project generation after guarded host adoption succeeds.
    ///
    /// # Arguments
    /// * `key` - Canonical key returned by the accepting host after target preflight.
    ///
    /// # Returns
    /// Unit after stamping the detached candidate; no allocation or host mutation.
    /// Call only after checking its idle state and target scientific owners.
    pub(crate) fn commit_prepared_generation(&mut self, key: SessionKey) {
        self.generation = Some(key);
    }

    /// Attaches a pre-admitted receiver after the host commits its genuine generation.
    ///
    /// # Arguments
    /// * `context` - Exact target, with the candidate's already committed canonical key.
    ///
    /// # Returns
    /// Unit after changing only this model's Context handle. No reservation, science,
    /// generation advancement, cancellation or result/history mutation occurs here.
    ///
    /// # Errors
    /// Preserves the candidate and both contexts on ownership, idle or stale-key rejection.
    pub(crate) fn attach_prepared(&mut self, context: Context) -> Result<(), String> {
        self.check_prepared_target(context.application.budget())?;
        if self.generation
            != Some(
                context
                    .application
                    .snapshot(crate::shell::Direction::Downstream)
                    .key,
            )
        {
            return Err("Prepared receiver generation does not match its target.".into());
        }
        self.context = context;
        Ok(())
    }
    /// Rebinds an independently validated project model to its actual GUI owner.
    /// # Arguments
    /// * `context` - Target downstream context after guarded project adoption.
    /// # Returns
    /// Unit after target coverage and generation admission, without scientific copies.
    /// Original aliases keep their source reservations until their final drop.
    /// # Errors
    /// Preserves source ownership on target coverage or host identity rejection.
    #[cfg(test)]
    pub(crate) fn rebind(&mut self, context: Context) -> Result<(), String> {
        let same_host = self
            .context
            .application
            .snapshot(crate::shell::Direction::Downstream)
            .key
            .session
            == context
                .application
                .snapshot(crate::shell::Direction::Downstream)
                .key
                .session;
        let fixed_guard = self
            .fixed_target(context.application.budget())
            .map_err(fixed_error)?;
        let replacement_guard = if options_owned(self, context.application.budget()) {
            None
        } else {
            Some(metadata::reserve_options(
                self.options.as_ref(),
                context.application.budget(),
            )?)
        };
        let replacement_error = if self.error.owns(context.application.budget()) {
            None
        } else {
            Some(
                self.error
                    .try_copy_for_budget(context.application.budget())?,
            )
        };
        let (request, plan) = if same_host {
            (self.request.clone(), self.plan.clone())
        } else {
            // Reserve both target owners while all source/worker aliases stay charged.
            let request = self
                .request
                .as_ref()
                .map(|request| {
                    request_retention::rebind_request(request, context.application.budget())
                })
                .transpose()
                .map_err(owner_error)?;
            let plan = self
                .plan
                .as_ref()
                .map(|plan| request_retention::rebind_plan(plan, context.application.budget()))
                .transpose()
                .map_err(owner_error)?;
            (request, plan)
        };
        let key = context
            .application
            .configure_downstream(request.is_some())
            .map_err(application_error)?;
        publish(&context, None);
        crate::downstream::instruments::receiver::performance::clear(&context);
        self.context = context;
        if let Some(guard) = fixed_guard {
            self.fixed_guard = guard;
        }
        if let Some(guard) = replacement_guard {
            self.options_guard = guard;
        }
        if let Some(error) = replacement_error {
            self.error = error;
        }
        self.request = request;
        self.plan = plan;
        self.generation = Some(key);
        Ok(())
    }
    fn configure_owned(
        &mut self,
        settings: &str,
        inputs: &Inputs,
        sections: &Sections,
        options: Option<Options>,
        verify: bool,
    ) -> Result<(), String> {
        let budget = self.context.application.budget();
        let fixed_guard = self.fixed_target(budget).map_err(fixed_error)?;
        let options_guard = metadata::reserve_options(options.as_ref(), budget)?;
        let prepared = prepare_admitted(settings, inputs, sections, options.as_ref(), budget)?;
        let prepared = admit_error(prepared, budget);
        let ready = matches!(&prepared, Some(Ok(_)));
        let key = self
            .context
            .application
            .configure_downstream(ready)
            .map_err(application_error)?;
        self.accept_generation(key);
        self.options = options;
        self.options_guard = options_guard;
        if let Some(guard) = fixed_guard {
            self.fixed_guard = guard;
        }
        self.verify = verify;
        self.accept_prepared(prepared);
        Ok(())
    }

    fn check_prepared_idle(&self) -> Result<(), String> {
        if self.worker.is_some() || self.timing.is_some() {
            return Err(
                "Only a receiver without tracked execution can transfer preparation.".into(),
            );
        }
        Ok(())
    }

    fn accept_prepared(&mut self, prepared: PreparedConfiguration<WorkerFailure>) {
        match prepared {
            Some(Ok((plan, request))) => {
                self.request = Some(request);
                self.plan = Some(plan);
            }
            Some(Err(error)) => {
                self.error = error;
            }
            None => {}
        }
    }
    fn accept_generation(&mut self, key: SessionKey) {
        self.generation = Some(key);
        if let Some(worker) = &mut self.worker {
            worker.cancel();
            worker.stale = true;
        }
        self.request = None;
        self.plan = None;
        self.timing = None;
        self.error = WorkerFailure::default();
        publish(&self.context, None);
        crate::downstream::instruments::receiver::performance::clear(&self.context);
        self.token = self.token.wrapping_add(1);
    }
}

fn fixed_error(error: crate::shell::RetentionError) -> String {
    application_error(crate::shell::ApplicationError::Retention(error))
}

fn options_owned(model: &Model, budget: &RetentionBudget) -> bool {
    match (&model.options, &model.options_guard) {
        (Some(_), Some(guard)) => budget.owns(guard),
        (None, None) => true,
        _ => false,
    }
}

fn admit_error(
    prepared: PreparedConfiguration,
    budget: &RetentionBudget,
) -> PreparedConfiguration<WorkerFailure> {
    prepared.map(|outcome| {
        outcome.map_err(|error| {
            WorkerFailure::admit_with_fallback(error, budget, WORKER_FAILURE_ADMISSION_MESSAGE)
        })
    })
}

fn try_admit_error(
    prepared: PreparedConfiguration,
    budget: &RetentionBudget,
) -> Result<PreparedConfiguration<WorkerFailure>, String> {
    match prepared {
        Some(Ok(roots)) => Ok(Some(Ok(roots))),
        Some(Err(error)) => Ok(Some(Err(WorkerFailure::try_admit(error, budget)?))),
        None => Ok(None),
    }
}

fn prepare_admitted(
    settings: &str,
    inputs: &Inputs,
    sections: &Sections,
    options: Option<&Options>,
    budget: &RetentionBudget,
) -> Result<PreparedConfiguration, String> {
    let prepared = options.map(|options| {
        serde_json::from_str::<TransmissionSettings>(settings)
            .map_err(|error| AdmissionError::from(error.to_string()))
            .and_then(|settings| {
                let plan = prepare_sweep(&settings, inputs, options, sections)?;
                let request = plan.request(0).map_err(AdmissionError::classified)?;
                Ok((plan, request))
            })
    });
    match prepared {
        Some(Ok((plan, request))) => {
            // Final candidates coexist with accepted and worker owners until commit.
            let request = request_retention::admit_request(request, budget).map_err(owner_error)?;
            let plan = request_retention::admit_plan(plan, budget).map_err(owner_error)?;
            Ok(Some(Ok((plan, request))))
        }
        Some(Err(error)) => Ok(Some(Err(error))),
        None => Ok(None),
    }
}

fn owner_error(error: request_retention::OwnerError) -> String {
    match error {
        request_retention::OwnerError::Metadata(error) => error.message().to_owned(),
        request_retention::OwnerError::Retention(error) => {
            application_error(crate::shell::ApplicationError::Retention(error))
        }
    }
}

#[cfg(test)]
#[path = "target_preflight_tests.rs"]
mod target_preflight_tests;
