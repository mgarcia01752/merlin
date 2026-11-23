/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded separate-process reference for passive sample-budget capacity in both numerologies.
use docsis_ofdm_fec::api::{SubcarrierSpacing, TransmissionConfiguration, TransmissionSettings};
use serde_json::json;

/// Prints independently chosen budget cases through the public Rust geometry operation.
/// # Returns
/// Process success after bounded JSON output; no capture, source, FFT or job is created.
/// # Errors
/// Propagates configuration or transport failures; expected budget errors are output data.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Independent FFT/half-overlap and existing UI-window boundaries, not production constants.
    let budgets = [
        0,
        1,
        4095,
        4096,
        6143,
        6144,
        8191,
        8192,
        12287,
        12288,
        32768,
        67584,
        135168,
        usize::MAX,
    ];
    let mut records = Vec::new();
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let configuration = TransmissionConfiguration::new(TransmissionSettings {
            subcarrier_spacing: spacing,
            ..Default::default()
        })?;
        let cases = budgets.into_iter().map(|budget| {
            match configuration.spectrum_average_capacity(budget) {
                Ok(capacity) => json!({"max_samples":budget,"capacity":capacity}),
                Err(error) => json!({"max_samples":budget,"error_code":error.code(),"engine_code":error.engine_code()}),
            }
        }).collect::<Vec<_>>();
        records.push(json!({"spacing":spacing.frequency_hz().to_string(),"cases":cases}));
    }
    println!("{}", serde_json::to_string_pretty(&records)?);
    Ok(())
}
