/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */

//! Real non-Qt source Apply transactions; shared admission is root-integrated separately.
use super::*;
use crate::downstream::workbench::session::{self, Context};
use crate::shell::{ApplicationSessions, RetainedResources};
fn context() -> Context {
    Context::for_application(
        ApplicationSessions::new(RetainedResources {
            payload_bytes: usize::MAX,
            ordered_entries: usize::MAX,
            shared_roots: usize::MAX,
            channel_count: usize::MAX,
            slot_count: usize::MAX,
        })
        .unwrap(),
    )
    .unwrap()
}
fn options() -> String {
    // Existing valid independent continuous/fill seeds; append remains disabled.
    json!({"mode":"file","seed":32767,"append":false,"target_bits":0,"fill_seed":32767}).to_string()
}
fn pressure(used: RetainedResources, dimension: usize) -> RetainedResources {
    // Explicit accounting-only pressure fills actual remaining host policy; not an estimate.
    match dimension {
        0 => RetainedResources {
            payload_bytes: usize::MAX - used.payload_bytes,
            ..Default::default()
        },
        1 => RetainedResources {
            ordered_entries: usize::MAX - used.ordered_entries,
            ..Default::default()
        },
        _ => RetainedResources {
            shared_roots: usize::MAX - used.shared_roots,
            ..Default::default()
        },
    }
}
#[test]
fn source_apply_publishes_matching_original_bytes_and_validation_failure_preserves_acceptance() {
    let context = context();
    let mut model = SourceModel::default();
    let original = b"\0original\n"; // Exact null/newline source bytes, not normalized text.
    model.stage("source.bin".into(), original.to_vec()).unwrap();
    model.apply_transactional(&context, &options()).unwrap();
    assert_eq!(
        session::snapshot(&context)
            .source
            .as_ref()
            .unwrap()
            .config
            .original_bytes(),
        Some(original.as_slice())
    );
    assert_eq!(&*session::saved(&context, "source"), &model.export());
    let before = model.export();
    let invalid =
        json!({"mode":"prbs15","seed":0,"append":false,"target_bits":0,"fill_seed":32767}); // Zero continuous seed must reject.
    assert!(model
        .apply_transactional(&context, &invalid.to_string())
        .is_err());
    assert_eq!(model.export(), before);
    assert_eq!(&*session::saved(&context, "source"), &before);
}
#[test]
fn source_host_rejection_in_every_dimension_preserves_editor_inputs_section_and_staged_draft() {
    for dimension in 0..3 {
        // Payload bytes, ordered entries, shared roots independently.
        let context = context();
        let mut model = SourceModel::default();
        model
            .stage("accepted.bin".into(), b"accepted".to_vec())
            .unwrap();
        model.apply_transactional(&context, &options()).unwrap();
        let editor = model.export();
        let old = session::snapshot(&context);
        let saved = session::saved(&context, "source");
        model
            .stage("candidate.bin".into(), b"candidate".to_vec())
            .unwrap();
        let budget = context.application.budget();
        let before = budget.usage();
        let guard = budget
            .reserve_resources(pressure(before, dimension))
            .unwrap();
        let full = budget.usage();
        assert!(model.apply_transactional(&context, &options()).is_err());
        assert_eq!(model.export(), editor);
        assert_eq!(model.file_summary()["name"], "candidate.bin");
        assert_eq!(
            session::snapshot(&context)
                .source
                .as_ref()
                .unwrap()
                .config
                .original_bytes(),
            old.source.as_ref().unwrap().config.original_bytes()
        );
        assert_eq!(&*session::saved(&context, "source"), &*saved);
        assert_eq!(budget.usage(), full);
        drop(guard);
        assert_eq!(budget.usage(), before);
    }
}
