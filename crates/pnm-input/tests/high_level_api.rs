/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Fixture-backed high-level API admission, static selection and direct-engine parity.
use docsis_ofdm_fec::{
    api::*,
    simulation::{
        channel::{run_channel, ChannelJobLimits},
        test_points::{NoopObserver, TestPointProbe},
    },
};
use pnm_input::CaptureLoader;
use std::sync::atomic::AtomicBool;
fn input(name: &str, bytes: &[u8]) -> NamedCapture {
    NamedCapture::new(name, bytes.to_vec()).unwrap()
}
const PROFILE: &[u8] =
    include_bytes!("../../../examples/measurements/single-channel/modulation_profile.json");
const RX: &[u8] = include_bytes!("../../../examples/measurements/single-channel/rxmer.json");
const CH: &[u8] =
    include_bytes!("../../../examples/measurements/single-channel/channel_estimation.json");
fn profile() -> Dpd {
    Dpd::from_bytes(&input("profile", PROFILE), ProfileId::PROFILE_A).unwrap()
}
fn ocd() -> Ocd {
    Ocd::from_profile(
        ProtocolScope::Docsis31,
        &profile(),
        OcdOverrides {
            bandwidth: Some(BandwidthHz::new(192_000_000).unwrap()),
            ..Default::default()
        },
    )
    .unwrap()
}
fn rx(epoch: u32, mer: f64) -> RxMer {
    let mut json: serde_json::Value = serde_json::from_slice(RX).unwrap();
    json["pnm_header"]["capture_time"] = epoch.into();
    for v in json["values"].as_array_mut().unwrap() {
        *v = mer.into();
    }
    RxMer::from_bytes(&input("rx", &serde_json::to_vec(&json).unwrap())).unwrap()
}
fn controller() -> Merlin {
    let mut sim = Merlin::new(ocd());
    sim.set_profile(profile()).unwrap();
    sim.set_options(
        EvaluationOptions::default()
            .with_channel_selection(ChannelSelection::Unity)
            .with_profile_tolerance(AssociationTolerance::new(u32::MAX).unwrap()),
    )
    .unwrap();
    sim
}
#[test]
fn binary_json_all_kinds_and_wrong_kind() {
    for (binary, json, kind) in [
        (
            include_bytes!("../../../examples/workbench/downstream/single/modulation-profile.bin")
                .as_slice(),
            PROFILE,
            10,
        ),
        (
            include_bytes!("../../../examples/workbench/downstream/single/rxmer.bin").as_slice(),
            RX,
            4,
        ),
        (
            include_bytes!("../../../examples/workbench/downstream/single/ds-channel-estimate.bin")
                .as_slice(),
            CH,
            2,
        ),
    ] {
        let original = input("wrong.json", binary);
        let decoded = input("wrong.bin", json);
        match kind {
            10 => {
                let a = CaptureLoader::profile(&original, ProfileId::PROFILE_A).unwrap();
                let b = CaptureLoader::profile(&decoded, ProfileId::PROFILE_A).unwrap();
                assert_eq!(a.grid(), b.grid());
                for captured in [&a, &b] {
                    let ocd = Ocd::from_profile(
                        ProtocolScope::Docsis31,
                        captured,
                        OcdOverrides::default(),
                    )
                    .unwrap();
                    assert_eq!(
                        ocd.configuration().bandwidth().origin(),
                        SettingOrigin::Derived
                    );
                    assert_eq!(ocd.configuration().bandwidth().value().value(), 192_000_000);
                }
                assert_eq!(a.sources()[0].original_bytes(), binary);
                assert_ne!(
                    a.sources()[0].original_sha256(),
                    a.sources()[0].decoded_sha256()
                );
            }
            4 => {
                let a = CaptureLoader::rxmer(&original).unwrap();
                let b = CaptureLoader::rxmer(&decoded).unwrap();
                assert_eq!(a.grid(), b.grid());
                assert_eq!(a.values(), b.values());
                assert_eq!(a.provenance().original_bytes(), binary);
            }
            _ => {
                let a = CaptureLoader::channel_estimate(&original).unwrap();
                let b = CaptureLoader::channel_estimate(&decoded).unwrap();
                assert_eq!(a.grid(), b.grid());
                assert_eq!(a.provenance().original_bytes(), binary);
            }
        }
    }
    assert!(CaptureLoader::rxmer(&input("profile", PROFILE)).is_err());
    assert!(CaptureLoader::profile(&input("rxmer", RX), ProfileId::PROFILE_A).is_err());
    let contract = docsis_ofdm_fec::scenario::pnm_json::DecoderContract {
        revision: 1,
        producer: "unknown".into(),
        producer_version: "0.3.6".into(),
    };
    assert_eq!(
        RxMer::from_decoded(&input("rx", RX), &input("rx", RX), Some(contract))
            .unwrap_err()
            .code(),
        ErrorCode::UnsupportedDecoderContract
    );
}
#[test]
fn defaults_derived_grid_overrides_and_immutable_edit() {
    let derived =
        Ocd::from_profile(ProtocolScope::Docsis31, &profile(), OcdOverrides::default()).unwrap();
    assert_eq!(
        derived.configuration().bandwidth().origin(),
        SettingOrigin::Derived
    );
    assert_eq!(
        *derived.configuration().bandwidth().value(),
        *ocd().configuration().bandwidth().value()
    );
    assert_eq!(
        serde_json::to_value(&derived).unwrap()["transmission"],
        serde_json::to_value(ocd()).unwrap()["transmission"]
    );
    let ocd = ocd();
    assert_eq!(
        ocd.configuration().bandwidth().origin(),
        SettingOrigin::Explicit
    );
    let json = serde_json::to_value(&ocd).unwrap();
    assert_eq!(json["transmission"]["cyclic_prefix_samples"], 512);
    assert_eq!(json["transmission"]["roll_off_samples"], 128);
    assert_eq!(
        json["configuration"]["interleaver_depth"]["origin"],
        "Defaulted"
    );
    let p = profile();
    let changed = p
        .with_modulation(
            CarrierRange::new(
                SubcarrierIndex::new(356).unwrap(),
                SubcarrierIndex::new(400).unwrap(),
            )
            .unwrap(),
            DataModulation::Qam64,
        )
        .unwrap();
    assert_eq!(changed.origin(), ProfileOrigin::Edited);
    assert_eq!(changed.profile_ids(), p.profile_ids());
    assert_eq!(p.origin(), ProfileOrigin::Captured);
    assert_eq!(p.sources()[0].original_bytes(), PROFILE);
    let created = Dpd::from_ocd(&ocd, ProfileId::PROFILE_A, DataModulation::Qam256).unwrap();
    assert_eq!(created.origin(), ProfileOrigin::Created);
    assert_eq!(created.profile_ids(), &[ProfileId::PROFILE_A]);
    assert!(created.sources().is_empty());
    let mut sim = Merlin::new(ocd);
    sim.set_profile(created).unwrap();
    assert!(PrbsSeed::new(0).is_err());
    assert!(MerDb::new(f64::NAN).is_err());
    assert!(BandwidthHz::new(187_000_000).is_err());
}
#[test]
fn static_policies_power_math_ids_epochs_and_loading_does_not_run() {
    let mut sim = controller();
    let first = sim.add_rxmer(rx(100, 20.0)).unwrap();
    sim.add_rxmer(rx(110, 30.0)).unwrap();
    let options = sim.options().clone();
    let plan = sim.prepare().unwrap();
    assert_eq!(plan.len(), 2);
    assert_eq!(sim.rxmer(first).unwrap().values()[0].unwrap().value(), 20.0);
    for policy in [
        RxMerSelection::AverageAll,
        RxMerSelection::InterpolateAt(CaptureTime::new(105).unwrap()),
    ] {
        sim.set_options(options.clone().with_rxmer_selection(policy))
            .unwrap();
        let plan = sim.prepare().unwrap();
        assert_eq!(plan.len(), 1);
        let docsis_ofdm_fec::signal_processing::channel::impairments::NoiseModel::Rxmer { frame } =
            &plan[0].engine_settings().impairments.noise
        else {
            panic!("not RxMER")
        };
        assert!((frame.mer_db[0].unwrap() - 22.59637310505756).abs() < 1e-10);
    }
    sim.set_options(
        options
            .clone()
            .with_rxmer_selection(RxMerSelection::Individual(first)),
    )
    .unwrap();
    assert_eq!(sim.prepare().unwrap().len(), 1);
    let other = controller();
    assert_eq!(
        other.rxmer(first).unwrap_err().code(),
        ErrorCode::InvalidIdentifier
    );
    sim.set_options(
        options
            .clone()
            .with_rxmer_selection(RxMerSelection::InterpolateAt(CaptureTime::new(99).unwrap())),
    )
    .unwrap();
    assert!(sim.prepare().is_err());
    sim.add_rxmer(rx(110, 40.0)).unwrap();
    sim.set_options(options.with_rxmer_selection(RxMerSelection::InterpolateAt(
        CaptureTime::new(110).unwrap(),
    )))
    .unwrap();
    assert_eq!(
        sim.prepare().err().unwrap().code(),
        ErrorCode::AmbiguousTimestamp
    );
}
#[test]
fn cancellation_preserves_all_requested_entries_without_execution() {
    let mut sim = controller();
    sim.add_rxmer(rx(100, 40.0)).unwrap();
    sim.add_rxmer(rx(110, 40.0)).unwrap();
    let control = RunControl::new();
    control.cancel();
    let report = sim.run_with_control(&control).unwrap();
    assert_eq!(report.status(), BatchStatus::Cancelled);
    assert_eq!(report.iter().count(), 2);
    assert!(report
        .iter()
        .all(|e| e.status() == RunStatus::NotStarted && e.report().is_none()));
}
#[test]
fn high_level_api_matches_direct_engine_and_owns_old_reports() {
    // Compare both explicit unity and a measured channel response.
    for measured_response in [false, true] {
        let mut sim = controller();
        sim.add_rxmer(RxMer::from_bytes(&input("rx", RX)).unwrap())
            .unwrap();
        let mut options = sim.options().clone();
        if measured_response {
            let channel = sim
                .add_channel_estimate(ChannelEstimate::from_bytes(&input("H", CH)).unwrap())
                .unwrap();
            options.channel_selection = Some(ChannelSelection::Explicit(channel));
        }
        options.observation = ObservationDuration::new(1).unwrap();
        options.limits.runtime = RuntimeLimit::new(300_000).unwrap();
        sim.set_options(options.clone()).unwrap();
        let prepared = sim.prepare().unwrap();
        let direct = run_channel(
            prepared[0].engine_settings(),
            &ChannelJobLimits {
                max_runtime_ms: 300_000,
                max_retained_words: 64,
            },
            &AtomicBool::new(false),
            &mut TestPointProbe::disabled(),
            &mut NoopObserver,
            |_| {},
        )
        .unwrap();
        let batch = sim.run().unwrap();
        let report = batch.iter().next().unwrap().report().unwrap();
        let engine = report.diagnostics().unwrap();
        assert_eq!(engine.status, direct.status);
        assert_eq!(engine.summary.fec, direct.summary.fec);
        assert_eq!(engine.summary.comparison, direct.summary.comparison);
        assert_eq!(
            engine.summary.window.as_ref().unwrap().fec,
            direct.summary.window.as_ref().unwrap().fec
        );
        assert_eq!(
            engine.summary.window.as_ref().unwrap().ber,
            direct.summary.window.as_ref().unwrap().ber
        );
        assert_eq!(
            report.fec().unwrap().scope(),
            report.profile().unwrap().scope()
        );
        assert_eq!(
            engine.summary.window.as_ref().unwrap().payload_rate_bps,
            direct.summary.window.as_ref().unwrap().payload_rate_bps
        );
        let before = batch.to_json().unwrap();
        sim.set_options(options.with_rxmer_selection(RxMerSelection::AverageAll))
            .unwrap();
        assert_eq!(before, batch.to_json().unwrap());
    }
}

