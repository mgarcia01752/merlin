/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Derived current or last-update pre-equalizer response; binary decoding remains in the parser.
use crate::{
    common::signal_processing::channel_response::{
        group_delay_seconds, magnitude_db, GroupDelaySeconds, MagnitudeDb,
    },
    diagnostic::{ErrorCode, ParseError},
    pnm::{
        data_type::{units::HZ_PER_KHZ, FrequencyHz},
        parser::{cm_us_ofdma_pre_eq::CmUsOfdmaPreEq, pnm_header::HasPnmHeader},
    },
};
use serde::Serialize;

// Relative offsets in the OSSI current/last-update measurement header.
const SPACING_OFFSET: usize = 19;
const ZERO_FREQUENCY_OFFSET: usize = 13;

/// One coefficient-response delay, in seconds, at its original carrier frequency.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PreEqualizationDelay {
    pub frequency_hz: FrequencyHz,
    pub delay_s: Option<GroupDelaySeconds>,
}
/// Full-resolution analysis of the correction coefficients, not measured plant response.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct PreEqualizationAnalysis {
    pub frequencies_hz: Vec<FrequencyHz>,
    pub magnitude_db: Vec<Option<MagnitudeDb>>,
    pub group_delay: Vec<PreEqualizationDelay>,
}
impl PreEqualizationAnalysis {
    /// Calculates coefficient magnitude and unwrapped phase-derived group delay.
    /// # Arguments
    /// * `capture` - Decoded coefficients, preserving their selected wire Q-format.
    /// # Returns
    /// Aligned full-resolution arrays; zero coefficients have null magnitude/delay.
    /// Delay is -d(phase)/d(frequency)/(2*pi), without smoothing or normalization.
    /// # Errors
    /// Reports zero spacing or frequency overflow with the source field offset.
    pub fn from_capture(capture: &CmUsOfdmaPreEq<'_>) -> Result<Self, ParseError> {
        let origin = capture.pnm_header().payload_offset();
        let spacing_hz = FrequencyHz::from(capture.subcarrier_spacing_khz) * HZ_PER_KHZ;
        if spacing_hz == 0 {
            return Err(ParseError {
                code: ErrorCode::UnsupportedLayout,
                offset: origin + SPACING_OFFSET,
                field: "spacing_khz",
            });
        }
        let samples: Vec<_> = capture.iq().collect();
        let frequencies_hz = (0..samples.len())
            .map(|ordinal| {
                let frequency = FrequencyHz::try_from(ordinal)
                    .ok()
                    .and_then(|ordinal| {
                        ordinal
                            .checked_add(FrequencyHz::from(capture.first_active_subcarrier_index))
                    })
                    .and_then(|index| index.checked_mul(spacing_hz))
                    .and_then(|offset| capture.subcarrier_zero_frequency_hz.checked_add(offset));
                frequency.ok_or(ParseError {
                    code: ErrorCode::ArithmeticOverflow,
                    offset: origin + ZERO_FREQUENCY_OFFSET,
                    field: "zero_frequency",
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        let group_delay = frequencies_hz
            .iter()
            .copied()
            .zip(group_delay_seconds(&samples, spacing_hz))
            .map(|(frequency_hz, delay_s)| PreEqualizationDelay {
                frequency_hz,
                delay_s,
            })
            .collect();
        Ok(Self {
            frequencies_hz,
            magnitude_db: samples.into_iter().map(magnitude_db).collect(),
            group_delay,
        })
    }
}

pub mod comparison;
