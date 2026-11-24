/*
 * Copyright 2026 MORGANA contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local ownership; explicit MERLIN schema identities.
 */
//! Exclusive Rust session; validation and execution remain in the shared core.
mod report_retention;
use super::{
    ApiError, ApiResult, BatchReport, ExperimentConfig, ExperimentOptions, ExperimentPreview,
    RunControl,
};
use crate::{
    simulation::{preview, run_observed},
    PhyError,
};
use merlin_api::{
    retention_support::{sum, RetainedPayload},
    RetainedFootprint,
};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ID: AtomicU64 = AtomicU64::new(1);

fn next_id() -> ApiResult<u64> {
    NEXT_ID
        .fetch_update(Ordering::AcqRel, Ordering::Acquire, |id| id.checked_add(1))
        .map_err(|_| {
            ApiError::Domain(PhyError::ResourceLimit {
                field: "api.identity",
            })
        })
}

/// Idle mutable API session; scientific inputs/results are owned Rust values.
///
/// Rust's exclusive mutable borrow serializes configure/run operations. Native
/// adapters additionally expose Busy while executing without their session lock.
pub struct Simulation {
    options: ExperimentOptions,
    configuration: Option<ExperimentConfig>,
    generation: u64,
    document: Option<super::ConfigurationDocument>,
    measured_provenance: Option<String>,
    receiver_observations: super::ReceiverObservationPolicy,
}

impl Simulation {
    /// Validates finite `options` and constructs an idle unconfigured session.
    ///
    /// Returns an owned session; no source, channel, thread or file is started.
    /// Invalid budgets reject without allocating simulation buffers.
    pub fn new(options: ExperimentOptions) -> ApiResult<Self> {
        options.validate()?;
        Ok(Self {
            options,
            configuration: None,
            generation: 0,
            document: None,
            measured_provenance: None,
            receiver_observations: super::ReceiverObservationPolicy::default(),
        })
    }
    /// Atomically accepts typed synthetic `configuration` through shared admission.
    ///
    /// Returns unit; failed admission preserves the prior accepted configuration,
    /// generation and run settings. Loading never runs or consumes source.
    pub fn configure(&mut self, configuration: ExperimentConfig) -> ApiResult<()> {
        preview(&configuration, &self.options)?;
        self.receiver_observations
            .validate_mode(configuration.us_configuration.waveform().numerology())?;
        let generation = next_id()?;
        self.configuration = Some(configuration);
        self.document = None;
        self.measured_provenance = None;
        self.generation = generation;
        Ok(())
    }
    /// Parses and atomically accepts a bounded generic US Configuration document.
    ///
    /// `bytes` is borrowed UTF-8 JSON. Returns unit; failure preserves accepted
    /// input/options/generation, and loading never starts work or consumes source.
    pub fn configure_json(&mut self, bytes: &[u8]) -> ApiResult<()> {
        let document = super::ConfigurationDocument::from_json(bytes)?;
        self.configure_document(document)
    }
    /// Atomically accepts a complete source-valid selected document and budgets.
    ///
    /// Returns unit; all preserved inventory/control fields remain inspectable.
    /// Unsupported selected execution modes reject before changing accepted state.
    pub fn configure_document(&mut self, document: super::ConfigurationDocument) -> ApiResult<()> {
        let (configuration, options) = document.prepare()?;
        self.receiver_observations
            .validate_mode(configuration.us_configuration.waveform().numerology())?;
        let generation = next_id()?;
        self.configuration = Some(configuration);
        self.options = options;
        self.document = Some(document);
        self.measured_provenance = None;
        self.generation = generation;
        Ok(())
    }
    /// Atomically accepts a capture-adapter-prepared measured configuration.
    ///
    /// Loading is idle. Original generic context remains separate from captured
    /// channel projection, and reports retain provenance on cancellation.
    ///
    /// # Arguments
    /// * `prepared` - Owned, fully admitted measured input and execution settings.
    ///
    /// # Returns
    /// Unit after replacing configuration, settings and generation together.
    ///
    /// # Errors
    /// Returns a resource error if no new session identity can be allocated;
    /// failure preserves all previously accepted input and settings.
    pub fn configure_measured(&mut self, prepared: super::MeasuredConfiguration) -> ApiResult<()> {
        self.receiver_observations.validate_mode(
            prepared
                .configuration
                .us_configuration
                .waveform()
                .numerology(),
        )?;
        let generation = next_id()?;
        self.configuration = Some(prepared.configuration);
        self.options = prepared.options;
        self.document = Some(prepared.document);
        self.measured_provenance = Some(prepared.provenance_json);
        self.generation = generation;
        Ok(())
    }
    /// Borrows the retained original generic document, including unused presets.
    ///
    /// Returns None for a typed-only configuration; later option changes are
    /// separate accepted settings and never rewrite the source document.
    pub fn document(&self) -> Option<&super::ConfigurationDocument> {
        self.document.as_ref()
    }
    /// Atomically replaces runtime `options`, retaining accepted scientific input.
    ///
    /// Returns unit; invalid budgets or incompatible aggregate work preserve state.
    /// No accepted source/RNG state advances during this operation.
    pub fn set_options(&mut self, options: ExperimentOptions) -> ApiResult<()> {
        options.validate()?;
        if let Some(config) = &self.configuration {
            preview(config, &options)?;
        }
        self.options = options;
        Ok(())
    }
    /// Atomically replaces optional completed receiver-estimate retention budgets.
    /// # Arguments
    /// * `policy` - Validated aggregate frame/scalar policy, independent of runtime options.
    /// # Returns
    /// Unit without changing accepted generation, source, configuration or existing reports.
    /// # Errors
    /// Rejects insufficient scalar space for the accepted numerology, preserving the old policy.
    pub fn set_receiver_observation_policy(
        &mut self,
        policy: super::ReceiverObservationPolicy,
    ) -> ApiResult<()> {
        if let Some(configuration) = &self.configuration {
            policy.validate_mode(configuration.us_configuration.waveform().numerology())?;
        }
        self.receiver_observations = policy;
        Ok(())
    }

