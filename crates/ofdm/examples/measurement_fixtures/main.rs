/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Offline Rust fixture preparation; not a PNM parser or simulation importer.
mod alignment;
use docsis_ofdm_fec::{
    scenario::grid::FrequencyGrid,
    signal_processing::noise::lte_ingress::{
        LteDownlinkIngress, CHANNEL_BANDWIDTH_HZ, FLAT_CORE_BANDWIDTH_HZ,
    },
};
use serde_json::{json, Value};
use std::{error::Error, fs::OpenOptions, io::Write, path::Path};

const FULL_BAND_REDUCTION_DB: f64 = 6.0;
const NOTCH_REDUCTION_DB: f64 = 12.0;
const INPUT_LIMIT_BYTES: u64 = 16 * 1024 * 1024;
const LTE_MODEL: LteDownlinkIngress = LteDownlinkIngress {
    center_frequency_hz: 737_500_000,
    interference_to_signal_db: -30.0,
};

#[derive(Clone, Copy)]
enum Variant {
    FullBand,
    CenterNotch,
    LteDownlink,
}

// Preserve the decoder document shape, metadata, nulls and original sample order.
// The notch uses a half-open middle-tenth interval in original array coordinates.
fn variant_document(original: &Value, variant: Variant) -> Result<Value, String> {
    if original.get("value_units").and_then(Value::as_str) != Some("dB") {
        return Err("Expected RxMER readings in dB.".into());
    }
    if matches!(variant, Variant::LteDownlink) {
        let count = original["values"]
            .as_array()
            .ok_or("Missing RxMER values.")?
            .len();
        let field = |name| {
            original[name]
                .as_u64()
                .and_then(|value| u32::try_from(value).ok())
                .ok_or("Missing frequency header.")
        };
        let grid = FrequencyGrid {
            zero_frequency_hz: field("subcarrier_zero_frequency")?,
            first_subcarrier_index: u16::try_from(field("first_active_subcarrier_index")?)
                .map_err(|error| error.to_string())?,
            spacing_hz: field("subcarrier_spacing")?,
            sample_count: count,
        };
        if original["data_length"].as_u64() != Some(count as u64) {
            return Err("RxMER sample count differs from data_length.".into());
        }
        let readings: Vec<Option<f64>> = serde_json::from_value(original["values"].clone())
            .map_err(|error| error.to_string())?;
        let result = LTE_MODEL
            .apply(&grid, &readings)
            .map_err(|error| error.to_string())?;
        let mut derived = original.clone();
        derived["values"] = json!(result.mer_db);
        return Ok(derived);
    }
    let mut derived = original.clone();
    let values = derived
        .get_mut("values")
        .and_then(Value::as_array_mut)
        .ok_or("Expected an RxMER values array.")?;
    let sample_count = values.len();
    if sample_count == 0
        || original.get("data_length").and_then(Value::as_u64) != Some(sample_count as u64)
    {
        return Err("RxMER sample count differs from data_length.".into());
    }
    let notch_count = (sample_count / 10).max(1);
    let notch_start = (sample_count - notch_count) / 2;
    let notch_stop = notch_start + notch_count;
    for (sample_index, sample) in values.iter_mut().enumerate() {
        if sample.is_null() {
            continue;
        }
        let original_mer_db = sample.as_f64().ok_or("Expected finite RxMER or null.")?;
        let reduction_db = match variant {
            Variant::FullBand => FULL_BAND_REDUCTION_DB,
            Variant::CenterNotch if (notch_start..notch_stop).contains(&sample_index) => {
                NOTCH_REDUCTION_DB
            }
            Variant::CenterNotch => 0.0,
            Variant::LteDownlink => unreachable!("LTE fixture handled by shared Rust model"),
        };
        *sample = Value::from(original_mer_db - reduction_db);
    }
    Ok(derived)
}

fn read_document(path: &Path) -> Result<Value, Box<dyn Error>> {
    if std::fs::metadata(path)?.len() > INPUT_LIMIT_BYTES {
        return Err("Measurement fixture exceeds the 16 MiB preparation limit.".into());
    }
    Ok(serde_json::from_slice(&std::fs::read(path)?)?)
}

fn main() -> Result<(), Box<dyn Error>> {
    let arguments: Vec<_> = std::env::args_os().skip(1).collect();
    if arguments.len() != 2 {
        return Err("Usage: prepare_rxmer_variants BASE_RXMER_JSON OUTPUT_DIRECTORY".into());
    }
    let input_path = Path::new(&arguments[0]);
    let original = read_document(input_path)?;
    let source_directory = input_path.parent().ok_or("Missing fixture directory.")?;
    let channel = read_document(&source_directory.join("channel_estimation.json"))?;
    let profile = read_document(&source_directory.join("modulation_profile.json"))?;
    let alignment = alignment::validate(&original, &channel, &profile)?;
    let variants = [
        ("rxmer-reduced-6db.json", Variant::FullBand),
        ("rxmer-center-notch-12db.json", Variant::CenterNotch),
        ("rxmer-lte-downlink.json", Variant::LteDownlink),
    ];
    // Validate all documents and destinations before creating output. Never replace a file.
    let mut documents = variants
        .iter()
        .map(|(_, variant)| variant_document(&original, *variant))
        .collect::<Result<Vec<_>, _>>()?;
    documents.push(json!({"schema_version":1,"synthetic":true,"model":"lte_downlink_ingress_approximation",
        "settings":LTE_MODEL,"channel_bandwidth_hz":CHANNEL_BANDWIDTH_HZ,"flat_core_bandwidth_hz":FLAT_CORE_BANDWIDTH_HZ,
        "edge_shape":"cosine_power_shoulders","reference_plane":"normalized_equalized_residual_power",
        "additional_thermal_noise":false,"apply_resulting_rxmer_once":true,"alignment":alignment}));
    let filenames: Vec<_> = variants
        .iter()
        .map(|(name, _)| *name)
        .chain(["lte-ingress-model.json"])
        .collect();
    for filename in &filenames {
        if Path::new(&arguments[1]).join(filename).exists() {
            return Err(format!("Output already exists: {filename}").into());
        }
    }
    for (filename, document) in filenames.iter().zip(documents) {
        let output_path = Path::new(&arguments[1]).join(filename);
        let mut output = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(output_path)?;
        serde_json::to_writer_pretty(&mut output, &document)?;
        output.write_all(b"\n")?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn full_band_preserves_nulls_metadata_and_source() {
        let original = json!({"value_units":"dB", "data_length":3,
            "values":[42.75,null,6.0], "pnm_header":{"capture_time":123}});
        let derived = variant_document(&original, Variant::FullBand).unwrap();
        assert_eq!(derived["values"], json!([36.75, null, 0.0]));
        assert_eq!(derived["pnm_header"], original["pnm_header"]);
        assert_eq!(original["values"], json!([42.75, null, 6.0]));
    }

    #[test]
    fn notch_has_independent_half_open_edge_expectations() {
        let original = json!({"value_units":"dB", "data_length":20,
            "values":vec![40.0;20]});
        let derived = variant_document(&original, Variant::CenterNotch).unwrap();
        let mut expected = vec![40.0; 20];
        expected[9] = 28.0;
        expected[10] = 28.0;
        assert_eq!(derived["values"], json!(expected));
        assert!(variant_document(&json!({"values":[]}), Variant::FullBand).is_err());
    }
}
