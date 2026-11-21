/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local ownership; explicit MERLIN schema identities.
 */
//! Exclusive Rust session; validation and execution remain in the shared core.
use super::{
    ApiError, ApiResult, BatchReport, ExperimentConfig, ExperimentOptions, ExperimentPreview,
    RunControl,
};
use crate::{
    simulation::{preview, run},
    PhyError,
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
        })
    }
    /// Atomically accepts typed synthetic `configuration` through shared admission.
    ///
    /// Returns unit; failed admission preserves the prior accepted configuration,
    /// generation and run settings. Loading never runs or consumes source.
    pub fn configure(&mut self, configuration: ExperimentConfig) -> ApiResult<()> {
        preview(&configuration, &self.options)?;
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
        let generation = next_id()?;
        self.configuration = Some(configuration);
        self.options = options;
        self.document = Some(document);
        self.measured_provenance = None;
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
        let mut report = run(config, &self.options, next_id()?, self.generation, control)?;
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
