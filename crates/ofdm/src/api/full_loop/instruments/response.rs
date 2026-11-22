/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Configured H and actual receiver H-hat remain independent reference planes.
use super::*;
use crate::{
    signal_processing::{
        channel::composition::prepare_channel, receiver::diagnostics::response_diagnostics,
    },
    transmission::layout::{CarrierLayout, PhysicalCarrierRole},
};

/// Explicit response source; absence of H-hat never selects configured H implicitly.
#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum FullLoopResponseSource {
    Configured,
    ReceiverEstimate { snapshot: usize },
}
/// Response source and explicit uniform-cable velocity factor for the diagnostic delay ruler.
#[derive(Clone, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct FullLoopResponseQuery {
    pub source: FullLoopResponseSource,
    pub velocity_factor: f64,
}
impl Default for FullLoopResponseQuery {
    /// Selects configured transfer and the existing 0.87 uniform-cable diagnostic convention.
    /// # Returns
    /// Passive explicit source/ruler parameters; no receiver estimate is synthesized.
    fn default() -> Self {
        Self {
            source: FullLoopResponseSource::Configured,
            velocity_factor: 0.87,
        }
    }
}
/// Full original bounded 4K/8K response curves and band-limited impulse diagnostics.
#[derive(Serialize)]
#[non_exhaustive]
pub struct FullLoopResponse {
    pub source: FullLoopResponseSource,
    pub reference_plane: &'static str,
    pub channel_symbol_index: Option<u64>,
    pub zero_frequency_hz: u32,
    pub spacing_hz: u32,
    pub velocity_factor: f64,
    pub diagnostics: FullLoopResponseDiagnostics,
}

impl FullLoopResult {
    /// Derives existing response diagnostics from configured H or actual retained H-hat.
    /// # Arguments
    /// * `query` - Explicit source and finite velocity factor in (0,1].
    /// # Returns
    /// Owned original-bin curves with excluded gaps and signed band-limited delay diagnostics.
    /// Configured H is the existing composition plan before receiver correction; it does not
    /// replace original captured H or claim to measure a continuous filter's realized output.
    /// # Errors
    /// Rejects absent/incomplete receiver observations, invalid geometry/velocity and numerical
    /// errors. Missing receiver estimates never fall back to configured or supplied H.
    pub fn response(&self, query: FullLoopResponseQuery) -> ApiResult<FullLoopResponse> {
        let settings = &self.job.settings;
        let layout = CarrierLayout::new(&settings.transmission, &settings.layout)?;
        let (values, channel_symbol_index, reference_plane) = match query.source {
            FullLoopResponseSource::Configured => (
                prepare_channel(
                    &settings.transmission,
                    &layout,
                    &settings.impairments,
                    settings.llr_variance,
                    &settings.channel_filter,
                )?
                .plan
                .response,
                None,
                "configured_channel_transfer_before_receiver",
            ),
            FullLoopResponseSource::ReceiverEstimate { snapshot } => {
                let event = selected(self, FullLoopStage::RxChannelEstimate, snapshot)?;
                let FullLoopStageData::Iq(values) = &event.data else {
                    return Err(super::super::super::error::invalid(
                        "Retained H-hat is not complex I/Q",
                    ));
                };
                if event.element_start != 0 || values.len() != layout.static_roles().len() {
                    return Err(ApiError::new(
                        ErrorCode::MissingCoverage,
                        "Receiver H-hat requires the complete original physical grid",
                    ));
                }
                (
                    values.clone(),
                    event.context.channel_symbol_index,
                    "receiver_estimated_channel_transfer",
                )
            }
        };
        let masked: Vec<_> = values
            .into_iter()
            .zip(layout.static_roles())
            .map(|(value, role)| (*role != PhysicalCarrierRole::Excluded).then_some(value))
            .collect();
        Ok(FullLoopResponse {
            source: query.source,
            reference_plane,
            channel_symbol_index,
            zero_frequency_hz: settings.layout.zero_frequency_hz,
            spacing_hz: settings.transmission.subcarrier_spacing.frequency_hz(),
            velocity_factor: query.velocity_factor,
            diagnostics: response_diagnostics(
                settings.transmission.subcarrier_spacing,
                &masked,
                query.velocity_factor,
            )?,
        })
    }
}
