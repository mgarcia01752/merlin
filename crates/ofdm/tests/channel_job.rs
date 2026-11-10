/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Window-specific metrics, cooperative stops and retention of completed science.
#![cfg(feature = "simulation")]
mod acquisition_support;
use docsis_ofdm_fec::{
    simulation::{
        channel::{run_channel, ChannelJobLimits, ChannelJobStatus, ChannelRunSettings},
        test_points::{
            NoopObserver, ProbeConfig, ProbeEvent, ProbeObserver, ProbeSelection, TestPointId,
            TestPointProbe,
        },
    },
    transmission::SubcarrierSpacing,
    SimulationError,
};
use std::{
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};
fn settings() -> ChannelRunSettings {
    let (settings, layout) =
        acquisition_support::acquisition_setup(SubcarrierSpacing::Khz50, false);
    let mut settings = ChannelRunSettings::new(settings, layout);
    settings.duration = Duration::from_millis(3);
    settings
}

#[test]
fn tx_message_and_frame_observation_preserve_channel_results() {
    use docsis_ofdm_fec::simulation::test_points::{ProbeData, RunPhase};
    struct TxSink {
        messages: usize,
        grids: usize,
    }
    impl ProbeObserver for TxSink {
        fn observe(&mut self, event: &ProbeEvent<'_>) -> Result<(), SimulationError> {
            match event.stage {
                TestPointId::PlcMessage => {
                    assert!(matches!(event.data, ProbeData::Bytes(_)));
                    assert_eq!(event.element_start, 0);
                    self.messages += 1;
                }
                TestPointId::PhysicalBins => {
                    assert!(matches!(event.data, ProbeData::OptionalIq(_)));
                    assert!(event.context.physical_grid.is_some());
                    assert_eq!(event.context.phase, RunPhase::Observation);
                    self.grids += 1;
                }
                _ => panic!("Unselected stage"),
            }
            Ok(())
        }
    }
    let settings = settings();
    let limits = ChannelJobLimits::default();
    let baseline = run_channel(
        &settings,
        &limits,
        &AtomicBool::new(false),
        &mut TestPointProbe::disabled(),
        &mut NoopObserver,
        |_| {},
    )
    .unwrap();
    let mut probe = TestPointProbe::new(ProbeConfig {
        selections: [TestPointId::PlcMessage, TestPointId::PhysicalBins]
            .into_iter()
            .map(|stage| ProbeSelection {
                stage,
                elements: 0..8192,
                source_symbols: None,
                channel_symbols: None,
            })
            .collect(),
        max_events: 64,
        max_elements_per_event: 8192,
        max_total_elements: 65536,
    })
    .unwrap();
    probe.set_phase_filter(Some(RunPhase::Observation));
    probe
        .set_stage_phase_filter(TestPointId::PlcMessage, None)
        .unwrap();
    probe
        .limit_stage_events(TestPointId::PhysicalBins, 1)
        .unwrap();
    let mut sink = TxSink {
        messages: 0,
        grids: 0,
    };
    let observed = run_channel(
        &settings,
        &limits,
        &AtomicBool::new(false),
        &mut probe,
        &mut sink,
        |_| {},
    )
    .unwrap();
    assert!(sink.messages > 0);
    assert_eq!(sink.grids, 1);
    assert_eq!(
        serde_json::to_value(baseline.summary).unwrap(),
        serde_json::to_value(observed.summary).unwrap()
    );
    assert_eq!(baseline.retained_words, observed.retained_words);
}
#[test]
fn whole_word_window_counts_exclude_crossing_and_completion_only_words() {
    let result = run_channel(
        &settings(),
        &ChannelJobLimits::default(),
        &AtomicBool::new(false),
        &mut TestPointProbe::disabled(),
        &mut NoopObserver,
        |_| {},
    )
    .unwrap();
    assert_eq!(result.status, ChannelJobStatus::Complete);
    let window = result.summary.window.as_ref().unwrap();
    assert!(window.contained_words > 0);
    assert!(window.crossing_end_words > 0);
    assert!(window.outside_words > 0);
    assert_eq!(window.incomplete_starts_within, 0);
    assert_eq!(
        window.announced_starts_within,
        window.contained_words + window.crossing_end_words
    );
    assert_eq!(
        window.contained_words + window.crossing_end_words + window.outside_words,
        result.summary.recovered_payloads
    );
    assert_eq!(
        window.compared_bits + window.excluded_recovered_bits,
        result.summary.comparison.bits_tested
    );
    assert_eq!(window.compared_bits, window.contained_words * 14232);
    assert_eq!(window.ber, Some(0.0));
    assert_eq!(
        window.payload_rate_bps,
        Some(window.compared_bits as f64 / 0.003)
    );
    assert!(window.contained_words_recovered_during_drain > 0);
    assert!(result
        .retained_words
        .iter()
        .all(|word| word.first_source_sample <= word.end_source_sample));
}
#[test]
fn cancellation_keeps_completed_comparisons_and_explicit_unrecovered_count() {
    let cancel = AtomicBool::new(false);
    let limits = ChannelJobLimits {
        max_retained_words: 1,
        ..Default::default()
    };
    let result = run_channel(
        &settings(),
        &limits,
        &cancel,
        &mut TestPointProbe::disabled(),
        &mut NoopObserver,
        |progress| {
            if progress.recovered_payloads > 0 {
                cancel.store(true, Ordering::Relaxed);
            }
        },
    )
    .unwrap();
    assert_eq!(result.status, ChannelJobStatus::Cancelled);
    assert!(result.summary.comparison.bits_tested > 0);
    assert_eq!(result.summary.comparison.bit_errors, 0);
    assert!(result.unrecovered_generated_payloads > 0);
    assert_eq!(result.retained_words.len(), 1);
    assert_eq!(
        result.omitted_word_details + 1,
        result.summary.recovered_payloads
    );
    assert_eq!(result.summary.window.unwrap().payload_rate_bps, None);
}
#[test]
fn pre_cancel_and_invalid_work_limits_do_not_fabricate_compared_bits() {
    let result = run_channel(
        &settings(),
        &ChannelJobLimits::default(),
        &AtomicBool::new(true),
        &mut TestPointProbe::disabled(),
        &mut NoopObserver,
        |_| {},
    )
    .unwrap();
    assert_eq!(result.status, ChannelJobStatus::Cancelled);
    assert_eq!(result.summary.payload_ber, None);
    assert!(result.summary.window.is_none());
    let limits = ChannelJobLimits {
        max_runtime_ms: 0,
        ..Default::default()
    };
    assert!(run_channel(
        &settings(),
        &limits,
        &AtomicBool::new(false),
        &mut TestPointProbe::disabled(),
        &mut NoopObserver,
        |_| {}
    )
    .is_err());
}
#[test]
fn failed_sink_preserves_science_completed_before_the_callback() {
    struct Failure;
    impl ProbeObserver for Failure {
        fn observe(&mut self, _event: &ProbeEvent<'_>) -> Result<(), SimulationError> {
            Err(SimulationError::new("TestSink", "Injected failure."))
        }
    }
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
        &settings(),
        &ChannelJobLimits::default(),
        &AtomicBool::new(false),
        &mut probe,
        &mut Failure,
        |_| {},
    )
    .unwrap();
    assert_eq!(result.status, ChannelJobStatus::Failed);
    assert_eq!(result.error.unwrap().code, "TestSink");
    assert!(result.summary.recovered_payloads > 0);
    assert_eq!(result.summary.comparison.bit_errors, 0);
    assert_eq!(
        result.omitted_word_details,
        result.summary.recovered_payloads
    );
}

