/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
#![cfg(feature = "simulation")]
mod acquisition_support;
use docsis_ofdm_fec::{
    scenario::{FrequencyGrid, RxMerFrame, ScenarioKey, SourceProvenance},
    signal_processing::channel::impairments::NoiseModel,
    simulation::{
        channel::{timeline::ImpairmentEpoch, ChannelRun, ChannelRunSettings},
        test_points::{NoopObserver, TestPointProbe},
    },
    transmission::SubcarrierSpacing,
};
fn settings() -> ChannelRunSettings {
    let (transmission, raw) =
        acquisition_support::acquisition_setup(SubcarrierSpacing::Khz50, false);
    let mut settings = ChannelRunSettings::new(transmission, raw);
    settings.duration = std::time::Duration::from_micros(200);
    let frame = RxMerFrame {
        key: ScenarioKey {
            mac_address: [0, 202, 0, 0, 0, 1],
            channel_id: 7,
            grid: FrequencyGrid {
                zero_frequency_hz: 100_000_000,
                first_subcarrier_index: 0,
                spacing_hz: 50_000,
                sample_count: 4096,
            },
        },
        source: SourceProvenance {
            name: "synthetic-timeline".into(),
            sha256: "0".repeat(64),
            byte_length: 0,
            capture_time: Some(1),
            file_kind: "synthetic".into(),
            format_major: 1,
            format_minor: 0,
        },
        capture_time: 1,
        mer_db: vec![Some(50.0); 4096],
    };
    settings.impairments.noise = NoiseModel::Rxmer { frame };
    settings.impairments.seed = 19;
    let mut next = settings.impairments.clone();
    if let NoiseModel::Rxmer { frame } = &mut next.noise {
        frame.capture_time = 2;
        frame.source.capture_time = Some(2);
    }
    settings.impairment_timeline = vec![ImpairmentEpoch {
        first_observation_symbol: 2,
        capture_time: 2,
        impairments: next,
    }];
    settings
}
fn run(settings: &ChannelRunSettings) -> docsis_ofdm_fec::simulation::channel::ChannelSummary {
    let mut run = ChannelRun::new(settings).unwrap();
    while run
        .advance(&mut TestPointProbe::disabled(), &mut NoopObserver)
        .unwrap()
        .is_some()
    {}
    run.summary().clone()
}
#[test]
fn identical_measurement_transition_preserves_continuous_rng_and_fec() {
    let settings = settings();
    let changed = run(&settings);
    let mut constant = settings.clone();
    constant.impairment_timeline.clear();
    let baseline = run(&constant);
    assert_eq!(
        serde_json::to_value(changed).unwrap(),
        serde_json::to_value(baseline).unwrap()
    );
}
#[test]
fn out_of_window_duplicate_seed_and_identity_changes_reject_before_work() {
    let original = settings();
    assert!(original.validate().is_ok());
    let mut invalid = original.clone();
    invalid.impairment_timeline[0].first_observation_symbol = 0;
    assert!(invalid.validate().is_err());
    let mut invalid = original.clone();
    invalid.impairment_timeline[0].impairments.seed += 1;
    assert!(invalid.validate().is_err());
    let mut invalid = original.clone();
    invalid.impairment_timeline[0].first_observation_symbol = 1_000_000;
    assert!(invalid.validate().is_err());
    let mut invalid = original;
    if let NoiseModel::Rxmer { frame } = &mut invalid.impairment_timeline[0].impairments.noise {
        frame.key.mac_address[5] = 2;
    }
    assert!(invalid.validate().is_err());
}

