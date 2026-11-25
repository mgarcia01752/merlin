/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Development-only deterministic temporal demo; never a production acquisition path.
use std::{io, path::PathBuf};
const CAPTURE_TIME_OFFSET: usize = 6;
const SPECTRUM_HEADER_BYTES: usize = 39;
const FRAME_COUNT: usize = 12;
const DEMO_START_EPOCH: pnm::pnm::data_type::CaptureTime = 1781027644;
/// Writes synthetic standard-format frames into a new, explicitly selected folder.
///
/// Existing directories are refused. The original sanitized fixture is retained;
/// only timestamp and synthetic amplitude variations change in derived captures.
///
/// # Errors
/// Returns I/O or missing-argument errors without changing the fixture source.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let output = PathBuf::from(std::env::args_os().nth(1).ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "Supply a new output directory")
    })?);
    std::fs::create_dir(&output)?;
    for frame in 0..FRAME_COUNT {
        let mut bytes = pnm_fixtures::SPECTRUM.to_vec();
        bytes[CAPTURE_TIME_OFFSET..CAPTURE_TIME_OFFSET + 4]
            .copy_from_slice(&(DEMO_START_EPOCH + frame as u32 * 5).to_be_bytes());
        for (index, amplitude) in bytes[SPECTRUM_HEADER_BYTES..]
            .chunks_exact_mut(2)
            .enumerate()
        {
            let base = i32::from(i16::from_be_bytes([amplitude[0], amplitude[1]]));
            let center = 3500 + frame * 650;
            let distance = index.abs_diff(center);
            let peak = if distance < 450 {
                ((450 - distance) * 900 / 450) as i32
            } else {
                0
            };
            let delta = (frame as i32 - 5) * 18 + peak;
            let value = (base + delta).clamp(i32::from(i16::MIN), i32::from(i16::MAX)) as i16;
            amplitude.copy_from_slice(&value.to_be_bytes());
        }
        std::fs::write(output.join(format!("capture-{:03}.bin", frame + 1)), bytes)?;
    }
    std::fs::write(output.join("README.md"),"# Synthetic standard-spectrum time series\n\nTwelve deterministic standard PNN9 frames derived from the sanitized PyPNM\nspectrum fixture. Timestamps and levels are SYNTHETIC, including a moving peak.\nThese are UI/math demonstration data, not real temporal modem measurements.\n\nOpen from Spectrum Analyzer → Min-Avg-Max or Playback. All acquisition settings\nremain identical. MAC: 00:ca:00:00:00:01. Times advance by five seconds.\n\nReproduce with the generate_spectrum_demo Rust example and a new output folder.\nThe upstream fixture LICENSE/NOTICE/provenance are included.\n")?;
    Ok(())
}
