/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Offline fixture/request preparation through shared import, layout and impairment APIs.
use docsis_ofdm_fec::{
    scenario::pnm_json::{import, JsonCapture},
    signal_processing::noise::lte_ingress::LteDownlinkIngress,
    simulation::channel::measurement::captured_layout,
    transmission::{layout::CarrierLayout, TransmissionSettings},
    types::{modulation::CarrierType, FrequencyHz, ProtocolScope},
};
use serde_json::json;
use std::{io::Write, path::Path};
// Engineering fixture bounds, not DOCSIS limits or an LTE channel allocation.
const STRESS_START_HZ: FrequencyHz = 700_000_000;
const STRESS_END_HZ: FrequencyHz = 727_000_000;
const DEFAULT_INTERFERENCE_TO_SIGNAL_DB: f64 = -24.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    let stress_mode = args.get(2).is_some_and(|value| value == "--stress-mer-db");
    if !(2..=4).contains(&args.len()) || (args.len() == 4) != stress_mode {
        return Err(
            "Usage: prepare_impairment_runs SOURCE_FIXTURE_DIRECTORY NEW_OUTPUT_DIRECTORY [CORE_INTERFERENCE_TO_SIGNAL_DB | --stress-mer-db TARGET_MER_DB]".into(),
        );
    }
    let selected_level_db = if let Some(value) = args.get(if stress_mode { 3 } else { 2 }) {
        value
            .to_str()
            .ok_or("Invalid dB level encoding.")?
            .parse::<f64>()?
    } else {
        DEFAULT_INTERFERENCE_TO_SIGNAL_DB
    };
    if !selected_level_db.is_finite() {
        return Err("Selected dB level must be finite.".into());
    }
    let source = Path::new(&args[0]);
    let output = Path::new(&args[1]);
    if output.exists() {
        return Err("Output directory must be new.".into());
    }
    let names = [
        "rxmer.json",
        "channel_estimation.json",
        "modulation_profile.json",
    ];
    let mut bytes = Vec::new();
    for name in names {
        let path = source.join(name);
        if std::fs::metadata(&path)?.len() > 16 * 1024 * 1024 {
            return Err("Input exceeds16MiB.".into());
        }
        bytes.push(std::fs::read(path)?);
    }
    let scenario = import(
        JsonCapture {
            name: names[0],
            bytes: &bytes[0],
        },
        JsonCapture {
            name: names[1],
            bytes: &bytes[1],
        },
        JsonCapture {
            name: names[2],
            bytes: &bytes[2],
        },
        ProtocolScope::Docsis31,
        90_000,
    )?;
    let transmission = TransmissionSettings::default();
    let profile = scenario
        .profiles
        .iter()
        .find(|profile| profile.profile_id == 0)
        .ok_or("Missing256-QAM profile0.")?;
    let layout = captured_layout(profile, &transmission)?;
    let physical = CarrierLayout::new(&transmission, &layout)?;
    let protected = physical.plc_neighborhood_hz()?;
    let model = LteDownlinkIngress {
        center_frequency_hz: 737_500_000,
        interference_to_signal_db: selected_level_db,
    };
    let mut documents = Vec::new();
    let cases = if stress_mode {
        vec![("baseline", 0), ("rxmer-stress", 0)]
    } else {
        vec![
            ("baseline", 0),
            ("lte-1rb", 1),
            ("lte-6rb", 6),
            ("lte-12rb", 12),
            ("lte-25rb", 25),
            ("lte-50rb", 50),
        ]
    };
    if stress_mode && STRESS_START_HZ < protected.end && STRESS_END_HZ > protected.start {
        return Err("Stress band overlaps protected PLC neighborhood.".into());
    }
    for (label, blocks) in cases {
        let mut variant = scenario.clone();
        let mut affected = 0;
        if label == "rxmer-stress" {
            for (offset, (mer_db, assignment)) in variant.frames[0]
                .mer_db
                .iter_mut()
                .zip(&profile.assignments)
                .enumerate()
            {
                let frequency_hz = scenario.frames[0].key.grid.frequency(offset)?;
                if assignment.carrier_type == CarrierType::Data
                    && (STRESS_START_HZ..STRESS_END_HZ).contains(&frequency_hz)
                {
                    if let Some(value) = mer_db {
                        if *value > selected_level_db {
                            *value = selected_level_db;
                            affected += 1;
                        }
                    }
                }
            }
        }
        if blocks > 0 {
            let result = model.apply_resource_blocks(
                &scenario.frames[0].key.grid,
                &scenario.frames[0].mer_db,
                blocks,
                std::slice::from_ref(&protected),
            )?;
            affected = result
                .interference_to_signal_power
                .iter()
                .filter(|power| **power > 0.0)
                .count();
            variant.frames[0].mer_db = result.mer_db;
        }
        if blocks > 0 || label == "rxmer-stress" {
            variant.frames[0].source.file_kind = "synthetic_rxmer_from_decoded_json".into();
            variant.frames[0].source.name =
                format!("{label} derived from {}", scenario.frames[0].source.name);
            // This scenario is synthetic, not another captured JSON source. The original
            // hashed provenance is intentionally retained as the input identity; the
            // separate request/report records the transformation.
        }
        documents.push((
            format!("{label}-scenario.json"),
            serde_json::to_value(variant)?,
        ));
        documents.push((format!("{label}-request.json"),json!({"schema_version":1,"transmission":transmission,"layout":layout,
            "duration_ms":3,"measurement":{"frame_index":0,"profile_id":0,"channel_index":0,"tolerance_seconds":90000,"noise_seed":41}})));
        documents.push((format!("{label}-model.json"),json!({"synthetic":blocks>0 || label == "rxmer-stress",
            "stress_target_mer_db":if stress_mode { Some(selected_level_db) } else { None },
            "stress_band_hz":if stress_mode { Some([STRESS_START_HZ,STRESS_END_HZ]) } else { None },
            "stress_data_assignments_only":stress_mode,"source":"original decoded JSON provenance retained",
            "resource_blocks":if stress_mode { None } else { Some(blocks) },
            "settings":if stress_mode { None } else { Some(model) },"affected_grid_carriers":affected,
            "grid_carriers":scenario.frames[0].mer_db.len(),"protected_plc_hz":[protected.start,protected.end],
            "additional_thermal_noise":false,"note":"Run labels are scenarios, not new measurement epochs;15% is not a FEC cutoff."})));
    }
    std::fs::create_dir_all(output)?;
    for (name, document) in documents {
        let mut file = std::fs::OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(output.join(name))?;
        serde_json::to_writer_pretty(&mut file, &document)?;
        file.write_all(b"\n")?;
    }
    Ok(())
}
