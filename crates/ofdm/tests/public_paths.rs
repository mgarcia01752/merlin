/*
 * Copyright 2026 DOCSIS OFDM FEC Toolkit contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Consumers can use canonical modules while established imports remain valid.
use docsis_ofdm_fec::{profile, scenario, signal_processing, types::ProtocolScope};
#[test]
fn scenario_and_profile_compatibility_imports_keep_the_same_types() {
    let scenario: scenario::document::ScenarioDocument =
        serde_json::from_str(include_str!("../../../examples/scenario.json")).unwrap();
    let legacy: &profile::document::ScenarioDocument = &scenario;
    legacy.validate().unwrap();
    let key: &profile::model::ScenarioKey = &scenario.frames[0].key;
    assert_eq!(key, &scenario.frames[0].key);
    let protocol: profile::model::ProtocolScope = ProtocolScope::Docsis31;
    assert_eq!(protocol, scenario.protocol);
    assert_eq!(
        signal_processing::noise::equivalent_noise_mer(&[10., 20.]),
        profile::evaluation::equivalent_noise_mer(&[10., 20.])
    );
    assert_eq!(
        signal_processing::ofdm::useful_symbol_rate(8, 25000),
        Some(200000)
    );
    assert_eq!(
        docsis_ofdm_fec::math::ofdm::useful_symbol_rate(8, 25000),
        Some(200000)
    );
}
#[cfg(feature = "simulation")]
#[test]
fn new_simulation_source_and_old_imports_share_one_implementation() {
    use docsis_ofdm_fec::{signal_processing::modulation, simulation};
    let mut first = simulation::source::SeededSource::new(7);
    let mut second = simulation::source::SeededSource::new(7);
    assert_eq!(first.next_payload(), second.next_payload());
    assert_eq!(first.next_payload(), second.next_payload());
    let settings: profile::simulation::SimulationSettings =
        simulation::config::SimulationSettings::default();
    settings.validate().unwrap();
    assert_eq!(modulation::map_qam16(3), profile::simulation::map_qam16(3));
}
