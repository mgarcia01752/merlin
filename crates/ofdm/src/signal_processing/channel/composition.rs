/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Shared composition of diagonal loss and continuous channel memory.
use super::{
    filter::{ChannelFilterPlan, ChannelFilterSettings},
    impairments::{ChannelImpairments, ImpairmentPlan, NoiseModel},
};
use crate::{
    transmission::{
        layout::{CarrierLayout, PhysicalCarrierRole},
        TransmissionSettings,
    },
    types::IqPair,
    SimulationError,
};

/// Frozen response/noise plan and its distinct pre-filter synthesis boundary.
pub struct PreparedChannel {
    /// Total realized channel response and decoder-reference noise powers.
    pub plan: ImpairmentPlan,
    /// Optional continuous filter, retaining the configured finite memory only.
    pub filter: Option<ChannelFilterPlan>,
    /// Per-carrier gain before the continuous filter; None selects diagonal operation.
    pub synthesis_response: Option<Vec<IqPair>>,
}

/// Prepares the same bounded channel for admission, execution and previews.
/// # Arguments
/// * `settings` - Accepted FFT geometry and physical frequency grid.
/// * `layout` - Explicit active/excluded carrier roles on that grid.
/// * `impairments` - Immutable supplied response, loss, conditioning and noise.
/// * `variance` - Positive fallback decoder variance for noiseless bins.
/// * `filter_settings` - Diagonal, synthetic echo or bounded captured-FIR policy.
/// # Returns
/// Owned total response plus independent synthesis/filter plans. Echoes follow
/// diagonal loss/conditioning; captured FIR already contains the supplied response.
/// # Errors
/// Rejects imported-H/echo ambiguity, unsupported noise combinations, bad fits,
/// invalid grids, bounds and active-bin nulls. Inputs are never mutated.
pub fn prepare_channel(
    settings: &TransmissionSettings,
    layout: &CarrierLayout,
    impairments: &ChannelImpairments,
    variance: f64,
    filter_settings: &ChannelFilterSettings,
) -> Result<PreparedChannel, SimulationError> {
    let mut plan = impairments.plan(settings, layout, variance)?;
    let echoes = matches!(filter_settings, ChannelFilterSettings::Echoes { .. });
    if echoes && impairments.response.is_some() {
        return Err(SimulationError::new("ChannelFilterInvalid", "Synthetic echoes require Flat / unity input; imported H is a separate response source. Tilt and cable loss may be combined with echoes."));
    }
    if matches!(filter_settings, ChannelFilterSettings::CapturedFir { .. })
        && impairments.response.is_none()
    {
        return Err(SimulationError::new(
            "ChannelFilterInvalid",
            "Import a DS channel response before selecting captured FIR.",
        ));
    }
    let active: Vec<_> = layout
        .static_roles()
        .iter()
        .map(|role| *role != PhysicalCarrierRole::Excluded)
        .collect();
    let unity = vec![[1.0, 0.0]; plan.response.len()];
    let filter = filter_settings.prepare(
        settings.subcarrier_spacing,
        if echoes { &unity } else { &plan.response },
        &active,
    )?;
    let synthesis_response =
        filter
            .as_ref()
            .map(|_| if echoes { plan.response.clone() } else { unity });
    if let Some(filter) = &filter {
        for (bin, gain) in plan.response.iter_mut().enumerate() {
            let filtered = filter.response()[bin];
            *gain = if echoes {
                [
                    gain[0] * filtered[0] - gain[1] * filtered[1],
                    gain[0] * filtered[1] + gain[1] * filtered[0],
                ]
            } else {
                filtered
            };
            if !active[bin] {
                continue;
            }
            let power = gain[0].powi(2) + gain[1].powi(2);
            if !power.is_finite() || power < 1e-12 {
                return Err(SimulationError::new(
                    "ChannelFilterInvalid",
                    "Realized channel has a deep null below the supported inverse floor.",
                ));
            }
            plan.llr_variance[bin] = match impairments.noise {
                NoiseModel::Awgn { sample_power } if sample_power > 0.0 => sample_power / power,
                _ if plan.receiver_noise_power > 0.0 => {
                    plan.residual_power[bin] + plan.receiver_noise_power / power
                }
                _ => plan.llr_variance[bin],
            };
        }
    }
    Ok(PreparedChannel {
        plan,
        filter,
        synthesis_response,
    })
}
