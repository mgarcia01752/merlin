/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Inverse-interleaved128-symbol pilot placeholders, PHY I20 section7.5.6.3.
use super::{ncp::PreInterleavingRole, CarrierLayout, PhysicalCarrierRole};
use crate::{
    error::invalid,
    signal_processing::interleaving::{FrequencyInterleaver, InterleaverSettings},
    types::{SubcarrierIdx, PLC_FRAME_SYMBOLS},
    LabError,
};

/// Fixed association of physical bins and correctly phased pre-interleaving pilots.
pub struct PilotReference {
    settings: InterleaverSettings,
    physical_bins: Vec<SubcarrierIdx>,
    preinterleaving: Vec<Vec<PreInterleavingRole>>,
}
impl PilotReference {
    /// Derives a periodic reference without modifying the physical layout.
    /// # Arguments
    /// * `layout` - Validated static PLC/CP/exclusion geometry and scattered pattern.
    /// * `depth` - Fixed time-interleaver depth M for this job.
    /// # Returns
    /// Owned reduced-domain association and128 phases, aligned so physical phase0
    /// is the first symbol after the PLC preamble. No pilot BPSK values are generated.
    /// # Errors
    /// Rejects invalid NI/depth or incompatible physical roles/permutation.
    pub fn new(layout: &CarrierLayout, depth: usize) -> Result<Self, LabError> {
        let first = layout.symbol_roles(0);
        let mut physical_bins = Vec::new();
        for (bin, role) in first.iter().enumerate() {
            if matches!(
                role,
                PhysicalCarrierRole::Data | PhysicalCarrierRole::ScatteredPilot
            ) {
                physical_bins.push(
                    SubcarrierIdx::try_from(bin)
                        .map_err(|_| invalid("Physical bin exceeds SubcarrierIdx."))?,
                );
            }
        }
        let settings =
            InterleaverSettings::new(layout.subcarrier_spacing(), physical_bins.len(), depth)?;
        let frequency = FrequencyInterleaver::new(settings.spacing(), settings.cells())?;
        let mut before_frequency = Vec::new();
        for phase in 0..PLC_FRAME_SYMBOLS {
            let roles = layout.symbol_roles(phase);
            let physical: Vec<_> = physical_bins
                .iter()
                .map(|bin| roles[usize::from(*bin)] == PhysicalCarrierRole::ScatteredPilot)
                .collect();
            before_frequency.push(frequency.deinterleave(&physical)?);
        }
        let period = PLC_FRAME_SYMBOLS as usize;
        let mut preinterleaving = vec![vec![PreInterleavingRole::Data; settings.cells()]; period];
        for (phase, roles) in preinterleaving.iter_mut().enumerate() {
            for (position, role) in roles.iter_mut().enumerate() {
                // TX delay at rank p is p mod M whole symbols. Advance the known
                // periodic output reference by that delay, rather than shifting
                // its PLC phase by the matched receiver's extra delay.
                let branch_delay = position % settings.depth();
                if before_frequency[(phase + branch_delay) % period][position] {
                    *role = PreInterleavingRole::ScatteredPilotPlaceholder;
                }
            }
        }
        Ok(Self {
            settings,
            physical_bins,
            preinterleaving,
        })
    }
    /// Returns settings derived from this layout and selected depth.
    /// # Returns
    /// Validated copy for the packer domain and matched interleaver pair.
    pub fn settings(&self) -> InterleaverSettings {
        self.settings
    }
    /// Borrows physical bins in increasing reduced-domain rank order.
    /// # Returns
    /// Read-only mapping excluding PLC, continuous pilots and excluded bins.
    pub fn physical_bins(&self) -> &[SubcarrierIdx] {
        &self.physical_bins
    }
    /// Borrows the pre-interleaving role mask for a symbol's PLC-aligned phase.
    /// # Arguments
    /// * `symbol_index` -0 is aligned to the first post-preamble symbol; wraps128.
    /// # Returns
    /// Exactly NI data/placeholder roles, valid for this reference's lifetime.
    pub fn roles(&self, symbol_index: u64) -> &[PreInterleavingRole] {
        &self.preinterleaving[(symbol_index % PLC_FRAME_SYMBOLS) as usize]
    }
}
