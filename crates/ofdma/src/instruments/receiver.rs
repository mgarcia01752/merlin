/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Opt-in bounded retention of actual completed received-pilot estimates.
use super::transfer::{amplitude_db, frequency_hz, phase};
use crate::{
    configuration::{Estimator, Numerology, UsConfiguration, SAMPLE_RATE_HZ},
    error::{invalid, limit},
    simulation::BatchReport,
    types::{Identity, Iq},
    PhyError, PhyResult,
};
use merlin_api::{
    retention_support::{sum, RetainedPayload},
    RetainedFootprint,
};
use serde::{ser::SerializeSeq, Deserialize, Serialize, Serializer};
use std::sync::Arc;

#[cfg(test)]
#[path = "receiver_tests.rs"]
mod tests;

/// Aggregate retained-frame application limit, independent of estimator frame geometry.
pub const MAX_RECEIVER_ESTIMATE_FRAMES: usize = 8;
/// Aggregate scalar-storage application limit, independent of FEC decision-page limits.
pub const MAX_RECEIVER_OBSERVATION_SCALARS: usize = 262_144;
/// Query-copy application limit, independent of configured-response display limits.
pub const MAX_RECEIVER_ESTIMATE_POINTS: usize = 4096;
/// One optional complex bin reserves both Cartesian scalar slots, even when unavailable.
const SCALARS_PER_COMPLEX_ESTIMATE: usize = 2;

/// Validated optional observation policy; defaults preserve unobserved engine behavior.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
pub struct ReceiverObservationPolicy {
    maximum_frames: usize,
    maximum_scalar_slots: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    soft_metrics: Option<super::SoftObservationPolicy>,
}

/// Owned policy projection, separate from scientific receiver configuration.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[non_exhaustive]
pub struct ReceiverObservationPolicyInfo {
    /// Maximum complete estimates across all grants; zero disables retention.
    pub maximum_frames: usize,
    /// Aggregate reserved Cartesian scalar slots, including unavailable bins.
    pub maximum_scalar_slots: usize,
    /// Independent optional soft budgets; absent when soft retention is disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub soft_metrics: Option<super::SoftObservationPolicyInfo>,
}

impl ReceiverObservationPolicy {
    /// Validates independent aggregate frame and scalar-storage budgets.
    /// # Arguments
    /// * `maximum_frames` - Zero for disabled, otherwise at most eight complete estimates.
    /// * `maximum_scalar_slots` - Zero when disabled; otherwise positive and at most 262144.
    /// # Returns
    /// Owned immutable policy; no simulation or allocation of observation buffers starts.
    /// # Errors
    /// Rejects inconsistent disabled budgets and application ceilings. A configured session
    /// additionally requires enough scalar slots for one complete estimate in its numerology.
    pub fn new(maximum_frames: usize, maximum_scalar_slots: usize) -> PhyResult<Self> {
        if maximum_frames > MAX_RECEIVER_ESTIMATE_FRAMES
            || maximum_scalar_slots > MAX_RECEIVER_OBSERVATION_SCALARS
            || (maximum_frames == 0) != (maximum_scalar_slots == 0)
        {
            return Err(invalid("receiver_observations.policy"));
        }
        Ok(Self {
            maximum_frames,
            maximum_scalar_slots,
            soft_metrics: None,
        })
    }

    /// Projects validated budgets without changing scientific or observer state.
    /// # Returns
    /// Owned scalar metadata, including both zero values when disabled.
    pub fn info(&self) -> ReceiverObservationPolicyInfo {
        ReceiverObservationPolicyInfo {
            maximum_frames: self.maximum_frames,
            maximum_scalar_slots: self.maximum_scalar_slots,
            soft_metrics: self.soft_metrics.map(|policy| policy.info()),
        }
    }

