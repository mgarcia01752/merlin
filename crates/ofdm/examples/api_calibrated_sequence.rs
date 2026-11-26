/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded separate-process reference for calibrated ordered-frame API parity.
use docsis_ofdm_fec::api::*;
use serde_json::{json, Value};
// Existing clean deterministic fixture, independent of application defaults.
const CLEAN_REFERENCE_MER_DB: f64 = 80.0;
const CAPTURED_CARRIER_COUNT: usize = 7480;
const REFERENCE_CAPTURE_TIME_SECONDS: u32 = 1_466_967;
const SOURCE_PRBS15_SEED: u16 = 32_767;
const MEASUREMENT_INTERVAL_SYMBOLS: u64 = 1000;
const OBSERVATION_DURATION_MS: u64 = 1;
const MAX_DECODE_ITERATIONS: usize = 10;
const MAX_HOST_RUNTIME_MS: u64 = 60_000;
// Three retained frames and three analysis averages are separate fixture policies.
const RETAINED_FRAMES: usize = 3;
const QUERY_AVERAGES: usize = 3;
const QUERY_START: usize = 123;
const QUERY_COUNT: usize = 7;
const STAGES: &[FullLoopStage] = &[
    FullLoopStage::TxIfft,
    FullLoopStage::TxPrefix,
    FullLoopStage::TxWindow,
    FullLoopStage::RxUseful,
];

fn observed(electrical: bool) -> Result<FullLoopResult, Box<dyn std::error::Error>> {
    let profile = Dpd::from_bytes(
        &NamedCapture::new(
            "profile",
            include_bytes!("../../../examples/measurements/single-channel/modulation_profile.json")
                .to_vec(),
        )?,
        ProfileId::new(0)?,
    )?;
    let transmission = TransmissionConfiguration::new(TransmissionSettings::default())?;
    let mut capture: Value = serde_json::from_slice(include_bytes!(
        "../../../examples/measurements/single-channel/rxmer.json"
    ))?;
    capture["values"] = json!(vec![CLEAN_REFERENCE_MER_DB; CAPTURED_CARRIER_COUNT]);
    capture["pnm_header"]["capture_time"] = json!(REFERENCE_CAPTURE_TIME_SECONDS);
    let measured = RxMer::from_bytes(&NamedCapture::new("clean", serde_json::to_vec(&capture)?)?)?;
    let measurements = MeasurementSequence::new(
        &profile,
        &transmission,
        vec![measured],
        MeasurementSequenceOptions {
            mode: MeasurementMode::Single,
            interval_symbols: MEASUREMENT_INTERVAL_SYMBOLS,
            tolerance: AssociationTolerance::ZERO,
        },
    )?;
    let request = Simulation::prepare_full_loop(
        FullLoopInputs {
            transmission,
            profile,
            source: SourceInput::prbs15(PrbsSeed::new(SOURCE_PRBS15_SEED)?),
            measurements,
            channel: ChannelConfiguration::new(ChannelConfigurationInput {
                power: PowerConditioning {
                    input_data_level_dbmv_per_6mhz: electrical.then_some(0.0),
                    ..Default::default()
                },
                ..Default::default()
            })?,
            response: None,
        },
        FullLoopOptions {
            duration_ms: OBSERVATION_DURATION_MS,
            max_iterations: MAX_DECODE_ITERATIONS,
            max_runtime_ms: MAX_HOST_RUNTIME_MS,
            ..Default::default()
        },
    )?;
    let mut policy = FullLoopObservationInput::default();
    for entry in &mut policy.stages {
        if STAGES.contains(&entry.stage) {
            entry.max_events = RETAINED_FRAMES;
        }
    }
    Ok(request
        .with_observations(FullLoopObservationPolicy::new(policy)?)?
        .run(&FullLoopControl::new())?)
}

/// Prints two bounded actual-run references using unchanged deterministic source inputs.
/// # Returns
/// JSON containing normalized metadata and calibrated power for four accepted frame stages.
/// # Errors
/// Propagates input, bounded execution, observation, calibration and output failures.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut cases = Vec::new();
    for electrical in [false, true] {
        let result = observed(electrical)?;
        let spectra = STAGES
            .iter()
            .map(|stage| {
                result.calibrated_spectrum_sequence(FullLoopSpectrumQuery {
                    stage: *stage,
                    max_averages: QUERY_AVERAGES,
                    start: QUERY_START,
                    count: QUERY_COUNT,
                    ..Default::default()
                })
            })
            .collect::<ApiResult<Vec<_>>>()?;
        cases.push(json!({"electrical":electrical,"spectra":spectra,
            "comparison":result.report().summary.comparison}));
    }
    println!("{}", serde_json::to_string(&cases)?);
    Ok(())
}
