/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Public preview assembly delegates layout, H admission and actual composition to scientific owners.
use super::*;
use crate::{
    common::constants::MAX_CHANNEL_PREVIEW_RESPONSE_POINTS,
    profile::candidate::CapturedProfile,
    scenario::measurement_view::{measurement_view, response_value, ResponseMetric},
    signal_processing::channel::{composition::prepare_channel, impairments::ChannelImpairments},
    simulation::channel::measurement::captured_layout,
    transmission::layout::{CarrierLayout, PhysicalCarrierRole},
};

pub(super) fn prepare(
    configuration: &ChannelConfiguration,
    transmission: &TransmissionConfiguration,
    profile: &CapturedProfile,
    mut sources: Vec<CaptureProvenance>,
    response: Option<&ChannelEstimate>,
    tolerance: AssociationTolerance,
) -> ApiResult<ChannelPreview> {
    transmission.check_response_profile(profile, response, tolerance)?;
    let layout = CarrierLayout::new(
        transmission.settings(),
        &captured_layout(profile, transmission.settings())?,
    )?;
    let prepared = prepare_channel(
        transmission.settings(),
        &layout,
        &ChannelImpairments {
            response: response.map(|response| response.frame.clone()),
            additional_loss: configuration.settings.additional_loss.clone(),
            power: configuration.settings.power.clone(),
            ..Default::default()
        },
        1.0,
        &configuration.settings.filter,
    )?;
    if let Some(response) = response {
        sources.push(response.provenance.clone());
    }
    let grid = &profile.key.grid;
    let first = usize::from(grid.first_subcarrier_index);
    let mut effective = Vec::with_capacity(grid.sample_count);
    let mut active = Vec::new();
    for offset in 0..grid.sample_count {
        let bin = first + offset;
        if layout.static_roles()[bin] == PhysicalCarrierRole::Excluded {
            effective.push(None);
        } else {
            effective.push(Some(prepared.plan.response[bin]));
            active.push(offset);
        }
    }
    let low = *active.first().ok_or_else(|| {
        ApiError::new(ErrorCode::MissingCoverage, "Channel has no active carrier")
    })?;
    let high = *active.last().ok_or_else(|| {
        ApiError::new(ErrorCode::MissingCoverage, "Channel has no active carrier")
    })?;
    if low == high {
        return Err(ApiError::new(
            ErrorCode::MissingCoverage,
            "Channel preview requires two active carrier centers",
        ));
    }
    let low_hz = grid.frequency(low)?;
    let high_hz = grid.frequency(high)?;
    let loss = &configuration.settings.additional_loss;
    let count = active.len().min(MAX_CHANNEL_PREVIEW_RESPONSE_POINTS);
    let additional_points = (0..count)
        .map(|index| {
            let offset = active[index * (active.len() - 1) / (count - 1)];
            let frequency_hz = grid.frequency(offset)?;
            Ok(ChannelResponsePoint {
                physical_carrier: u16::try_from(first + offset).map_err(|_| {
                    ApiError::new(
                        ErrorCode::IncompatibleGrid,
                        "Channel physical carrier exceeds index range",
                    )
                })?,
                frequency_hz,
                additional_gain_db: loss.gain_db(frequency_hz, low_hz, high_hz)?,
                effective_gain_db: response_value(effective[offset], ResponseMetric::GainDb),
            })
        })
        .collect::<ApiResult<Vec<_>>>()?;
    let gains = effective
        .iter()
        .map(|sample| response_value(*sample, ResponseMetric::GainDb))
        .collect::<Vec<_>>();
    let gain = measurement_view(grid, &gains, low, high - low + 1, "Gain (dB)")?;
    let filter_coefficients = prepared
        .filter
        .as_ref()
        .map_or_else(Vec::new, |filter| filter.coefficients().to_vec());
    let filter_diagnostics = prepared
        .filter
        .as_ref()
        .map(|filter| filter.diagnostics().clone());
    Ok(ChannelPreview {
        reference_plane: "configured_transfer_before_receiver",
        profile_id: profile.profile_id,
        key: profile.key.clone(),
        sources,
        effective_response: effective,
        active_start_offset: low,
        active_count: high - low + 1,
        active_low_frequency_hz: low_hz,
        active_high_frequency_hz: high_hz,
        gain,
        additional_points,
        additional_low_gain_db: loss.gain_db(low_hz, low_hz, high_hz)?,
        additional_high_gain_db: loss.gain_db(high_hz, low_hz, high_hz)?,
        power: configuration.power_reference(),
        filter_coefficients,
        filter_diagnostics,
    })
}
