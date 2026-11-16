/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Materialize one saved flat Designer experiment and prepare bounded CL comparisons.
use base64::{engine::general_purpose::STANDARD, Engine};
use docsis_ofdm_fec::{scenario::FrequencyGrid, signal_processing::rxmer_designer::RxmerRecipe};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{error::Error, fs, path::Path};

const MAX_INPUT_BYTES: u64 = 16 * 1024 * 1024;
const CL_CENTER_HZ: u32 = 375_000_000;
const CL_WIDTH_HZ: u32 = 8_000_000;
const CL_BACKGROUND_DB: f64 = 48.0;
const DEPTHS_DB: [u32; 7] = [0, 8, 18, 28, 38, 43, 48];

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.len() != 2 {
        return Err("Supply INPUT_WB NEW_OUTPUT_DIRECTORY".into());
    }
    let source = Path::new(&args[0]);
    let output = Path::new(&args[1]);
    if fs::metadata(source)?.len() > MAX_INPUT_BYTES {
        return Err("WB exceeds 16 MiB".into());
    }
    let original = fs::read(source)?;
    let wb: Value = serde_json::from_slice(&original)?;
    let inputs = wb["sections"]["measurements"]["rxmer"]
        .as_array()
        .ok_or("Missing RxMER")?;
    if inputs.len() != 1 {
        return Err("This experiment requires one Designer capture".into());
    }
    let envelope: Value = serde_json::from_slice(
        &STANDARD.decode(inputs[0]["base64"].as_str().ok_or("Missing bytes")?)?,
    )?;
    if envelope["merlin_rxmer_designer"] != 1
        || envelope["baseline"]["mode"] != "flat"
        || envelope["capture_index"] != 1
        || envelope["capture_count"] != 1
    {
        return Err("This experiment supports one flat Designer baseline only".into());
    }
    let base = &envelope["baseline"];
    let grid: FrequencyGrid = serde_json::from_value(base["key"]["grid"].clone())?;
    grid.validate()?;
    let valid: Vec<bool> = serde_json::from_value(base["valid"].clone())?;
    if valid.len() != grid.sample_count {
        return Err("Invalid baseline mask".into());
    }
    let baseline_db = base["baseline_db"]
        .as_f64()
        .ok_or("Missing baseline level")?;
    let mut cases = vec![(
        "user-materialized".to_string(),
        wb.clone(),
        envelope["recipe"].clone(),
        baseline_db,
    )];
    let mut aligned = envelope["recipe"].clone();
    aligned["notch"]["center_hz"] = json!(CL_CENTER_HZ);
    aligned["notch"]["bandwidth_hz"] = json!(CL_WIDTH_HZ);
    cases.push(("user-cl-notch".into(), wb.clone(), aligned, baseline_db));
    for depth in DEPTHS_DB {
        let mut simple = wb.clone();
        simple["sections"]["measurements"]["response"] = Value::Null;
        simple["receiver"]["receiver_mode"] = json!("supplied_channel");
        let recipe = json!({"notch":{"scope":{"mode":"all"},"center_hz":CL_CENTER_HZ,"bandwidth_hz":CL_WIDTH_HZ,"depth_db":depth,"edge_hz":0}});
        cases.push((
            format!("cl-unity-notch-{}db", CL_BACKGROUND_DB as u32 - depth),
            simple,
            recipe,
            CL_BACKGROUND_DB,
        ));
    }
    fs::create_dir(output)?;
    let mut manifest = Vec::new();
    for (name, mut candidate, recipe, level) in cases {
        let recipe: RxmerRecipe = serde_json::from_value(recipe)?;
        let baseline: Vec<_> = valid.iter().map(|valid| valid.then_some(level)).collect();
        let values = recipe.apply(&grid, &baseline, 1, 1)?;
        let rxmer = json!({"pnm_header":{"file_type":"PNN","file_type_version":4,"major_version":1,"minor_version":0,"capture_time":base["anchor_epoch"]},
            "mac_address":"00:ca:00:00:00:01","channel_id":base["key"]["channel_id"],
            "subcarrier_zero_frequency":grid.zero_frequency_hz,"first_active_subcarrier_index":grid.first_subcarrier_index,
            "subcarrier_spacing":grid.spacing_hz,"data_length":grid.sample_count,"value_units":"dB","values":values});
        let bytes = serde_json::to_vec_pretty(&rxmer)?;
        let capture_name = format!("{name}-rxmer.json");
        fs::write(output.join(&capture_name), &bytes)?;
        candidate["sections"]["measurements"]["rxmer"] = json!([{"name":capture_name,"byte_length":bytes.len(),"sha256":format!("{:x}",Sha256::digest(&bytes)),"base64":STANDARD.encode(&bytes)}]);
        let bytes = serde_json::to_vec_pretty(&candidate)?;
        pnm_input::Workbench::from_bytes(&bytes)?.controllers()[0].prepare()?;
        let filename = format!("{name}.merlin.json");
        fs::write(output.join(&filename), &bytes)?;
        manifest.push(json!({"workbench":filename,"sha256":format!("{:x}",Sha256::digest(&bytes)),"recipe":recipe,"baseline_db":level,"receiver":candidate["receiver"],"unity":candidate["sections"]["measurements"]["response"].is_null()}));
    }
    fs::write(
        output.join("manifest.json"),
        serde_json::to_vec_pretty(
            &json!({"source":source,"source_sha256":format!("{:x}",Sha256::digest(&original)),"cases":manifest}),
        )?,
    )?;
    println!("Prepared {} cases in {}", manifest.len(), output.display());
    Ok(())
}