#[test]
fn channel_nearest_explicit_reuse_ties_and_unset_policy() {
    let mut sim = controller();
    sim.add_rxmer(rx(100, 40.0)).unwrap();
    let channel_at = |epoch: u32| {
        let mut json: serde_json::Value = serde_json::from_slice(CH).unwrap();
        json["pnm_header"]["capture_time"] = epoch.into();
        ChannelEstimate::from_bytes(&input("H", &serde_json::to_vec(&json).unwrap())).unwrap()
    };
    let channel = sim.add_channel_estimate(channel_at(90)).unwrap();
    let options = sim.options().clone();
    sim.set_options(
        options
            .clone()
            .with_channel_selection(ChannelSelection::Explicit(channel)),
    )
    .unwrap();
    assert!(sim.prepare().is_ok());
    sim.set_options(
        options
            .clone()
            .with_channel_selection(ChannelSelection::NearestWithin(
                AssociationTolerance::new(10).unwrap(),
            )),
    )
    .unwrap();
    assert!(sim.prepare().is_ok());
    sim.add_channel_estimate(channel_at(110)).unwrap();
    assert_eq!(
        sim.prepare().err().unwrap().code(),
        ErrorCode::AmbiguousTimestamp
    );
    sim.set_options(
        options
            .clone()
            .with_rxmer_selection(RxMerSelection::AverageAll)
            .with_channel_selection(ChannelSelection::NearestWithin(
                AssociationTolerance::new(10).unwrap(),
            )),
    )
    .unwrap();
    assert!(sim.prepare().is_err());
    let mut unset = options;
    unset.channel_selection = None;
    sim.set_options(unset).unwrap();
    assert_eq!(
        sim.prepare().err().unwrap().code(),
        ErrorCode::MissingSetting
    );
}
#[test]
fn nulls_grid_errors_preflight_and_atomic_additions() {
    let mut sim = controller();
    let mut json: serde_json::Value = serde_json::from_slice(RX).unwrap();
    json["values"][0] = serde_json::Value::Null;
    let missing =
        RxMer::from_bytes(&input("missing", &serde_json::to_vec(&json).unwrap())).unwrap();
    assert!(missing.values()[0].is_none());
    sim.add_rxmer(missing).unwrap();
    assert!(sim.prepare().is_err());
    let first = sim.add_rxmer(rx(100, 40.0)).unwrap();
    let count = sim.prepare().err().unwrap();
    assert!(!count.message().is_empty());
    assert_eq!(sim.rxmer(first).unwrap().values()[0].unwrap().value(), 40.0);
    let mut other = controller();
    json["values"][0] = 40.into();
    json["channel_id"] = 35.into();
    other
        .add_rxmer(
            RxMer::from_bytes(&input("different", &serde_json::to_vec(&json).unwrap())).unwrap(),
        )
        .unwrap();
    assert_eq!(
        other.prepare().err().unwrap().code(),
        ErrorCode::IncompatibleGrid
    );
    let oversized = NamedCapture::new("too big", vec![0; MAX_CAPTURE_BYTES + 1]);
    assert_eq!(oversized.unwrap_err().code(), ErrorCode::ResourceLimit);
}
#[test]
fn runtime_limit_returns_partial_and_continues_batch() {
    let mut sim = controller();
    sim.add_rxmer(rx(100, 40.0)).unwrap();
    sim.add_rxmer(rx(110, 40.0)).unwrap();
    let mut options = sim.options().clone();
    options.limits.runtime = RuntimeLimit::new(1).unwrap();
    sim.set_options(options).unwrap();
    let report = sim.run().unwrap();
    assert_eq!(report.status(), BatchStatus::CompletedWithErrors);
    assert_eq!(report.iter().count(), 2);
    for entry in report.iter() {
        assert_eq!(entry.status(), RunStatus::RuntimeExceeded);
        let report = entry.report().unwrap();
        assert_eq!(report.diagnostics().unwrap().summary.payload_ber, None);
        assert!(report.profile().is_none());
    }
}
#[test]
fn export_never_overwrites_existing_destination() {
    let output = std::env::temp_dir().join(format!("merlin-api-export-{}", std::process::id()));
    pnm_input::save_json(&output, "first").unwrap();
    assert_eq!(
        pnm_input::save_json(&output, "second").unwrap_err().code(),
        ErrorCode::Io
    );
    assert_eq!(std::fs::read_to_string(&output).unwrap(), "first");
    std::fs::remove_file(output).unwrap();
}

