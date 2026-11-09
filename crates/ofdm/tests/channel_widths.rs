/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
#![cfg(feature = "simulation")]
#[path = "../examples/channel_widths/fixtures.rs"]
mod fixtures;
use docsis_ofdm_fec::{
    scenario::pnm_json::{import, JsonCapture},
    simulation::{
        channel::{
            document::ChannelDocument,
            measurement::{captured_layout, MeasurementSelection},
            ChannelPhase, ChannelRun, ChannelRunSettings,
        },
        test_points::{NoopObserver, TestPointProbe},
    },
    transmission::{layout::PhysicalCarrierRole, TransmissionSettings},
    types::{modulation::CarrierType, ProtocolScope},
};
use std::{path::PathBuf, time::Duration};
fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../examples/measurements")
}
#[test]
fn synthetic_widths_import_four_profiles_and_recover_noiseless_payloads() {
    let original = serde_json::from_slice(
        &std::fs::read(root().join("single-channel/modulation_profile.json")).unwrap(),
    )
    .unwrap();
    for (width, count) in [(24_000_000, 880), (90_000_000, 3520)] {
        let documents = fixtures::documents(width, &original).unwrap();
        let directory = root().join(format!("channel-widths/{}mhz", width / 1_000_000));
        for (name, expected) in &documents {
            let actual: serde_json::Value =
                serde_json::from_slice(&std::fs::read(directory.join(name)).unwrap()).unwrap();
            assert_eq!(&actual, expected, "Fixture regeneration drift: {name}");
        }
        let read = |name: &str| std::fs::read(directory.join(name)).unwrap();
        let rxmer = read("rxmer.json");
        let channel = read("channel_estimation.json");
        let profile = read("modulation_profile.json");
        let scenario = import(
            JsonCapture {
                name: "synthetic-rxmer",
                bytes: &rxmer,
            },
            JsonCapture {
                name: "synthetic-channel",
                bytes: &channel,
            },
            JsonCapture {
                name: "synthetic-profile",
                bytes: &profile,
            },
            ProtocolScope::Docsis31,
            0,
        )
        .unwrap();
        assert_eq!(scenario.profiles.len(), 4);
        assert_eq!(scenario.frames[0].key.grid.sample_count, count);
        assert_eq!(
            scenario.frames[0].key.grid.frequency(0).unwrap(),
            640_000_000
        );
        assert!(scenario.channels[0]
            .iq
            .iter()
            .all(|pair| *pair == Some([1.0, 0.0])));
        let transmission = TransmissionSettings {
            occupied_bandwidth_hz: width,
            ..Default::default()
        };
        for (id, bits) in [(0, 8), (1, 10), (2, 11), (3, 12)] {
            let captured = scenario
                .profiles
                .iter()
                .find(|profile| profile.profile_id == id)
                .unwrap();
            assert!(captured
                .assignments
                .iter()
                .filter(|assignment| assignment.carrier_type == CarrierType::Data)
                .all(|assignment| assignment.bit_loading == Some(bits)));
            let layout = captured_layout(captured, &transmission).unwrap();
            let document: ChannelDocument =
                serde_json::from_slice(&read(&format!("channel-profile-{id}.json"))).unwrap();
            let mut settings = document.settings().unwrap();
            assert_eq!(settings.layout, layout);
            let physical =
                docsis_ofdm_fec::transmission::layout::CarrierLayout::new(&transmission, &layout)
                    .unwrap();
            for (offset, assignment) in captured.assignments.iter().enumerate() {
                let bin = usize::from(layout.first_active_bin) + offset;
                if physical.static_roles()[bin] == PhysicalCarrierRole::Data {
                    assert_eq!(
                        settings.data_bit_loading.as_ref().unwrap()[bin],
                        assignment.bit_loading.unwrap()
                    );
                }
            }
            // Runner transports selected loading as its internal ProfileA; the
            // captured ID stays unchanged in the imported scenario.
            settings.duration = Duration::from_micros(200);
            let mut run = ChannelRun::new(&settings).unwrap();
            while run
                .advance(&mut TestPointProbe::disabled(), &mut NoopObserver)
                .unwrap()
                .is_some()
            {}
            assert_eq!(run.summary().phase, ChannelPhase::Finished);
            assert!(
                run.summary().comparison.bits_tested > 0,
                "width={width},profile={id}"
            );
            assert_eq!(
                run.summary().comparison.bit_errors,
                0,
                "width={width},profile={id}"
            );
        }
        let captured = scenario
            .profiles
            .iter()
            .find(|profile| profile.profile_id == 0)
            .unwrap();
        let mut settings = ChannelRunSettings::new(
            transmission.clone(),
            captured_layout(captured, &transmission).unwrap(),
        );
        MeasurementSelection {
            use_rxmer_noise: false,
            transitions: Vec::new(),
            frame_index: 0,
            profile_id: 0,
            channel_index: Some(0),
            tolerance_seconds: 0,
            noise_seed: 41,
        }
        .apply(&scenario, &mut settings)
        .unwrap();
    }
}
#[test]
fn rejects_out_of_range_or_non_step_channel_widths() {
    let source = serde_json::json!({});
    for width in [0, 18_000_000, 25_000_000, 198_000_000] {
        let settings = TransmissionSettings {
            occupied_bandwidth_hz: width,
            ..Default::default()
        };
        assert!(settings.validate().is_err());
        assert!(fixtures::documents(width, &source).is_err());
    }
}
