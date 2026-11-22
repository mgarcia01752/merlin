/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Pure bounded queries over an admitted immutable measurement sequence.
use super::*;

impl MeasurementSequence {
    /// Reports the accepted synthetic clock without altering original capture times.
    /// # Returns
    /// Mode/count/interval/span metadata from the existing planner.
    pub fn info(&self) -> MeasurementSequenceInfo {
        let (last_transition_symbol, last_transition_milliseconds) = self.plan.span();
        MeasurementSequenceInfo {
            mode: self.options.mode,
            source_count: self.captures.len(),
            interval_symbols: self.options.interval_symbols,
            interval_milliseconds: self.plan.interval_milliseconds(),
            last_transition_symbol,
            last_transition_milliseconds,
        }
    }

    /// Reports permitted selection capacity for a proposed observation, without admitting a run.
    /// # Arguments
    /// * `duration` - Checked source observation interval, distinct from host runtime.
    /// # Returns
    /// One for Single, 256 for static Average, or bounded temporal reachability.
    /// # Errors
    /// Preserves exact timing/overflow/insufficient-symbol errors.
    pub fn capacity(&self, duration: ObservationDuration) -> ApiResult<usize> {
        let symbols = self.transmission.timing(duration)?.complete_symbols;
        Ok(match self.options.mode {
            MeasurementMode::Single => 1,
            MeasurementMode::Average => MAX_PLAYBACK_FRAMES,
            _ => ((symbols.saturating_sub(1) / self.options.interval_symbols).saturating_add(1))
                .min(MAX_PLAYBACK_FRAMES as u64) as usize,
        })
    }

    /// Queries normalized residual powers at an explicit synthetic physical symbol.
    /// # Arguments
    /// * `symbol` - Zero-based observation symbol; temporal modes hold their last frame.
    /// # Returns
    /// Owned powers and all actual source weights/provenance, without RNG or state changes.
    pub fn at(&self, symbol: u64) -> MeasurementResidual {
        let snapshot = self.plan.at(symbol);
        let weights = if self.options.mode == MeasurementMode::Average {
            (0..self.captures.len())
                .map(|i| (i, 1.0 / self.captures.len() as f64))
                .collect::<Vec<_>>()
        } else if snapshot.first_source == snapshot.second_source || snapshot.second_weight == 0.0 {
            vec![(snapshot.first_source, 1.0)]
        } else {
            vec![
                (snapshot.first_source, 1.0 - snapshot.second_weight),
                (snapshot.second_source, snapshot.second_weight),
            ]
        };
        MeasurementResidual {
            symbol,
            normalized_power: snapshot.power,
            contributors: weights
                .into_iter()
                .map(|(index, weight)| MeasurementContribution {
                    index,
                    weight: Weight(weight),
                    provenance: self.captures[index].provenance.clone(),
                })
                .collect(),
        }
    }

    /// Previews equivalent residual RxMER in dB through the bounded measurement-view owner.
    /// # Arguments
    /// * `symbol` - Explicit synthetic symbol; original source epochs remain intact.
    /// * `viewport` - Original carrier offsets; zero count requests the complete grid.
    /// # Returns
    /// Owned bounded equivalent view and every actual source contribution.
    /// # Errors
    /// Preserves viewport and representable-range admission failures without mutation.
    pub fn preview(
        &self,
        symbol: u64,
        viewport: MeasurementViewport,
    ) -> ApiResult<MeasurementSequencePreview> {
        let snapshot = self.at(symbol);
        let values = snapshot
            .normalized_power
            .into_iter()
            .map(|v| v.and_then(residual_power_mer))
            .collect::<Vec<_>>();
        let grid = self.captures[0].grid().clone();
        Ok(MeasurementSequencePreview {
            view: measurement_view(
                &grid,
                &values,
                viewport.start,
                viewport.count,
                "Equivalent RxMER (dB)",
            )?,
            grid,
            symbol,
            contributors: snapshot.contributors,
        })
    }
}
