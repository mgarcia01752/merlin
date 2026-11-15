/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Generate a synthetic CableLabs-style experiment without modifying field inputs.
#[path = "support/adapt_channel.rs"]
mod adapt_channel;

use base64::{engine::general_purpose::STANDARD, Engine};
use docsis_ofdm_fec::{
    transmission::{
        layout::{
            continuous::{generate_continuous_pilots, ContinuousPilotSettings},
            CarrierLayout, CarrierLayoutSettings, PhysicalCarrierRole,
        },
        TransmissionSettings,
    },
    types::modulation::ModulationAssignment,
};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{error::Error, fs, path::Path};

const OCCUPIED_HZ: u32 = 192_000_000;
const ZERO_HZ: u32 = 292_600_000;
const FIRST_BIN: u16 = 296;
const ACTIVE_COUNT: u16 = 7600;
const PLC_FIRST_BIN: u16 = 1608;
const NOTCH_FIRST_HZ: u32 = 371_000_000;
const NOTCH_END_HZ: u32 = 379_000_000;
const BACKGROUND_DB: f64 = 42.0;
const NOTCH_DB: f64 = 10.0;
const PROFILE_ID: u8 = 3;
const CAPTURE_TIME: u32 = 1_700_000_000;
const OBSERVATION_MS: u64 = 10;
const RUNTIME_MS: u64 = 300_000;
const ITERATIONS: usize = 50;
const NOISE_SEED: u64 = 41;
const PRBS_SEED: u16 = 32767;

fn stored(root: &Path, name: &str, value: &Value) -> Result<Value, Box<dyn Error>> {
    let bytes = serde_json::to_vec_pretty(value)?;
    fs::write(root.join(name), &bytes)?;
    Ok(
        json!({"name":name,"byte_length":bytes.len(),"sha256":format!("{:x}",Sha256::digest(&bytes)),"base64":STANDARD.encode(&bytes)}),
    )
}

