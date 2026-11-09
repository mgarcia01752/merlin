/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Independent window edges, source/noise draw invariance and bounded export failures.
#![cfg(feature = "simulation")]
use docsis_ofdm_fec::{
    simulation::{
        channel::accounting::{ObservationWindow, WordPlacement},
        runner::simulate_with_probes,
        source::{observation, SeededSource},
        test_points::{
            JsonlProbeExporter, NoopObserver, ProbeConfig, ProbeContext, ProbeEvent, ProbeObserver,
            ProbeSelection, RunPhase, TestPointId, TestPointProbe,
        },
        SimulationSettings,
    },
    transmission::TransmissionSettings,
    LabError,
};
use std::{io::Write, sync::atomic::AtomicBool};
fn probes(budget: usize) -> TestPointProbe {
    TestPointProbe::new(ProbeConfig {
        selections: [TestPointId::SourcePayload, TestPointId::LegacyNoiseOutput]
            .into_iter()
            .map(|stage| ProbeSelection {
                stage,
                elements: 2..6,
                source_symbols: None,
                channel_symbols: None,
            })
            .collect(),
        max_events: budget,
        max_elements_per_event: 4,
        max_total_elements: budget * 4,
    })
    .unwrap()
}
#[test]
fn window_classification_has_exact_half_open_edges_and_checked_samples() {
    let timing = TransmissionSettings::default()
        .sample_timing(10 * 8704)
        .unwrap();
    let window = ObservationWindow::new(10, timing).unwrap();
    for (first, last, expected) in [
        (10, 19, WordPlacement::Contained),
        (9, 10, WordPlacement::CrossesStart),
        (19, 20, WordPlacement::CrossesEnd),
        (9, 20, WordPlacement::CrossesBoth),
        (20, 21, WordPlacement::Outside),
        (8, 9, WordPlacement::Outside),
    ] {
        assert_eq!(window.classify(first, last).unwrap(), expected);
    }
    assert!(window.classify(20, 19).is_err());
    assert!(window.classify(0, u64::MAX).is_err());
    assert!(ObservationWindow::new(u64::MAX, timing).is_err());
    assert_eq!(window.first_sample, 87040);
    assert_eq!(window.end_sample, 174080);
}
#[test]
fn seeded_source_and_existing_awgn_draws_are_unchanged_by_enabled_and_exhausted_export() {
    let settings = SimulationSettings {
        codewords: 2,
        max_runtime_ms: 300000,
        ..Default::default()
    };
    let mut baseline = serde_json::to_value(
        simulate_with_probes(
            &settings,
            None,
            &AtomicBool::new(false),
            |_| {},
            &mut TestPointProbe::disabled(),
            &mut NoopObserver,
        )
        .unwrap(),
    )
    .unwrap();
    baseline.as_object_mut().unwrap().remove("elapsed_ms");
    for budget in [1, 4] {
        let mut exporter = JsonlProbeExporter::new(Vec::new(), 100_000).unwrap();
        let mut selected = probes(budget);
        let mut result = serde_json::to_value(
            simulate_with_probes(
                &settings,
                None,
                &AtomicBool::new(false),
                |_| {},
                &mut selected,
                &mut exporter,
            )
            .unwrap(),
        )
        .unwrap();
        result.as_object_mut().unwrap().remove("elapsed_ms");
        assert_eq!(result, baseline);
        let bytes = exporter.finish().unwrap();
        let lines: Vec<serde_json::Value> = bytes
            .split(|byte| *byte == b'\n')
            .filter(|line| !line.is_empty())
            .map(|line| serde_json::from_slice(line).unwrap())
            .collect();
        assert_eq!(lines.len(), budget);
        assert_eq!(lines[0]["schema_version"], 1);
        assert_eq!(lines[0]["event"]["element_start"], 2);
        assert_eq!(lines[0]["event"]["total_elements"], 14232);
        assert_eq!(
            lines[0]["event"]["data"]["values"]
                .as_array()
                .unwrap()
                .len(),
            4
        );
    }
}
#[test]
fn callback_and_export_failures_do_not_change_future_source_draws() {
    struct Failure;
    impl ProbeObserver for Failure {
        fn observe(&mut self, _event: &ProbeEvent<'_>) -> Result<(), LabError> {
            Err(LabError::new("TestFailure", "Injected failure."))
        }
    }
    let mut source = SeededSource::new(99);
    let mut baseline = SeededSource::new(99);
    let bits = source.next_payload();
    assert_eq!(bits, baseline.next_payload());
    let context = ProbeContext::new("source-test", "seed99", RunPhase::Observation).unwrap();
    assert!(observation::source_payload(&bits, &context, &mut probes(1), &mut Failure).is_err());
    assert_eq!(source.next_payload(), baseline.next_payload());
    let mut limited = JsonlProbeExporter::new(Vec::new(), 1).unwrap();
    assert_eq!(
        observation::source_payload(&bits, &context, &mut probes(1), &mut limited)
            .unwrap_err()
            .code,
        "ProbeExportLimit"
    );
    assert_eq!(limited.bytes_written(), 0);
    assert!(limited.finish().is_err());
}
#[test]
fn failing_writer_returns_typed_io_error_and_closes_export() {
    struct Fails;
    impl Write for Fails {
        fn write(&mut self, _bytes: &[u8]) -> std::io::Result<usize> {
            Err(std::io::Error::other("disk failure"))
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let context = ProbeContext::new("export", "source", RunPhase::Observation).unwrap();
    let mut exporter = JsonlProbeExporter::new(Fails, 100_000).unwrap();
    assert_eq!(
        observation::source_payload(&[0; 8], &context, &mut probes(1), &mut exporter)
            .unwrap_err()
            .code,
        "ProbeExportIo"
    );
    assert_eq!(
        observation::source_payload(&[0; 8], &context, &mut probes(1), &mut exporter)
            .unwrap_err()
            .code,
        "ProbeExportClosed"
    );
}

#[test]
fn oversized_serialized_record_rejects_before_mutating_writer() {
    use docsis_ofdm_fec::simulation::test_points::SignalReference;
    let mut context =
        ProbeContext::new("large-record", "noise-output", RunPhase::Observation).unwrap();
    context.reference = SignalReference::OtherSubcarrierRms { rms: 1.0 };
    let mut selection = TestPointProbe::new(ProbeConfig {
        selections: vec![ProbeSelection {
            stage: TestPointId::LegacyNoiseOutput,
            elements: 0..65536,
            source_symbols: None,
            channel_symbols: None,
        }],
        max_events: 1,
        max_elements_per_event: 65536,
        max_total_elements: 65536,
    })
    .unwrap();
    let mut exporter = JsonlProbeExporter::new(Vec::new(), 64 * 1048576).unwrap();
    let samples = vec![[1.123456789012345, 4.432234567845674]; 65536];
    assert_eq!(
        observation::noise_output(&samples, &context, &mut selection, &mut exporter)
            .unwrap_err()
            .code,
        "ProbeExportLimit"
    );
    assert_eq!(exporter.bytes_written(), 0);
}

#[test]
fn final_writer_flush_failure_is_not_reported_as_success() {
    struct FlushFailure;
    impl Write for FlushFailure {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Err(std::io::Error::other("flush failure"))
        }
    }
    let exporter = JsonlProbeExporter::new(FlushFailure, 1000).unwrap();
    assert!(matches!(exporter.finish(),Err(error) if error.code=="ProbeExportIo"));
}
