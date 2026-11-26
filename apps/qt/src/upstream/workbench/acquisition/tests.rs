/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Real bounded file input and accepted native ownership, without a picker mock.
use super::*;
use crate::shell::{ApplicationSessions, RunCancellation};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

const SOURCE: &str = "examples/cli/upstream/us-50khz.json";
const ACCEPTANCE_DEADLINE: Duration = Duration::from_secs(30);
const POLL_INTERVAL: Duration = Duration::from_millis(2);

fn source() -> (std::path::PathBuf, String) {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(SOURCE)
        .canonicalize()
        .unwrap();
    let uri = url::Url::from_file_path(&path).unwrap().to_string();
    (path, uri)
}

fn drain(model: &mut Model) {
    let deadline = Instant::now() + ACCEPTANCE_DEADLINE;
    while model.poll()["preparing"] == true {
        assert!(Instant::now() < deadline, "{}", model.state());
        std::thread::sleep(POLL_INTERVAL);
    }
}

#[test]
fn cancelled_native_selection_preserves_unconfigured_and_accepted_state() {
    let application = ApplicationSessions::desktop().unwrap();
    let mut model = Model::new(application.clone()).unwrap();
    let before = model.state();
    let usage = application.budget().usage();
    assert!(!model.load_configuration_uri(None).unwrap());
    assert_eq!(model.state(), before);
    assert_eq!(application.budget().usage(), usage);
    let (_, uri) = source();
    assert!(model.load_configuration_uri(Some(&uri)).unwrap());
    assert!(!model.current());
    drain(&mut model);
    assert!(model.current(), "{}", model.state());
    let key = model.key;
    assert!(!model.load_configuration_uri(None).unwrap());
    assert_eq!(model.key, key);
    assert!(model.report.is_none());
    assert!(application.active().is_none());
}

#[test]
fn real_source_loading_preserves_exact_bytes_hash_and_upstream_path_protection() {
    let application = ApplicationSessions::desktop().unwrap();
    let mut model = Model::new(application.clone()).unwrap();
    let (path, uri) = source();
    let bytes = std::fs::read(&path).unwrap();
    let downstream = application.snapshot(Direction::Downstream);
    model.load_configuration_uri(Some(&uri)).unwrap();
    drain(&mut model);
    assert!(model.current(), "{}", model.state());
    model
        .controller
        .as_ref()
        .unwrap()
        .inspect(|simulation| {
            let document = simulation.document().unwrap();
            assert_eq!(document.original_bytes().unwrap(), bytes);
            assert_eq!(
                document.original_sha256(),
                merlin::upstream::ConfigurationDocument::from_json(&bytes)
                    .unwrap()
                    .original_sha256()
            );
        })
        .unwrap();
    assert!(application.protects_input_path(&path));
    application.clear_input_paths(Direction::Downstream);
    assert!(application.protects_input_path(&path));
    assert_eq!(application.snapshot(Direction::Downstream), downstream);
    assert_eq!(std::fs::read(path).unwrap(), bytes);
    assert!(application.active().is_none());
}

#[test]
fn invalid_uri_and_invalid_document_preserve_previous_accepted_owner() {
    let application = ApplicationSessions::desktop().unwrap();
    let mut model = Model::new(application.clone()).unwrap();
    let (_, uri) = source();
    model.load_configuration_uri(Some(&uri)).unwrap();
    drain(&mut model);
    let key = model.key;
    assert!(model
        .load_configuration_uri(Some("https://example.invalid/config.json"))
        .is_err());
    assert_eq!(model.key, key);
    let invalid = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../docs/development/local/unified-qt-workbench/us-01-invalid-input");
    std::fs::create_dir_all(&invalid).unwrap();
    let invalid = invalid.join(format!("invalid-{}.json", std::process::id()));
    std::fs::write(&invalid, b"{}").unwrap();
    let invalid_uri = url::Url::from_file_path(invalid.canonicalize().unwrap())
        .unwrap()
        .to_string();
    model.load_configuration_uri(Some(&invalid_uri)).unwrap();
    drain(&mut model);
    assert_eq!(model.key, key);
    assert!(model.current());
    assert!(!model.error.message().is_empty());
    std::fs::remove_file(invalid).unwrap();
}

struct Control;
impl RunCancellation for Control {
    /// Supplies a nonblocking fixture control without executing any engine.
    /// Cancellation is intentionally inert; only terminal disposal releases its lease.
    fn request_cancel(&self) {}
}

#[test]
fn active_downstream_rejects_input_acquisition_without_cancelling_or_retargeting() {
    let application = ApplicationSessions::desktop().unwrap();
    let key = application.configure_downstream(true).unwrap();
    let lease = application.begin(key, Arc::new(Control)).unwrap();
    let token = lease.token();
    let mut model = Model::new(application.clone()).unwrap();
    let (path, uri) = source();
    assert!(model.load_configuration_uri(Some(&uri)).is_err());
    assert!(model.pending.is_none());
    assert!(!application.protects_input_path(&path));
    assert_eq!(application.active().unwrap().token, token);
    assert_eq!(application.snapshot(Direction::Downstream).key, key);
    assert!(!model.load_configuration_uri(None).unwrap());
    application.finish(lease).unwrap();
}