#[test]
fn created_profile_runs_as_hypothesis_without_fabricated_capture_provenance() {
    let ocd = ocd();
    let dpd = Dpd::from_ocd(&ocd, ProfileId::new(3).unwrap(), DataModulation::Qam256).unwrap();
    let edited = dpd
        .with_modulation(
            CarrierRange::new(
                SubcarrierIndex::new(356).unwrap(),
                SubcarrierIndex::new(400).unwrap(),
            )
            .unwrap(),
            DataModulation::Qam64,
        )
        .unwrap();
    let mut sim = Merlin::new(ocd);
    sim.set_profile(edited).unwrap();
    sim.add_rxmer(rx(100, 40.0)).unwrap();
    let mut options = EvaluationOptions::default().with_channel_selection(ChannelSelection::Unity);
    options.observation = ObservationDuration::new(1).unwrap();
    options.limits.runtime = RuntimeLimit::new(300_000).unwrap();
    sim.set_options(options).unwrap();
    let batch = sim.run().unwrap();
    assert_eq!(batch.status(), BatchStatus::Complete);
    let report = batch.iter().next().unwrap().report().unwrap();
    assert_eq!(report.profile().unwrap().profile_id().value(), 3);
    assert!(report.profile().unwrap().compared_bits().value() > 0);
    assert!(!batch.to_json().unwrap().contains("internal_hypothesis"));
}

