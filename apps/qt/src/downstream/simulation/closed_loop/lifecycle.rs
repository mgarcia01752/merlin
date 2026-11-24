/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Transactional receiver configuration and canonical application generations.
use super::{
    application_error, prepare_sweep, publish, AdmissionError, Context, Inputs, Model, Options,
    Sections, SessionKey, TransmissionSettings,
};

impl Model {
    /// Cancels active work after the new unconfigured generation is admitted.
    /// # Returns
    /// True after admission; rejection preserves the accepted request and report.
    pub(crate) fn invalidate(&mut self) -> bool {
        let key = match self.context.application.configure_downstream(false) {
            Ok(key) => key,
            Err(error) => {
                self.error = application_error(error);
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
            self.invalidate();
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
        let sections = crate::downstream::workbench::session::saved_sections(&self.context);
        let result = self.configure_owned(settings, &inputs, &sections, options, verify);
        if let Err(message) = &result {
            self.error = message.clone();
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
    pub(crate) fn prepare_inputs(&mut self, settings: &str, inputs: &Inputs, sections: &Sections) {
        if let Err(message) = self.configure_owned(
            settings,
            inputs,
            sections,
            self.options.clone(),
            self.verify,
        ) {
            self.error = message;
        }
    }
    /// Rebinds an independently validated project model to its actual GUI owner.
    /// # Arguments
    /// * `context` - Target downstream context after guarded project adoption.
    /// # Errors
    /// Preserves host admission/identity failure and prevents a stale Start.
    pub(crate) fn rebind(&mut self, context: Context) -> Result<(), String> {
        let key = context
            .application
            .configure_downstream(self.request.is_some())
            .map_err(application_error)?;
        publish(&context, None);
        crate::downstream::instruments::receiver::performance::clear(&context);
        self.context = context;
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
        let prepared = options.as_ref().map(|options| {
            serde_json::from_str::<TransmissionSettings>(settings)
                .map_err(|error| AdmissionError::from(error.to_string()))
                .and_then(|settings| {
                    let plan = prepare_sweep(&settings, inputs, options, sections)?;
                    let request = plan.request(0).map_err(AdmissionError::classified)?;
                    Ok((plan, request))
                })
        });
        let ready = matches!(&prepared, Some(Ok(_)));
        let key = self
            .context
            .application
            .configure_downstream(ready)
            .map_err(application_error)?;
        self.accept_generation(key);
        self.options = options;
        self.verify = verify;
        match prepared {
            Some(Ok((plan, request))) => {
                self.request = Some(request);
                self.plan = Some(plan);
            }
            Some(Err(error)) => {
                self.error = error.message;
                self.diagnostic = error.diagnostic;
            }
            None => {}
        }
        Ok(())
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
        self.error.clear();
        self.diagnostic = None;
        publish(&self.context, None);
        crate::downstream::instruments::receiver::performance::clear(&self.context);
        self.token = self.token.wrapping_add(1);
    }
}
