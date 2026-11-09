/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Narrow current-decoder fixture checks; not a general PNM import adapter.
use docsis_ofdm_fec::scenario::grid::FrequencyGrid;
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Debug, Deserialize, PartialEq)]
struct DecodedHeader {
    channel_id: u8,
    mac_address: String,
    subcarrier_zero_frequency: u32,
    first_active_subcarrier_index: u16,
    subcarrier_spacing: u32,
}

/// Checks exact decoded fixture grids and identity without changing observations.
///
/// # Arguments
/// * `rxmer` - Current decoder RxMER document.
/// * `channel` - Current decoder channel-estimate document.
/// * `profile` - Current decoder modulation-profile document.
///
/// # Returns
/// Grid and independent timestamps for the fixture preparation report.
///
/// # Errors
/// Rejects header mismatches, wrong capture types, malformed arrays or unequal
/// expanded carrier counts. Different timestamps are retained, not rejected here.
pub(super) fn validate(rxmer: &Value, channel: &Value, profile: &Value) -> Result<Value, String> {
    let rxmer_header: DecodedHeader =
        serde_json::from_value(rxmer.clone()).map_err(|error| error.to_string())?;
    for document in [channel, profile] {
        let header: DecodedHeader =
            serde_json::from_value(document.clone()).map_err(|error| error.to_string())?;
        if header != rxmer_header {
            return Err(
                "Measurement MAC, channel or frequency grid mismatch; refusing relabeling.".into(),
            );
        }
    }
    let samples = rxmer["values"].as_array().ok_or("Missing RxMER values.")?;
    let grid = FrequencyGrid {
        zero_frequency_hz: rxmer_header.subcarrier_zero_frequency,
        first_subcarrier_index: rxmer_header.first_active_subcarrier_index,
        spacing_hz: rxmer_header.subcarrier_spacing,
        sample_count: samples.len(),
    };
    grid.validate().map_err(|error| error.to_string())?;
    let iq = channel["iq"].as_array().ok_or("Missing channel IQ.")?;
    if iq.len() != samples.len()
        || iq.iter().any(|pair| {
            !pair.is_null()
                && pair.as_array().is_none_or(|components| {
                    components.len() != 2
                        || components
                            .iter()
                            .any(|component| component.as_f64().is_none())
                })
        })
    {
        return Err("Channel IQ count or pair shape differs from the source grid.".into());
    }
    let profiles = profile["profiles"].as_array().ok_or("Missing profiles.")?;
    if profiles.is_empty() {
        return Err("No modulation profiles to align.".into());
    }
    for assignment in profiles {
        let schemes = assignment["schemes"]
            .as_array()
            .ok_or("Missing profile schemes.")?;
        let count = schemes.iter().try_fold(0u64, |count, scheme| {
            count.checked_add(scheme["num_subcarriers"].as_u64()?)
        });
        if count != Some(samples.len() as u64) {
            return Err("Profile coverage differs from the RxMER carrier count.".into());
        }
    }
    let mut timestamps = Vec::new();
    for (document, kind) in [(rxmer, 4), (channel, 2), (profile, 10)] {
        if document["pnm_header"]["file_type_version"].as_u64() != Some(kind) {
            return Err("Unexpected decoded PNM capture type.".into());
        }
        timestamps.push(
            document["pnm_header"]["capture_time"]
                .as_u64()
                .ok_or("Missing capture epoch.")?,
        );
    }
    Ok(json!({
        "mac_address":rxmer_header.mac_address,
        "channel_id":rxmer_header.channel_id,
        "grid":grid,
        "first_carrier_center_hz":grid.frequency(0).map_err(|error| error.to_string())?,
        "last_carrier_center_hz":grid.frequency(samples.len()-1).map_err(|error| error.to_string())?,
        "capture_times":{"rxmer":timestamps[0],"channel_estimation":timestamps[1],"modulation_profile":timestamps[2]},
        "simultaneous":timestamps.iter().all(|time| *time == timestamps[0]),
        "policy":"exact_identity_and_grid_for_fixture_preparation_only"
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn captures() -> [Value; 3] {
        let header = json!({"channel_id":34,"mac_address":"00:ca:00:00:00:01",
            "subcarrier_zero_frequency":631100000,"first_active_subcarrier_index":356,
            "subcarrier_spacing":25000});
        let mut captures = [header.clone(), header.clone(), header];
        for (offset, kind) in [4, 2, 10].iter().enumerate() {
            captures[offset]["pnm_header"] =
                json!({"file_type_version":kind,"capture_time":100+offset});
        }
        captures[0]["values"] = json!([40.0, null]);
        captures[1]["iq"] = json!([[1.0, 0.0], null]);
        captures[2]["profiles"] = json!([{"schemes":[{"num_subcarriers":2}]}]);
        captures
    }

    #[test]
    fn accepts_matching_grids_and_keeps_separate_epochs() {
        let [rxmer, channel, profile] = captures();
        let report = validate(&rxmer, &channel, &profile).unwrap();
        assert_eq!(report["simultaneous"], false);
        assert_eq!(report["capture_times"]["modulation_profile"], 102);
        assert_eq!(report["first_carrier_center_hz"], 640000000);
    }

    #[test]
    fn rejects_each_identity_grid_and_coverage_mismatch() {
        let [rxmer, channel, profile] = captures();
        for (field, value) in [
            ("channel_id", json!(35)),
            ("mac_address", json!("00:ca:00:00:00:02")),
            ("subcarrier_zero_frequency", json!(631125000)),
            ("first_active_subcarrier_index", json!(357)),
            ("subcarrier_spacing", json!(50000)),
        ] {
            let mut invalid = channel.clone();
            invalid[field] = value.clone();
            assert!(validate(&rxmer, &invalid, &profile).is_err());
            let mut invalid = profile.clone();
            invalid[field] = value;
            assert!(validate(&rxmer, &channel, &invalid).is_err());
        }
        let mut invalid = profile.clone();
        invalid["profiles"][0]["schemes"][0]["num_subcarriers"] = json!(3);
        assert!(validate(&rxmer, &channel, &invalid).is_err());
        assert_eq!(channel["subcarrier_spacing"], 25000);
    }
}
