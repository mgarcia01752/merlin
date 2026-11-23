/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Separate public Rust process for synthetic-unity and existing Designer-limit parity.
use docsis_ofdm_fec::api::{
    ChannelResponseMetric, MeasurementViewport, NamedCapture, ProfileCapture,
};
use pnm_input::RxMerDesign;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let original = NamedCapture::new(
        "source",
        include_bytes!("../../../examples/measurements/single-channel/modulation_profile.json")
            .to_vec(),
    )?;
    let capture = ProfileCapture::from_bytes(&original)?;
    let mut views = serde_json::Map::new();
    for (name, metric) in [
        ("gain_db", ChannelResponseMetric::GainDb),
        ("phase_degrees", ChannelResponseMetric::PhaseDegrees),
        ("real", ChannelResponseMetric::Real),
        ("imaginary", ChannelResponseMetric::Imaginary),
    ] {
        views.insert(
            name.into(),
            serde_json::to_value(capture.unity_response_preview(
                0,
                MeasurementViewport::default(),
                metric,
            )?)?,
        );
    }
    println!(
        "{}",
        serde_json::json!({"unity":views,"limits":RxMerDesign::limits()})
    );
    Ok(())
}
