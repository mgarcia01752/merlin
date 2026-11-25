/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Dedicated public-API coverage for CmDsOfdmModulationProfile.
mod common;
use common::{assert_fixture_contract, failure, json_capture};
use pnm::{
    diagnostic::ErrorCode,
    pnm::parser::{
        cm_ds_ofdm_modulation_profile::CmDsOfdmModulationProfile, parse_limits::ParseLimits,
    },
};
use serde_json::json;
const MODULATION: &[u8] = pnm_fixtures::MODULATION_PROFILE;
#[test]
fn public_parser_and_input_contract() {
    let capture = CmDsOfdmModulationProfile::parse(MODULATION, ParseLimits::default()).unwrap();
    assert_eq!(
        serde_json::from_str::<serde_json::Value>(&capture.to_json(4_000_000).unwrap()).unwrap(),
        json_capture(MODULATION)
    );
    assert_fixture_contract(MODULATION);
}
#[test]
fn golden_fixture() {
    let modulation = json_capture(MODULATION);
    let profiles = modulation["profiles"].as_array().unwrap();
    assert_eq!(profiles.len(), 4);
    assert_eq!(profiles[0]["profile_id"], 3);
    assert_eq!(
        profiles[0]["schemes"][0],
        json!({"schema_type":"range","modulation_order":"qam_4096",
            "modulation_order_code":12,"bit_loading":12,"carrier_type":"data",
            "constellation":"qam_4096","shannon_minimum_mer_db":36.12253906096437,"preamble":null,
            "num_subcarriers":110})
    );
}
#[test]
fn malformed_lengths_and_boundaries() {
    let mut profiles = MODULATION.to_vec();
    profiles[17] += 1;
    assert_eq!(failure(&profiles), ErrorCode::Truncated);
    profiles = MODULATION.to_vec();
    profiles[32] = 255;
    assert_eq!(failure(&profiles), ErrorCode::UnsupportedLayout);
}
use pnm::pnm::parser::pnm_file::PnmFile;
#[test]
fn synthetic_skip_modulation_preserves_unknown_assignments() {
    let mut bytes = MODULATION[..29].to_vec();
    bytes[17] = 1;
    bytes[25..29].copy_from_slice(&8_u32.to_be_bytes());
    bytes.extend_from_slice(&[7, 0, 5, 1, 255, 2, 0, 9]);
    let output = json_capture(&bytes);
    assert_eq!(
        output["profiles"][0]["schemes"][0],
        json!({"schema_type":"skip","main_modulation_order":"unknown",
            "main_modulation_order_code":255,"main_bit_loading":null,"main_carrier_type":"unknown",
            "skip_modulation_order":"qpsk","skip_modulation_order_code":2,
            "skip_bit_loading":2,"skip_carrier_type":"data","num_subcarriers":9,
            "main_constellation":null,"main_shannon_minimum_mer_db":null,"main_preamble":null,
            "skip_constellation":"qpsk","skip_shannon_minimum_mer_db":4.771212547196624,"skip_preamble":null})
    );
    assert!(output["profiles"][0]["bandwidth_bps"].is_null());
    for end in 29..bytes.len() {
        assert!(PnmFile::from_bytes(&bytes[..end])
            .decode(ParseLimits::default())
            .is_err());
    }
}

