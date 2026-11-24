/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Real capture requests preserve independent science, original evidence and charges.
use super::super::tests::{configured, wait_for};
use super::*;
use crate::{
    shell::{ApplicationSessions, ProjectSnapshot, SessionKey},
    upstream::RetainedReport,
};
use merlin::upstream::{
    inputs::{PreEqCapture, RxmerCapture, UcdCapture},
    RunStatus,
};
use std::{
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

const UCD: &str = include_str!("../../../../../../examples/measurements/upstream/ucd/minimal.hex");
const RXMER: &str =
    include_str!("../../../../../../examples/measurements/upstream/rxmer/ccap-50khz.hex");
const PRE_EQ: &str =
    include_str!("../../../../../../examples/measurements/upstream/pre-eq/applied.hex");
const LONG_RXMER: &str =
    include_str!("../../../../../../examples/measurements/upstream/measured/rxmer-40db.hex");
static NEXT_DELIVERY: AtomicU64 = AtomicU64::new(1);

// Independent finite test policies, not estimates of any scientific footprint.
const LIMITS: RetainedResources = RetainedResources {
    payload_bytes: 4 * 1024 * 1024,
    ordered_entries: 16_384,
    shared_roots: 32,
    channel_count: 0,
    slot_count: 0,
    presentation_utf16_bytes: 0,
    presentation_entries: 0,
    retained_views: 0,
    open_views: 0,
};

struct Delivery {
    directory: PathBuf,
    path: PathBuf,
    uri: String,
}

impl Delivery {
    fn new(bytes: &[u8]) -> Self {
        let ordinal = NEXT_DELIVERY.fetch_add(1, Ordering::Relaxed);
        let directory = std::env::temp_dir().join(format!(
            "merlin-capture-test-{}-{ordinal}",
            std::process::id()
        ));
        std::fs::create_dir(&directory).unwrap();
        let path = directory.join("capture.input");
        std::fs::write(&path, bytes).unwrap();
        let uri = url::Url::from_file_path(path.canonicalize().unwrap())
            .unwrap()
            .to_string();
        Self {
            directory,
            path,
            uri,
        }
    }
}

impl Drop for Delivery {
    fn drop(&mut self) {
        std::fs::remove_dir_all(&self.directory).unwrap();
    }
}

struct Science {
    key: Option<SessionKey>,
    address: usize,
    report: RetainedReport,
    downstream: ProjectSnapshot,
    upstream: ProjectSnapshot,
}

impl Science {
    fn new(model: &Model, application: &ApplicationSessions) -> Self {
        Self {
            key: model.key,
            address: model
                .controller
                .as_ref()
                .unwrap()
                .inspect(|simulation| simulation as *const _ as usize)
                .unwrap(),
            report: Arc::clone(&model.report.as_ref().unwrap().1),
            downstream: application.snapshot(Direction::Downstream),
            upstream: application.snapshot(Direction::Upstream),
        }
    }

    fn assert_owner(&self, model: &Model) {
        assert_eq!(model.key, self.key);
        assert_eq!(
            model
                .controller
                .as_ref()
                .unwrap()
                .inspect(|simulation| simulation as *const _ as usize)
                .unwrap(),
            self.address
        );
        assert!(Arc::ptr_eq(&model.report.as_ref().unwrap().1, &self.report));
    }

    fn assert_unchanged(&self, model: &Model, application: &ApplicationSessions) {
        self.assert_owner(model);
        assert_eq!(application.snapshot(Direction::Downstream), self.downstream);
        assert_eq!(application.snapshot(Direction::Upstream), self.upstream);
        assert!(application.active().is_none());
    }
}

fn hex(text: &str) -> Vec<u8> {
    let digits: String = text.split_whitespace().collect();
    digits
        .as_bytes()
        .chunks_exact(2)
        .map(|pair| u8::from_str_radix(std::str::from_utf8(pair).unwrap(), 16).unwrap())
        .collect()
}

fn reported(application: ApplicationSessions) -> Box<Model> {
    let mut model = configured(application);
    model.start().unwrap();
    wait_for(|| model.poll()["busy"] == false);
    assert_eq!(
        model.report.as_ref().unwrap().1.value().status,
        RunStatus::Completed
    );
    model
}

fn load(model: &mut Model, family: Family, bytes: &[u8]) -> Delivery {
    let delivery = Delivery::new(bytes);
    assert!(model.load_capture_uri(family, Some(&delivery.uri)).unwrap());
    wait_for(|| model.poll()["preparing"] == false);
    assert!(model.error.message().is_empty(), "{}", model.state());
    delivery
}

fn request(family: Family, bytes: &[u8], budget: &RetentionBudget) -> Request {
    let reservation = budget
        .reserve_resources(RetainedResources {
            payload_bytes: size_of::<Retained<Box<[u8]>>>() + bytes.len(),
            ..Default::default()
        })
        .unwrap();
    Request::Load {
        family,
        bytes: reservation.retain(bytes.to_vec().into_boxed_slice()),
    }
}

#[test]
fn real_local_binary_and_source_json_families_leave_science_and_dirty_state_unchanged() {
    let application = ApplicationSessions::desktop().unwrap();
    let mut model = reported(application.clone());
    let science = Science::new(&model, &application);
    let ucd = hex(UCD);
    let rxmer = hex(RXMER);
    let pre_eq = hex(PRE_EQ);
    let sources = [
        (
            Family::Ucd,
            ucd.clone(),
            UcdCapture::from_mac_management(&ucd)
                .unwrap()
                .to_source_json()
                .unwrap(),
        ),
        (
            Family::Rxmer,
            rxmer.clone(),
            RxmerCapture::from_pnm(&rxmer)
                .unwrap()
                .to_source_json()
                .unwrap(),
        ),
        (
            Family::PreEq,
            pre_eq.clone(),
            PreEqCapture::from_pnm(&pre_eq)
                .unwrap()
                .to_source_json()
                .unwrap(),
        ),
    ];
    for (family, original, json) in sources {
        for bytes in [original.as_slice(), json.as_bytes()] {
            let file = load(&mut model, family, bytes);
            let display = model.capture.as_ref().unwrap().value();
            assert_eq!(display.source.value().family, family);
            assert_eq!(display.source.value().delivery.as_ref(), bytes);
            assert_eq!(display.source.value().original.as_ref(), original);
            // Independent presentation policy, not imported from the producer.
            assert!(display.view["data_rows"].as_array().unwrap().len() <= 64);
            assert_eq!(model.state()["capture_view"]["family"], family.tag());
            assert_eq!(model.capture_state()["view"], display.view);
            assert!(application.protects_input_path(&file.path));
            assert_eq!(std::fs::read(&file.path).unwrap(), bytes);
            science.assert_unchanged(&model, &application);
        }
    }
}

#[test]
fn picker_cancel_invalid_delivery_and_wrong_family_preserve_previous_source_and_page() {
    let application = ApplicationSessions::desktop().unwrap();
    let mut model = reported(application.clone());
    let _file = load(&mut model, Family::Rxmer, &hex(RXMER));
    let science = Science::new(&model, &application);
    let source = Arc::clone(&model.capture.as_ref().unwrap().value().source);
    let old_view = model.capture.as_ref().unwrap().value().view.clone();
    assert!(!model.load_capture_uri(Family::Rxmer, None).unwrap());
    assert_eq!(model.capture.as_ref().unwrap().value().view, old_view);
    assert!(model
        .load_capture_uri(Family::Rxmer, Some("https://example.invalid/capture"))
        .is_err());
    for (family, bytes) in [(Family::Rxmer, b"{}".to_vec()), (Family::PreEq, hex(RXMER))] {
        let invalid = Delivery::new(&bytes);
        assert!(model.load_capture_uri(family, Some(&invalid.uri)).unwrap());
        wait_for(|| model.poll()["preparing"] == false);
        assert!(!model.error.message().is_empty());
        let retained = model.capture.as_ref().unwrap().value();
        assert!(Arc::ptr_eq(&retained.source, &source));
        assert_eq!(retained.view, old_view);
        science.assert_unchanged(&model, &application);
    }
    assert!(model.capture_page(Family::Ucd, 0).is_err());
    assert!(Arc::ptr_eq(
        &model.capture.as_ref().unwrap().value().source,
        &source
    ));
    assert_eq!(model.capture.as_ref().unwrap().value().view, old_view);
    science.assert_unchanged(&model, &application);
}

#[test]
fn real_page_reuses_original_source_and_out_of_range_keeps_last_valid_page() {
    let application = ApplicationSessions::desktop().unwrap();
    let mut model = reported(application.clone());
    let _file = load(&mut model, Family::Rxmer, &hex(LONG_RXMER));
    let science = Science::new(&model, &application);
    let source = Arc::clone(&model.capture.as_ref().unwrap().value().source);
    model.capture_page(Family::Rxmer, 64).unwrap();
    wait_for(|| model.poll()["preparing"] == false);
    assert!(model.error.message().is_empty());
    let accepted = model.capture.as_ref().unwrap().value();
    assert!(Arc::ptr_eq(&accepted.source, &source));
    assert_eq!(accepted.view["data_rows"][0]["coordinate"], "192");
    let page = accepted.view.clone();
    model.capture_page(Family::Rxmer, usize::MAX).unwrap();
    wait_for(|| model.poll()["preparing"] == false);
    assert!(model.error.message().contains("page origin"));
    assert_eq!(model.capture.as_ref().unwrap().value().view, page);
    assert!(Arc::ptr_eq(
        &model.capture.as_ref().unwrap().value().source,
        &source
    ));
    science.assert_unchanged(&model, &application);
}

#[test]
fn stopping_capture_preparation_preserves_previous_view_and_real_report() {
    let application = ApplicationSessions::desktop().unwrap();
    let mut model = reported(application.clone());
    let _file = load(&mut model, Family::Rxmer, &hex(LONG_RXMER));
    let science = Science::new(&model, &application);
    let source = Arc::clone(&model.capture.as_ref().unwrap().value().source);
    let view = model.capture.as_ref().unwrap().value().view.clone();
    model.capture_page(Family::Rxmer, 64).unwrap();
    assert!(model.pending.is_some());
    model.stop().unwrap();
    wait_for(|| model.poll()["preparing"] == false);
    assert_eq!(model.capture.as_ref().unwrap().value().view, view);
    assert!(Arc::ptr_eq(
        &model.capture.as_ref().unwrap().value().source,
        &source
    ));
    science.assert_unchanged(&model, &application);
}

#[test]
fn changed_canonical_generation_discards_a_queued_capture_page_without_adoption() {
    let application = ApplicationSessions::desktop().unwrap();
    let mut model = reported(application.clone());
    let _file = load(&mut model, Family::Rxmer, &hex(LONG_RXMER));
    let science = Science::new(&model, &application);
    let source = Arc::clone(&model.capture.as_ref().unwrap().value().source);
    let view = model.capture.as_ref().unwrap().value().view.clone();
    model.capture_page(Family::Rxmer, 64).unwrap();
    let replacement = application.configure_upstream(true).unwrap();
    wait_for(|| model.poll()["preparing"] == false);
    assert!(model
        .error
        .message()
        .contains("changed during capture inspection"));
    science.assert_owner(&model);
    assert_eq!(
        application.snapshot(Direction::Downstream),
        science.downstream
    );
    assert_eq!(application.snapshot(Direction::Upstream).key, replacement);
    assert_eq!(model.capture.as_ref().unwrap().value().view, view);
    assert!(Arc::ptr_eq(
        &model.capture.as_ref().unwrap().value().source,
        &source
    ));
    assert!(application.active().is_none());
}

#[test]
fn request_pages_and_external_source_alias_share_one_charge_until_final_drop() {
    let budget = RetentionBudget::with_limits(LIMITS);
    let bytes = hex(LONG_RXMER);
    let first = request(Family::Rxmer, &bytes, &budget)
        .prepare(&budget)
        .unwrap();
    let source = Arc::clone(&first.value().source);
    let weak = Arc::downgrade(&source);
    let source_charge = source.fixture_reserved_resources();
    assert_eq!(source_charge.shared_roots, 1);
    assert_eq!(
        source_charge.payload_bytes,
        size_of::<Retained<CaptureSource>>() + bytes.len() * 2
    );
    let first_charge = first.fixture_reserved_resources();
    assert_eq!(
        budget.usage(),
        source_charge.checked_add(first_charge).unwrap()
    );
    let second = Request::Page {
        source: Arc::clone(&source),
        first: 64,
    }
    .prepare(&budget)
    .unwrap();
    assert!(Arc::ptr_eq(&first.value().source, &second.value().source));
    assert_eq!(
        budget.usage(),
        source_charge
            .checked_add(first_charge)
            .unwrap()
            .checked_add(second.fixture_reserved_resources())
            .unwrap()
    );
    drop(first);
    drop(second);
    assert_eq!(budget.usage(), source_charge);
    assert!(weak.upgrade().is_some());
    assert_eq!(source.value().delivery.as_ref(), bytes);
    assert_eq!(source.value().original.as_ref(), bytes);
    drop(source);
    assert!(weak.upgrade().is_none());
    assert_eq!(budget.usage(), RetainedResources::default());
}

#[test]
fn actual_request_candidate_rejection_in_each_dimension_keeps_previous_display() {
    for dimension in ["bytes", "entries", "roots"] {
        let budget = RetentionBudget::with_limits(LIMITS);
        let bytes = hex(RXMER);
        let previous = request(Family::Rxmer, &bytes, &budget)
            .prepare(&budget)
            .unwrap();
        let source = Arc::clone(&previous.value().source);
        let view = previous.value().view.clone();
        // Admit actual request input first. This targets Request::prepare rather
        // than substituting an earlier input, closure or transport rejection.
        let candidate = request(Family::Rxmer, &bytes, &budget);
        let used = budget.usage();
        let pressure = budget
            .reserve_resources(match dimension {
                "bytes" => RetainedResources {
                    payload_bytes: LIMITS.payload_bytes - used.payload_bytes,
                    ..Default::default()
                },
                "entries" => RetainedResources {
                    ordered_entries: LIMITS.ordered_entries - used.ordered_entries,
                    ..Default::default()
                },
                _ => RetainedResources {
                    shared_roots: LIMITS.shared_roots - used.shared_roots,
                    ..Default::default()
                },
            })
            .unwrap();
        assert!(candidate.prepare(&budget).is_err(), "{dimension}");
        assert_eq!(previous.value().view, view);
        assert!(Arc::ptr_eq(&previous.value().source, &source));
        assert_eq!(source.value().original.as_ref(), bytes);
        drop(pressure);
        assert_eq!(
            budget.usage(),
            source
                .fixture_reserved_resources()
                .checked_add(previous.fixture_reserved_resources())
                .unwrap()
        );
        drop(previous);
        assert_eq!(budget.usage(), source.fixture_reserved_resources());
        drop(source);
        assert_eq!(budget.usage(), RetainedResources::default());
    }
}
