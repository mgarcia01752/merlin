/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Owned read-only reports from actual channel jobs, with explicit metric scopes.
use super::*;
use crate::simulation::channel::{ChannelJobResult, ChannelJobStatus, FecCounters};
use serde::Serialize;
use std::sync::Arc;
/// Batch lifecycle, independent of FEC success.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum BatchStatus {
    Complete,
    CompletedWithErrors,
    Cancelled,
}
/// Per-experiment lifecycle; complete does not mean zero errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum RunStatus {
    Complete,
    Cancelled,
    RuntimeExceeded,
    Failed,
    NotStarted,
}
/// Scope explicitly shared by default FEC and profile summaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub enum MetricScope {
    ObservationWindow,
    AllCompleted,
    ObservationCells,
}
/// Decoder outcome counters derived from actual hard-decision changes and acceptance.
#[derive(Debug, Clone, Serialize)]
pub struct FecReport {
    scope: MetricScope,
    counters: FecCounters,
}
macro_rules! fec_getter {
    ($method:ident) => {
        /// Reads a completed decoder outcome count in this report's explicit scope.
        /// # Returns
        /// Engine-derived count; corrections are not inferred from iterations.
        pub fn $method(&self) -> CodewordCount {
            CodewordCount(self.counters.$method())
        }
    };
}
impl FecReport {
    /// Reads the scientific counter scope.
    /// # Returns
    /// Observation-window or separately labeled all-completed scope.
    pub fn scope(&self) -> MetricScope {
        self.scope
    }
    fec_getter!(total_codewords);
    fec_getter!(clean_codewords);
    fec_getter!(corrected_codewords);
    fec_getter!(uncorrectable_codewords);
    fec_getter!(undetected_errors);
}
/// Observation-window source comparison; rates are payload information, not goodput.
#[derive(Debug, Clone, Serialize)]
pub struct ProfilePerformance {
    scope: MetricScope,
    profile_id: ProfileId,
    compared_bits: BitCount,
    bit_errors: BitCount,
    post_fec_ber: Option<BitErrorRatio>,
    payload_rate: Option<PayloadRate>,
}
macro_rules! getter {
    ($name:ident,$ty:ty,$description:literal) => {
        #[doc=$description]
        /// # Returns
        /// Immutable value in its named units; unavailable science remains None.
        pub fn $name(&self) -> $ty {
            self.$name
        }
    };
}
impl ProfilePerformance {
    getter!(scope, MetricScope, "Reads the observation-window scope.");
    getter!(
        profile_id,
        ProfileId,
        "Reads original selected profile identity, not bootstrap wire ID."
    );
    getter!(
        compared_bits,
        BitCount,
        "Reads independently compared source bits."
    );
    getter!(
        bit_errors,
        BitCount,
        "Reads post-FEC source bit mismatches."
    );
    getter!(
        post_fec_ber,
        Option<BitErrorRatio>,
        "Reads actual post-FEC BER; absent with no denominator."
    );
    getter!(
        payload_rate,
        Option<PayloadRate>,
        "Reads finalized payload information bits per second."
    );
}
/// Owned same-bin MER diagnostic sums, never decoder inputs.
#[derive(Debug, Clone, Serialize)]
pub struct CarrierMer {
    subcarrier: SubcarrierIndex,
    scope: MetricScope,
    samples: SampleCount,
    signal_power: LinearPower,
    error_power: LinearPower,
    mer: Option<MerDb>,
}
impl CarrierMer {
    getter!(subcarrier, SubcarrierIndex, "Reads the physical FFT bin.");
    getter!(scope, MetricScope, "Reads observation-cell scope.");
    getter!(samples, SampleCount, "Reads valid matched-cell count.");
    getter!(
        signal_power,
        LinearPower,
        "Reads accumulated normalized reference power."
    );
    getter!(
        error_power,
        LinearPower,
        "Reads accumulated normalized error power."
    );
    getter!(
        mer,
        Option<MerDb>,
        "Reads finite MER; no samples or zero error remains optional."
    );
}
/// Executed experiment snapshot, retaining the complete existing job diagnostics.
#[derive(Serialize)]
pub struct PerformanceReport {
    status: RunStatus,
    sources: MeasurementSources,
    ocd: Ocd,
    dpd: Dpd,
    options: EvaluationOptions,
    fec: Option<FecReport>,
    profile: Option<ProfilePerformance>,
    all_completed_fec: FecReport,
    carrier_mer: Vec<CarrierMer>,
    assumptions: Vec<String>,
    construction_error: Option<ApiError>,
    #[serde(skip)]
    engine: Option<Arc<ChannelJobResult>>,
    diagnostics: Option<serde_json::Value>,
}
impl PerformanceReport {
    fn new(
        sources: MeasurementSources,
        ocd: Ocd,
        dpd: Dpd,
        options: EvaluationOptions,
        result: Result<ChannelJobResult, crate::SimulationError>,
    ) -> Self {
        let (engine, construction_error, status) = match result {
            Ok(job) => {
                let status = match job.status {
                    ChannelJobStatus::Complete => RunStatus::Complete,
                    ChannelJobStatus::Cancelled => RunStatus::Cancelled,
                    ChannelJobStatus::RuntimeExceeded => RunStatus::RuntimeExceeded,
                    ChannelJobStatus::Failed => RunStatus::Failed,
                };
                (Some(Arc::new(job)), None, status)
            }
            Err(error) => (None, Some(ApiError::from(error)), RunStatus::Failed),
        };
        let fec = engine
            .as_ref()
            .and_then(|e| e.summary.window.as_ref())
            .map(|w| FecReport {
                scope: MetricScope::ObservationWindow,
                counters: w.fec,
            });
        let profile = engine
            .as_ref()
            .and_then(|e| e.summary.window.as_ref())
            .map(|w| ProfilePerformance {
                scope: MetricScope::ObservationWindow,
                profile_id: dpd.profile_id(),
                compared_bits: BitCount(w.compared_bits),
                bit_errors: BitCount(w.bit_errors),
                post_fec_ber: w.ber.map(BitErrorRatio),
                payload_rate: w.payload_rate_bps.map(PayloadRate),
            });
        let all_completed_fec = FecReport {
            scope: MetricScope::AllCompleted,
            counters: engine
                .as_ref()
                .map_or_else(FecCounters::new, |e| e.summary.fec),
        };
        let carrier_mer = engine
            .as_ref()
            .map(|e| {
                e.carrier_mer
                    .iter()
                    .enumerate()
                    .map(|(bin, m)| CarrierMer {
                        subcarrier: SubcarrierIndex(bin as u16),
                        scope: MetricScope::ObservationCells,
                        samples: SampleCount(m.samples()),
                        signal_power: LinearPower(m.signal_power()),
                        error_power: LinearPower(m.error_power()),
                        mer: m.mer_db().map(MerDb),
                    })
                    .collect()
            })
            .unwrap_or_default();
        let diagnostics = engine
            .as_ref()
            .and_then(|job| serde_json::to_value(job).ok())
            .map(|mut value| {
                if let Some(object) = value.as_object_mut() {
                    object.remove("settings");
                    object.insert(
                        "decoder_scope".into(),
                        serde_json::json!(MetricScope::AllCompleted),
                    );
                    object.insert(
                        "carrier_mer_scope".into(),
                        serde_json::json!(MetricScope::ObservationCells),
                    );
                }
                value
            });
        let receiver_assumption = match options.receiver_mode {
            ReceiverMode::SuppliedChannel => "Supplied-channel correction uses the existing known-reference equalizer; explicit unity is synthetic",
            ReceiverMode::PilotEstimated => "Pilot-estimated correction uses received pilots; supplied H impairs the channel and is not receiver correction input",
            ReceiverMode::MerlinEstimated => "MCE uses received pilots and received-only reliability with the engine's default MCE policies; supplied H and noise power are not receiver inputs",
        };
        let assumptions = vec![
            "PRBS15 source; independent experiments reset source and noise seeds".into(),
            "RxMER conditions Gaussian residual power at the equalized reference plane; it does not uniquely reconstruct interference or predict a modem".into(),
            receiver_assumption.into(),
            "Channel selection controls applied H independently of receiver correction; explicit H may be reused across epochs".into(),
            "CP/RP/depth are application defaults where their origin is Defaulted, not measured modem settings".into(),
            "Captured profile ID is retained; fixed engine bootstrap uses wire Profile A".into(),
        ];
        Self {
            status,
            sources,
            ocd,
            dpd,
            options,
            fec,
            profile,
            all_completed_fec,
            carrier_mer,
            assumptions,
            construction_error,
            engine,
            diagnostics,
        }
    }
    getter!(
        status,
        RunStatus,
        "Reads execution status independently of decoder success."
    );
    /// Borrows original/derived input identity and selection assumptions.
    /// # Returns
    /// Immutable sources retained independently of the controller.
    pub fn measurement_sources(&self) -> &MeasurementSources {
        &self.sources
    }
    /// Borrows resolved configuration and value origins.
    /// # Returns
    /// Frozen settings used for this run.
    pub fn configuration(&self) -> &ResolvedConfiguration {
        self.ocd.configuration()
    }
    /// Borrows accepted options for this executed experiment.
    /// # Returns
    /// Immutable request snapshot.
    pub fn options(&self) -> &EvaluationOptions {
        &self.options
    }
    /// Reads host elapsed time separately from requested observation duration.
    /// # Returns
    /// Actual job milliseconds, zero for construction failure.
    pub fn elapsed(&self) -> ElapsedTime {
        ElapsedTime(self.engine.as_ref().map_or(0, |e| e.elapsed_ms))
    }
    /// Borrows default FEC statistics in observation-window scope.
    /// # Returns
    /// None before the observation window exists.
    pub fn fec(&self) -> Option<&FecReport> {
        self.fec.as_ref()
    }
    /// Borrows source comparison and payload rate in the same observation scope.
    /// # Returns
    /// None before window establishment; denominators retain optionality.
    pub fn profile(&self) -> Option<&ProfilePerformance> {
        self.profile.as_ref()
    }
    /// Borrows all completed decoder outcomes with a distinct scope.
    /// # Returns
    /// Actual whole-run counters, independent of default window summary.
    pub fn all_completed_fec(&self) -> &FecReport {
        &self.all_completed_fec
    }
    /// Borrows per-bin accumulated MER observations.
    /// # Returns
    /// Empty before simulation; gaps retain zero samples and absent MER.
    pub fn carrier_mer(&self) -> &[CarrierMer] {
        &self.carrier_mer
    }
    /// Borrows the existing detailed engine result without exposing mutable state.
    /// # Returns
    /// Full coverage, failure/iteration histograms, bounded words and omissions;
    /// None if engine construction failed (construction_error is serialized).
    pub fn diagnostics(&self) -> Option<&ChannelJobResult> {
        self.engine.as_deref()
    }
    /// Serializes an independent schema-versioned owned report.
    /// # Returns
    /// JSON preserving status, scopes and null metrics.
    /// # Errors
    /// Reports serialization errors without filesystem side effects.
    pub fn to_json(&self) -> ApiResult<String> {
        serde_json::to_string(&serde_json::json!({"schema_version":1,"report":self}))
            .map_err(|e| super::error::invalid(e.to_string()))
    }
}
/// Requested experiment, including not-started entries after cancellation.
#[derive(Serialize)]
pub struct ExperimentEntry {
    id: ExperimentId,
    status: RunStatus,
    sources: MeasurementSources,
    report: Option<PerformanceReport>,
}
impl ExperimentEntry {
    pub(crate) fn not_started(id: ExperimentId, sources: MeasurementSources) -> Self {
        Self {
            id,
            status: RunStatus::NotStarted,
            sources,
            report: None,
        }
    }
    pub(crate) fn executed(
        id: ExperimentId,
        sources: MeasurementSources,
        ocd: Ocd,
        dpd: Dpd,
        options: EvaluationOptions,
        result: Result<ChannelJobResult, crate::SimulationError>,
    ) -> Self {
        let report = PerformanceReport::new(sources.clone(), ocd, dpd, options, result);
        Self {
            id,
            status: report.status,
            sources,
            report: Some(report),
        }
    }
    getter!(id, ExperimentId, "Reads the opaque experiment identity.");
    getter!(
        status,
        RunStatus,
        "Reads lifecycle status, distinct from FEC outcome."
    );
    /// Borrows effective observation contributors, even if not started.
    /// # Returns
    /// Immutable original provenance and operation metadata.
    pub fn measurement_sources(&self) -> &MeasurementSources {
        &self.sources
    }
    /// Borrows actual executed results, independent of controller lifetime.
    /// # Returns
    /// None only when not started.
    pub fn report(&self) -> Option<&PerformanceReport> {
        self.report.as_ref()
    }
}
/// Owned batch snapshot with stable schema identity and one entry per request.
#[derive(Serialize)]
pub struct BatchReport {
    schema_version: u16,
    status: BatchStatus,
    entries: Vec<ExperimentEntry>,
}
impl BatchReport {
    pub(crate) fn new(entries: Vec<ExperimentEntry>, cancelled: bool) -> Self {
        let status = if cancelled {
            BatchStatus::Cancelled
        } else if entries.iter().any(|e| e.status != RunStatus::Complete) {
            BatchStatus::CompletedWithErrors
        } else {
            BatchStatus::Complete
        };
        Self {
            schema_version: 1,
            status,
            entries,
        }
    }
    getter!(
        status,
        BatchStatus,
        "Reads batch status; completion does not imply correct FEC."
    );
    /// Borrows experiment entries in original requested order.
    /// # Returns
    /// Iterator tied to this owned snapshot, not the controller.
    pub fn iter(&self) -> impl Iterator<Item = &ExperimentEntry> {
        self.entries.iter()
    }
    /// Serializes an independent owned batch snapshot.
    /// # Returns
    /// Versioned JSON with explicit scopes, original provenance and nullability.
    /// # Errors
    /// Reports serialization errors; no file writes occur.
    pub fn to_json(&self) -> ApiResult<String> {
        serde_json::to_string(self).map_err(|e| super::error::invalid(e.to_string()))
    }
}
