/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Separate Rust-process actual QAM/pairing/received-display references at both spacings.
use docsis_ofdma::api::*;

const PREFIX_SAMPLES: usize = 192;
const ROLLOFF_SAMPLES: usize = 64;
const FRAME_SYMBOLS: usize = 6;
const BANDWIDTH_HZ: u32 = 20_000_000;
const PROFILE_ID: u16 = 224;
const DATA_IUC: u8 = 5;
const PROFILE_MASK_SEED: u32 = 0x7f_ffff;
const SOURCE_PRBS15_SEED: u16 = 1;
const FIRST_ACTIVE_CARRIER: usize = 128;
const ACTIVE_MINISLOTS: usize = 8;
const DATA_LOADING: u8 = 4;
const BOOSTED_50KHZ_PATTERN: u8 = 5;
const BOOSTED_25KHZ_PATTERN: u8 = 12;
const GRANT_ID: u64 = 7;
const GRANT_EPOCH_SAMPLE: u64 = 73;
const FIRST_FRAME_ID: u64 = 3;
const GRANT_FRAMES: usize = 2;
const RETAINED_POINT_BUDGET: usize = 8192;
const ESTIMATE_FRAME_BUDGET: usize = 1;
const ESTIMATE_SCALAR_BUDGET: usize = 8192;
const REFERENCE_VARIANCE: f64 = 0.02;
const POWER_FLOOR: f64 = 1e-12;
const ITERATION_BUDGET: usize = 50;

/// Exports actual original QAM/unit-reference pairs and received transfer display values.
/// # Returns
/// Unit after printing both modes; all scientific values come from the normal Rust engine/API.
/// # Errors
/// Propagates real configuration, run, bounded-query and serialization failures without replay.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut references = Vec::new();
    for (mode, pattern) in [
        (Numerology::Khz50, BOOSTED_50KHZ_PATTERN),
        (Numerology::Khz25, BOOSTED_25KHZ_PATTERN),
    ] {
        let width = mode.carriers_per_minislot();
        let profile = Profile::new(
            PROFILE_ID,
            DATA_IUC,
            PROFILE_MASK_SEED,
            (0..ACTIVE_MINISLOTS)
                .map(|index| Minislot {
                    first_carrier: FIRST_ACTIVE_CARRIER + index * width,
                    loading: DATA_LOADING,
                    pilot_pattern: pattern,
                })
                .collect(),
        )?;
        let grant = SyntheticGrant {
            id: GRANT_ID,
            profile_id: PROFILE_ID,
            epoch_sample: GRANT_EPOCH_SAMPLE,
            first_frame: FIRST_FRAME_ID,
            frames: GRANT_FRAMES,
            minislots: (0..ACTIVE_MINISLOTS).collect(),
        };
        let us = UsConfiguration::new(
            Waveform::new(
                mode,
                PREFIX_SAMPLES,
                ROLLOFF_SAMPLES,
                FRAME_SYMBOLS,
                BANDWIDTH_HZ,
            )?,
            vec![profile],
            grant,
            vec![0, 1],
        )?;
        let mut config = ExperimentConfig::new(
            us,
            SOURCE_PRBS15_SEED,
            ReceiverConfig::new(
                Estimator::FrameFlatLs,
                REFERENCE_VARIANCE,
                POWER_FLOOR,
                ITERATION_BUDGET,
            )?,
        )?;
        config.channel.taps = vec![Iq { re: -1.0, im: 2.0 }];
        let mut simulation = Simulation::new(ExperimentOptions {
            retained_points: RETAINED_POINT_BUDGET,
            retained_taps: vec![Tap::TxGrid, Tap::TxPreEq, Tap::RxFft, Tap::RxEqualized],
            ..Default::default()
        })?;
        simulation.configure(config)?;
        simulation.set_receiver_observation_policy(ReceiverObservationPolicy::new(
            ESTIMATE_FRAME_BUDGET,
            ESTIMATE_SCALAR_BUDGET,
        )?)?;
        let report = simulation.run()?;
        references.push(serde_json::json!({
            "constellation_rx": report.qam_constellation(Default::default())?,
            "constellation_tx_pilots": report.qam_constellation(QamConstellationRequest { tap: Tap::TxGrid,
                loading: 0, ..Default::default() })?,
            "pairing": report.qam_pairing(Default::default())?,
            "received_response": report.receiver_estimate(ReceiverEstimateRequest {
                first_carrier: FIRST_ACTIVE_CARRIER, carrier_count: 1, ..Default::default() })?,
            "missing_response": report.receiver_estimate(ReceiverEstimateRequest {
                first_carrier: 0, carrier_count: 1, ..Default::default() })?,
        }));
    }
    println!("{}", serde_json::to_string(&references)?);
    Ok(())
}