#[test]
fn assignment_json_covers_ossi_roles_and_all_wire_codes() {
    use pnm::pnm::parser::cm_ds_ofdm_modulation_profile::ModulationScheme;
    // Independent expected assignments from I27 / I13 D.2.1.7.
    // PHY I20 7.5.13 / 7.5.15.3 supplies PLC/pilot constellation loading.
    let known = [
        (0, "zero_bit_loaded", Some(0), "zero_bit_loaded"),
        (1, "continuous_pilot", Some(1), "continuous_pilot"),
        (2, "qpsk", Some(2), "data"),
        (3, "reserved_3", None, "reserved"),
        (4, "qam_16", Some(4), "data"),
        (5, "reserved_5", None, "reserved"),
        (6, "qam_64", Some(6), "data"),
        (7, "qam_128", Some(7), "data"),
        (8, "qam_256", Some(8), "data"),
        (9, "qam_512", Some(9), "data"),
        (10, "qam_1024", Some(10), "data"),
        (11, "qam_2048", Some(11), "data"),
        (12, "qam_4096", Some(12), "data"),
        (13, "qam_8192", Some(13), "data"),
        (14, "qam_16384", Some(14), "data"),
        (16, "exclusion", None, "exclusion"),
        (20, "plc", Some(4), "plc"),
    ];
    for code in u8::MIN..=u8::MAX {
        let (_, name, bits, carrier) = known
            .iter()
            .find(|entry| entry.0 == code)
            .copied()
            .unwrap_or((code, "unknown", None, "unknown"));
        let mut range = serde_json::to_value(ModulationScheme::Range {
            modulation_order: code,
            num_subcarriers: 7,
        })
        .unwrap();
        let expected_constellation = match code {
            1 => Some("bpsk"),
            20 => Some("qam_16"),
            2 | 4 | 6..=14 => Some(name),
            _ => None,
        };
        assert_eq!(range["constellation"], json!(expected_constellation));
        assert_eq!(
            range["shannon_minimum_mer_db"].is_number(),
            expected_constellation.is_some()
        );
        assert_eq!(range["preamble"].is_object(), code == 20);
        for field in ["constellation", "shannon_minimum_mer_db", "preamble"] {
            range.as_object_mut().unwrap().remove(field);
        }
        assert_eq!(
            range,
            json!({"schema_type":"range","modulation_order":name,
            "modulation_order_code":code,"bit_loading":bits,"carrier_type":carrier,
            "num_subcarriers":7})
        );
        let mut skip = serde_json::to_value(ModulationScheme::Skip {
            main_modulation_order: code,
            skip_modulation_order: code,
            num_subcarriers: 7,
        })
        .unwrap();
        for prefix in ["main_", "skip_"] {
            assert_eq!(
                skip[format!("{prefix}constellation")],
                json!(expected_constellation)
            );
            assert_eq!(
                skip[format!("{prefix}shannon_minimum_mer_db")].is_number(),
                expected_constellation.is_some()
            );
            assert_eq!(skip[format!("{prefix}preamble")].is_object(), code == 20);
            for field in ["constellation", "shannon_minimum_mer_db", "preamble"] {
                skip.as_object_mut()
                    .unwrap()
                    .remove(&format!("{prefix}{field}"));
            }
        }
        assert_eq!(
            skip,
            json!({"schema_type":"skip","main_modulation_order":name,
            "main_modulation_order_code":code,"main_bit_loading":bits,"main_carrier_type":carrier,
            "skip_modulation_order":name,"skip_modulation_order_code":code,
            "skip_bit_loading":bits,"skip_carrier_type":carrier,"num_subcarriers":7})
        );
    }
}

