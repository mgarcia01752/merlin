/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Deterministic ideal-channel fixtures; no field measurements are invented.
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
use std::error::Error;

const EDGE_TAPER_HZ: u32 = 1_000_000;
const FIRST_ACTIVE_BIN: u16 = 356;
const PLC_FIRST_BIN: u16 = 788;

/// Builds synthetic decoder-shaped inputs and runnable channel configurations.
/// # Arguments
/// * `occupied_hz` - Requested occupied width, either24or90MHz.
/// * `source` - Existing four-profile JSON, used only for profile IDs/data orders.
/// # Returns
/// Named JSON documents using25kHz spacing, unity correction and48dB RxMER.
/// # Errors
/// Rejects unsupported width, malformed source profiles or invalid PHY geometry.
pub fn documents(occupied_hz: u32, source: &Value) -> Result<Vec<(String, Value)>, Box<dyn Error>> {
    if !matches!(occupied_hz, 24_000_000 | 90_000_000) {
        return Err("Fixture width must be24or90MHz.".into());
    }
    let transmission = TransmissionSettings {
        occupied_bandwidth_hz: occupied_hz,
        ..Default::default()
    };
    let spacing = transmission.subcarrier_spacing.frequency_hz();
    let count = (occupied_hz - 2 * EDGE_TAPER_HZ) / spacing;
    let raw = CarrierLayoutSettings {
        zero_frequency_hz: 631_100_000,
        first_active_bin: FIRST_ACTIVE_BIN,
        active_end_exclusive: FIRST_ACTIVE_BIN + u16::try_from(count)?,
        plc_first_bin: PLC_FIRST_BIN,
        excluded_bins: Vec::new(),
        additional_continuous_pilots: Vec::new(),
    };
    let layout =
        generate_continuous_pilots(&transmission, &raw, &ContinuousPilotSettings::default())?;
    let physical = CarrierLayout::new(&transmission, &layout)?;
    let mut profiles = Vec::new();
    let mut runs = Vec::new();
    for original in source["profiles"].as_array().ok_or("Missing profiles")? {
        let id = original["profile_id"]
            .as_u64()
            .ok_or("Missing profile ID")?;
        let data_code = original["schemes"]
            .as_array()
            .ok_or("Missing schemes")?
            .iter()
            .find(|scheme| scheme["carrier_type"] == "data")
            .and_then(|scheme| scheme["modulation_order_code"].as_u64())
            .ok_or("Missing data QAM")?;
        let mut schemes: Vec<Value> = Vec::new();
        let mut loading = vec![0u8; physical.static_roles().len()];
        for bin in layout.first_active_bin..layout.active_end_exclusive {
            let code = match physical.static_roles()[usize::from(bin)] {
                PhysicalCarrierRole::Data => {
                    loading[usize::from(bin)] = u8::try_from(data_code)?;
                    u8::try_from(data_code)?
                }
                PhysicalCarrierRole::Plc => 20,
                PhysicalCarrierRole::ContinuousPilot => 1,
                _ => return Err("Unexpected static role".into()),
            };
            let assignment = ModulationAssignment::from_code(code);
            if let Some(last) = schemes
                .last_mut()
                .filter(|scheme| scheme["modulation_order_code"] == code)
            {
                last["num_subcarriers"] = json!(last["num_subcarriers"].as_u64().unwrap() + 1);
            } else {
                schemes.push(json!({"schema_type":"range", "num_subcarriers":1,
                    "modulation_order_code":code, "modulation_order":assignment.modulation_order,
                    "bit_loading":assignment.bit_loading, "carrier_type":assignment.carrier_type}));
            }
        }
        profiles.push(json!({"profile_id":id,"schemes":schemes}));
        runs.push((format!("channel-profile-{id}.json"), json!({"schema_version":1,
            "transmission":transmission,"layout":layout,"duration_ms":1,"data_bit_loading":loading})));
    }
    let base = json!({"channel_id":34,"mac_address":"00:ca:00:00:00:01",
        "subcarrier_zero_frequency":layout.zero_frequency_hz,"first_active_subcarrier_index":layout.first_active_bin,
        "subcarrier_spacing":spacing});
    let header = |kind| {
        json!({"file_type":"PNN","file_type_version":kind,
        "major_version":1,"minor_version":0,"capture_time":1_700_000_000})
    };
    let mut rxmer = base.clone();
    rxmer["pnm_header"] = header(4);
    rxmer["value_units"] = json!("dB");
    rxmer["data_length"] = json!(count);
    rxmer["values"] = json!(vec![48.0; count as usize]);
    let mut channel = base.clone();
    channel["pnm_header"] = header(2);
    channel["data_length"] = json!(count * 4);
    channel["iq"] = json!(vec![json!([1.0, 0.0]); count as usize]);
    let mut profile = base;
    profile["pnm_header"] = header(10);
    profile["num_profiles"] = json!(profiles.len());
    profile["profiles"] = json!(profiles);
    let mut output = vec![
        ("rxmer.json".into(), rxmer),
        ("channel_estimation.json".into(), channel),
        ("modulation_profile.json".into(), profile),
        (
            "import.json".into(),
            json!({"schema_version":1,"protocol":"docsis31",
            "rxmer":"rxmer.json","channel_estimation":"channel_estimation.json",
            "modulation_profile":"modulation_profile.json","tolerance_seconds":0}),
        ),
        (
            "provenance.json".into(),
            json!({"synthetic":true,"occupied_bandwidth_hz":occupied_hz,
            "active_envelope_hz":count*spacing,"edge_taper_hz":EDGE_TAPER_HZ,
            "source":"../../single-channel/modulation_profile.json",
            "purpose":"Ideal width/geometry and QAM transport tests; not a field capture or external vector",
            "rxmer_db":48.0,"channel_correction":"unity","noise_in_channel_requests":false}),
        ),
    ];
    output.extend(runs);
    Ok(output)
}
