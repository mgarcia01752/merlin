/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent Rust process supplying exact clean full-loop native API reference outcomes.
use docsis_ofdm_fec::api::*;
use serde_json::{json, Value};
fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    capture["values"] = json!(vec![80.0; 7480]);
    capture["pnm_header"]["capture_time"] = json!(1466967);
    let measured = RxMer::from_bytes(&NamedCapture::new("clean", serde_json::to_vec(&capture)?)?)?;
    let measurements = MeasurementSequence::new(
        &profile,
        &transmission,
        vec![measured],
        MeasurementSequenceOptions {
            mode: MeasurementMode::Single,
            interval_symbols: 1000,
            tolerance: AssociationTolerance::ZERO,
        },
    )?;
    let request = Simulation::prepare_full_loop(
        FullLoopInputs {
            transmission,
            profile,
            source: SourceInput::prbs15(PrbsSeed::new(32767)?),
            measurements,
            channel: ChannelConfiguration::new(ChannelConfigurationInput::default())?,
            response: None,
        },
        FullLoopOptions {
            duration_ms: 1,
            max_iterations: 10,
            max_runtime_ms: 60000,
            ..Default::default()
        },
    )?;
    let output = request.run(&FullLoopControl::new())?;
    let report = output.report();
    let observed = request
        .with_observations(FullLoopObservationPolicy::new(
            FullLoopObservationInput::default(),
        )?)?
        .run(&FullLoopControl::new())?;
    let stages = json!({"source":observed.stage(FullLoopStage::SourcePayload,0,FullLoopStageViewport{start:0,count:32})?,"physical":observed.stage(FullLoopStage::PhysicalBins,0,FullLoopStageViewport{start:0,count:16})?,"pilots":observed.stage(FullLoopStage::ContinuousPilots,0,FullLoopStageViewport{start:0,count:5})?,"waveform":observed.stage(FullLoopStage::TxOverlap,0,FullLoopStageViewport{start:7,count:4})?});
    let instruments = json!({
        "spectrum": observed.spectrum(FullLoopSpectrumQuery {
            start: 123, count: 7, ..Default::default()
        })?,
        "response": observed.response(FullLoopResponseQuery::default())?,
        "llr": observed.llr_statistics(0)?,
    });
    let cells = json!({"qam":observed.qam(FullLoopCellQuery {count:5,..Default::default()})?,
        "decisions":observed.cell_decisions(FullLoopCellQuery {count:3,..Default::default()})?});
    println!(
        "{}",
        serde_json::to_string(
            &json!({"status":report.status,"summary":report.summary,"words":report.retained_words,"settings":request.settings(),"inputs":output.inputs().summary()?,"stages":stages,"instruments":instruments,"messages":observed.plc_messages()?,"cells":cells})
        )?
    );
    Ok(())
}