#[test]
fn profile_capacity_json_matches_independent_fixture_totals() {
    let output = json_capture(MODULATION);
    for (profile, (bits, rate)) in output["profiles"].as_array().unwrap().iter().zip([
        (88_896, 2_222_400_000_u64),
        (81_488, 2_037_200_000),
        (74_080, 1_852_000_000),
        (59_264, 1_481_600_000),
    ]) {
        let analysis = &profile["analysis"];
        assert_eq!(analysis["data_subcarrier_count"], 7408);
        assert_eq!(analysis["data_bits_per_symbol"], bits);
        assert_eq!(analysis["data_bandwidth_hz"], 185_200_000);
        assert_eq!(analysis["theoretical_throughput_bps"], rate);
        assert_eq!(profile["bandwidth_bps"], rate);
        assert_eq!(analysis["useful_symbol_duration_s"], 0.00004);
        assert_eq!(analysis["complete"], true);
        assert_eq!(analysis["unclassified_subcarrier_count"], 0);
        assert_eq!(analysis["basis"], "ideal_gross_useful_symbol");
    }
}
#[test]
fn compressed_capacity_handles_odd_skip_specials_and_unknowns() {
    use pnm::pnm::{
        analysis::modulation_profile::ModulationProfileAnalysis,
        parser::cm_ds_ofdm_modulation_profile::{ModulationProfile, ModulationScheme},
    };
    let mut profile = ModulationProfile {
        profile_id: 0,
        schemes: vec![
            ModulationScheme::Skip {
                main_modulation_order: 8,
                skip_modulation_order: 10,
                num_subcarriers: 5,
            },
            ModulationScheme::Range {
                modulation_order: 1,
                num_subcarriers: 50,
            },
            ModulationScheme::Range {
                modulation_order: 20,
                num_subcarriers: 16,
            },
            ModulationScheme::Range {
                modulation_order: 16,
                num_subcarriers: 4,
            },
            ModulationScheme::Range {
                modulation_order: 0,
                num_subcarriers: 7,
            },
        ],
    };
    let analysis = ModulationProfileAnalysis::calculate(&profile, 25);
    assert_eq!(analysis.data_subcarrier_count, 5);
    assert_eq!(analysis.data_bits_per_symbol, 44); // 3*8 + 2*10.
    assert_eq!(analysis.data_bandwidth_hz, Some(125_000));
    assert_eq!(analysis.theoretical_throughput_bps, Some(1_100_000));
    assert!(analysis.complete);
    profile.schemes.push(ModulationScheme::Skip {
        main_modulation_order: 255,
        skip_modulation_order: 3,
        num_subcarriers: 3,
    });
    let unknown = ModulationProfileAnalysis::calculate(&profile, 25);
    assert!(!unknown.complete);
    assert_eq!(unknown.unclassified_subcarrier_count, 3);
    assert_eq!(unknown.theoretical_throughput_bps, None);
    assert_eq!(unknown.data_subcarrier_count, 5);
    assert_eq!(
        ModulationProfileAnalysis::calculate(&profile, 0).useful_symbol_duration_s,
        None
    );
    let empty = ModulationProfile {
        profile_id: 0,
        schemes: vec![],
    };
    assert_eq!(
        ModulationProfileAnalysis::calculate(&empty, 50).theoretical_throughput_bps,
        Some(0)
    );
}

#[test]
fn plc_and_continuous_pilots_export_physical_modulation_without_data_capacity() {
    use pnm::pnm::parser::cm_ds_ofdm_modulation_profile::ModulationScheme;
    // PHY I20 7.5.13: PLC 16-QAM data plus BPSK preamble;
    // 7.5.15.3: continuous pilots are BPSK. Not measured/required MER.
    let output = serde_json::to_value(ModulationScheme::Skip {
        main_modulation_order: 20,
        skip_modulation_order: 1,
        num_subcarriers: 3,
    })
    .unwrap();
    assert_eq!(output["main_modulation_order"], "plc");
    assert_eq!(output["main_constellation"], "qam_16");
    assert_eq!(output["main_bit_loading"], 4);
    assert!(
        (output["main_shannon_minimum_mer_db"].as_f64().unwrap() - 11.760912590556813).abs()
            < 1e-10
    );
    assert_eq!(
        output["main_preamble"],
        json!({"constellation":"bpsk", "bit_loading":1, "shannon_minimum_mer_db":0.0})
    );
    assert_eq!(output["skip_carrier_type"], "continuous_pilot");
    assert_eq!(output["skip_constellation"], "bpsk");
    assert_eq!(output["skip_bit_loading"], 1);
    assert_eq!(output["skip_shannon_minimum_mer_db"], 0.0);
    assert!(output["skip_preamble"].is_null());
}
