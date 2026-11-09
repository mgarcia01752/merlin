/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Offline, exact RxMER fixture export; not a production capture input adapter.
use docsis_ofdm_fec::scenario::document::ScenarioDocument;
use std::{error::Error, io::Write, path::Path};

const HEADER_BYTES: usize = 28;
const CAPTURE_TIME_OFFSET: usize = 6;
const CHANNEL_OFFSET: usize = 10;
const MAC_OFFSET: usize = 11;
const ZERO_FREQUENCY_OFFSET: usize = 17;
const FIRST_CARRIER_OFFSET: usize = 21;
const SPACING_OFFSET: usize = 23;
const LENGTH_OFFSET: usize = 24;
const QUARTER_DB_PER_DB: f64 = 4.0;
const UNAVAILABLE: u8 = 0xff;
const MAX_JSON_BYTES: u64 = 16 * 1024 * 1024;
const RXMER_V1_IDENTITY: [u8; 6] = [b'P', b'N', b'N', 4, 1, 0];

fn encode_sample(mer_db: Option<f64>) -> Result<u8, Box<dyn Error>> {
    let Some(mer_db) = mer_db else {
        return Ok(UNAVAILABLE);
    };
    let quarters = mer_db * QUARTER_DB_PER_DB;
    if !quarters.is_finite()
        || !(0.0..f64::from(UNAVAILABLE)).contains(&quarters)
        || quarters.fract() != 0.0
    {
        return Err("RxMER must be exactly representable in unsigned quarter-dB units.".into());
    }
    Ok(quarters as u8)
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() != 3 {
        return Err("Usage: export_rxmer_pnm BASE_RXMER_PNM SCENARIO_JSON NEW_PNM".into());
    }
    let original_path = Path::new(&args[0]);
    let scenario_path = Path::new(&args[1]);
    if std::fs::metadata(original_path)?.len() > HEADER_BYTES as u64 + 65_536
        || std::fs::metadata(scenario_path)?.len() > MAX_JSON_BYTES
    {
        return Err("Fixture input exceeds its size limit.".into());
    }
    let mut output = std::fs::read(original_path)?;
    if output.len() < HEADER_BYTES || output[..6] != RXMER_V1_IDENTITY {
        return Err("Expected an original PNN4 RxMER version1.0 fixture.".into());
    }
    let scenario: ScenarioDocument = serde_json::from_slice(&std::fs::read(scenario_path)?)?;
    scenario.validate()?;
    if scenario.frames.len() != 1 {
        return Err("Exactly one RxMER frame is required.".into());
    }
    let frame = &scenario.frames[0];
    let grid = &frame.key.grid;
    let read_u32 = |offset| u32::from_be_bytes(output[offset..offset + 4].try_into().unwrap());
    if read_u32(CAPTURE_TIME_OFFSET) != frame.capture_time
        || output[CHANNEL_OFFSET] != frame.key.channel_id
        || output[MAC_OFFSET..MAC_OFFSET + 6] != frame.key.mac_address
        || read_u32(ZERO_FREQUENCY_OFFSET) != grid.zero_frequency_hz
        || u16::from_be_bytes(output[FIRST_CARRIER_OFFSET..FIRST_CARRIER_OFFSET + 2].try_into()?)
            != grid.first_subcarrier_index
        || u32::from(output[SPACING_OFFSET]) * 1000 != grid.spacing_hz
        || read_u32(LENGTH_OFFSET) as usize != grid.sample_count
        || output.len() != HEADER_BYTES + grid.sample_count
    {
        return Err(
            "Original binary header does not match the scenario identity/grid/time.".into(),
        );
    }
    let encoded: Result<Vec<_>, _> = frame.mer_db.iter().copied().map(encode_sample).collect();
    output[HEADER_BYTES..].copy_from_slice(&encoded?);
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&args[2])?;
    file.write_all(&output)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::encode_sample;

    #[test]
    fn exact_quarter_db_encoding_preserves_unavailable_and_rejects_loss() {
        assert_eq!(encode_sample(None).unwrap(), 255);
        assert_eq!(encode_sample(Some(0.0)).unwrap(), 0);
        assert_eq!(encode_sample(Some(11.0)).unwrap(), 44);
        assert_eq!(encode_sample(Some(63.5)).unwrap(), 254);
        for invalid in [-0.25, 63.75, 11.1, f64::NAN, f64::INFINITY] {
            assert!(encode_sample(Some(invalid)).is_err());
        }
    }
}
