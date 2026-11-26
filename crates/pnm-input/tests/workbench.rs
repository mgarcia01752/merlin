/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Saved-WB contract, integrity and actual native UI FEC anchors.
use docsis_ofdm_fec::api::*;
use pnm_input::Workbench;
use serde_json::{json, Value};
const WB: &[u8] =
    include_bytes!("../../../examples/workbench/downstream/multi-rxmer-2/workbench.merlin.json");
fn document() -> Value {
    serde_json::from_slice(WB).unwrap()
}
fn load(value: &Value) -> Result<Workbench, ApiError> {
    Workbench::from_bytes(&serde_json::to_vec(value).unwrap())
}
#[test]
fn load_is_idle_self_contained_and_preserves_saved_options_and_bytes() {
    let wb = Workbench::from_bytes(WB).unwrap();
    assert_eq!(&*wb.original_document(), WB);
    assert_eq!(
        wb.controllers()
            .iter()
            .map(|sim| sim.profile().unwrap().profile_id().value())
            .collect::<Vec<_>>(),
        [0, 3, 4]
    );
    for sim in wb.controllers() {
        assert_eq!(
            sim.options().receiver_mode,
            ReceiverMode::CmChannelEstimated
        );
        assert_eq!(sim.options().observation.value(), 1);
        assert_eq!(sim.options().noise_seed.value(), 41);
        assert_eq!(sim.options().prbs_seed.value(), 32767);
        assert_eq!(sim.options().limits.runtime.value(), 300_000);
        assert_eq!(sim.options().limits.iterations.value(), 50);
        let prepared = sim.prepare().unwrap();
        let settings = prepared[0].engine_settings();
        assert_eq!(settings.transmission.occupied_bandwidth_hz, 192_000_000);
        assert!(settings.impairments.response.is_none());
        assert!(settings.playback.is_none());
    }
    let originals = wb.original_document();
    drop(wb);
    assert_eq!(&*originals, WB);
}
#[test]
fn imports_schema_two_profile_and_capture_selection_and_processing() {
    let mut value = document();
    value["schema_version"] = json!(2);
    value["receiver"]["selected_profile_ids"] = json!([4, 0]);
    value["receiver"]["receiver_mode"] = json!("merlin_channel_estimated");
    value["receiver"]["mce"] = json!({"averaging_symbols":32,"minimum_variance":1e-7});
    value["receiver"]["channel_filter"] = json!({"mode":"echoes","paths":[
        {"delay_samples":0,"gain":[1.0,0.0]}, {"delay_samples":10,"gain":[0.1,0.0]}
    ]});
    value["sections"]["channel_model"] =
        json!({"schema_version":1,"additional_loss":{"mode":"manual","tilt_db":-2.0}});
    value["sections"]["channel_power"] =
        json!({"schema_version":1,"conditioning":{"amplifier_gain_db":3.0}});
    let hash = value["sections"]["measurements"]["rxmer"][0]["sha256"].clone();
    value["sections"]["measurements"]["options"]["selected_captures"] = json!([hash]);
    let wb = load(&value).unwrap();
    assert_eq!(wb.controllers().len(), 2);
    assert_eq!(
        wb.controllers()[0].profile().unwrap().profile_id().value(),
        0
    );
    assert_eq!(
        wb.controllers()[1].profile().unwrap().profile_id().value(),
        4
    );
    for sim in wb.controllers() {
        let jobs = sim.prepare().unwrap();
        let settings = jobs[0].engine_settings();
        assert_eq!(settings.receiver_mode, ReceiverMode::MerlinChannelEstimated);
        assert_eq!(settings.mce.averaging_symbols, 32);
        assert_eq!(settings.mce.minimum_variance, 1e-7);
        assert_eq!(settings.impairments.power.amplifier_gain_db, 3.0);
        assert_eq!(jobs[0].sources().rxmer_ids().len(), 1);
        assert!(matches!(
            settings.channel_filter,
            docsis_ofdm_fec::signal_processing::channel::filter::ChannelFilterSettings::Echoes { .. }
        ));
    }
    value["sections"]["measurements"]["options"]["mode"] = json!("single");
    assert!(load(&value).is_ok());
}
#[test]
fn rejects_corrupt_unknown_unsupported_and_out_of_range_settings() {
    let cases = [
        ("hash", ErrorCode::InvalidInput),
        ("timeline", ErrorCode::UnsupportedCapability),
        ("source", ErrorCode::UnsupportedCapability),
        ("version", ErrorCode::UnsupportedCapability),
        ("profile", ErrorCode::InvalidInput),
        ("selection", ErrorCode::InvalidInput),
        ("unknown", ErrorCode::InvalidInput),
        ("limit", ErrorCode::InvalidInput),
    ];
    for (case, code) in cases {
        let mut v = document();
        match case {
            "hash" => v["sections"]["measurements"]["rxmer"][131]["sha256"] = json!("bad"),
            "timeline" => v["sections"]["measurements"]["options"]["mode"] = json!("sequence_hold"),
            "source" => v["sections"]["source"]["options"]["mode"] = json!("file"),
            "version" => v["schema_version"] = json!(99),
            "profile" => v["sections"]["profile"]["profile_id"] = json!(15),
            "selection" => {
                v["sections"]["measurements"]["options"]["selected_captures"] = json!([])
            }
            "unknown" => v["sections"]["surprise"] = json!({}),
            "limit" => v["receiver"]["duration_ms"] = json!(0),
            _ => unreachable!(),
        }
        assert_eq!(load(&v).err().unwrap().code(), code, "{case}");
    }
}
#[test]
fn saved_profile_precedence_legacy_and_current_and_cancellation() {
    let mut v = document();
    v["receiver"]["all_profiles"] = json!(false);
    assert_eq!(load(&v).unwrap().controllers().len(), 1);
    v["receiver"]["selected_profile_ids"] = json!([3]);
    assert!(load(&v).is_err());
    v["schema_version"] = json!(2);
    v["receiver"]["all_profiles"] = json!(true);
    let mut wb = load(&v).unwrap();
    assert_eq!(
        wb.controllers()[0].profile().unwrap().profile_id().value(),
        3
    );
    let control = RunControl::new();
    control.cancel();
    let results = wb.run_with_control(&control).unwrap();
    assert!(results[0].iter().all(|entry| entry.report().is_none()));
}
#[test]
fn real_saved_workbench_fec_matches_independently_observed_native_ui() {
    let mut wb = Workbench::from_bytes(WB).unwrap();
    let reports = wb.run().unwrap();
    for (batch, (total, corrected)) in reports.iter().zip([(83, 0), (114, 5), (124, 19)]) {
        let report = batch.iter().next().unwrap().report().unwrap();
        let fec = report.fec().unwrap();
        assert_eq!(fec.total_codewords().value(), total);
        assert_eq!(fec.corrected_codewords().value(), corrected);
        assert_eq!(fec.uncorrectable_codewords().value(), 0);
    }
}