#[test]
fn piecewise_residual_powers_and_independence_are_verified_without_inferring_bursts() {
    use rand::SeedableRng;
    let settings = settings();
    let layout = docsis_ofdm_fec::transmission::layout::CarrierLayout::new(
        &settings.transmission,
        &settings.layout,
    )
    .unwrap();
    let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(891);
    let roles = vec![docsis_ofdm_fec::transmission::layout::PhysicalCarrierRole::Data; 4096];
    let input = vec![Some([0.0, 0.0]); 4096];
    for (mer, expected) in [(40.0, 1e-4), (20.0, 1e-2)] {
        let mut impairments = settings.impairments.clone();
        if let NoiseModel::Rxmer { frame } = &mut impairments.noise {
            frame.mer_db.fill(Some(mer));
        }
        let mut plan = impairments
            .plan(&settings.transmission, &layout, 0.01)
            .unwrap();
        // The statistical test deliberately supplies a complete normalized physical plane.
        plan.residual_power.fill(expected);
        let points = plan.apply_bins(&input, &roles, &mut rng).unwrap();
        let points: Vec<_> = points.into_iter().map(Option::unwrap).collect();
        let measured = points
            .iter()
            .map(|point| point[0] * point[0] + point[1] * point[1])
            .sum::<f64>()
            / 4096.0;
        let lag = points
            .windows(2)
            .map(|pair| pair[0][0] * pair[1][0] + pair[0][1] * pair[1][1])
            .sum::<f64>()
            / 4095.0;
        assert!((measured / expected - 1.0).abs() < 0.06);
        assert!((lag / expected).abs() < 0.06);
    }
}

#[test]
fn changed_response_is_applied_at_exact_physical_observation_boundary() {
    use docsis_ofdm_fec::{
        scenario::ChannelFrame,
        simulation::test_points::{
            ProbeConfig, ProbeData, ProbeEvent, ProbeObserver, ProbeSelection, TestPointId,
        },
        SimulationError,
    };
    struct Samples(Vec<(u64, [f64; 2])>);
    impl ProbeObserver for Samples {
        /// Records the selected received pilot for an independent transition check.
        /// # Arguments
        /// * `event` - Bounded immutable receive-FFT observation.
        /// # Returns
        /// Unit after retaining one physical pilot sample.
        /// # Errors
        /// Rejects missing physical-symbol metadata or a mismatched event domain.
        fn observe(&mut self, event: &ProbeEvent<'_>) -> Result<(), SimulationError> {
            if let ProbeData::Iq(points) = event.data {
                self.0.push((
                    event.context.channel_symbol_index.ok_or_else(|| {
                        SimulationError::new("MissingSymbol", "Missing physical symbol")
                    })?,
                    points[0],
                ));
                Ok(())
            } else {
                Err(SimulationError::new("WrongDomain", "Expected FFT I/Q"))
            }
        }
    }
    let mut settings = settings();
    let NoiseModel::Rxmer { frame } = &settings.impairment_timeline[0].impairments.noise else {
        unreachable!()
    };
    settings.impairment_timeline[0].impairments.response = Some(ChannelFrame {
        key: frame.key.clone(),
        source: frame.source.clone(),
        capture_time: 2,
        iq: vec![Some([2.0, 0.0]); 4096],
    });
    let bin = usize::from(settings.layout.additional_continuous_pilots[0]);
    let mut probe = TestPointProbe::new(ProbeConfig {
        selections: vec![ProbeSelection {
            stage: TestPointId::RxFft,
            elements: bin..bin + 1,
            source_symbols: None,
            channel_symbols: Some(128..132),
        }],
        max_events: 4,
        max_elements_per_event: 1,
        max_total_elements: 4,
    })
    .unwrap();
    let mut sink = Samples(Vec::new());
    let mut run = ChannelRun::new(&settings).unwrap();
    while run.advance(&mut probe, &mut sink).unwrap().is_some() {}
    assert_eq!(sink.0.len(), 4);
    for (symbol, point) in sink.0 {
        let amplitude = if symbol < 130 { 2.0 } else { 4.0 };
        assert!((point[0].abs() - amplitude).abs() < 0.05, "symbol={symbol}");
        assert!(point[1].abs() < 0.05);
    }
    assert_eq!(run.summary().comparison.bit_errors, 0);
}
