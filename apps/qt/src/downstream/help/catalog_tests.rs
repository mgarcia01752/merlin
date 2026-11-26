/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Owned catalog queries release temporary parsing without altering help output.
use super::*;

// Bundled diagnostic identity used by the existing instrument and FAQ consumers.
const KNOWN_CODE: &str = "QamRunUnavailable";

#[test]
fn simultaneous_catalog_queries_have_independent_owned_storage() {
    let mut first = catalog().unwrap();
    let second = catalog().unwrap();
    assert!(!first.entries.is_empty());
    assert_ne!(first.entries.as_ptr(), second.entries.as_ptr());
    let original = second.entries[0].code.clone();
    assert_eq!(first.entries[0].code, original);
    first.entries[0].code.clear();
    assert_eq!(second.entries[0].code, original);
    drop(first);
    drop(second);
    assert_eq!(catalog().unwrap().entries[0].code, original);
}

#[test]
fn selected_entry_and_detached_help_survive_other_catalog_disposal() {
    let selected = entry(KNOWN_CODE).unwrap();
    let diagnostic = static_diagnostic(KNOWN_CODE);
    let faq = faq_document(KNOWN_CODE);
    let temporary = catalog().unwrap();
    drop(temporary);
    assert_eq!(selected.code, KNOWN_CODE);
    assert_eq!(diagnostic["code"], KNOWN_CODE);
    assert_eq!(faq["anchor"], "qam-profile-has-no-retained-result");
    assert_eq!(static_diagnostic(KNOWN_CODE), diagnostic);
    assert_eq!(faq_document(KNOWN_CODE), faq);
    assert!(entry(&"x".repeat(MAX_DIAGNOSTIC_CODE_BYTES + 1)).is_none());
}

#[test]
fn temporary_catalog_parser_keeps_existing_source_schema_and_entry_limits() {
    let mut exact_source = CATALOG.to_owned();
    exact_source.push_str(&" ".repeat(MAX_HELP_DOCUMENT_BYTES - exact_source.len()));
    assert_eq!(exact_source.len(), MAX_HELP_DOCUMENT_BYTES);
    assert!(catalog_from_source(&exact_source).is_some());
    exact_source.push(' ');
    assert!(catalog_from_source(&exact_source).is_none());
    assert!(catalog_from_source("{").is_none());
    let mut document: Value = serde_json::from_str(CATALOG).unwrap();
    document["schema_version"] = json!(HELP_CATALOG_SCHEMA_VERSION + 1);
    assert!(catalog_from_source(&document.to_string()).is_none());
    document["schema_version"] = json!(HELP_CATALOG_SCHEMA_VERSION);

    // Empty strings isolate the entry-count boundary from the source-byte ceiling.
    let minimal_entry = json!({
        "code":"", "title":"", "faq_heading":"", "faq_anchor":"",
        "summary":"", "context":"", "suggestion":"",
        "over_limit_suggestion":"", "note":""
    });
    document["entries"] = json!(vec![minimal_entry.clone(); MAX_HELP_ENTRIES]);
    let exact = document.to_string();
    assert!(exact.len() <= MAX_HELP_DOCUMENT_BYTES);
    assert_eq!(
        catalog_from_source(&exact).unwrap().entries.len(),
        MAX_HELP_ENTRIES
    );
    document["entries"]
        .as_array_mut()
        .unwrap()
        .push(minimal_entry);
    let above = document.to_string();
    assert!(above.len() <= MAX_HELP_DOCUMENT_BYTES);
    assert!(catalog_from_source(&above).is_none());
    assert!(catalog().is_some());
}