    /// Composes independent optional soft budgets without changing estimate retention.
    /// # Arguments
    /// * `policy` - Validated immutable computed-cell/completed-word policy.
    /// # Returns
    /// A new owned policy, enabling soft-only observation when estimate retention is disabled.
    /// An all-zero soft policy removes soft retention; the original value remains unchanged.
    pub fn with_soft_metrics(mut self, policy: super::SoftObservationPolicy) -> Self {
        let info = policy.info();
        self.soft_metrics = if info.maximum_cells == 0 && info.maximum_words == 0 {
            // Preserve the existing omitted-disabled JSON contract.
            None
        } else {
            Some(policy)
        };
        self
    }

    /// Checks that an enabled policy can retain one complete estimate in `mode`.
    /// # Returns
    /// Unit without allocation or state changes.
    /// # Errors
    /// Insufficient Cartesian scalar slots return the shared resource-limit category.
    pub(crate) fn validate_mode(&self, mode: Numerology) -> PhyResult<()> {
        if self.maximum_frames > 0 && self.maximum_scalar_slots < scalar_cost(mode) {
            return Err(limit("receiver_observations.scalar_slots"));
        }
        Ok(())
    }
}

/// Original retained-frame identity; positions do not change after filtering or queries.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ReceiverEstimateIdentity {
    /// Position in the globally bounded retained history, independent of grant index.
    pub retained_index: usize,
    /// Actual run, accepted configuration generation and original grant.
    pub identity: Identity,
    /// Original absolute physical frame identity.
    pub frame: u64,
}

/// Bounded availability inventory, including completed frames omitted by retention policy.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ReceiverObservationSummary {
    /// Frozen policy used by this run; disabled when observation was not requested.
    pub policy: ReceiverObservationPolicyInfo,
    /// Completed receiver frames observed while retention was enabled.
    pub completed_frames: usize,
    /// Complete estimate vectors retained; never an interrupted partial frame.
    pub retained_frames: usize,
    /// Completed frames omitted after frame/scalar budgets were exhausted.
    pub dropped_frames: usize,
    /// Actual reserved Cartesian scalar slots, counting unavailable bins.
    pub used_scalar_slots: usize,
    /// Original identities of all retained frames, bounded by the policy ceiling.
    pub frames: Vec<ReceiverEstimateIdentity>,
}

/// Retained-frame selection and original physical-frequency viewport.
#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ReceiverEstimateRequest {
    /// Index discovered through receiver_observations, not a scheduled frame ordinal.
    pub retained_index: usize,
    /// First original physical centered-FFT bin.
    pub first_carrier: usize,
    /// Positive original span, or zero for the remaining full FFT.
    pub carrier_count: usize,
    /// Maximum copied points, at most MAX_RECEIVER_ESTIMATE_POINTS.
    pub maximum_points: usize,
}
impl Default for ReceiverEstimateRequest {
    /// Selects the first retained frame over its complete physical FFT.
    /// # Returns
    /// A read-only finite query; no missing history is reconstructed.
    fn default() -> Self {
        Self {
            retained_index: 0,
            first_carrier: 0,
            carrier_count: 0,
            maximum_points: MAX_RECEIVER_ESTIMATE_POINTS,
        }
    }
}

/// One actual received-pilot estimate at an original physical carrier.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ReceiverEstimatePoint {
    /// Original physical FFT index without compaction of unavailable bins.
    pub physical_carrier: usize,
    /// Centered complex-baseband frequency in hertz; no inferred RF origin.
    pub frequency_hz: f64,
    /// Actual optional estimator output; zero and unavailable remain distinct.
    pub value: Option<Iq>,
    /// Twenty-log magnitude of the actual estimate, absent for missing or exact-zero bins.
    pub amplitude_db: Option<f64>,
    /// Principal phase in radians of the actual estimate, absent for missing or exact-zero bins.
    pub phase_radians: Option<f64>,
}

