/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Paired actual waveform observations, receive-only eligibility, bounds and invariance.
#![cfg(feature = "simulation")]
mod acquisition_support;
use docsis_ofdm_fec::{
    scenario::{ChannelFrame, FrequencyGrid, RxMerFrame, ScenarioKey, SourceProvenance},
    signal_processing::{channel::impairments::NoiseModel, interleaving::FrequencyInterleaver},
    simulation::{
        channel::{
            constellation::{ConstellationSelection, ConstellationStatus},
            run_channel, ChannelJobLimits, ChannelJobStatus, ChannelRun, ChannelRunSettings,
        },
        test_points::{NoopObserver, TestPointProbe},
    },
    transmission::{
        layout::{ncp::PreInterleavingRole, CarrierLayout, PhysicalCarrierRole, PilotReference},
        SubcarrierSpacing,
    },
};
use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};
fn settings(spacing: SubcarrierSpacing) -> ChannelRunSettings {
    let (mut transmission, layout) = acquisition_support::acquisition_setup(spacing, false);
    transmission.time_interleaver_depth = spacing.max_time_interleaver_depth();
    let mut settings = ChannelRunSettings::new(transmission, layout);
    settings.duration = Duration::from_millis(3);
    settings
}
fn measured(settings: &mut ChannelRunSettings, noisy: bool) {
    let spacing = settings.transmission.subcarrier_spacing;
    let count = spacing.fft_samples() as usize;
    let layout = CarrierLayout::new(&settings.transmission, &settings.layout).unwrap();
    let key = ScenarioKey {
        mac_address: [0, 202, 0, 0, 0, 1],
        channel_id: 7,
        grid: FrequencyGrid {
            zero_frequency_hz: settings.layout.zero_frequency_hz,
            first_subcarrier_index: 0,
            spacing_hz: spacing.frequency_hz(),
            sample_count: count,
        },
    };
    let source = SourceProvenance {
        name: "synthetic paired fixture".into(),
        sha256: "0".repeat(64),
        byte_length: 0,
        capture_time: Some(1),
        file_kind: "synthetic".into(),
        format_major: 1,
        format_minor: 0,
    };
    let response: Vec<_> = (0..count)
        .map(|bin| {
            Some(if bin % 2 == 0 {
                [0.5, 0.5]
            } else {
                [0.8, -0.2]
            })
        })
        .collect();
    settings.impairments.response = Some(ChannelFrame {
        key: key.clone(),
        source: source.clone(),
        capture_time: 1,
        iq: response,
    });
    if noisy {
        settings.impairments.seed = 41;
        settings.impairments.noise = NoiseModel::Rxmer {
            frame: RxMerFrame {
                key,
                source,
                capture_time: 1,
                mer_db: layout
                    .static_roles()
                    .iter()
                    .map(|role| {
                        Some(if *role == PhysicalCarrierRole::Data {
                            12.0
                        } else {
                            40.0
                        })
                    })
                    .collect(),
            },
        };
    }
}
fn job(settings: &ChannelRunSettings) -> docsis_ofdm_fec::simulation::channel::ChannelJobResult {
    run_channel(
        settings,
        &ChannelJobLimits::default(),
        &AtomicBool::new(false),
        &mut TestPointProbe::disabled(),
        &mut NoopObserver,
        |_| {},
    )
    .unwrap()
}
#[test]
fn both_spacings_pair_real_bins_in_receive_order_and_trace_payload_coordinates() {
    for spacing in [SubcarrierSpacing::Khz25, SubcarrierSpacing::Khz50] {
        let mut settings = settings(spacing);
        measured(&mut settings, false);
        settings.constellation = Some(ConstellationSelection::default());
        let result = job(&settings);
        assert_eq!(result.status, ChannelJobStatus::Complete);
        assert_eq!(result.summary.comparison.bit_errors, 0);
        let capture = result.constellation.unwrap();
        assert_eq!(capture.status, ConstellationStatus::Complete);
        assert_eq!(capture.collected_pairs, 8192);
        let layout = CarrierLayout::new(&settings.transmission, &settings.layout).unwrap();
        let reference = PilotReference::new(
            &layout,
            usize::from(settings.transmission.time_interleaver_depth),
        )
        .unwrap();
        let permutation = FrequencyInterleaver::new(spacing, reference.settings().cells()).unwrap();
        let origin = result.summary.payload_origin_symbol.unwrap();
        for pair in &capture.pairs {
            assert!((origin..origin + result.summary.timing.complete_symbols)
                .contains(&pair.source_symbol_index));
            assert_eq!(pair.profile_id, 0);
            assert_eq!(pair.modulation_order, 256);
            assert_eq!(
                pair.frequency_hz,
                layout.frequency_hz(pair.physical_bin).unwrap()
            );
            assert_eq!(
                layout.symbol_roles(pair.channel_symbol_index)[usize::from(pair.physical_bin)],
                PhysicalCarrierRole::Data
            );
            // Independently reconstruct the TX branch delay from the source rank, not a TX object.
            let reduced_rank = reference
                .roles(pair.source_symbol_index - origin)
                .iter()
                .enumerate()
                .filter(|(_, role)| **role == PreInterleavingRole::Data)
                .nth(pair.data_position)
                .unwrap()
                .0;
            assert_eq!(
                pair.channel_symbol_index,
                pair.source_symbol_index + (reduced_rank % reference.settings().depth()) as u64
            );
            let output_rank = permutation
                .output_to_input()
                .iter()
                .position(|&input| input == reduced_rank)
                .unwrap();
            assert_eq!(pair.physical_bin, reference.physical_bins()[output_rank]);
            let [gain_i, gain_q] = settings.impairments.response.as_ref().unwrap().iq
                [usize::from(pair.physical_bin)]
            .unwrap();
            let [post_i, post_q] = pair.after_equalization;
            // Independent normalized square-256-QAM geometry: odd levels / sqrt(170).
            for component in pair.after_equalization {
                let level = component * 170.0_f64.sqrt();
                assert!((level - level.round()).abs() < 1e-11);
                assert_eq!((level.round().abs() as u32) % 2, 1);
                assert!(level.abs() <= 15.0 + 1e-11);
            }

            assert!(
                (pair.before_equalization[0] - (gain_i * post_i - gain_q * post_q)).abs() < 1e-12
            );
            assert!(
                (pair.before_equalization[1] - (gain_i * post_q + gain_q * post_i)).abs() < 1e-12
            );
        }
        assert!(capture.pairs.windows(2).all(|pair| (
            pair[0].channel_symbol_index,
            pair[0].physical_bin
        ) < (
            pair[1].channel_symbol_index,
            pair[1].physical_bin
        )));
    }
}
#[test]
fn collection_retains_failed_payloads_and_changes_neither_noise_nor_fec() {
    let mut settings = settings(SubcarrierSpacing::Khz50);
    settings.duration = Duration::from_micros(100);
    measured(&mut settings, true);
    let baseline = job(&settings);
    assert!(baseline.summary.comparison.bit_errors > 0);
    for requested_pairs in [1, 8192] {
        settings.constellation = Some(ConstellationSelection {
            requested_pairs,
            ..Default::default()
        });
        let captured = job(&settings);
        assert_eq!(captured.status, baseline.status);
        assert_eq!(
            serde_json::to_value(&captured.summary).unwrap(),
            serde_json::to_value(&baseline.summary).unwrap()
        );
        let pairs = &captured.constellation.as_ref().unwrap().pairs;
        assert!(!pairs.is_empty());
        assert!(pairs
            .iter()
            .any(|pair| pair.before_equalization != pair.after_equalization));
        settings.receive_block_samples = 137;
        let split = job(&settings);
        assert_eq!(captured.constellation, split.constellation);
        assert_eq!(
            serde_json::to_value(&captured.summary).unwrap(),
            serde_json::to_value(&split.summary).unwrap()
        );
    }
}
#[test]
fn single_carrier_short_run_and_early_cancellation_report_honest_counts() {
    let mut settings = settings(SubcarrierSpacing::Khz50);
    settings.duration = Duration::from_micros(100);
    settings.constellation = Some(ConstellationSelection {
        requested_pairs: 1,
        ..Default::default()
    });
    let frequency = job(&settings).constellation.unwrap().pairs[0].frequency_hz;
    settings.constellation = Some(ConstellationSelection {
        start_hz: Some(frequency),
        stop_hz: Some(frequency),
        ..Default::default()
    });
    let result = job(&settings);
    let capture = result.constellation.unwrap();
    assert_eq!(capture.status, ConstellationStatus::Partial);
    assert!(capture.collected_pairs > 0 && capture.collected_pairs < 8192);
    assert!(capture
        .pairs
        .iter()
        .all(|pair| pair.frequency_hz == frequency));
    let cancelled = run_channel(
        &settings,
        &ChannelJobLimits::default(),
        &AtomicBool::new(true),
        &mut TestPointProbe::disabled(),
        &mut NoopObserver,
        |_| {},
    )
    .unwrap();
    assert_eq!(cancelled.status, ChannelJobStatus::Cancelled);
    let empty = cancelled.constellation.unwrap();
    assert_eq!(empty.status, ConstellationStatus::Unavailable);
    assert_eq!(empty.collected_pairs, 0);
    let token = AtomicBool::new(false);
    let interrupted = run_channel(
        &settings,
        &ChannelJobLimits::default(),
        &token,
        &mut TestPointProbe::disabled(),
        &mut NoopObserver,
        |progress| {
            if progress.recovered_payloads > 0 {
                token.store(true, Ordering::Relaxed);
            }
        },
    )
    .unwrap();
    assert_eq!(interrupted.status, ChannelJobStatus::Cancelled);
    assert!(interrupted.constellation.unwrap().collected_pairs > 0);
}
#[test]
fn selection_rejects_invalid_counts_profiles_and_frequency_bounds() {
    let mut settings = settings(SubcarrierSpacing::Khz50);
    let selection = ConstellationSelection::default();
    for invalid in [
        ConstellationSelection {
            requested_pairs: 0,
            ..selection.clone()
        },
        ConstellationSelection {
            requested_pairs: 65537,
            ..selection.clone()
        },
        ConstellationSelection {
            profile_id: 3,
            ..selection.clone()
        },
        ConstellationSelection {
            start_hz: Some(1),
            ..selection.clone()
        },
        ConstellationSelection {
            start_hz: Some(1),
            stop_hz: Some(u32::MAX),
            ..selection
        },
    ] {
        settings.constellation = Some(invalid);
        assert!(ChannelRun::new(&settings).is_err());
    }
}

