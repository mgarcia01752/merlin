/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Borrowed Save preserves the same legacy JSON and original-byte validation.
use super::*;
use crate::downstream::workbench::session::{self, Context};

const PROJECT: &[u8] =
    include_bytes!("../../../../../../examples/workbench/single/workbench.merlin.json");

#[test]
fn borrowed_sections_preserve_schema_originals_and_old_owner_during_atomic_write() {
    let original: Project = serde_json::from_slice(PROJECT).unwrap();
    let context = Context::isolated();
    session::restore(&context, original.validate().unwrap(), original.sections).unwrap();
    let before = context.application.budget().usage();
    let borrowed = Project {
        schema_version: original.schema_version,
        application: original.application,
        transmission: original.transmission,
        sections: session::saved_sections(&context),
        receiver: original.receiver,
        comparison_enabled: original.comparison_enabled,
        dark: original.dark,
    };
    let expected: Value = serde_json::from_slice(PROJECT).unwrap();
    // Serde's typed roundtrip also normalizes existing default/legacy fields.
    let owned: Project = serde_json::from_value(expected).unwrap();
    assert_eq!(
        serde_json::to_vec_pretty(&borrowed).unwrap(),
        serde_json::to_vec_pretty(&owned).unwrap()
    );
    assert_eq!(context.application.budget().usage(), before);
    session::restore(&context, Default::default(), Default::default()).unwrap();
    assert!(session::saved_sections(&context).is_empty());
    assert!(!borrowed.sections.is_empty());
    // Save must continue using the old matching document, while both owners coexist.
    borrowed.validate().unwrap();
    let root =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/development/local/qt-closed-loop");
    std::fs::create_dir_all(&root).unwrap();
    let path = root.join(format!("borrowed-save-{}.merlin.json", std::process::id()));
    borrowed.write(&path).unwrap();
    let restored: Project = serde_json::from_slice(&std::fs::read(&path).unwrap()).unwrap();
    restored.validate().unwrap();
    assert_eq!(
        serde_json::to_vec_pretty(&restored).unwrap(),
        serde_json::to_vec_pretty(&owned).unwrap()
    );
    std::fs::remove_file(path).unwrap();
}

#[test]
fn empty_legacy_and_current_borrowed_projects_keep_the_exact_typed_encoding() {
    for schema_version in [1, 2] {
        // Independent established legacy/current format anchors, not new schemas.
        let context = Context::isolated();
        session::restore(&context, Default::default(), Default::default()).unwrap();
        let borrowed = Project {
            schema_version,
            application: "MERLIN".into(),
            transmission: Default::default(),
            sections: session::saved_sections(&context),
            receiver: None,
            comparison_enabled: false,
            dark: true,
        };
        let bytes = serde_json::to_vec_pretty(&borrowed).unwrap();
        let owned: Project = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(owned.schema_version, schema_version);
        assert!(owned.sections.is_empty());
        owned.validate().unwrap();
        borrowed.validate().unwrap();
        assert_eq!(serde_json::to_vec_pretty(&owned).unwrap(), bytes);
    }
}