#[test]
fn paused_run_resumes_without_restarting_science_and_stop_works_while_paused() {
    use docsis_ofdm_fec::simulation::channel::job::run_channel_controlled;
    use std::sync::Arc;
    let baseline = run_channel(
        &settings(),
        &ChannelJobLimits::default(),
        &AtomicBool::new(false),
        &mut TestPointProbe::disabled(),
        &mut NoopObserver,
        |_| {},
    )
    .unwrap();
    let paused = Arc::new(AtomicBool::new(false));
    let gate = paused.clone();
    let (send, receive) = std::sync::mpsc::sync_channel(1);
    let resume = std::thread::spawn(move || {
        receive.recv().unwrap();
        std::thread::sleep(Duration::from_millis(40));
        gate.store(false, Ordering::Relaxed);
    });
    let mut first = true;
    let result = run_channel_controlled(
        &settings(),
        &ChannelJobLimits::default(),
        &AtomicBool::new(false),
        &paused,
        &mut TestPointProbe::disabled(),
        &mut NoopObserver,
        |_| {
            if first {
                first = false;
                paused.store(true, Ordering::Relaxed);
                send.send(()).unwrap();
            }
        },
    )
    .unwrap();
    resume.join().unwrap();
    assert_eq!(result.status, ChannelJobStatus::Complete);
    assert_eq!(
        serde_json::to_value(result.summary).unwrap(),
        serde_json::to_value(baseline.summary).unwrap()
    );
    let cancelled = Arc::new(AtomicBool::new(false));
    let stop = cancelled.clone();
    let stopper = std::thread::spawn(move || {
        std::thread::sleep(Duration::from_millis(40));
        stop.store(true, Ordering::Relaxed);
    });
    let result = run_channel_controlled(
        &settings(),
        &ChannelJobLimits::default(),
        &cancelled,
        &AtomicBool::new(true),
        &mut TestPointProbe::disabled(),
        &mut NoopObserver,
        |_| panic!("No step may advance while initially paused"),
    )
    .unwrap();
    stopper.join().unwrap();
    assert_eq!(result.status, ChannelJobStatus::Cancelled);
    assert_eq!(result.summary.comparison.bits_tested, 0);
}