#[test]
fn imports_single_and_multi_workbenches_with_measured_channel_response() {
    for bytes in [
        include_bytes!("../../../examples/workbench/downstream/single/workbench.merlin.json")
            .as_slice(),
        include_bytes!("../../../examples/workbench/downstream/multi-rxmer/workbench.merlin.json")
            .as_slice(),
    ] {
        let wb = Workbench::from_bytes(bytes).unwrap();
        for sim in wb.controllers() {
            let prepared = sim.prepare().unwrap();
            assert!(prepared[0].engine_settings().impairments.response.is_some());
        }
    }
}

#[test]
fn resource_limits_reject_embedded_counts_lengths_and_oversize_files() {
    let mut value = document();
    value["sections"]["profile"]["file"]["byte_length"] = json!(16 * 1024 * 1024 + 1);
    assert_eq!(load(&value).err().unwrap().code(), ErrorCode::ResourceLimit);
    let mut value = document();
    let capture = value["sections"]["measurements"]["rxmer"][0].clone();
    value["sections"]["measurements"]["rxmer"] = json!(vec![capture; 257]);
    assert_eq!(load(&value).err().unwrap().code(), ErrorCode::ResourceLimit);
    let file = tempfile::NamedTempFile::new().unwrap();
    file.as_file()
        .set_len(pnm_input::workbench::MAX_WORKBENCH_BYTES as u64 + 1)
        .unwrap();
    assert_eq!(
        Workbench::from_file(file.path()).err().unwrap().code(),
        ErrorCode::ResourceLimit
    );
}

