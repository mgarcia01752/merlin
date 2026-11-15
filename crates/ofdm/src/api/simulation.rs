/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Idle simulation session using the existing validated controller and engine.
use super::*;

/// Single-channel session. Load profiles before adding measurements or running.
/// Loading is atomic; reports independently own their results. No Qt or I/O.
#[derive(Default)]
pub struct Simulation {
    controller: Option<Merlin>,
}
impl From<Merlin> for Simulation {
    fn from(controller: Merlin) -> Self {
        Self {
            controller: Some(controller),
        }
    }
}
impl Simulation {
    /// Creates an idle session without channel geometry or simulation work.
    /// # Returns
    /// Empty session; load profiles before measurement and execution operations.
    pub fn new() -> Self {
        Self::default()
    }
    /// Creates an idle session from explicit validated channel geometry.
    /// # Arguments
    /// * `ocd` - Owned validated geometry for manual profile construction.
    /// # Returns
    /// Session without a selected profile or measurements.
    pub fn with_ocd(ocd: Ocd) -> Self {
        Merlin::new(ocd).into()
    }
    /// Loads profiles using DOCSIS 3.1 and default OCD settings.
    /// # Arguments
    /// * `profiles` - Complete decoded profile collection to retain.
    /// # Returns
    /// Unit; selects the first profile without running simulation.
    /// # Errors
    /// Preserves validation failures without changing the accepted configuration.
    pub fn load_profiles(&mut self, profiles: Vec<Dpd>) -> ApiResult<()> {
        self.load_profiles_with_options(ProtocolScope::default(), profiles, OcdOverrides::default())
    }
    /// Loads profiles with explicit interpretation and optional channel settings.
    /// # Arguments
    /// * `protocol`, `profiles`, `overrides` - Owned profile loading request.
    /// # Returns
    /// Unit; retained measurements and identifiers remain valid on success.
    /// # Errors
    /// Rejects invalid profiles or incompatible retained captures atomically.
    pub fn load_profiles_with_options(
        &mut self,
        protocol: ProtocolScope,
        profiles: Vec<Dpd>,
        overrides: OcdOverrides,
    ) -> ApiResult<()> {
        if let Some(controller) = &mut self.controller {
            controller.load_profiles_with_options(protocol, profiles, overrides)
        } else {
            self.controller = Some(Merlin::from_profiles(protocol, profiles, overrides)?);
            Ok(())
        }
    }
    fn controller(&self) -> ApiResult<&Merlin> {
        self.controller.as_ref().ok_or_else(|| {
            ApiError::new(
                ErrorCode::InvalidInput,
                "Load profiles or provide an OCD before configuring or running the simulation",
            )
        })
    }
    fn controller_mut(&mut self) -> ApiResult<&mut Merlin> {
        self.controller.as_mut().ok_or_else(|| {
            ApiError::new(
                ErrorCode::InvalidInput,
                "Load profiles or provide an OCD before configuring or running the simulation",
            )
        })
    }
    /// Lists retained profile IDs without running or reading a file.
    /// # Returns
    /// Empty until profiles are loaded; otherwise IDs in retained order.
    pub fn profile_ids(&self) -> Vec<ProfileId> {
        self.controller
            .as_ref()
            .map(Merlin::profile_ids)
            .unwrap_or_default()
    }
    /// Borrows the selected profile, if one is configured.
    /// # Returns
    /// None for an empty session.
    pub fn profile(&self) -> Option<&Dpd> {
        self.controller.as_ref().and_then(Merlin::profile)
    }
    /// Delegates `ocd` to the existing validated controller.
    /// # Returns
    /// The controller result; only run methods execute simulation.
    /// # Errors
    /// Rejects unconfigured sessions; preserves controller validation and run errors.
    pub fn ocd(&self) -> ApiResult<&Ocd> {
        Ok(self.controller()?.ocd())
    }
    /// Delegates `options` to the existing validated controller.
    /// # Returns
    /// The controller result; only run methods execute simulation.
    /// # Errors
    /// Rejects unconfigured sessions; preserves controller validation and run errors.
    pub fn options(&self) -> ApiResult<&EvaluationOptions> {
        Ok(self.controller()?.options())
    }
    /// Delegates `profile_by_id` to the existing validated controller.
    /// # Arguments
    /// * `id` - Typed request passed unchanged to the controller.
    /// # Returns
    /// The controller result; only run methods execute simulation.
    /// # Errors
    /// Rejects unconfigured sessions; preserves controller validation and run errors.
    pub fn profile_by_id(&self, id: ProfileId) -> ApiResult<&Dpd> {
        self.controller()?.profile_by_id(id)
    }
    /// Delegates `select_profile` to the existing validated controller.
    /// # Arguments
    /// * `id` - Typed request passed unchanged to the controller.
    /// # Returns
    /// The controller result; only run methods execute simulation.
    /// # Errors
    /// Rejects unconfigured sessions; preserves controller validation and run errors.
    pub fn select_profile(&mut self, id: ProfileId) -> ApiResult<()> {
        self.controller_mut()?.select_profile(id)
    }
    /// Delegates `set_profile` to the existing validated controller.
    /// # Arguments
    /// * `profile` - Typed request passed unchanged to the controller.
    /// # Returns
    /// The controller result; only run methods execute simulation.
    /// # Errors
    /// Rejects unconfigured sessions; preserves controller validation and run errors.
    pub fn set_profile(&mut self, profile: Dpd) -> ApiResult<()> {
        self.controller_mut()?.set_profile(profile)
    }
    /// Delegates `add_rxmer` to the existing validated controller.
    /// # Arguments
    /// * `frame` - Typed request passed unchanged to the controller.
    /// # Returns
    /// The controller result; only run methods execute simulation.
    /// # Errors
    /// Rejects unconfigured sessions; preserves controller validation and run errors.
    pub fn add_rxmer(&mut self, frame: RxMer) -> ApiResult<RxMerId> {
        self.controller_mut()?.add_rxmer(frame)
    }
    /// Delegates `add_channel_estimate` to the existing validated controller.
    /// # Arguments
    /// * `frame` - Typed request passed unchanged to the controller.
    /// # Returns
    /// The controller result; only run methods execute simulation.
    /// # Errors
    /// Rejects unconfigured sessions; preserves controller validation and run errors.
    pub fn add_channel_estimate(&mut self, frame: ChannelEstimate) -> ApiResult<ChannelEstimateId> {
        self.controller_mut()?.add_channel_estimate(frame)
    }
    /// Delegates `rxmer` to the existing validated controller.
    /// # Arguments
    /// * `id` - Typed request passed unchanged to the controller.
    /// # Returns
    /// The controller result; only run methods execute simulation.
    /// # Errors
    /// Rejects unconfigured sessions; preserves controller validation and run errors.
    pub fn rxmer(&self, id: RxMerId) -> ApiResult<&RxMer> {
        self.controller()?.rxmer(id)
    }
    /// Delegates `channel_estimate` to the existing validated controller.
    /// # Arguments
    /// * `id` - Typed request passed unchanged to the controller.
    /// # Returns
    /// The controller result; only run methods execute simulation.
    /// # Errors
    /// Rejects unconfigured sessions; preserves controller validation and run errors.
    pub fn channel_estimate(&self, id: ChannelEstimateId) -> ApiResult<&ChannelEstimate> {
        self.controller()?.channel_estimate(id)
    }
    /// Delegates `set_options` to the existing validated controller.
    /// # Arguments
    /// * `options` - Typed request passed unchanged to the controller.
    /// # Returns
    /// The controller result; only run methods execute simulation.
    /// # Errors
    /// Rejects unconfigured sessions; preserves controller validation and run errors.
    pub fn set_options(&mut self, options: EvaluationOptions) -> ApiResult<()> {
        self.controller_mut()?.set_options(options)
    }
    /// Sets Common Path Attenuator (CPA) attenuation through the Rust controller.
    /// # Arguments
    /// * `value_db` - Finite value in the inclusive 0..60 dB model range.
    /// # Returns
    /// Unit after atomic acceptance without running simulation.
    /// # Errors
    /// Rejects unloaded sessions and invalid settings without mutation.
    pub fn set_cpa_attenuation_db(&mut self, value_db: f64) -> ApiResult<()> {
        self.controller_mut()?.set_cpa_attenuation_db(value_db)
    }
    /// Reads effective Common Path Attenuator (CPA) attenuation without running simulation.
    /// # Returns
    /// Accepted value in dB.
    /// # Errors
    /// Rejects sessions without loaded profiles or explicit OCD.
    pub fn get_cpa_attenuation_db(&self) -> ApiResult<f64> {
        Ok(self.controller()?.get_cpa_attenuation_db())
    }
    /// Sets launch amplifier gain through the Rust controller.
    /// # Arguments
    /// * `value_db` - Finite value in the inclusive 0..60 dB model range.
    /// # Returns
    /// Unit after atomic acceptance without running simulation.
    /// # Errors
    /// Rejects unloaded sessions and invalid settings without mutation.
    pub fn set_launch_amplifier_gain_db(&mut self, value_db: f64) -> ApiResult<()> {
        self.controller_mut()?
            .set_launch_amplifier_gain_db(value_db)
    }
    /// Reads effective launch amplifier gain without running simulation.
    /// # Returns
    /// Accepted value in dB.
    /// # Errors
    /// Rejects sessions without loaded profiles or explicit OCD.
    pub fn get_launch_amplifier_gain_db(&self) -> ApiResult<f64> {
        Ok(self.controller()?.get_launch_amplifier_gain_db())
    }
    /// Delegates `prepare` to the existing validated controller.
    /// # Returns
    /// The controller result; only run methods execute simulation.
    /// # Errors
    /// Rejects unconfigured sessions; preserves controller validation and run errors.
    pub fn prepare(&self) -> ApiResult<Vec<PreparedExperiment>> {
        self.controller()?.prepare()
    }
    /// Delegates `run` to the existing validated controller.
    /// # Returns
    /// The controller result; only run methods execute simulation.
    /// # Errors
    /// Rejects unconfigured sessions; preserves controller validation and run errors.
    pub fn run(&mut self) -> ApiResult<BatchReport> {
        self.controller_mut()?.run()
    }
    /// Delegates `run_with_control` to the existing validated controller.
    /// # Arguments
    /// * `control` - Typed request passed unchanged to the controller.
    /// # Returns
    /// The controller result; only run methods execute simulation.
    /// # Errors
    /// Rejects unconfigured sessions; preserves controller validation and run errors.
    pub fn run_with_control(&mut self, control: &RunControl) -> ApiResult<BatchReport> {
        self.controller_mut()?.run_with_control(control)
    }
    /// Delegates `run_all_profiles` to the existing validated controller.
    /// # Returns
    /// The controller result; only run methods execute simulation.
    /// # Errors
    /// Rejects unconfigured sessions; preserves controller validation and run errors.
    pub fn run_all_profiles(&mut self) -> ApiResult<BatchReport> {
        self.controller_mut()?.run_all_profiles()
    }
    /// Delegates `run_all_profiles_with_control` to the existing validated controller.
    /// # Arguments
    /// * `control` - Typed request passed unchanged to the controller.
    /// # Returns
    /// The controller result; only run methods execute simulation.
    /// # Errors
    /// Rejects unconfigured sessions; preserves controller validation and run errors.
    pub fn run_all_profiles_with_control(
        &mut self,
        control: &RunControl,
    ) -> ApiResult<BatchReport> {
        self.controller_mut()?
            .run_all_profiles_with_control(control)
    }
}
