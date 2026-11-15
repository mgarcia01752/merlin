/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent collections, complete static batch preparation and bounded execution.
use super::*;
mod profiles;
use crate::{
    signal_processing::channel::impairments::{ChannelImpairments, NoiseModel},
    simulation::{
        channel::{
            measurement_playback::{MeasurementPlayback, PlaybackMode},
            run_channel, ChannelJobLimits, ChannelRunSettings,
        },
        test_points::{NoopObserver, TestPointProbe},
    },
};
use serde::Serialize;
use std::sync::{
    atomic::{AtomicBool, AtomicU64, Ordering},
    Arc,
};
static NEXT_SCOPE: AtomicU64 = AtomicU64::new(1);
macro_rules! identifier {
    ($name:ident) => {
        #[doc = "Opaque controller-scoped identity; callers cannot construct numeric IDs."]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
        pub struct $name {
            scope: u64,
            slot: usize,
        }
    };
}
identifier!(RxMerId);
identifier!(ChannelEstimateId);
identifier!(ExperimentId);
/// Static RxMER preparation policy; independent experiments are not playback.
#[derive(Debug, Clone, Copy, Serialize)]
pub enum RxMerSelection {
    Each,
    Individual(RxMerId),
    AverageAll,
    InterpolateAt(CaptureTime),
}
/// Explicit response policy; no omitted-policy unity fallback.
#[derive(Debug, Clone, Copy, Serialize)]
pub enum ChannelSelection {
    Unity,
    Explicit(ChannelEstimateId),
    NearestWithin(AssociationTolerance),
}
/// Finite execution/detail limits copied atomically by set_options.
#[derive(Debug, Clone, Serialize)]
pub struct RunLimits {
    pub runtime: RuntimeLimit,
    pub retained_words: RetainedWordLimit,
    pub iterations: IterationLimit,
}
impl Default for RunLimits {
    /// Selects finite application runtime, iteration and detail defaults.
    /// # Returns
    /// A 30-second host budget, 50 iterations and 64 retained words.
    fn default() -> Self {
        Self {
            runtime: RuntimeLimit(30_000),
            retained_words: RetainedWordLimit(64),
            iterations: IterationLimit(50),
        }
    }
}
/// Existing receiver and physical-channel policies, copied into each engine job.
/// These are independent of measured H and RxMER selection; no calculations occur here.
#[derive(Debug, Clone, Default, Serialize, serde::Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ProcessingOptions {
    pub mce: crate::signal_processing::receiver::channel_estimation::MceSettings,
    pub channel_filter: crate::signal_processing::channel::filter::ChannelFilterSettings,
    pub additional_loss: crate::signal_processing::channel::additional_loss::AdditionalLoss,
    pub power: crate::signal_processing::channel::power::PowerConditioning,
}
/// Mutable request; the controller takes an independent validated copy.
#[derive(Debug, Clone, Serialize)]
pub struct EvaluationOptions {
    pub rxmer_selection: RxMerSelection,
    pub channel_selection: Option<ChannelSelection>,
    /// Receiver correction source, independent of the applied channel response.
    pub receiver_mode: ReceiverMode,
    /// Owned existing numerical policies, including saved-workbench MCE tuning.
    pub processing: ProcessingOptions,
    pub observation: ObservationDuration,
    pub noise_seed: NoiseSeed,
    pub prbs_seed: PrbsSeed,
    pub profile_tolerance: AssociationTolerance,
    pub limits: RunLimits,
}
impl Default for EvaluationOptions {
    /// Selects documented application defaults without implying a channel response.
    /// # Returns
    /// Each/100ms/seed41/PRBS0x7fff with zero profile tolerance and unset H policy.
    fn default() -> Self {
        Self {
            rxmer_selection: RxMerSelection::Each,
            channel_selection: None,
            receiver_mode: ReceiverMode::CmChannelEstimated,
            processing: ProcessingOptions::default(),
            observation: ObservationDuration(100),
            noise_seed: NoiseSeed(41),
            prbs_seed: PrbsSeed(0x7fff),
            profile_tolerance: AssociationTolerance::ZERO,
            limits: RunLimits::default(),
        }
    }
}
impl EvaluationOptions {
    /// Selects a static RxMER policy on this owned request.
    /// # Arguments
    /// * `selection` - Each, original, average or time-selected observation.
    /// # Returns
    /// Updated request; controller remains unchanged until set_options.
    pub fn with_rxmer_selection(mut self, selection: RxMerSelection) -> Self {
        self.rxmer_selection = selection;
        self
    }
    /// Chooses explicit unity, selected H or nearest-time H.
    /// # Arguments
    /// * `selection` - Required response policy.
    /// # Returns
    /// Updated owned request.
    pub fn with_channel_selection(mut self, selection: ChannelSelection) -> Self {
        self.channel_selection = Some(selection);
        self
    }
    /// Selects existing receiver correction without changing applied channel H.
    /// # Arguments
    /// * `mode` - Supplied H, received-pilot estimation, or existing MCE defaults.
    /// # Returns
    /// Updated owned request; selection does not prepare or run simulation.
    pub fn with_receiver_mode(mut self, mode: ReceiverMode) -> Self {
        self.receiver_mode = mode;
        self
    }
    /// Sets independent profile-time association policy.
    /// # Arguments
    /// * `tolerance` - Maximum source/profile epoch distance in seconds.
    /// # Returns
    /// Updated request without changing any captured epoch.
    pub fn with_profile_tolerance(mut self, tolerance: AssociationTolerance) -> Self {
        self.profile_tolerance = tolerance;
        self
    }
}
/// Cloneable Send+Sync cooperative cancellation, independent of controller locking.
#[derive(Debug, Clone, Default)]
pub struct RunControl {
    pub(crate) cancel: Arc<AtomicBool>,
}
impl RunControl {
    /// Creates a fresh uncancelled control.
    /// # Returns
    /// Shareable control; one-way cancellation affects future dispatch too.
    pub fn new() -> Self {
        Self::default()
    }
    /// Requests cooperative stop at the next bounded engine boundary.
    /// # Returns
    /// Unit; already completed scientific results remain owned.
    pub fn cancel(&self) {
        self.cancel.store(true, Ordering::Relaxed);
    }
}
/// Selected contributors and original provenance, including explicit response reuse.
#[derive(Debug, Clone, Serialize)]
pub struct MeasurementSources {
    pub(crate) rxmer: Vec<RxMerId>,
    pub(crate) epochs: Vec<CaptureTime>,
    pub(crate) weights: Vec<Weight>,
    pub(crate) operation: RxMerSelection,
    pub(crate) target_epoch: Option<CaptureTime>,
    pub(crate) channel: Option<ChannelEstimateId>,
    pub(crate) channel_policy: ChannelSelection,
    pub(crate) rxmer_sources: Vec<CaptureProvenance>,
    pub(crate) channel_source: Option<CaptureProvenance>,
    pub(crate) missing_policy: String,
}
impl MeasurementSources {
    /// Borrows contributor identities in derivation order.
    /// # Returns
    /// Original controller-scoped RxMER IDs.
    pub fn rxmer_ids(&self) -> &[RxMerId] {
        &self.rxmer
    }
    /// Borrows original contributor epochs, never an average's fabricated epoch.
    /// # Returns
    /// Whole-second capture timestamps in contributor order.
    pub fn epochs(&self) -> &[CaptureTime] {
        &self.epochs
    }
    /// Borrows normalized residual-power weights.
    /// # Returns
    /// Equal capture weights or the actual temporal interpolation weights.
    pub fn weights(&self) -> &[Weight] {
        &self.weights
    }
    /// Reads the explicit response selection, including synthetic unity.
    /// # Returns
    /// Original response policy independent of RxMER derivation.
    pub fn channel_policy(&self) -> ChannelSelection {
        self.channel_policy
    }
    /// Reads the original selected response identity.
    /// # Returns
    /// None only for explicit unity.
    pub fn channel_id(&self) -> Option<ChannelEstimateId> {
        self.channel
    }
}
/// Immutable prepared experiment for inspection/parity; preparation runs no simulation.
#[derive(Clone)]
pub struct PreparedExperiment {
    pub(crate) settings: ChannelRunSettings,
    pub(crate) sources: MeasurementSources,
}
impl PreparedExperiment {
    /// Borrows frozen low-level engine settings for parity and inspection.
    /// # Returns
    /// Existing engine-domain snapshot; no execution takes place.
    pub fn engine_settings(&self) -> &ChannelRunSettings {
        &self.settings
    }
    /// Borrows static derivation and original provenance.
    /// # Returns
    /// Selected IDs, weights and epochs without mutating captures.
    pub fn sources(&self) -> &MeasurementSources {
        &self.sources
    }
}
/// Single-channel controller; mutable run borrowing prevents concurrent configuration.
pub struct Merlin {
    ocd: Ocd,
    profile: Option<Dpd>,
    loaded_profiles: Vec<Dpd>,
    rxmer: Vec<RxMer>,
    channels: Vec<ChannelEstimate>,
    options: EvaluationOptions,
    scope: u64,
    retained: usize,
    runs: usize,
}
impl Merlin {
    /// Creates a controller with validated OCD and no inputs or implicit channel policy.
    /// # Arguments
    /// * `ocd` - Owned validated geometry.
    /// # Returns
    /// Empty controller; no engine construction or simulation.
    pub fn new(ocd: Ocd) -> Self {
        Self {
            ocd,
            profile: None,
            loaded_profiles: vec![],
            rxmer: vec![],
            channels: vec![],
            options: EvaluationOptions::default(),
            scope: NEXT_SCOPE.fetch_add(1, Ordering::Relaxed),
            retained: 0,
            runs: 0,
        }
    }
    /// Borrows the accepted OCD.
    /// # Returns
    /// Immutable controller-scoped configuration.
    pub fn ocd(&self) -> &Ocd {
        &self.ocd
    }
    /// Borrows the selected DPD, if assigned.
    /// # Returns
    /// None until explicit profile assignment.
    pub fn profile(&self) -> Option<&Dpd> {
        self.profile.as_ref()
    }
    /// Borrows accepted options.
    /// # Returns
    /// Read-only request snapshot.
    pub fn options(&self) -> &EvaluationOptions {
        &self.options
    }
    /// Assigns an immutable profile after local geometry validation.
    /// # Arguments
    /// * `dpd` - Captured or explicitly labeled hypothesis profile.
    /// # Returns
    /// Unit; existing reports remain independent.
    /// # Errors
    /// Rejects mismatching device/grid or protected geometry without mutation.
    pub fn set_profile(&mut self, dpd: Dpd) -> ApiResult<()> {
        if dpd.profile.key != self.ocd.key {
            return Err(ApiError::new(
                ErrorCode::IncompatibleGrid,
                "DPD and OCD identity/grid differ",
            ));
        }
        if crate::simulation::channel::measurement::captured_layout(
            &dpd.profile,
            &self.ocd.transmission,
        )? != self.ocd.layout
        {
            return Err(ApiError::new(
                ErrorCode::IncompatibleRoles,
                "DPD protected layout differs from OCD",
            ));
        }
        if self
            .retained
            .checked_add(
                self.loaded_profiles
                    .iter()
                    .map(Self::profile_storage)
                    .sum::<usize>(),
            )
            .and_then(|bytes| bytes.checked_add(Self::profile_storage(&dpd)))
            .is_none_or(|bytes| bytes > 64 * 1024 * 1024)
        {
            return Err(ApiError::new(
                ErrorCode::ResourceLimit,
                "Profile exceeds the aggregate capture storage budget",
            ));
        }
        self.profile = Some(dpd);
        Ok(())
    }
    fn profile_storage(profile: &Dpd) -> usize {
        profile
            .sources
            .iter()
            .map(|source| source.original.len() + source.decoded.len())
            .sum::<usize>()
            + std::mem::size_of_val(profile.carriers())
    }
    fn budget(&self, bytes: usize, count: usize) -> ApiResult<usize> {
        if self.rxmer.len() + self.channels.len() >= 256 || count > 65536 {
            return Err(ApiError::new(
                ErrorCode::ResourceLimit,
                "Capture collection limit exceeded",
            ));
        }
        self.retained
            .checked_add(bytes)
            .filter(|n| {
                n.checked_add(
                    self.loaded_profiles
                        .iter()
                        .map(Self::profile_storage)
                        .sum::<usize>(),
                )
                .and_then(|bytes| {
                    bytes.checked_add(self.profile.as_ref().map_or(0, Self::profile_storage))
                })
                .is_some_and(|bytes| bytes <= 64 * 1024 * 1024)
            })
            .ok_or_else(|| {
                ApiError::new(
                    ErrorCode::ResourceLimit,
                    "Aggregate capture storage exceeds 64 MiB",
                )
            })
    }
    /// Adds one original RxMER independently of channel estimates and timestamps.
    /// # Arguments
    /// * `frame` - Immutable validated capture.
    /// # Returns
    /// Stable controller-scoped identifier.
    /// # Errors
    /// Rejects structural/storage limits atomically; no time pairing occurs.
    pub fn add_rxmer(&mut self, frame: RxMer) -> ApiResult<RxMerId> {
        frame.frame.validate()?;
        let retained = self.budget(
            frame.provenance.original.len()
                + frame.provenance.decoded.len()
                + frame.frame.mer_db.len() * 16,
            frame.frame.mer_db.len(),
        )?;
        let id = RxMerId {
            scope: self.scope,
            slot: self.rxmer.len(),
        };
        self.rxmer.push(frame);
        self.retained = retained;
        Ok(id)
    }
    /// Adds one response independently, retaining its original epoch and H.
    /// # Arguments
    /// * `frame` - Immutable validated channel capture.
    /// # Returns
    /// Stable response ID; no RxMER pairing or duplication.
    /// # Errors
    /// Rejects structural/storage limits atomically.
    pub fn add_channel_estimate(&mut self, frame: ChannelEstimate) -> ApiResult<ChannelEstimateId> {
        let retained = self.budget(
            frame.provenance.original.len()
                + frame.provenance.decoded.len()
                + frame.frame.iq.len() * 24,
            frame.frame.iq.len(),
        )?;
        let id = ChannelEstimateId {
            scope: self.scope,
            slot: self.channels.len(),
        };
        self.channels.push(frame);
        self.retained = retained;
        Ok(id)
    }
    /// Borrows an original RxMER by controller-scoped identity.
    /// # Arguments
    /// * `id` - Identifier returned by this controller.
    /// # Returns
    /// Immutable original observation.
    /// # Errors
    /// Rejects foreign or out-of-range IDs.
    pub fn rxmer(&self, id: RxMerId) -> ApiResult<&RxMer> {
        if id.scope != self.scope {
            return Err(ApiError::new(
                ErrorCode::InvalidIdentifier,
                "Foreign RxMER ID",
            ));
        }
        self.rxmer
            .get(id.slot)
            .ok_or_else(|| ApiError::new(ErrorCode::InvalidIdentifier, "Unknown RxMER ID"))
    }
    /// Borrows original H by controller-scoped identity.
    /// # Arguments
    /// * `id` - Identifier returned by this controller.
    /// # Returns
    /// Immutable original response.
    /// # Errors
    /// Rejects foreign or out-of-range IDs.
    pub fn channel_estimate(&self, id: ChannelEstimateId) -> ApiResult<&ChannelEstimate> {
        if id.scope != self.scope {
            return Err(ApiError::new(
                ErrorCode::InvalidIdentifier,
                "Foreign channel ID",
            ));
        }
        self.channels
            .get(id.slot)
            .ok_or_else(|| ApiError::new(ErrorCode::InvalidIdentifier, "Unknown channel ID"))
    }
    /// Copies options atomically; complete cross-input checks occur at prepare/run.
    /// # Arguments
    /// * `options` - Owned validated request.
    /// # Returns
    /// Unit without simulation or input mutation.
    /// # Errors
    /// Rejects foreign selection IDs; missing inputs remain a preflight concern.
    pub fn set_options(&mut self, options: EvaluationOptions) -> ApiResult<()> {
        if let RxMerSelection::Individual(id) = options.rxmer_selection {
            self.rxmer(id)?;
        }
        if let Some(ChannelSelection::Explicit(id)) = options.channel_selection {
            self.channel_estimate(id)?;
        }
        self.options = options;
        Ok(())
    }
    /// Sets Common Path Attenuator (CPA) attenuation without starting simulation.
    /// # Arguments
    /// * `value_db` - Finite value in the inclusive 0..60 dB model range.
    /// # Returns
    /// Unit after atomic acceptance; other options are preserved.
    /// # Errors
    /// Returns InvalidInput for invalid power settings; retains previous state.
    pub fn set_cpa_attenuation_db(&mut self, value_db: f64) -> ApiResult<()> {
        let mut power = self.options.processing.power.clone();
        power.cpa_attenuation_db = value_db;
        power.validate().map_err(ApiError::from)?;
        self.options.processing.power = power;
        Ok(())
    }
    /// Reads accepted Common Path Attenuator (CPA) attenuation in dB.
    /// # Returns
    /// The effective scalar value; no simulation is started.
    pub fn get_cpa_attenuation_db(&self) -> f64 {
        self.options.processing.power.cpa_attenuation_db
    }
    /// Sets launch amplifier gain without starting simulation.
    /// # Arguments
    /// * `value_db` - Finite value in the inclusive 0..60 dB model range.
    /// # Returns
    /// Unit after atomic acceptance; other options are preserved.
    /// Explicit launch gain replaces any configured amplifier target.
    /// # Errors
    /// Returns InvalidInput for invalid power settings; retains previous state.
    pub fn set_launch_amplifier_gain_db(&mut self, value_db: f64) -> ApiResult<()> {
        let mut power = self.options.processing.power.clone();
        power.amplifier_gain_db = value_db;
        power.amplifier_target_dbmv_per_6mhz = None;
        power.validate().map_err(ApiError::from)?;
        self.options.processing.power = power;
        Ok(())
    }
    /// Reads accepted launch amplifier gain in dB.
    /// # Returns
    /// The effective scalar value; no simulation is started.
    pub fn get_launch_amplifier_gain_db(&self) -> f64 {
        self.options.processing.power.gain_db()
    }
    /// Freezes and validates every effective experiment without constructing a runner.
    /// # Returns
    /// Independent immutable prepared settings suitable for engine parity inspection.
    /// # Errors
    /// Rejects missing policy/profile/input, grid/time/coverage errors for the whole batch.
    pub fn prepare(&self) -> ApiResult<Vec<PreparedExperiment>> {
        let profile = self.profile.as_ref().ok_or_else(|| {
            ApiError::new(ErrorCode::MissingSetting, "Assign a DPD before running")
        })?;
        let policy = self.options.channel_selection.ok_or_else(|| {
            ApiError::new(
                ErrorCode::MissingSetting,
                "Choose a channel response policy explicitly",
            )
        })?;
        if self.rxmer.is_empty() {
            return Err(super::error::invalid(
                "At least one RxMER capture is required",
            ));
        }
        let all: Vec<_> = (0..self.rxmer.len()).collect();
        let (groups, mode, target) = match self.options.rxmer_selection {
            RxMerSelection::Each => (
                all.iter().map(|i| vec![*i]).collect::<Vec<_>>(),
                PlaybackMode::Single,
                None,
            ),
            RxMerSelection::Individual(id) => {
                self.rxmer(id)?;
                (vec![vec![id.slot]], PlaybackMode::Single, None)
            }
            RxMerSelection::AverageAll => {
                if matches!(policy, ChannelSelection::NearestWithin(_)) {
                    return Err(super::error::invalid(
                        "AverageAll has no unique epoch; choose explicit H or unity",
                    ));
                }
                (vec![all], PlaybackMode::Average, None)
            }
            RxMerSelection::InterpolateAt(epoch) => {
                let mut ordered = all;
                ordered.sort_by_key(|i| self.rxmer[*i].frame.capture_time);
                if ordered
                    .iter()
                    .any(|i| self.rxmer[*i].frame.key != profile.profile.key)
                {
                    return Err(ApiError::new(
                        ErrorCode::IncompatibleGrid,
                        "Interpolation requires one exact identity/grid",
                    ));
                }
                if ordered.windows(2).any(|p| {
                    self.rxmer[p[0]].frame.capture_time == self.rxmer[p[1]].frame.capture_time
                }) {
                    return Err(ApiError::new(
                        ErrorCode::AmbiguousTimestamp,
                        "Duplicate RxMER epochs",
                    ));
                }
                if let Some(index) = ordered
                    .iter()
                    .find(|i| self.rxmer[**i].frame.capture_time == epoch.0)
                {
                    (vec![vec![*index]], PlaybackMode::Single, Some(epoch))
                } else {
                    let pair = ordered
                        .windows(2)
                        .find(|p| {
                            self.rxmer[p[0]].frame.capture_time < epoch.0
                                && epoch.0 < self.rxmer[p[1]].frame.capture_time
                        })
                        .ok_or_else(|| {
                            ApiError::new(
                                ErrorCode::AssociationFailed,
                                "Interpolation cannot extrapolate",
                            )
                        })?;
                    (vec![pair.to_vec()], PlaybackMode::Interpolate, Some(epoch))
                }
            }
        };
        let mut prepared = Vec::new();
        for group in groups {
            let frames: Vec<_> = group.iter().map(|i| self.rxmer[*i].frame.clone()).collect();
            for frame in &frames {
                if frame.key != profile.profile.key {
                    return Err(ApiError::new(
                        ErrorCode::IncompatibleGrid,
                        "RxMER identity/grid differs from selected DPD",
                    ));
                }
                if !profile.sources.is_empty()
                    && frame.capture_time.abs_diff(profile.profile.capture_time)
                        > self.options.profile_tolerance.0
                {
                    return Err(ApiError::new(
                        ErrorCode::AssociationFailed,
                        "DPD epoch exceeds explicit profile tolerance",
                    ));
                }
            }
            let interval = if mode == PlaybackMode::Interpolate {
                u64::from(frames[1].capture_time - frames[0].capture_time)
            } else {
                1
            };
            let position = if mode == PlaybackMode::Interpolate {
                u64::from(target.expect("interpolation target").0 - frames[0].capture_time)
            } else {
                0
            };
            let playback = MeasurementPlayback::new(
                &self.ocd.transmission,
                &profile.profile,
                frames.clone(),
                mode,
                interval,
                u32::MAX,
            )?;
            let snapshot = playback.at(position);
            let mut effective = frames[0].clone();
            if mode != PlaybackMode::Single {
                effective.mer_db = snapshot
                    .power
                    .iter()
                    .map(|power| power.map(|value| -10.0 * value.log10()))
                    .collect();
            }
            let epoch = target.or_else(|| {
                (mode != PlaybackMode::Average).then_some(CaptureTime(frames[0].capture_time))
            });
            let channel = match policy {
                ChannelSelection::Unity => None,
                ChannelSelection::Explicit(id) => {
                    self.channel_estimate(id)?;
                    Some(id)
                }
                ChannelSelection::NearestWithin(tolerance) => {
                    let epoch = epoch.ok_or_else(|| super::error::invalid("No unique epoch"))?;
                    let mut candidates: Vec<_> = self
                        .channels
                        .iter()
                        .enumerate()
                        .filter(|(_, h)| {
                            h.frame.key == effective.key
                                && h.frame.capture_time.abs_diff(epoch.0) <= tolerance.0
                        })
                        .map(|(i, h)| (h.frame.capture_time.abs_diff(epoch.0), i))
                        .collect();
                    candidates.sort();
                    if candidates.is_empty() {
                        return Err(ApiError::new(
                            ErrorCode::AssociationFailed,
                            "No channel within tolerance",
                        ));
                    }
                    if candidates
                        .get(1)
                        .is_some_and(|next| next.0 == candidates[0].0)
                    {
                        return Err(ApiError::new(
                            ErrorCode::AmbiguousTimestamp,
                            "Equally near channel estimates",
                        ));
                    }
                    Some(ChannelEstimateId {
                        scope: self.scope,
                        slot: candidates[0].1,
                    })
                }
            };
            let response = channel.map(|id| self.channels[id.slot].frame.clone());
            if response.as_ref().is_some_and(|h| h.key != effective.key) {
                return Err(ApiError::new(
                    ErrorCode::IncompatibleGrid,
                    "Selected channel identity/grid differs",
                ));
            }
            let mut settings =
                ChannelRunSettings::new(self.ocd.transmission.clone(), self.ocd.layout.clone());
            settings.duration = std::time::Duration::from_millis(self.options.observation.0);
            settings.prbs_seed = self.options.prbs_seed.0;
            settings.receiver_mode = self.options.receiver_mode;
            settings.mce = self.options.processing.mce.clone();
            settings.channel_filter = self.options.processing.channel_filter.clone();
            settings.max_iterations = self.options.limits.iterations.0;
            // Shared playback validation checks exact protected roles and coverage.
            let layout = crate::transmission::layout::CarrierLayout::new(
                &settings.transmission,
                &settings.layout,
            )?;
            let mut loading = vec![0; layout.static_roles().len()];
            for (offset, assignment) in profile.profile.assignments.iter().enumerate() {
                if assignment.carrier_type == crate::types::modulation::CarrierType::Data {
                    loading[usize::from(effective.key.grid.first_subcarrier_index) + offset] =
                        assignment
                            .bit_loading
                            .ok_or_else(|| super::error::invalid("Unresolved data loading"))?;
                }
            }
            settings.data_bit_loading = Some(loading);
            settings.impairments = ChannelImpairments {
                seed: self.options.noise_seed.0,
                noise: NoiseModel::Rxmer { frame: effective },
                response,
                additional_loss: self.options.processing.additional_loss.clone(),
                power: self.options.processing.power.clone(),
            };
            settings.validate()?;
            let weights = if mode == PlaybackMode::Average {
                vec![Weight(1.0 / group.len() as f64); group.len()]
            } else if mode == PlaybackMode::Interpolate {
                vec![
                    Weight(1.0 - snapshot.second_weight),
                    Weight(snapshot.second_weight),
                ]
            } else {
                vec![Weight(1.0)]
            };
            let sources = MeasurementSources {
                rxmer: group
                    .iter()
                    .map(|i| RxMerId {
                        scope: self.scope,
                        slot: *i,
                    })
                    .collect(),
                epochs: frames.iter().map(|f| CaptureTime(f.capture_time)).collect(),
                weights,
                operation: self.options.rxmer_selection,
                target_epoch: target,
                channel,
                channel_policy: policy,
                rxmer_sources: group
                    .iter()
                    .map(|i| self.rxmer[*i].provenance.clone())
                    .collect(),
                channel_source: channel.map(|id| self.channels[id.slot].provenance.clone()),
                missing_policy: "missing if any contributing reading is missing; no resampling"
                    .into(),
            };
            prepared.push(PreparedExperiment { settings, sources });
        }
        Ok(prepared)
    }
    /// Runs a fully preflighted batch with fresh engine state for each entry.
    /// # Returns
    /// Owned complete/partial reports; no earlier results or inputs are mutated.
    /// # Errors
    /// Rejects the complete batch before execution on configuration errors.
    pub fn run(&mut self) -> ApiResult<BatchReport> {
        self.run_with_control(&RunControl::new())
    }
    /// Runs synchronously with cooperative cross-thread cancellation.
    /// # Arguments
    /// * `control` - Shared stop token checked by the existing engine and dispatch loop.
    /// # Returns
    /// Owned batch including not-started entries after cancellation.
    /// # Errors
    /// Preflight failures reject the entire batch before any experiment starts.
    pub fn run_with_control(&mut self, control: &RunControl) -> ApiResult<BatchReport> {
        let prepared = self.prepare()?;
        let profile = self.profile.as_ref().expect("preflight profile").clone();
        self.execute_prepared(
            prepared
                .into_iter()
                .map(|entry| (profile.clone(), entry))
                .collect(),
            control,
        )
    }
    fn execute_prepared(
        &mut self,
        prepared: Vec<(Dpd, PreparedExperiment)>,
        control: &RunControl,
    ) -> ApiResult<BatchReport> {
        let mut entries = Vec::new();
        for (index, (profile, experiment)) in prepared.into_iter().enumerate() {
            let id = ExperimentId {
                scope: self.scope,
                slot: self.runs * 256 + index,
            };
            if control.cancel.load(Ordering::Relaxed) {
                entries.push(ExperimentEntry::not_started(
                    id,
                    profile.profile_id(),
                    experiment.sources,
                ));
                continue;
            }
            let limits = ChannelJobLimits {
                max_runtime_ms: self.options.limits.runtime.0,
                max_retained_words: self.options.limits.retained_words.0,
            };
            let result = run_channel(
                &experiment.settings,
                &limits,
                &control.cancel,
                &mut TestPointProbe::disabled(),
                &mut NoopObserver,
                |_| {},
            );
            entries.push(ExperimentEntry::executed(
                id,
                experiment.sources,
                self.ocd.clone(),
                profile,
                self.options.clone(),
                result,
            ));
        }
        self.runs += 1;
        Ok(BatchReport::new(
            entries,
            control.cancel.load(Ordering::Relaxed),
        ))
    }
}
