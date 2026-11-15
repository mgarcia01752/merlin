/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Controller-owned profile inventory and bounded sequential execution.
use super::*;

impl Merlin {
    /// Creates a controller retaining all supplied profiles without running it.
    /// # Arguments
    /// * `protocol`, `overrides` - Explicit interpretation and optional OCD settings.
    /// * `profiles` - Nonempty compatible captured profiles; IDs must be unique.
    /// # Returns
    /// Owned controller with the first supplied profile selected.
    /// # Errors
    /// Rejects incompatible geometry, duplicate IDs or storage/count limits.
    pub fn from_profiles(
        protocol: ProtocolScope,
        profiles: Vec<Dpd>,
        overrides: OcdOverrides,
    ) -> ApiResult<Self> {
        if profiles.is_empty() || profiles.len() > 16 {
            return Err(super::super::error::invalid(
                "Supply between one and sixteen profiles",
            ));
        }
        let ocd = Ocd::from_profile(protocol, &profiles[0], overrides)?;
        let mut sim = Self::new(ocd);
        let mut ids = Vec::new();
        let mut storage = 0usize;
        for profile in &profiles {
            if ids.contains(&profile.profile_id()) {
                return Err(super::super::error::invalid("Duplicate profile ID"));
            }
            ids.push(profile.profile_id());
            storage = storage
                .checked_add(Self::profile_storage(profile))
                .ok_or_else(|| {
                    ApiError::new(ErrorCode::ResourceLimit, "Profile storage overflow")
                })?;
            sim.set_profile(profile.clone())?;
        }
        if storage
            .checked_add(Self::profile_storage(&profiles[0]))
            .is_none_or(|bytes| bytes > 64 * 1024 * 1024)
        {
            return Err(ApiError::new(
                ErrorCode::ResourceLimit,
                "Profile collection exceeds storage budget",
            ));
        }
        sim.profile = Some(profiles[0].clone());
        sim.loaded_profiles = profiles;
        Ok(sim)
    }
    /// Replaces profiles atomically while retaining compatible measurements and IDs.
    /// # Arguments
    /// * `protocol`, `profiles`, `overrides` - Validated loading request.
    /// # Returns
    /// Unit with the first new profile selected; no simulation runs.
    /// # Errors
    /// Rejects invalid profiles or incompatible retained measurements without mutation.
    pub fn load_profiles_with_options(
        &mut self,
        protocol: ProtocolScope,
        profiles: Vec<Dpd>,
        overrides: OcdOverrides,
    ) -> ApiResult<()> {
        let mut candidate = Self::from_profiles(protocol, profiles, overrides)?;
        if self
            .rxmer
            .iter()
            .any(|frame| frame.frame.key != candidate.ocd.key)
            || self
                .channels
                .iter()
                .any(|frame| frame.frame.key != candidate.ocd.key)
        {
            return Err(ApiError::new(
                ErrorCode::IncompatibleGrid,
                "Loaded profiles differ from retained measurement grids",
            ));
        }
        candidate.scope = self.scope;
        for frame in &self.rxmer {
            candidate.add_rxmer(frame.clone())?;
        }
        for frame in &self.channels {
            candidate.add_channel_estimate(frame.clone())?;
        }
        candidate.set_options(self.options.clone())?;
        candidate.runs = self.runs;
        *self = candidate;
        Ok(())
    }
    /// Lists IDs retained by this controller in loading order.
    /// # Returns
    /// Owned IDs; legacy controllers expose their selected profile if present.
    pub fn profile_ids(&self) -> Vec<ProfileId> {
        if self.loaded_profiles.is_empty() {
            self.profile.iter().map(Dpd::profile_id).collect()
        } else {
            self.loaded_profiles.iter().map(Dpd::profile_id).collect()
        }
    }
    /// Borrows one retained profile without rereading its source file.
    /// # Arguments
    /// * `id` - An ID from this controller's profile inventory.
    /// # Returns
    /// Immutable captured profile with original provenance.
    /// # Errors
    /// Rejects IDs absent from the loaded collection.
    pub fn profile_by_id(&self, id: ProfileId) -> ApiResult<&Dpd> {
        self.loaded_profiles
            .iter()
            .chain(self.profile.iter())
            .find(|profile| profile.profile_id() == id)
            .ok_or_else(|| super::super::error::invalid("Profile ID is not loaded"))
    }
    /// Selects a loaded profile for subsequent single-profile runs.
    /// # Arguments
    /// * `id` - Retained captured profile identity.
    /// # Returns
    /// Unit; does not prepare or run simulation.
    /// # Errors
    /// Rejects absent IDs or incompatible settings without changing selection.
    pub fn select_profile(&mut self, id: ProfileId) -> ApiResult<()> {
        let profile = self.profile_by_id(id)?.clone();
        self.set_profile(profile)
    }
    /// Runs all retained profiles through the existing engine.
    /// # Returns
    /// Owned batch with profile IDs and actual per-experiment results.
    /// # Errors
    /// Rejects invalid configuration before any simulation starts.
    pub fn run_all_profiles(&mut self) -> ApiResult<BatchReport> {
        self.run_all_profiles_with_control(&RunControl::new())
    }
    /// Runs all profiles with cooperative cancellation and preserved selection.
    /// # Arguments
    /// * `control` - Shared stop token; cancelled requests retain not-started entries.
    /// # Returns
    /// Owned batch in profile loading order and then RxMER selection order.
    /// # Errors
    /// Rejects any preflight failure or more than 256 combined experiments.
    pub fn run_all_profiles_with_control(
        &mut self,
        control: &RunControl,
    ) -> ApiResult<BatchReport> {
        let ids = self.profile_ids();
        if ids.is_empty() {
            return Err(super::super::error::invalid("No profiles are loaded"));
        }
        let selected = self.profile.clone();
        let prepared = (|| {
            let mut all = Vec::new();
            for id in ids {
                self.select_profile(id)?;
                let profile = self.profile.as_ref().expect("selected profile").clone();
                let experiments = self.prepare()?;
                if all.len() + experiments.len() > 256 {
                    return Err(ApiError::new(
                        ErrorCode::ResourceLimit,
                        "Combined profile batch exceeds 256 experiments",
                    ));
                }
                all.extend(
                    experiments
                        .into_iter()
                        .map(|experiment| (profile.clone(), experiment)),
                );
            }
            Ok(all)
        })();
        self.profile = selected;
        self.execute_prepared(prepared?, control)
    }
}