/// Owned actual effective C×H estimate, separate from configured deterministic transfers.
#[derive(Debug, Clone, PartialEq, Serialize)]
#[non_exhaustive]
pub struct ReceiverEstimateSnapshot {
    /// Original retained/run/grant/generation/frame association.
    pub frame: ReceiverEstimateIdentity,
    /// Explicit actual received-pilot effective transfer reference plane.
    pub reference_plane: &'static str,
    /// Dimensionless complex sample-amplitude transfer, without RF calibration.
    pub units: &'static str,
    /// Actual admitted normal receiver estimator, without an observer fallback.
    pub estimator: Estimator,
    /// Original complete physical FFT size, independent of this viewport.
    pub fft_samples: usize,
    /// Actual upstream complex-baseband sample clock in samples per second.
    pub sample_rate_hz: u32,
    /// Original carrier spacing in hertz.
    pub spacing_hz: u32,
    /// First original physical carrier of this viewport.
    pub first_carrier: usize,
    /// Resolved original carrier count, including unavailable bins.
    pub carrier_count: usize,
    /// Exact optional outputs; no configured channel or corrective values fill holes.
    pub points: Vec<ReceiverEstimatePoint>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
struct EstimateFrame {
    identity: Identity,
    frame: u64,
    estimator: Estimator,
    mode: Numerology,
    estimates: Vec<Option<Iq>>,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub(crate) struct ReceiverEstimateStore {
    policy: ReceiverObservationPolicy,
    completed_frames: usize,
    used_scalar_slots: usize,
    // Serialize immutable shared contents without enabling serde's global Arc feature.
    #[serde(serialize_with = "serialize_records")]
    frames: Vec<Arc<EstimateFrame>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) soft: Option<super::soft::store::SoftStore>,
}

impl ReceiverEstimateStore {
    /// Constructs empty metadata for an enabled validated aggregate `policy`.
    /// # Returns
    /// None for disabled retention; no estimate buffers are allocated.
    pub(crate) fn new(policy: ReceiverObservationPolicy) -> Option<Self> {
        if policy.maximum_frames == 0 && policy.soft_metrics.is_none() {
            return None;
        }
        Some(Self {
            policy,
            completed_frames: 0,
            used_scalar_slots: 0,
            frames: Vec::new(),
            soft: policy.soft_metrics.map(super::soft::store::SoftStore::new),
        })
    }

    /// Accepts an already calculated complete estimate at its original frame boundary.
    /// # Arguments
    /// * `config`, `identity`, `frame` - Actual receiving configuration and physical identities.
    /// * `estimator` - Admitted normal receiver policy, without observer-side substitution.
    /// * `estimates` - Full original optional FFT vector, moved after receiver processing.
    /// # Returns
    /// Unit; exhausted budgets count an omitted frame without retaining its coefficients.
    pub(crate) fn record(
        &mut self,
        config: &UsConfiguration,
        identity: Identity,
        frame: u64,
        estimator: &Estimator,
        estimates: Vec<Option<Iq>>,
    ) {
        if self.policy.maximum_frames == 0 {
            return;
        }
        self.completed_frames += 1;
        let mode = config.waveform().numerology();
        let cost = scalar_cost(mode);
        if self.frames.len() >= self.policy.maximum_frames
            || cost > self.policy.maximum_scalar_slots - self.used_scalar_slots
        {
            return;
        }
        self.used_scalar_slots += cost;
        self.frames.push(Arc::new(EstimateFrame {
            identity,
            frame,
            estimator: estimator.clone(),
            mode,
            estimates,
        }));
    }