#[test]
fn interpolation_uses_adjacent_unequal_intervals_and_exact_originals() {
    let mut sim = controller();
    sim.add_rxmer(rx(140, 40.0)).unwrap();
    sim.add_rxmer(rx(100, 20.0)).unwrap();
    let middle = sim.add_rxmer(rx(110, 30.0)).unwrap();
    let options = sim.options().clone();
    sim.set_options(
        options
            .clone()
            .with_rxmer_selection(RxMerSelection::InterpolateAt(
                CaptureTime::new(125).unwrap(),
            )),
    )
    .unwrap();
    let prepared = sim.prepare().unwrap();
    assert_eq!(
        prepared[0]
            .sources()
            .epochs()
            .iter()
            .map(|v| v.value())
            .collect::<Vec<_>>(),
        vec![110, 140]
    );
    assert_eq!(
        prepared[0]
            .sources()
            .weights()
            .iter()
            .map(|v| v.value())
            .collect::<Vec<_>>(),
        vec![0.5, 0.5]
    );
    let docsis_ofdm_fec::signal_processing::channel::impairments::NoiseModel::Rxmer { frame } =
        &prepared[0].engine_settings().impairments.noise
    else {
        panic!("expected RxMER")
    };
    assert!((frame.mer_db[0].unwrap() - 32.59637310505756).abs() < 1e-10);
    sim.set_options(options.with_rxmer_selection(RxMerSelection::InterpolateAt(
        CaptureTime::new(110).unwrap(),
    )))
    .unwrap();
    assert_eq!(sim.prepare().unwrap()[0].sources().rxmer_ids(), &[middle]);
    assert_eq!(
        sim.rxmer(middle).unwrap().values()[0].unwrap().value(),
        30.0
    );
}
#[test]
fn missing_contributors_cannot_turn_into_valid_averages_or_interpolations() {
    let mut sim = controller();
    let first = sim.add_rxmer(rx(100, 20.0)).unwrap();
    let mut missing: serde_json::Value = serde_json::from_slice(RX).unwrap();
    missing["pnm_header"]["capture_time"] = 110.into();
    missing["values"][0] = serde_json::Value::Null;
    sim.add_rxmer(
        RxMer::from_bytes(&input("missing", &serde_json::to_vec(&missing).unwrap())).unwrap(),
    )
    .unwrap();
    let options = sim.options().clone();
    for policy in [
        RxMerSelection::AverageAll,
        RxMerSelection::InterpolateAt(CaptureTime::new(105).unwrap()),
    ] {
        sim.set_options(options.clone().with_rxmer_selection(policy))
            .unwrap();
        assert!(sim.prepare().is_err());
    }
    sim.set_options(options.with_rxmer_selection(RxMerSelection::InterpolateAt(
        CaptureTime::new(100).unwrap(),
    )))
    .unwrap();
    assert_eq!(sim.prepare().unwrap()[0].sources().rxmer_ids(), &[first]);
}
#[test]
fn aggregate_budget_rejects_addition_and_profile_replacement_atomically() {
    let mut sim = controller();
    let frame = rx(100, 40.0);
    let first = sim.add_rxmer(frame.clone()).unwrap();
    let mut exhausted = false;
    for _ in 0..256 {
        match sim.add_rxmer(frame.clone()) {
            Ok(_) => {}
            Err(error) => {
                assert_eq!(error.code(), ErrorCode::ResourceLimit);
                exhausted = true;
                break;
            }
        }
    }
    assert!(exhausted);
    assert_eq!(sim.rxmer(first).unwrap().values()[0].unwrap().value(), 40.0);
    let original = sim.profile().unwrap().sources()[0]
        .original_sha256()
        .to_owned();
    let mut padded = PROFILE.to_vec();
    padded.resize(MAX_CAPTURE_BYTES, b' ');
    let large = Dpd::from_bytes(&input("large", &padded), ProfileId::PROFILE_A).unwrap();
    assert_eq!(
        sim.set_profile(large).unwrap_err().code(),
        ErrorCode::ResourceLimit
    );
    assert_eq!(
        sim.profile().unwrap().sources()[0].original_sha256(),
        original
    );
}

