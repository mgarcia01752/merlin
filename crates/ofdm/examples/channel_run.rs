/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Duration-based actual waveform stepping; synthetic24MHz50kHz layout.
use docsis_ofdm_fec::{
    simulation::{
        channel::{ChannelRun, ChannelRunSettings},
        test_points::{ProbeEvent, ProbeObserver, TestPointProbe},
    },
    transmission::{layout::CarrierLayoutSettings, SubcarrierSpacing, TransmissionSettings},
    LabError,
};
use std::time::Duration;
struct UnusedObserver;
impl ProbeObserver for UnusedObserver {
    /// Accepts no events when the example's probe is disabled.
    /// # Arguments
    /// * `_event` - Immutable optional stage event.
    /// # Returns
    /// Unit; diagnostics and scientific output remain separate.
    fn observe(&mut self, _event: &ProbeEvent<'_>) -> Result<(), LabError> {
        Ok(())
    }
}
/// Streams one frozen channel, optionally selecting observation milliseconds.
/// # Returns
/// A JSON summary; default observation is1000ms, independent of host elapsed time.
/// # Errors
/// Propagates argument, configuration, scientific stage and JSON failures.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let duration_ms = std::env::args()
        .nth(1)
        .map(|value| value.parse::<u64>())
        .transpose()?;
    let settings = TransmissionSettings {
        occupied_bandwidth_hz: 24_000_000,
        subcarrier_spacing: SubcarrierSpacing::Khz50,
        time_interleaver_depth: 32,
        ..Default::default()
    };
    let layout = CarrierLayoutSettings {
        zero_frequency_hz: 100_000_000,
        first_active_bin: 160,
        active_end_exclusive: 640,
        plc_first_bin: 376,
        excluded_bins: vec![600],
        additional_continuous_pilots: vec![176, 224, 272, 460, 504, 540, 588, 628],
    };
    let mut settings = ChannelRunSettings::new(settings, layout);
    if let Some(duration_ms) = duration_ms {
        settings.duration = Duration::from_millis(duration_ms);
    }
    let mut run = ChannelRun::new(&settings)?;
    let mut probe = TestPointProbe::disabled();
    while run.advance(&mut probe, &mut UnusedObserver)?.is_some() {}
    println!("{}", serde_json::to_string_pretty(run.summary())?);
    Ok(())
}
