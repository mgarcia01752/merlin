/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Compact-profile rate analysis using OSSI assignments and useful OFDM timing.
use crate::{
    common::signal_processing::ofdm,
    pnm::{
        data_type::{
            modulation::{CarrierType, ModulationAssignment},
            units::HZ_PER_KHZ,
            BitCount, BitRateBps, CarrierCount, FrequencyHz, SubcarrierSpacingKhz,
        },
        parser::cm_ds_ofdm_modulation_profile::{ModulationProfile, ModulationScheme},
    },
};
use serde::Serialize;

/// Ideal known-data contribution; unclassified assignments make the rate unknown.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ModulationProfileAnalysis {
    pub data_subcarrier_count: CarrierCount,
    pub data_bits_per_symbol: BitCount,
    pub data_bandwidth_hz: Option<FrequencyHz>,
    pub theoretical_throughput_bps: Option<BitRateBps>,
    pub useful_symbol_duration_s: Option<f64>,
    pub unclassified_subcarrier_count: CarrierCount,
    pub complete: bool,
    pub basis: &'static str,
    pub excluded_overheads: [&'static str; 3],
}
impl ModulationProfileAnalysis {
    /// Calculates data-only capacity directly from compressed schemes.
    /// # Arguments
    /// * `profile` - Parsed assignments; odd skip lengths begin and end with main.
    /// * `spacing_khz` - Encoded subcarrier spacing, converted once to Hz.
    /// # Returns
    /// Data counts/bandwidth and ideal rate before overhead. Reserved/unknown
    /// assignments or zero spacing produce an incomplete analysis and null rate.
    /// Known-data counts remain available; PLC and pilots contribute no data rate,
    /// even when their physical constellations have nonzero bit loading.
    pub fn calculate(profile: &ModulationProfile, spacing_khz: SubcarrierSpacingKhz) -> Self {
        let spacing = FrequencyHz::from(spacing_khz) * HZ_PER_KHZ;
        let mut data_subcarrier_count = 0;
        let mut data_bits_per_symbol = 0;
        let mut unclassified_subcarrier_count = 0;
        // A decoded profile has <=65536 schemes of u16 counts: these sums and
        // products fit u64. No expanded arrays or per-carrier allocations occur.
        for scheme in &profile.schemes {
            let groups = match *scheme {
                ModulationScheme::Range {
                    modulation_order,
                    num_subcarriers,
                } => [(modulation_order, num_subcarriers), (modulation_order, 0)],
                ModulationScheme::Skip {
                    main_modulation_order,
                    skip_modulation_order,
                    num_subcarriers,
                } => [
                    (main_modulation_order, num_subcarriers.div_ceil(2)),
                    (skip_modulation_order, num_subcarriers / 2),
                ],
            };
            for (code, count) in groups {
                let assignment = ModulationAssignment::from_code(code);
                let count = CarrierCount::from(count);
                match assignment.carrier_type {
                    CarrierType::Data => {
                        data_subcarrier_count += count;
                        data_bits_per_symbol +=
                            count * BitCount::from(assignment.bit_loading.unwrap_or(0));
                    }
                    CarrierType::Reserved | CarrierType::Unknown => {
                        unclassified_subcarrier_count += count
                    }
                    _ => {}
                }
            }
        }
        let data_bandwidth_hz = ofdm::data_bandwidth(data_subcarrier_count, spacing);
        let complete = unclassified_subcarrier_count == 0 && data_bandwidth_hz.is_some();
        Self {
            data_subcarrier_count,
            data_bits_per_symbol,
            data_bandwidth_hz,
            theoretical_throughput_bps: if complete {
                ofdm::useful_symbol_rate(data_bits_per_symbol, spacing)
            } else {
                None
            },
            useful_symbol_duration_s: if spacing > 0 {
                Some(1.0 / f64::from(spacing))
            } else {
                None
            },
            unclassified_subcarrier_count,
            complete,
            basis: "ideal_gross_useful_symbol",
            excluded_overheads: ["cyclic_prefix", "fec", "mac_and_other_phy_overhead"],
        }
    }
}