#[test]
fn individual_and_exact_epoch_keep_original_mer_values_without_round_trip() {
    let mut sim = controller();
    let capture = rx(100, 20.123456789012345);
    let expected = capture
        .values()
        .into_iter()
        .map(|v| v.map(|m| m.value()))
        .collect::<Vec<_>>();
    let first = sim.add_rxmer(capture).unwrap();
    let options = sim.options().clone();
    for selection in [
        RxMerSelection::Each,
        RxMerSelection::Individual(first),
        RxMerSelection::InterpolateAt(CaptureTime::new(100).unwrap()),
    ] {
        sim.set_options(options.clone().with_rxmer_selection(selection))
            .unwrap();
        let prepared = sim.prepare().unwrap();
        let docsis_ofdm_fec::signal_processing::channel::impairments::NoiseModel::Rxmer { frame } =
            &prepared[0].engine_settings().impairments.noise
        else {
            panic!("expected original RxMER")
        };
        assert_eq!(frame.mer_db, expected);
    }
}

#[test]
fn profile_catalog_discovery_matches_core_json_and_binary_without_selection() {
    let json = input("profiles.bin", PROFILE);
    let original = json.bytes().to_vec();
    let core = Dpd::profile_ids_from_bytes(&json).unwrap();
    assert_eq!(
        core.iter().map(|id| id.value()).collect::<Vec<_>>(),
        vec![0, 1, 2, 3]
    );
    assert_eq!(CaptureLoader::profile_ids(&json).unwrap(), core);
    let binary = input(
        "profiles.json",
        include_bytes!("../../../examples/workbench/downstream/single/modulation-profile.bin"),
    );
    assert_eq!(CaptureLoader::profile_ids(&binary).unwrap(), core);
    let collection = input("collection", include_bytes!("../../../examples/workbench/downstream/multi-rxmer-2/ds_ofdm_modulation_profile_aabbccddeeff_193_1764824373.bin"));
    let ids = CaptureLoader::profile_ids(&collection).unwrap();
    assert_eq!(
        ids.iter().map(|id| id.value()).collect::<Vec<_>>(),
        vec![0, 3, 4]
    );
    for id in ids {
        let profile = CaptureLoader::profile(&collection, id).unwrap();
        assert_eq!(profile.profile_id(), id);
        assert_eq!(
            profile.profile_ids(),
            CaptureLoader::profile_ids(&collection).unwrap()
        );
        assert_eq!(profile.sources()[0].original_bytes(), collection.bytes());
    }
    assert_eq!(json.bytes(), original);
}

#[test]
fn profile_catalog_rejects_malformed_empty_duplicate_and_out_of_domain_ids() {
    for input in [input("malformed", b"{"), input("wrong kind", RX)] {
        assert!(CaptureLoader::profile_ids(&input).is_err());
        assert!(Dpd::profile_ids_from_bytes(&input).is_err());
    }
    let original: serde_json::Value = serde_json::from_slice(PROFILE).unwrap();
    for case in ["empty", "duplicate", "unsupported id", "count"] {
        let mut document = original.clone();
        match case {
            "empty" => {
                document["profiles"] = serde_json::json!([]);
                document["num_profiles"] = 0.into();
            }
            "duplicate" => {
                document["profiles"][1]["profile_id"] =
                    document["profiles"][0]["profile_id"].clone();
            }
            "unsupported id" => {
                document["profiles"][0]["profile_id"] = 16.into();
            }
            "count" => {
                document["num_profiles"] = 1.into();
            }
            _ => unreachable!(),
        }
        let capture = input(case, &serde_json::to_vec(&document).unwrap());
        let core = Dpd::profile_ids_from_bytes(&capture).unwrap_err();
        let adapter = CaptureLoader::profile_ids(&capture).unwrap_err();
        assert_eq!(core.code(), adapter.code());
    }
    let mut corrupt =
        include_bytes!("../../../examples/workbench/downstream/single/modulation-profile.bin")
            .to_vec();
    corrupt.truncate(20);
    assert!(CaptureLoader::profile_ids(&input("truncated", &corrupt)).is_err());
}