    /// Inspects optional receiver observation budgets without starting work.
    /// # Returns
    /// An owned immutable policy; default disables retention independently of I/Q taps.
    pub fn receiver_observation_policy(&self) -> super::ReceiverObservationPolicy {
        self.receiver_observations
    }

    /// Returns owned current readiness, resource counts and discovered inventory.
    ///
    /// Missing configuration returns MissingSetting; no processing is triggered.
    pub fn preview(&self) -> ApiResult<ExperimentPreview> {
        Ok(preview(
            self.configuration
                .as_ref()
                .ok_or(ApiError::MissingSetting)?,
            &self.options,
        )?)
    }
    /// Inspects the authoritative configured base-grant allocation without executing science.
    /// # Arguments
    /// * `request` - Original frame/symbol/carrier viewport and at-most-16384 cell budget.
    /// # Returns
    /// Owned resource roles, original visits/timing and unchanged accepted generation.
    /// No source/RNG advances and no receiver or transmitter is started.
    /// # Errors
    /// MissingSetting before configure; shared frame/extent/resource/overflow errors otherwise.
    pub fn resource_grid(
        &self,
        request: super::ResourceGridRequest,
    ) -> ApiResult<super::ResourceGridSnapshot> {
        let configuration = self
            .configuration
            .as_ref()
            .ok_or(ApiError::MissingSetting)?;
        let grid = crate::resources::view::view(&configuration.us_configuration, request)?;
        let transmission_planned = crate::grant::select(grid.grant_bits)?.transmit;
        Ok(super::ResourceGridSnapshot {
            format: super::resources::RESOURCE_GRID_FORMAT,
            schema_version: super::resources::RESOURCE_GRID_SCHEMA_VERSION,
            configuration_generation: self.generation,
            transmission_planned,
            grid,
        })
    }
    /// Inspects configured C, deterministic H or their composite without starting a simulation.
    /// # Arguments
    /// * `request` - Original base-grant symbol/frequency window, finite budget and impulse opt-in.
    /// # Returns
    /// Owned configured transfer and optional periodic inverse with the unchanged accepted generation.
    /// Source, receiver, channel stream and RNG state do not advance.
    /// # Errors
    /// MissingSetting before configure; shared extent, budget, finite-arithmetic and static-H
    /// capability failures otherwise. Nonzero frequency offset rejects H/composite queries.
    pub fn configured_response(
        &self,
        request: super::ConfiguredResponseRequest,
    ) -> ApiResult<super::ConfiguredResponseSnapshot> {
        let configuration = self
            .configuration
            .as_ref()
            .ok_or(ApiError::MissingSetting)?;
        let mut response = crate::instruments::response::configured(configuration, request)?;
        response.configuration_generation = Some(self.generation);
        Ok(response)
    }

    /// Runs the complete finite experiment using a fresh cancellation handle.
    ///
    /// Returns owned completed/partial report; shared engine errors retain causes.
    /// Each call starts the accepted source seed anew, independently of old reports.
    pub fn run(&mut self) -> ApiResult<BatchReport> {
        self.run_with_control(&RunControl::new())
    }
    /// Executes the same frozen engine with independently shared `control`.
    ///
    /// Returns owned science, preserving completed FEC/source outcomes on cancel.
    /// Missing settings, admission or an already-running control return typed errors.
    pub fn run_with_control(&mut self, control: &RunControl) -> ApiResult<BatchReport> {
        let config = self
            .configuration
            .as_ref()
            .ok_or(ApiError::MissingSetting)?;
        let mut report = run_observed(
            config,
            &self.options,
            next_id()?,
            self.generation,
            control,
            self.receiver_observations,
        )?;
        report.measurement_provenance_json = self.measured_provenance.clone();
        report.source_configuration_json = self
            .document
            .as_ref()
            .map(|document| document.to_json())
            .transpose()?;
        if let Some(document) = &self.document {
            report.source_configuration_original = document.original_bytes().map(<[u8]>::to_vec);
            report.source_configuration_sha256 = document.original_sha256();
        }
        Ok(report)
    }
    /// Borrows accepted typed input for inspection without executing work.
    ///
    /// Returns None before configure; caller cannot mutate session state here.
    pub fn configuration(&self) -> Option<&ExperimentConfig> {
        self.configuration.as_ref()
    }
}

impl RetainedPayload for Simulation {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            options,
            configuration,
            generation,
            document,
            measured_provenance,
            receiver_observations,
        } = self;
        sum([
            options.retained_children(),
            configuration.retained_children(),
            generation.retained_children(),
            document.retained_children(),
            measured_provenance.retained_children(),
            receiver_observations.retained_children(),
        ])
    }
}

impl Simulation {
    /// Measures the complete currently retained session without advancing science.
    ///
    /// # Returns
    /// Target-layout fixed root plus requested payload capacities, ordered-entry
    /// counts and conservatively repeated shared roots. Includes original source,
    /// options, configuration, provenance and private caches. Excludes allocator,
    /// tree-node and reference-count headers; this is not heap/RSS or a future
    /// output/workspace bound. No serialization, allocation or mutation occurs.
    ///
    /// # Errors
    /// ResourceLimit with field `retained_footprint` on checked arithmetic overflow.
    pub fn retained_footprint(&self) -> ApiResult<RetainedFootprint> {
        merlin_api::retention_support::retained_footprint(self)
            .ok_or_else(|| crate::error::limit("retained_footprint").into())
    }
}
