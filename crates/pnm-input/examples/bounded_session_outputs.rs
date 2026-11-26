/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Direct Rust-process target bounds and actual scientific views for installed Python.
use docsis_ofdm_fec::api as downstream;
use docsis_ofdma::api as upstream;
use pnm_input::Workbench;
use serde_json::{json, Value};

/// Small observed-frame fixture policy, separate from decoder work limits.
const ESTIMATE_FRAMES: usize = 2;
/// Same finite observation payload as the native reference fixture.
const ESTIMATE_SCALARS: usize = 16_384;
/// Retained physical soft cells, unrelated to observation frames.
const SOFT_CELLS: usize = 8;
/// Retained decoder words, unrelated to physical cell count.
const SOFT_WORDS: usize = 2;
/// Retained scalar LLR ceiling for each decoder word.
const SOFT_WORD_LLRS: usize = 32_768;
/// Small sample prefix for process parity, not the result population.
const SAMPLE_PREFIX: usize = 3;
/// Explicit valid continuous PRBS15 recurrence seed.
const PREVIEW_SEED: u16 = 7;
/// Literal source bytes, shared with the native test without conversion.
const FILE_BYTES: &[u8] = b"\xef\xbb\xbfbounded\0source\r\n";
const PROFILE: &[u8] =
    include_bytes!("../../../examples/measurements/single-channel/modulation_profile.json");

/// Emits all three real bounded output paths without an application allocator estimate.
///
/// # Returns
/// JSON for two downstream projects and upstream numerologies, observation/
/// cancellation modes, plus independent PRBS/file idle previews.
///
/// # Errors
/// Preserves public preparation, execution, query and serialization errors.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut ds = Vec::new();
    for (name, bytes) in [
        (
            "single",
            include_bytes!("../../../examples/workbench/downstream/single/workbench.merlin.json")
                .as_slice(),
        ),
        (
            "multi-rxmer",
            include_bytes!(
                "../../../examples/workbench/downstream/multi-rxmer/workbench.merlin.json"
            )
            .as_slice(),
        ),
    ] {
        for observed in [false, true] {
            for cancelled in [false, true] {
                let request = Workbench::prepare_full_loop(bytes)?.request(0)?;
                let request = if observed {
                    request.with_observations(downstream::FullLoopObservationPolicy::new(
                        Default::default(),
                    )?)?
                } else {
                    request
                };
                let bound = request.result_retained_bound()?;
                let control = downstream::FullLoopControl::new();
                if cancelled {
                    control.cancel();
                }
                let result = request.run_bounded(&control, bound)?;
                ds.push(
                    json!({"project":name,"observed":observed,"cancelled":cancelled,
                    "bound":bound,"actual":result.retained_footprint()?,"report":result.report(),
                    "inputs":result.inputs().summary()?,"recovery":result.fec_recovery(),
                    "inventory":result.stage_inventory(),"stats":result.observation_stats()}),
                );
            }
        }
    }
    let mut us = Vec::new();
    for (spacing, bytes) in [
        (
            50,
            include_bytes!("../../../examples/cli/upstream/us-50khz.json").as_slice(),
        ),
        (
            25,
            include_bytes!("../../../examples/cli/upstream/us-25khz.json").as_slice(),
        ),
    ] {
        for observed in [false, true] {
            for cancelled in [false, true] {
                let mut simulation = upstream::Simulation::new(Default::default())?;
                simulation.configure_json(bytes)?;
                if observed {
                    simulation.set_receiver_observation_policy(
                        upstream::ReceiverObservationPolicy::new(
                            ESTIMATE_FRAMES,
                            ESTIMATE_SCALARS,
                        )?
                        .with_soft_metrics(
                            upstream::SoftObservationPolicy::new(
                                SOFT_CELLS,
                                SOFT_WORDS,
                                SOFT_WORD_LLRS,
                            )?,
                        ),
                    )?;
                }
                let bound = simulation.batch_report_retained_bound()?;
                let control = upstream::RunControl::new();
                if cancelled {
                    control.cancel();
                }
                let report = simulation.run_with_control_bounded(&control, bound)?;
                us.push(
                    json!({"spacing":spacing,"observed":observed,"cancelled":cancelled,
                    "bound":bound,"actual":report.retained_footprint()?,"report":report}),
                );
            }
        }
    }
    let mut tx = Vec::new();
    for file in [false, true] {
        let capture = downstream::ProfileCapture::from_bytes(&downstream::NamedCapture::new(
            "profile",
            PROFILE.to_vec(),
        )?)?;
        let source = if file {
            downstream::SourceInput::file(
                downstream::NamedCapture::new("source", FILE_BYTES.to_vec())?,
                None,
            )?
        } else {
            downstream::SourceInput::prbs15(downstream::PrbsSeed::new(PREVIEW_SEED)?)
        };
        let request = downstream::TransmitterPreviewRequest::from_capture(
            &downstream::TransmissionConfiguration::new(Default::default())?,
            &capture,
            0,
            &source,
        )?;
        let bound = request.preview_retained_bound()?;
        let preview =
            request.generate_bounded(&downstream::TransmitterPreviewControl::new(), bound)?;
        tx.push(json!({"file":file,"bound":bound,"actual":preview.retained_footprint()?,
            "info":preview.info(),"samples":preview.sample_view(0,SAMPLE_PREFIX)?,"spectrum":preview.spectrum(0,0)?}));
    }
    let output: Value = json!({"downstream":ds,"upstream":us,"transmitter":tx});
    println!("{}", serde_json::to_string(&output)?);
    Ok(())
}