fn main() -> Result<(), Box<dyn Error>> {
    let output = std::env::args_os()
        .nth(1)
        .ok_or("Supply a new output directory")?;
    let root = Path::new(&output);
    let transmission = TransmissionSettings {
        occupied_bandwidth_hz: OCCUPIED_HZ,
        cyclic_prefix_samples: 512,
        roll_off_samples: 128,
        time_interleaver_depth: 16,
        ..Default::default()
    };
    let geometry = CarrierLayoutSettings {
        zero_frequency_hz: ZERO_HZ,
        first_active_bin: FIRST_BIN,
        active_end_exclusive: FIRST_BIN + ACTIVE_COUNT,
        plc_first_bin: PLC_FIRST_BIN,
        excluded_bins: vec![],
        additional_continuous_pilots: vec![],
    };
    let geometry = generate_continuous_pilots(
        &transmission,
        &geometry,
        &ContinuousPilotSettings::default(),
    )?;
    let layout = CarrierLayout::new(&transmission, &geometry)?;
    let spacing = transmission.subcarrier_spacing.frequency_hz();
    let mut schemes: Vec<Value> = Vec::new();
    for bin in FIRST_BIN..FIRST_BIN + ACTIVE_COUNT {
        let code = match layout.static_roles()[usize::from(bin)] {
            PhysicalCarrierRole::Data => 12,
            PhysicalCarrierRole::Plc => 20,
            PhysicalCarrierRole::ContinuousPilot => 1,
            _ => return Err("Unexpected generated carrier role".into()),
        };
        if let Some(last) = schemes
            .last_mut()
            .filter(|s| s["modulation_order_code"] == code)
        {
            last["num_subcarriers"] = json!(last["num_subcarriers"].as_u64().unwrap() + 1);
        } else {
            let assignment = ModulationAssignment::from_code(code);
            schemes.push(json!({"schema_type":"range","num_subcarriers":1,"modulation_order_code":code,
                "modulation_order":assignment.modulation_order,"bit_loading":assignment.bit_loading,"carrier_type":assignment.carrier_type}));
        }
    }
    let base = json!({"channel_id":34,"mac_address":"00:ca:00:00:00:01",
        "subcarrier_zero_frequency":ZERO_HZ,"first_active_subcarrier_index":FIRST_BIN,"subcarrier_spacing":spacing});
    let header = |kind| json!({"file_type":"PNN","file_type_version":kind,"major_version":1,"minor_version":0,"capture_time":CAPTURE_TIME});
    let mut profile = base.clone();
    profile["pnm_header"] = header(10);
    profile["num_profiles"] = json!(1);
    profile["profiles"] = json!([{"profile_id":PROFILE_ID,"schemes":schemes}]);
    fs::create_dir(root)?;
    let profile_file = stored(root, "modulation_profile.json", &profile)?;
    for (rxmer_name, wb_name, notch) in [
        ("rxmer-notch-10db.json", "workbench.merlin.json", true),
        ("rxmer-flat-42db.json", "baseline.merlin.json", false),
    ] {
        let mut rxmer = base.clone();
        rxmer["pnm_header"] = header(4);
        rxmer["value_units"] = json!("dB");
        rxmer["data_length"] = json!(ACTIVE_COUNT);
        rxmer["values"] = json!((FIRST_BIN..FIRST_BIN + ACTIVE_COUNT)
            .map(|bin| {
                let hz = ZERO_HZ + u32::from(bin) * spacing;
                if notch && (NOTCH_FIRST_HZ..NOTCH_END_HZ).contains(&hz) {
                    NOTCH_DB
                } else {
                    BACKGROUND_DB
                }
            })
            .collect::<Vec<_>>());
        let measurement = stored(root, rxmer_name, &rxmer)?;
        let wb = json!({"schema_version":2,"application":"MERLIN","transmission":transmission,
            "sections":{"profile":{"file":profile_file,"profile_id":PROFILE_ID},
            "measurements":{"response":null,"response_tolerance":0,"rxmer":[measurement],
                "options":{"mode":"single","interval_symbols":"1000","tolerance_seconds":0}},
            "source":{"file":null,"options":{"mode":"prbs15","seed":PRBS_SEED,"fill_seed":PRBS_SEED,"append":false,"target_bits":0}}},
            "receiver":{"receiver_mode":"supplied_channel","selected_profile_ids":[PROFILE_ID],
                "duration_ms":OBSERVATION_MS,"max_iterations":ITERATIONS,"max_runtime_ms":RUNTIME_MS,"noise_seed":NOISE_SEED},
            "comparison_enabled":true,"dark":true});
        let bytes = serde_json::to_vec_pretty(&wb)?;
        let loaded = pnm_input::Workbench::from_bytes(&bytes)?;
        loaded.controllers()[0].prepare()?;
        fs::write(root.join(wb_name), bytes)?;
    }

    // Adapt an existing response into a separate, explicitly synthetic experiment.
    let original =
        include_bytes!("../../../examples/measurements/single-channel/channel_estimation.json");
    let mut response: Value = serde_json::from_slice(original)?;
    let source_values: Vec<Option<[f64; 2]>> = serde_json::from_value(response["iq"].clone())?;
    let original_header = response["pnm_header"].clone();
    response["iq"] = json!(adapt_channel::interpolate(
        &source_values,
        usize::from(ACTIVE_COUNT)
    )?);
    response["subcarrier_zero_frequency"] = json!(ZERO_HZ);
    response["first_active_subcarrier_index"] = json!(FIRST_BIN);
    response["data_length"] = json!(u32::from(ACTIVE_COUNT) * 4);
    response["pnm_header"]["capture_time"] = json!(CAPTURE_TIME);
    let response_file = stored(root, "channel-estimation-adapted.json", &response)?;
    let mut companion: Value =
        serde_json::from_slice(&fs::read(root.join("workbench.merlin.json"))?)?;
    companion["sections"]["measurements"]["response"] = response_file;
    let bytes = serde_json::to_vec_pretty(&companion)?;
    pnm_input::Workbench::from_bytes(&bytes)?.controllers()[0].prepare()?;
    fs::write(root.join("cmce.merlin.json"), bytes)?;
    let provenance = json!({
        "source": "examples/measurements/single-channel/channel_estimation.json",
        "source_sha256": format!("{:x}", Sha256::digest(original)),
        "source_header": original_header,
        "source_first_hz": 640_000_000, "source_last_hz": 826_975_000,
        "source_count": source_values.len(),
        "destination_first_hz": 300_000_000, "destination_last_hz": 489_975_000,
        "destination_count": ACTIVE_COUNT,
        "synthetic": true,
        "transformation": "Endpoint-aligned frequency remap; linear complex I/Q interpolation; exact knots and missing samples preserved; no extrapolation. Derived timestamp matches the synthetic experiment.",
        "license": "Apache-2.0; retain source NOTICE"
    });
    fs::write(
        root.join("channel-estimation-provenance.json"),
        serde_json::to_vec_pretty(&provenance)?,
    )?;
    println!(
        "Created and validated three workbenches in {}",
        root.display()
    );
    Ok(())
}