#[test]
fn loaded_profile_catalog_survives_source_removal_and_serializes() {
    let path = std::env::temp_dir().join(format!("merlin-dpd-member-{}.json", std::process::id()));
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&path)
        .unwrap();
    std::io::Write::write_all(&mut file, PROFILE).unwrap();
    drop(file);
    let loaded = CaptureLoader::profile(
        &pnm_input::read_capture(&path).unwrap(),
        ProfileId::new(2).unwrap(),
    )
    .unwrap();
    std::fs::remove_file(&path).unwrap();
    assert_eq!(loaded.profile_id().value(), 2);
    assert_eq!(
        loaded
            .profile_ids()
            .iter()
            .map(|id| id.value())
            .collect::<Vec<_>>(),
        vec![0, 1, 2, 3]
    );
    assert_eq!(
        serde_json::to_value(&loaded).unwrap()["profile_ids"],
        serde_json::json!([0, 1, 2, 3])
    );
    assert_eq!(loaded.clone().profile_ids(), loaded.profile_ids());
    assert_eq!(loaded.sources()[0].original_bytes(), PROFILE);
}

#[test]
fn receiver_selection_is_independent_copied_and_forwarded_for_every_policy() {
    let mut sim = controller();
    let epoch = 1380970;
    let first = sim.add_rxmer(rx(epoch, 40.0)).unwrap();
    sim.add_rxmer(rx(epoch + 10, 42.0)).unwrap();
    assert_eq!(
        sim.options().receiver_mode,
        ReceiverMode::CmChannelEstimated
    );
    for mode in [
        ReceiverMode::CmChannelEstimated,
        ReceiverMode::PilotEstimated,
        ReceiverMode::MerlinChannelEstimated,
    ] {
        for selection in [
            RxMerSelection::Each,
            RxMerSelection::Individual(first),
            RxMerSelection::AverageAll,
            RxMerSelection::InterpolateAt(CaptureTime::new(epoch + 5).unwrap()),
        ] {
            let mut options = sim
                .options()
                .clone()
                .with_receiver_mode(mode)
                .with_rxmer_selection(selection);
            sim.set_options(options.clone()).unwrap();
            options.receiver_mode = ReceiverMode::CmChannelEstimated;
            assert_eq!(sim.options().receiver_mode, mode);
            for prepared in sim.prepare().unwrap() {
                assert_eq!(prepared.engine_settings().receiver_mode, mode);
                assert!(prepared.engine_settings().impairments.response.is_none());
            }
        }
    }
}
fn receiver_mode_matches_direct_engine(mode: ReceiverMode) {
    let mut sim = controller();
    sim.add_rxmer(RxMer::from_bytes(&input("rx", RX)).unwrap())
        .unwrap();
    let mut options = sim.options().clone().with_receiver_mode(mode);
    options.observation = ObservationDuration::new(1).unwrap();
    options.limits.runtime = RuntimeLimit::new(300_000).unwrap();
    sim.set_options(options).unwrap();
    let prepared = sim.prepare().unwrap();
    let direct = run_channel(
        prepared[0].engine_settings(),
        &ChannelJobLimits {
            max_runtime_ms: 300_000,
            max_retained_words: 64,
        },
        &AtomicBool::new(false),
        &mut TestPointProbe::disabled(),
        &mut NoopObserver,
        |_| {},
    )
    .unwrap();
    let batch = sim.run().unwrap();
    let report = batch.iter().next().unwrap().report().unwrap();
    assert_eq!(report.options().receiver_mode, mode);
    assert_eq!(report.diagnostics().unwrap().status, direct.status);
    assert_eq!(
        report.diagnostics().unwrap().summary.fec,
        direct.summary.fec
    );
    assert_eq!(
        report.diagnostics().unwrap().summary.comparison,
        direct.summary.comparison
    );
    assert_eq!(
        report
            .diagnostics()
            .unwrap()
            .summary
            .window
            .as_ref()
            .unwrap()
            .fec,
        direct.summary.window.as_ref().unwrap().fec
    );
    let before = batch.to_json().unwrap();
    assert!(before.contains(if mode == ReceiverMode::PilotEstimated {
        "Pilot-estimated correction"
    } else {
        "MCE uses received pilots"
    }));
    sim.set_options(
        sim.options()
            .clone()
            .with_receiver_mode(ReceiverMode::CmChannelEstimated),
    )
    .unwrap();
    assert_eq!(report.options().receiver_mode, mode);
    assert_eq!(batch.to_json().unwrap(), before);
}
#[test]
fn pilot_estimated_api_matches_direct_engine_fec_and_owns_mode() {
    receiver_mode_matches_direct_engine(ReceiverMode::PilotEstimated);
}
#[test]
fn merlin_estimated_api_matches_direct_engine_fec_and_owns_mode() {
    receiver_mode_matches_direct_engine(ReceiverMode::MerlinChannelEstimated);
}

