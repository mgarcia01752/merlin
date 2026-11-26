/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent Rust process supplying exact clean full-loop native API reference outcomes.
use docsis_ofdm_fec::api::*;
use serde_json::{json, Value};
// Deterministic scientific fixture choices; these do not change application defaults.
const CLEAN_REFERENCE_MER_DB: f64 = 80.0;
const CAPTURED_CARRIER_COUNT: usize = 7480;
const REFERENCE_CAPTURE_TIME_SECONDS: u32 = 1_466_967;
const SOURCE_PRBS15_SEED: u16 = 32_767;
const MEASUREMENT_INTERVAL_SYMBOLS: u64 = 1000;
const OBSERVATION_DURATION_MS: u64 = 1;
const MAX_DECODE_ITERATIONS: usize = 10;
const MAX_HOST_RUNTIME_MS: u64 = 60_000;

// Independent view choices: equal values here do not imply shared policies.
const FRAME_EVENTS_PER_STAGE: usize = 3;
const FRAME_SEQUENCE_WELCH_AVERAGES: usize = 3;
const SOURCE_BIT_VIEW_COUNT: usize = 32;
const RAW_PHYSICAL_VIEW_COUNT: usize = 16;
const PILOT_VIEW_COUNT: usize = 5;
const WAVEFORM_VIEW_START: usize = 7;
const WAVEFORM_VIEW_COUNT: usize = 4;
const SPECTRUM_VIEW_START: usize = 123;
const SPECTRUM_VIEW_COUNT: usize = 7;
const QAM_VIEW_COUNT: usize = 5;
const DECISION_ROW_COUNT: usize = 3;
const SOFT_CELL_VIEW_COUNT: usize = 3;
const REFERENCE_CONSTELLATION_ORDER: u16 = 128;
const REFERENCE_AMPLITUDE: f64 = 2.0;
const REFERENCE_PAGE_START: usize = 3;
const REFERENCE_PAGE_COUNT: usize = 5;
const PHYSICAL_VIEW_SYMBOL: u64 = 1;
const PHYSICAL_VIEW_START: usize = 350;
const PHYSICAL_VIEW_COUNT: usize = 10;
const TRANSMIT_CELL_VIEW_START: usize = 7;
const TRANSMIT_CELL_VIEW_COUNT: usize = 13;

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
            channel: ChannelConfiguration::new(ChannelConfigurationInput::default())?,
            response: None,
        },
        FullLoopOptions {
            duration_ms: OBSERVATION_DURATION_MS,
            max_iterations: MAX_DECODE_ITERATIONS,
            max_runtime_ms: MAX_HOST_RUNTIME_MS,
            ..Default::default()
        },
    )?;
    let output = request.run(&FullLoopControl::new())?;
    let report = output.report();
    let mut observation_input = FullLoopObservationInput::default();
    for retention in &mut observation_input.stages {
        if matches!(
            retention.stage,
            FullLoopStage::TxIfft
                | FullLoopStage::TxPrefix
                | FullLoopStage::TxWindow
                | FullLoopStage::RxUseful
        ) {
            retention.max_events = FRAME_EVENTS_PER_STAGE;
        }
    }
    let observed = request
        .with_observations(FullLoopObservationPolicy::new(observation_input)?)?
        .run(&FullLoopControl::new())?;
    let stages = json!({"source":observed.stage(FullLoopStage::SourcePayload,0,FullLoopStageViewport{start:0,count:SOURCE_BIT_VIEW_COUNT})?,"physical":observed.stage(FullLoopStage::PhysicalBins,0,FullLoopStageViewport{start:0,count:RAW_PHYSICAL_VIEW_COUNT})?,"pilots":observed.stage(FullLoopStage::ContinuousPilots,0,FullLoopStageViewport{start:0,count:PILOT_VIEW_COUNT})?,"waveform":observed.stage(FullLoopStage::TxOverlap,0,FullLoopStageViewport{start:WAVEFORM_VIEW_START,count:WAVEFORM_VIEW_COUNT})?});
    let instruments = json!({
        "spectrum": observed.spectrum(FullLoopSpectrumQuery {
            start: SPECTRUM_VIEW_START, count: SPECTRUM_VIEW_COUNT, ..Default::default()
        })?,
        "response": observed.response(FullLoopResponseQuery::default())?,
        "llr": observed.llr_statistics(0)?,
    });
    let archived = observed.receiver_archive(FullLoopReceiverArchivePolicy::new(
        FullLoopReceiverArchiveInput::default(),
    )?)?;
    let archive = json!({"info":archived.info(),"qam":archived.qam(FullLoopCellQuery {count:QAM_VIEW_COUNT,..Default::default()})?,"decisions":archived.cell_decisions(FullLoopCellQuery {count:DECISION_ROW_COUNT,..Default::default()})?,"cell_soft":archived.cell_soft_view(FullLoopCellQuery {count:SOFT_CELL_VIEW_COUNT,..Default::default()})?,"decoder_soft":archived.decoder_soft_view()?});
    let cells = json!({"qam":observed.qam(FullLoopCellQuery {count:QAM_VIEW_COUNT,..Default::default()})?,
        "decisions":observed.cell_decisions(FullLoopCellQuery {count:DECISION_ROW_COUNT,..Default::default()})?});
    let slicer = json!({"reference":Simulation::constellation_reference(REFERENCE_CONSTELLATION_ORDER,REFERENCE_AMPLITUDE,REFERENCE_PAGE_START,REFERENCE_PAGE_COUNT)?,
        "region":Simulation::decision_region(REFERENCE_CONSTELLATION_ORDER,0,1.0)?,
        "cells":observed.cell_soft_view(FullLoopCellQuery {count:SOFT_CELL_VIEW_COUNT,..Default::default()})?,
        "decoder":observed.decoder_soft_view(0)?});
    let physical = json!({
        "layout": observed.inputs().transmission.physical_layout(&observed.inputs().profile, PHYSICAL_VIEW_SYMBOL, PHYSICAL_VIEW_START, PHYSICAL_VIEW_COUNT)?,
        "tx": observed.physical_bins(FullLoopStage::PhysicalBins, 0, PHYSICAL_VIEW_START, PHYSICAL_VIEW_COUNT)?,
        "rx": observed.physical_bins(FullLoopStage::RxFft, 0, PHYSICAL_VIEW_START, PHYSICAL_VIEW_COUNT)?,
        "mapped": observed.transmit_cells(FullLoopStage::PayloadMapped, 0, TRANSMIT_CELL_VIEW_START, TRANSMIT_CELL_VIEW_COUNT)?,
        "interleaved": observed.transmit_cells(FullLoopStage::PayloadInterleaved, 0, TRANSMIT_CELL_VIEW_START, TRANSMIT_CELL_VIEW_COUNT)?,
    });
    let sequence_spectra = [
        FullLoopStage::TxIfft,
        FullLoopStage::TxPrefix,
        FullLoopStage::TxWindow,
        FullLoopStage::RxUseful,
    ]
    .into_iter()
    .map(|stage| {
        observed.spectrum_sequence(FullLoopSpectrumQuery {
            stage,
            max_averages: FRAME_SEQUENCE_WELCH_AVERAGES,
            start: SPECTRUM_VIEW_START,
            count: SPECTRUM_VIEW_COUNT,
            ..Default::default()
        })
    })
    .collect::<Result<Vec<_>, _>>()?;
    // Separate channel fixture: twenty-dB amplifier and six-dB CPA are distinct owners.
    const AMPLIFIER_GAIN_DB: f64 = 20.0;
    const CPA_ATTENUATION_DB: f64 = 6.0;
    const INPUT_LEVEL_DBMV: f64 = 0.0;
    let mut channel_inputs = observed.inputs().clone();
    channel_inputs.channel = ChannelConfiguration::new(ChannelConfigurationInput {
        power: PowerConditioning {
            amplifier_gain_db: AMPLIFIER_GAIN_DB,
            cpa_attenuation_db: CPA_ATTENUATION_DB,
            input_data_level_dbmv_per_6mhz: Some(INPUT_LEVEL_DBMV),
            ..Default::default()
        },
        ..Default::default()
    })?;
    let channel_result = Simulation::prepare_full_loop(channel_inputs, request.options().clone())?
        .with_observations(FullLoopObservationPolicy::new(
            FullLoopObservationInput::default(),
        )?)?
        .run(&FullLoopControl::new())?;
    let channel_query = FullLoopSpectrumQuery {
        start: PHYSICAL_VIEW_START,
        count: SPECTRUM_VIEW_COUNT,
        ..Default::default()
    };
    let channel_spectra = json!({
        "before": channel_result.channel_spectrum(FullLoopChannelTap::BeforeAmplifier, channel_query.clone())?,
        "after": channel_result.channel_spectrum(FullLoopChannelTap::AfterAmplifier, channel_query.clone())?,
        "receiver": channel_result.channel_spectrum(FullLoopChannelTap::ReceiverInput, FullLoopSpectrumQuery { stage: FullLoopStage::ChannelOutput, ..channel_query.clone() })?,
        "calibrated": channel_result.calibrated_spectrum(channel_query)?,
        "bins_before": channel_result.amplifier_bins(AmplifierBoundary::Before, 0, PHYSICAL_VIEW_START, PHYSICAL_VIEW_COUNT)?,
        "bins_after": channel_result.amplifier_bins(AmplifierBoundary::After, 0, PHYSICAL_VIEW_START, PHYSICAL_VIEW_COUNT)?,
    });
    // Explicit captured identities share loading here; identity is independent of array position.
    const SWEEP_SECOND_PROFILE_ID: u8 = 7;
    const SWEEP_OTHER_CATALOG_ID: u8 = 2;
    let mut catalog_document: Value = serde_json::from_slice(include_bytes!(
        "../../../examples/measurements/single-channel/modulation_profile.json"
    ))?;
    let template = catalog_document["profiles"]
        .as_array()
        .unwrap()
        .iter()
        .find(|profile| profile["profile_id"] == 0)
        .unwrap()
        .clone();
    let source_ids = [SWEEP_SECOND_PROFILE_ID, SWEEP_OTHER_CATALOG_ID, 0];
    catalog_document["profiles"] = Value::Array(
        source_ids
            .iter()
            .map(|id| {
                let mut profile = template.clone();
                profile["profile_id"] = json!(id);
                profile
            })
            .collect(),
    );
    catalog_document["num_profiles"] = json!(source_ids.len());
    let catalog = ProfileCapture::from_bytes(&NamedCapture::new(
        "sweep-catalog",
        serde_json::to_vec(&catalog_document)?,
    )?)?;
    let sweep_request = request.with_observations(FullLoopObservationPolicy::new(
        FullLoopObservationInput::default(),
    )?)?;
    let plan = FullLoopSweepPlan::new(
        &sweep_request,
        &catalog,
        FullLoopProfileSelection::Selected(vec![
            ProfileId::new(SWEEP_SECOND_PROFILE_ID)?,
            ProfileId::new(0)?,
        ]),
    )?;
    let mut sweep_results = Vec::new();
    for index in 0..plan.info().selected_profile_ids.len() {
        let output = plan.request(index)?.run(&FullLoopControl::new())?;
        sweep_results.push(json!({"profile_id":output.inputs().profile.profile_id(),
            "status":output.report().status,"summary":output.report().summary}));
    }
    let sweep = json!({"info":plan.info(),"results":sweep_results});
    println!(
        "{}",
        serde_json::to_string(
            &json!({"status":report.status,"summary":report.summary,"recovery":output.fec_recovery(),"words":report.retained_words,"settings":request.settings(),"inputs":output.inputs().summary()?,"stages":stages,"instruments":instruments,"messages":observed.plc_messages()?,"archive":archive,"cells":cells,"slicer":slicer,"physical":physical,"sequence_spectra":sequence_spectra,"channel_spectra":channel_spectra,"sweep":sweep})
        )?
    );
    Ok(())
}