#[test]
fn cablelabs_notch_fixture_preserves_frequency_plan_and_unity() {
    use base64::{engine::general_purpose::STANDARD, Engine};
    const NOTCH: &[u8] = include_bytes!(
        "../../../examples/workbench/downstream/cl-192mhz-8mhz-notch/workbench.merlin.json"
    );
    const FLAT: &[u8] = include_bytes!(
        "../../../examples/workbench/downstream/cl-192mhz-8mhz-notch/baseline.merlin.json"
    );
    for (bytes, has_notch) in [(NOTCH, true), (FLAT, false)] {
        let wb = Workbench::from_bytes(bytes).unwrap();
        assert_eq!(wb.controllers().len(), 1);
        let sim = &wb.controllers()[0];
        assert_eq!(sim.profile().unwrap().profile_id().value(), 3);
        assert_eq!(
            sim.options().receiver_mode,
            ReceiverMode::CmChannelEstimated
        );
        assert_eq!(sim.options().observation.value(), 10);
        let prepared = sim.prepare().unwrap();
        assert_eq!(
            prepared[0]
                .engine_settings()
                .transmission
                .occupied_bandwidth_hz,
            192_000_000
        );
        assert!(prepared[0].engine_settings().impairments.response.is_none());
        let doc: Value = serde_json::from_slice(bytes).unwrap();
        let decode = |file: &Value| -> Value {
            serde_json::from_slice(&STANDARD.decode(file["base64"].as_str().unwrap()).unwrap())
                .unwrap()
        };
        let profile = decode(&doc["sections"]["profile"]["file"]);
        let rxmer = decode(&doc["sections"]["measurements"]["rxmer"][0]);
        let zero = rxmer["subcarrier_zero_frequency"].as_u64().unwrap();
        let first = rxmer["first_active_subcarrier_index"].as_u64().unwrap();
        let spacing = rxmer["subcarrier_spacing"].as_u64().unwrap();
        assert_eq!(zero + first * spacing, 300_000_000);
        assert_eq!(spacing, 25_000);
        let values = rxmer["values"].as_array().unwrap();
        assert_eq!(values.len(), 7600);
        assert_eq!(zero + (first + values.len() as u64) * spacing, 490_000_000);
        let mut low_count = 0;
        for (offset, value) in values.iter().enumerate() {
            let hz = zero + (first + offset as u64) * spacing;
            let expected = if has_notch && (371_000_000..379_000_000).contains(&hz) {
                low_count += 1;
                10.0
            } else {
                42.0
            };
            assert_eq!(value.as_f64().unwrap(), expected);
        }
        assert_eq!(low_count, if has_notch { 320 } else { 0 });
        let mut bin = first;
        let mut plc_frequencies = Vec::new();
        for scheme in profile["profiles"][0]["schemes"].as_array().unwrap() {
            let count = scheme["num_subcarriers"].as_u64().unwrap();
            match scheme["modulation_order_code"].as_u64().unwrap() {
                20 => {
                    plc_frequencies.extend((bin..bin + count).map(|index| zero + index * spacing))
                }
                1 | 12 => {}
                unexpected => panic!("Unexpected carrier code {unexpected}"),
            }
            bin += count;
        }
        assert_eq!(bin, first + 7600);
        assert_eq!(plc_frequencies.len(), 16);
        assert_eq!(plc_frequencies.first(), Some(&332_800_000));
        assert_eq!(plc_frequencies.last(), Some(&333_175_000));
    }
}

#[test]
fn cablelabs_adapted_channel_prepares_without_changing_notch_settings() {
    use base64::{engine::general_purpose::STANDARD, Engine};
    let bytes = include_bytes!(
        "../../../examples/workbench/downstream/cl-192mhz-8mhz-notch/cmce.merlin.json"
    );
    let mut adapted: Value = serde_json::from_slice(bytes).unwrap();
    let original: Value = serde_json::from_slice(include_bytes!(
        "../../../examples/measurements/single-channel/channel_estimation.json"
    ))
    .unwrap();
    let response: Value = serde_json::from_slice(
        &STANDARD
            .decode(
                adapted["sections"]["measurements"]["response"]["base64"]
                    .as_str()
                    .unwrap(),
            )
            .unwrap(),
    )
    .unwrap();
    let samples = response["iq"].as_array().unwrap();
    assert_eq!(samples.len(), 7600);
    assert_eq!(samples.first(), original["iq"].as_array().unwrap().first());
    assert_eq!(samples.last(), original["iq"].as_array().unwrap().last());
    assert_eq!(response["subcarrier_zero_frequency"], 292_600_000);
    assert_eq!(response["first_active_subcarrier_index"], 296);
    assert_eq!(response["subcarrier_spacing"], 25_000);
    let wb = Workbench::from_bytes(bytes).unwrap();
    assert_eq!(
        wb.controllers()[0].options().receiver_mode,
        ReceiverMode::CmChannelEstimated
    );
    let prepared = wb.controllers()[0].prepare().unwrap();
    assert!(prepared[0].engine_settings().impairments.response.is_some());
    adapted["sections"]["measurements"]["response"] = Value::Null;
    let unity: Value = serde_json::from_slice(include_bytes!(
        "../../../examples/workbench/downstream/cl-192mhz-8mhz-notch/workbench.merlin.json"
    ))
    .unwrap();
    assert_eq!(adapted, unity);
}