#[test]
fn narrower_capture_keeps_bandwidth_explicit() {
    let mut json: serde_json::Value = serde_json::from_slice(PROFILE).unwrap();
    for profile in json["profiles"].as_array_mut().unwrap() {
        let last = profile["schemes"]
            .as_array_mut()
            .unwrap()
            .last_mut()
            .unwrap();
        last["num_subcarriers"] = (last["num_subcarriers"].as_u64().unwrap() - 40).into();
    }
    let capture = input("narrow-profile", &serde_json::to_vec(&json).unwrap());
    let profile = Dpd::from_bytes(&capture, ProfileId::PROFILE_A).unwrap();
    assert_eq!(
        Ocd::from_profile(ProtocolScope::Docsis31, &profile, OcdOverrides::default())
            .unwrap_err()
            .code(),
        ErrorCode::MissingSetting
    );
    let explicit = Ocd::from_profile(
        ProtocolScope::Docsis31,
        &profile,
        OcdOverrides {
            bandwidth: Some(BandwidthHz::new(192_000_000).unwrap()),
            ..Default::default()
        },
    )
    .unwrap();
    assert_eq!(
        explicit.configuration().bandwidth().origin(),
        SettingOrigin::Explicit
    );
}

#[test]
fn loaded_profiles_preserve_selection_ids_and_cancelled_entries() {
    let mut sim = CaptureLoader::controller_from_profiles(
        &input("profiles", PROFILE),
        ProtocolScope::Docsis31,
        OcdOverrides::default(),
    )
    .unwrap();
    let ids = sim.profile_ids();
    assert_eq!(
        ids.iter().map(|id| id.value()).collect::<Vec<_>>(),
        vec![0, 1, 2, 3]
    );
    sim.select_profile(ids[2]).unwrap();
    assert_eq!(sim.profile_by_id(ids[1]).unwrap().profile_id(), ids[1]);
    assert!(sim.select_profile(ProfileId::new(15).unwrap()).is_err());
    assert_eq!(sim.profile().unwrap().profile_id(), ids[2]);
    sim.add_rxmer(rx(0, 40.0)).unwrap();
    let mut options = EvaluationOptions::default()
        .with_channel_selection(ChannelSelection::Unity)
        .with_profile_tolerance(AssociationTolerance::new(u32::MAX).unwrap());
    options.observation = ObservationDuration::new(1).unwrap();
    sim.set_options(options).unwrap();
    let control = RunControl::new();
    control.cancel();
    let batch = sim.run_all_profiles_with_control(&control).unwrap();
    assert_eq!(
        batch
            .iter()
            .map(|entry| entry.profile_id())
            .collect::<Vec<_>>(),
        ids
    );
    assert!(batch
        .iter()
        .all(|entry| entry.status() == RunStatus::NotStarted));
    assert_eq!(sim.profile().unwrap().profile_id(), ids[2]);
    let snapshot = batch.to_json().unwrap();
    sim.select_profile(ids[0]).unwrap();
    assert_eq!(snapshot, batch.to_json().unwrap());
    for epoch in 1..65 {
        sim.add_rxmer(rx(epoch, 40.0)).unwrap();
    }
    assert_eq!(
        sim.run_all_profiles_with_control(&control)
            .err()
            .unwrap()
            .code(),
        ErrorCode::ResourceLimit
    );
    assert_eq!(sim.profile().unwrap().profile_id(), ids[0]);
    assert!(Merlin::from_profiles(
        ProtocolScope::Docsis31,
        vec![profile(), profile()],
        OcdOverrides::default()
    )
    .is_err());
}

#[test]
fn default_capture_protocol_matches_explicit_loading() {
    let capture = input(
        "profiles",
        include_bytes!("../../../examples/workbench/downstream/single/modulation-profile.bin"),
    );
    let default = CaptureLoader::controller_from_capture(&capture).unwrap();
    let explicit = CaptureLoader::controller_from_profiles(
        &capture,
        ProtocolScope::Docsis31,
        OcdOverrides::default(),
    )
    .unwrap();
    assert_eq!(ProtocolScope::default(), ProtocolScope::Docsis31);
    assert_eq!(default.profile_ids(), explicit.profile_ids());
    assert_eq!(
        format!("{:?}", default.ocd()),
        format!("{:?}", explicit.ocd())
    );
}

