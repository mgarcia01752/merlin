/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent native-process reference for installed Python idle-TX parity.
use docsis_ofdm_fec::api::*;
use serde_json::json;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let transmission = TransmissionConfiguration::new(TransmissionSettings::default())?;
    let input = NamedCapture::new(
        "profile",
        include_bytes!("../../../examples/measurements/single-channel/modulation_profile.json")
            .to_vec(),
    )?;
    let profile = Dpd::from_bytes(&input, ProfileId::new(0)?)?;
    let source = SourceInput::prbs15(PrbsSeed::new(32767)?);
    let request = TransmitterPreviewRequest::new(&transmission, &profile, &source)?;
    let control = TransmitterPreviewControl::new();
    let result = request.generate(&control)?;
    println!(
        "{}",
        serde_json::to_string(
            &json!({"info":result.info(),"samples":result.sample_view(0,8)?,"spectrum":result.spectrum(100,16)?,"timing":transmission.symbol_timing(Some(0.82))?,"progress":control.snapshot()})
        )?
    );
    Ok(())
}
