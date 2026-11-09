/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
use super::{
    interpolation::{interpolate, MIN_RESPONSE_POWER},
    ChannelEstimate, PilotEstimate,
};
use crate::{
    error::invalid,
    signal_processing::pilots::{PilotPower, PilotSequence},
    transmission::layout::{CarrierLayout, PhysicalCarrierRole},
    types::{IqPair, PLC_FRAME_SYMBOLS},
    SimulationError,
};
/// RX-only history; stores at most one latest observation per FFT bin.
pub struct PilotChannelEstimator {
    layout: CarrierLayout,
    sequence: PilotSequence,
    amplitude: f64,
    history: Vec<Option<(u64, IqPair)>>,
    previous: Option<Vec<IqPair>>,
    first: Option<u64>,
    next: Option<u64>,
}
impl PilotChannelEstimator {
    /// Creates independent pilot references from receiver bootstrap geometry.
    /// # Arguments
    /// * `layout` - Receiver-known carrier assignments and spacing.
    /// * `reference_rms` - Positive nonpilot RMS; pilot boost is preserved.
    /// # Returns
    /// Empty bounded history, with no supplied channel or TX truth.
    /// # Errors
    /// Rejects invalid RMS/reference power.
    pub fn new(layout: CarrierLayout, reference_rms: f64) -> Result<Self, SimulationError> {
        let sequence = PilotSequence::new(layout.subcarrier_spacing());
        let size = layout.static_roles().len();
        Ok(Self {
            layout,
            sequence,
            amplitude: PilotPower::new(reference_rms)?.amplitude(),
            history: vec![None; size],
            previous: None,
            first: None,
            next: None,
        })
    }
    /// Estimates H=Y/P at pilots and interpolates bounded, age-limited history.
    /// # Arguments
    /// * `symbol` - Consecutive physical symbol index with known PLC phase.
    /// * `bins` - Received FFT samples only; input remains unchanged.
    /// # Returns
    /// Coefficients, raw pilot estimates and prior predictions. Ready after a full
    ///128-symbol pattern; current pilot residuals are in-fit, prior values are not.
    /// # Errors
    /// Rejects shape, numeric/sequence errors or near-zero estimates atomically.
    /// Invalid calls leave history and counters unchanged.
    pub fn estimate(
        &mut self,
        symbol: u64,
        bins: &[IqPair],
    ) -> Result<ChannelEstimate, SimulationError> {
        if bins.len() != self.history.len()
            || self.next.is_some_and(|next| next != symbol)
            || bins.iter().flatten().any(|value| !value.is_finite())
        {
            return Err(invalid("Invalid CH-EST-RX FFT grid or symbol sequence."));
        }
        let next = symbol
            .checked_add(1)
            .ok_or_else(|| invalid("CH-EST-RX symbol overflow."))?;
        let roles = self.layout.symbol_roles(symbol);
        let mut history = self.history.clone();
        for entry in &mut history {
            if entry.is_some_and(|(observed, _)| symbol - observed >= PLC_FRAME_SYMBOLS) {
                *entry = None;
            }
        }
        let mut pilots = Vec::new();
        for (bin, role) in roles.iter().enumerate() {
            if !matches!(
                role,
                PhysicalCarrierRole::ContinuousPilot | PhysicalCarrierRole::ScatteredPilot
            ) {
                continue;
            }
            let sign = 1.0 - 2.0 * f64::from(self.sequence.bits()[bin]);
            let reference = sign * self.amplitude;
            let response = bins[bin].map(|value| value / reference);
            let response_power = response[0] * response[0] + response[1] * response[1];
            if !response_power.is_finite() || response_power < MIN_RESPONSE_POWER {
                return Err(invalid(
                    "CH-EST-RX pilot gain is below the inverse-power floor.",
                ));
            }
            pilots.push(PilotEstimate {
                bin: bin as u16,
                scattered: *role == PhysicalCarrierRole::ScatteredPilot,
                reference: [reference, 0.0],
                received: bins[bin],
                response,
                prior_prediction: self.previous.as_ref().map(|previous| previous[bin]),
            });
            history[bin] = Some((symbol, response));
        }
        let nodes: Vec<_> = history
            .iter()
            .enumerate()
            .filter_map(|(bin, entry)| entry.map(|(_, point)| (bin, point)))
            .collect();
        let (mut response, slope) = interpolate(&nodes, bins.len())?;
        for (point, role) in response.iter_mut().zip(self.layout.static_roles()) {
            if *role == PhysicalCarrierRole::Excluded {
                *point = [1.0, 0.0];
            }
        }
        let first = self.first.unwrap_or(symbol);
        let ready = symbol - first >= PLC_FRAME_SYMBOLS - 1;
        let maximum_age_symbols = history
            .iter()
            .flatten()
            .map(|(observed, _)| symbol - observed)
            .max()
            .unwrap_or(0);
        self.history = history;
        self.first = Some(first);
        self.next = Some(next);
        self.previous = Some(response.clone());
        Ok(ChannelEstimate {
            channel_symbol_index: symbol,
            ready,
            observed_bins: nodes.len(),
            maximum_age_symbols,
            phase_slope_radians_per_bin: slope,
            pilots,
            response,
        })
    }
}