#[test]
fn simulation_loading_is_idle_atomic_and_preserves_scoped_measurements() {
    let capture = input("profiles", PROFILE);
    let mut sim = Simulation::new();
    assert!(sim.profile_ids().is_empty());
    assert!(sim.profile().is_none());
    assert!(sim.run().is_err());
    sim.load_profiles(CaptureLoader::profiles(&capture).unwrap())
        .unwrap();
    let ids = sim.profile_ids();
    let rx = sim
        .add_rxmer(CaptureLoader::rxmer(&input("rx", RX)).unwrap())
        .unwrap();
    sim.set_options(
        EvaluationOptions::default()
            .with_rxmer_selection(RxMerSelection::Individual(rx))
            .with_channel_selection(ChannelSelection::Unity),
    )
    .unwrap();
    assert!(sim.load_profiles(vec![]).is_err());
    let incompatible = input("other-channel", include_bytes!("../../../examples/workbench/downstream/multi-rxmer-2/ds_ofdm_modulation_profile_aabbccddeeff_193_1764824373.bin"));
    assert!(sim
        .load_profiles(CaptureLoader::profiles(&incompatible).unwrap())
        .is_err());
    assert_eq!(sim.profile_ids(), ids);
    sim.load_profiles(CaptureLoader::profiles(&capture).unwrap())
        .unwrap();
    assert!(sim.rxmer(rx).is_ok());
    let mut other = Simulation::new();
    other
        .load_profiles(CaptureLoader::profiles(&capture).unwrap())
        .unwrap();
    assert!(other.rxmer(rx).is_err());
    let token = RunControl::new();
    token.cancel();
    // Use the fixture's association allowance; cancellation prevents dispatch.
    let mut options = sim.options().unwrap().clone();
    options.profile_tolerance = AssociationTolerance::new(90_000).unwrap();
    sim.set_options(options).unwrap();
    assert!(sim.run_with_control(&token).is_ok());
}

#[test]
fn cpa_controls_validate_atomically_and_preserve_idle_settings() {
    let mut sim = Simulation::from(controller());
    let original = sim.options().unwrap().clone();
    sim.set_launch_amplifier_gain_db(12.0).unwrap();
    sim.set_cpa_attenuation_db(6.0).unwrap();
    assert_eq!(sim.get_launch_amplifier_gain_db().unwrap(), 12.0);
    assert_eq!(sim.get_cpa_attenuation_db().unwrap(), 6.0);
    assert_eq!(sim.options().unwrap().noise_seed, original.noise_seed);
    for invalid in [-1.0, 60.01, f64::NAN, f64::INFINITY] {
        assert_eq!(
            sim.set_cpa_attenuation_db(invalid).unwrap_err().code(),
            ErrorCode::InvalidInput
        );
        assert!(sim.set_launch_amplifier_gain_db(invalid).is_err());
        assert_eq!(sim.get_cpa_attenuation_db().unwrap(), 6.0);
        assert_eq!(sim.get_launch_amplifier_gain_db().unwrap(), 12.0);
    }
    for valid in [0.0, 60.0] {
        sim.set_cpa_attenuation_db(valid).unwrap();
        sim.set_launch_amplifier_gain_db(valid).unwrap();
        assert_eq!(sim.get_cpa_attenuation_db().unwrap(), valid);
    }
    let mut options = sim.options().unwrap().clone();
    options.processing.power.input_data_level_dbmv_per_6mhz = Some(0.0);
    options.processing.power.amplifier_target_dbmv_per_6mhz = Some(20.0);
    sim.set_options(options).unwrap();
    assert_eq!(sim.get_launch_amplifier_gain_db().unwrap(), 20.0);
    sim.set_launch_amplifier_gain_db(3.0).unwrap();
    assert_eq!(sim.get_launch_amplifier_gain_db().unwrap(), 3.0);
    assert!(sim
        .options()
        .unwrap()
        .processing
        .power
        .amplifier_target_dbmv_per_6mhz
        .is_none());
    assert!(Simulation::new().set_cpa_attenuation_db(1.0).is_err());
}

#[test]
fn cpa_serialization_preserves_saved_workbenches() {
    use docsis_ofdm_fec::signal_processing::channel::power::PowerConditioning;
    for key in ["attenuation_db", "cpa_attenuation_db"] {
        let power: PowerConditioning =
            serde_json::from_value(serde_json::json!({key: 6.0})).unwrap();
        assert_eq!(power.cpa_attenuation_db, 6.0);
        let json = serde_json::to_value(power).unwrap();
        assert_eq!(json["attenuation_db"], 6.0);
        assert!(json.get("cpa_attenuation_db").is_none());
    }
}

#[test]
fn borrowed_channel_frame_keeps_binary_original_and_decoded_provenance_distinct() {
    const ORIGINAL: &[u8] =
        include_bytes!("../../../examples/workbench/downstream/single/ds-channel-estimate.bin");
    let original = input("wrong.json", ORIGINAL);
    let capture = CaptureLoader::channel_estimate(&original).unwrap();
    let frame = capture.frame();

    // Independent source identity/size/epoch anchors from this retained capture.
    assert_eq!(frame.key.channel_id, 34);
    assert_eq!(frame.key.mac_address, [0, 202, 0, 0, 0, 1]);
    assert_eq!(frame.iq.len(), 7480);
    assert_eq!(frame.capture_time, 1391100);
    assert_eq!(frame.source.sha256, capture.provenance().decoded_sha256());
    assert_ne!(frame.source.sha256, capture.provenance().original_sha256());
    assert_eq!(capture.provenance().original_bytes(), ORIGINAL);
    assert_eq!(capture.source_kind(), "decoded_json_channel_response");
    let owned = frame.clone();
    drop(capture);
    drop(original);
    assert_eq!(owned.iq.len(), 7480);
}