    fn summary(&self) -> ReceiverObservationSummary {
        ReceiverObservationSummary {
            policy: self.policy.info(),
            completed_frames: self.completed_frames,
            retained_frames: self.frames.len(),
            dropped_frames: self.completed_frames - self.frames.len(),
            used_scalar_slots: self.used_scalar_slots,
            frames: self.frames.iter().enumerate().map(frame_identity).collect(),
        }
    }
}

impl BatchReport {
    /// Inspects bounded availability of actual completed received-pilot estimates.
    /// # Returns
    /// Owned policy, coverage and original identities; disabled reports have zero observation
    /// counts. Querying never runs estimation, starts work or changes retained results.
    pub fn receiver_observations(&self) -> ReceiverObservationSummary {
        match &self.observations.receiver_estimates {
            Some(store) => store.summary(),
            None => ReceiverObservationSummary {
                policy: ReceiverObservationPolicy::default().info(),
                completed_frames: 0,
                retained_frames: 0,
                dropped_frames: 0,
                used_scalar_slots: 0,
                frames: Vec::new(),
            },
        }
    }

    /// Copies actual retained received-pilot coefficients in their original physical viewport.
    /// # Arguments
    /// * `request` - Retained index, original carrier extent and positive copy-point budget.
    /// # Returns
    /// Owned optional effective C×H estimates, actual identities, estimator and units.
    /// No scientific replay or configured-response substitution; results outlive this report.
    /// # Errors
    /// Invalid extents/budgets reject before copying. Disabled/unretained history returns
    /// Unsupported, including frames omitted by limits, cancellation or incomplete processing.
    pub fn receiver_estimate(
        &self,
        request: ReceiverEstimateRequest,
    ) -> PhyResult<ReceiverEstimateSnapshot> {
        if !(1..=MAX_RECEIVER_ESTIMATE_POINTS).contains(&request.maximum_points) {
            return Err(limit("receiver_estimate.maximum_points"));
        }
        let frame = self
            .observations
            .receiver_estimates
            .as_ref()
            .and_then(|store| store.frames.get(request.retained_index))
            .ok_or(PhyError::Unsupported {
                field: "receiver_estimate.unavailable",
            })?;
        let length = frame.mode.fft_len();
        if request.first_carrier >= length {
            return Err(invalid("receiver_estimate.first_carrier"));
        }
        let count = match request.carrier_count {
            0 => length - request.first_carrier,
            count => count,
        };
        let end = request
            .first_carrier
            .checked_add(count)
            .filter(|end| *end <= length)
            .ok_or_else(|| invalid("receiver_estimate.carrier_count"))?;
        if count > request.maximum_points {
            return Err(limit("receiver_estimate.maximum_points"));
        }
        let spacing = frame.mode.spacing_hz();
        let points = (request.first_carrier..end)
            .map(|physical_carrier| {
                let value = frame.estimates[physical_carrier];
                Ok(ReceiverEstimatePoint {
                    physical_carrier,
                    frequency_hz: frequency_hz(physical_carrier, length, spacing),
                    value,
                    amplitude_db: value
                        .map(|value| amplitude_db(value, "receiver_estimate.magnitude"))
                        .transpose()?
                        .flatten(),
                    phase_radians: value.and_then(phase),
                })
            })
            .collect::<PhyResult<Vec<_>>>()?;
        Ok(ReceiverEstimateSnapshot {
            frame: frame_identity((request.retained_index, frame)),
            reference_plane: "received_pilot_effective_c_times_h",
            units: "dimensionless_complex_sample_amplitude_transfer",
            estimator: frame.estimator.clone(),
            fft_samples: length,
            sample_rate_hz: SAMPLE_RATE_HZ,
            spacing_hz: spacing,
            first_carrier: request.first_carrier,
            carrier_count: count,
            points,
        })
    }
}

fn scalar_cost(mode: Numerology) -> usize {
    mode.fft_len() * SCALARS_PER_COMPLEX_ESTIMATE
}

fn frame_identity(
    (retained_index, frame): (usize, &Arc<EstimateFrame>),
) -> ReceiverEstimateIdentity {
    ReceiverEstimateIdentity {
        retained_index,
        identity: frame.identity,
        frame: frame.frame,
    }
}

/// Serializes actual immutable shared observation records without exposing Arc ownership.
/// # Arguments
/// * `Value`, `SerializerType` - Concrete observation record and serialization destination types.
/// * `frames` - Bounded actual retained values; concrete estimate/cell/word consumers share this adapter.
/// * `serializer` - Existing Rust serialization destination; no scientific processing occurs.
/// # Returns
/// Serialized sequence with unchanged record order and values.
/// # Errors
/// Propagates destination/record serialization failures without mutating observations.
pub(super) fn serialize_records<Value: Serialize, SerializerType: Serializer>(
    frames: &[Arc<Value>],
    serializer: SerializerType,
) -> Result<SerializerType::Ok, SerializerType::Error> {
    let mut sequence = serializer.serialize_seq(Some(frames.len()))?;
    for frame in frames {
        sequence.serialize_element(frame.as_ref())?;
    }
    sequence.end()
}

impl RetainedPayload for ReceiverObservationPolicy {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            maximum_frames,
            maximum_scalar_slots,
            soft_metrics,
        } = self;
        sum([
            maximum_frames.retained_children(),
            maximum_scalar_slots.retained_children(),
            soft_metrics.retained_children(),
        ])
    }
}

