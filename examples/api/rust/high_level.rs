/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Actual binary fixture workflow; run from the repository root.
use docsis_ofdm_fec::api::*;
use pnm_input::{read_capture, CaptureLoader};
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../examples/workbench/single");
    let dpd = CaptureLoader::profile(
        &read_capture(root.join("modulation-profile.bin"))?,
        ProfileId::PROFILE_A,
    )?;
    let ocd = Ocd::from_profile(
        ProtocolScope::Docsis31,
        &dpd,
        OcdOverrides {
            bandwidth: Some(BandwidthHz::new(192_000_000)?),
            ..Default::default()
        },
    )?;
    let mut sim = Merlin::new(ocd);
    sim.set_profile(dpd)?;
    let first = sim.add_rxmer(CaptureLoader::rxmer(&read_capture(
        root.join("rxmer.bin"),
    )?)?)?;
    let channel = sim.add_channel_estimate(CaptureLoader::channel_estimate(&read_capture(
        root.join("ds-channel-estimate.bin"),
    )?)?)?;
    let mut options = EvaluationOptions::default()
        .with_rxmer_selection(RxMerSelection::Individual(first))
        .with_channel_selection(ChannelSelection::Explicit(channel))
        .with_profile_tolerance(AssociationTolerance::new(90_000)?);
    options.observation = ObservationDuration::new(1)?;
    options.limits.runtime = RuntimeLimit::new(300_000)?;
    sim.set_options(options)?;
    for entry in sim.run()?.iter() {
        println!("{:?}", entry.status());
        if let Some(report) = entry.report() {
            println!("{}", report.to_json()?);
        }
    }
    Ok(())
}
