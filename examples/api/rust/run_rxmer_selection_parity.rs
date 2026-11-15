/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! File: examples/api/rust/run_rxmer_selection_parity.rs
//! Produces actual Rust policy results for independently executed Python parity tests.
use docsis_ofdm_fec::api::*;
use pnm_input::{read_capture, save_json, CaptureLoader};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = std::path::PathBuf::from(
        std::env::args()
            .nth(1)
            .ok_or("Supply a new parity output directory")?,
    );
    let receiver = match std::env::args()
        .nth(2)
        .as_deref()
        .unwrap_or("supplied_channel")
    {
        "supplied_channel" => ReceiverMode::SuppliedChannel,
        "pilot_estimated" => ReceiverMode::PilotEstimated,
        "merlin_estimated" => ReceiverMode::MerlinChannelEstimated,
        _ => {
            return Err(
                "Receiver mode must be supplied_channel, pilot_estimated or merlin_estimated"
                    .into(),
            )
        }
    };
    std::fs::create_dir(&output)?;
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/measurements/single-channel");
    let mut rx: serde_json::Value =
        serde_json::from_slice(&std::fs::read(root.join("rxmer.json"))?)?;
    let epoch = rx["pnm_header"]["capture_time"]
        .as_u64()
        .ok_or("Missing fixture epoch")? as u32;
    for (name, time, mer) in [
        ("first.json", epoch, 40.0),
        ("second.json", epoch + 10, 42.0),
    ] {
        rx["pnm_header"]["capture_time"] = time.into();
        for value in rx["values"]
            .as_array_mut()
            .ok_or("Missing fixture samples")?
        {
            *value = mer.into();
        }
        save_json(output.join(name), &serde_json::to_string(&rx)?)?;
    }
    let dpd = CaptureLoader::profile(
        &read_capture(root.join("modulation_profile.json"))?,
        ProfileId::PROFILE_A,
    )?;
    let ocd = Ocd::from_profile(ProtocolScope::Docsis31, &dpd, OcdOverrides::default())?;
    let mut sim = Merlin::new(ocd);
    sim.set_profile(dpd)?;
    let first = sim.add_rxmer(CaptureLoader::rxmer(&read_capture(
        output.join("first.json"),
    )?)?)?;
    sim.add_rxmer(CaptureLoader::rxmer(&read_capture(
        output.join("second.json"),
    )?)?)?;
    let channel = sim.add_channel_estimate(CaptureLoader::channel_estimate(&read_capture(
        root.join("channel_estimation.json"),
    )?)?)?;
    let mut results = serde_json::Map::new();
    for (name, selection) in [
        ("each", RxMerSelection::Each),
        ("individual", RxMerSelection::Individual(first)),
        ("average", RxMerSelection::AverageAll),
        (
            "interpolate",
            RxMerSelection::InterpolateAt(CaptureTime::new(epoch + 5)?),
        ),
    ] {
        let mut options = EvaluationOptions::default()
            .with_rxmer_selection(selection)
            .with_receiver_mode(receiver)
            .with_channel_selection(ChannelSelection::Explicit(channel))
            .with_profile_tolerance(AssociationTolerance::new(u32::MAX)?);
        options.observation = ObservationDuration::new(1)?;
        options.limits.runtime = RuntimeLimit::new(300_000)?;
        sim.set_options(options)?;
        let result = sim.run()?;
        if result.status() != BatchStatus::Complete {
            return Err(format!("Parity baseline {name} did not complete").into());
        }
        results.insert(name.into(), serde_json::from_str(&result.to_json()?)?);
    }
    save_json(
        output.join("rust-results.json"),
        &serde_json::to_string(&results)?,
    )?;
    println!("Rust parity baseline: {}", output.display());
    Ok(())
}