impl RetainedPayload for EstimateFrame {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            identity,
            frame,
            estimator,
            mode,
            estimates,
        } = self;
        sum([
            identity.retained_children(),
            frame.retained_children(),
            estimator.retained_children(),
            mode.retained_children(),
            estimates.retained_children(),
        ])
    }
}

impl RetainedPayload for ReceiverEstimateStore {
    /// Visits all owned children, including spare capacity and private caches.
    ///
    /// # Returns
    /// Checked additional payload/entry/shared-root quantities; no cloning or science.
    fn retained_children(&self) -> Option<RetainedFootprint> {
        let Self {
            policy,
            completed_frames,
            used_scalar_slots,
            frames,
            soft,
        } = self;
        sum([
            policy.retained_children(),
            completed_frames.retained_children(),
            used_scalar_slots.retained_children(),
            frames.retained_children(),
            soft.retained_children(),
        ])
    }
}

#[cfg(test)]
mod retained_footprint_tests {
    use super::*;
    use merlin_api::retention_support::retained_footprint;
    use std::mem::size_of;

    #[test]
    fn repeated_estimate_arc_edges_include_pointee_and_spare_capacity_each_time() {
        const ESTIMATE_CAPACITY: usize = 7;
        const FRAME_VECTOR_CAPACITY: usize = 5;
        let mut estimates = Vec::with_capacity(ESTIMATE_CAPACITY);
        estimates.push(Some(Iq { re: 1.0, im: 0.0 }));
        let estimate_capacity = estimates.capacity();
        let frame = Arc::new(EstimateFrame {
            identity: Identity {
                run: 1,
                generation: 2,
                grant: 3,
            },
            frame: 0,
            estimator: Estimator::FrameFlatLs,
            mode: Numerology::Khz50,
            estimates,
        });
        let mut frames = Vec::with_capacity(FRAME_VECTOR_CAPACITY);
        frames.push(Arc::clone(&frame));
        frames.push(frame);
        let frame_capacity = frames.capacity();
        let store = ReceiverEstimateStore {
            policy: ReceiverObservationPolicy::default(),
            completed_frames: frames.len(),
            used_scalar_slots: 0,
            frames,
            soft: None,
        };
        let footprint = retained_footprint(&store).unwrap();
        let repeated_frame =
            size_of::<EstimateFrame>() + estimate_capacity * size_of::<Option<Iq>>();
        assert_eq!(
            footprint.payload_bytes,
            size_of::<ReceiverEstimateStore>()
                + frame_capacity * size_of::<Arc<EstimateFrame>>()
                + repeated_frame
                + repeated_frame
        );
        assert_eq!(footprint.shared_roots, 2);
        assert_eq!(footprint.ordered_entries, 0);
    }
}