#[test]
fn stopping_capture_midstream_does_not_change_later_received_payloads() {
    let mut settings = settings(SubcarrierSpacing::Khz50);
    settings.duration = Duration::from_micros(100);
    measured(&mut settings, true);
    let baseline = job(&settings);
    settings.constellation = Some(ConstellationSelection::default());
    let mut run = ChannelRun::new(&settings).unwrap();
    let mut capture = None;
    while run
        .advance(&mut TestPointProbe::disabled(), &mut NoopObserver)
        .unwrap()
        .is_some()
    {
        if capture.is_none() && run.summary().recovered_payloads > 0 {
            capture = run.take_constellation();
            assert!(run.constellation().is_none());
        }
    }
    assert!(!capture.unwrap().pairs.is_empty());
    assert_eq!(
        serde_json::to_value(run.summary()).unwrap(),
        serde_json::to_value(baseline.summary).unwrap()
    );
}

#[test]
fn failed_probe_export_retains_already_classified_pairs_and_scientific_totals() {
    use docsis_ofdm_fec::{
        simulation::test_points::{
            ProbeConfig, ProbeEvent, ProbeObserver, ProbeSelection, TestPointId,
        },
        SimulationError,
    };
    struct FailedExport;
    impl ProbeObserver for FailedExport {
        fn observe(&mut self, _event: &ProbeEvent<'_>) -> Result<(), SimulationError> {
            Err(SimulationError::new(
                "TestSink",
                "Injected paired-run export failure.",
            ))
        }
    }
    let mut settings = settings(SubcarrierSpacing::Khz50);
    settings.constellation = Some(ConstellationSelection::default());
    let mut probe = TestPointProbe::new(ProbeConfig {
        selections: vec![ProbeSelection {
            stage: TestPointId::PayloadDecoded,
            elements: 0..1,
            source_symbols: None,
            channel_symbols: None,
        }],
        max_events: 1,
        max_elements_per_event: 1,
        max_total_elements: 1,
    })
    .unwrap();
    let result = run_channel(
        &settings,
        &ChannelJobLimits::default(),
        &AtomicBool::new(false),
        &mut probe,
        &mut FailedExport,
        |_| {},
    )
    .unwrap();
    assert_eq!(result.status, ChannelJobStatus::Failed);
    assert_eq!(result.error.unwrap().code, "TestSink");
    assert!(result.summary.recovered_payloads > 0);
    assert!(!result.constellation.unwrap().pairs.is_empty());
}
