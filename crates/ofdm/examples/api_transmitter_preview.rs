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
    // The electrical reference does not apply this deliberately nonzero amplifier gain.
    const UNUSED_AMPLIFIER_GAIN_DB: f64 = 20.0;
    const INPUT_LEVEL_DBMV: f64 = 0.0;
    const CALIBRATED_VIEW_FIRST_BIN: usize = 350;
    const CALIBRATED_VIEW_BIN_COUNT: usize = 7;
    let channel = ChannelConfiguration::new(ChannelConfigurationInput {
        power: PowerConditioning {
            amplifier_gain_db: UNUSED_AMPLIFIER_GAIN_DB,
            input_data_level_dbmv_per_6mhz: Some(INPUT_LEVEL_DBMV),
            ..Default::default()
        },
        ..Default::default()
    })?;
    let calibrated = result.calibrated_spectrum(
        &channel,
        CALIBRATED_VIEW_FIRST_BIN,
        CALIBRATED_VIEW_BIN_COUNT,
    )?;
    println!(
        "{}",
        serde_json::to_string(
            &json!({"info":result.info(),"samples":result.sample_view(0,8)?,"spectrum":result.spectrum(100,16)?,"timing":transmission.symbol_timing(Some(0.82))?,"progress":control.snapshot(),"calibrated":calibrated})
        )?
    );
    Ok(())
}
