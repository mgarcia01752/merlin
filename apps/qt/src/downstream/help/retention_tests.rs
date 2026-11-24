/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */

//! Independent private-layout and actual text-capacity fixtures; no admission behavior.
use super::*;
use crate::shell::RetainedResources;
use std::mem::size_of;

// A real bundled export diagnostic supplies the existing private guidance shape.
const FIXTURE_DIAGNOSTIC_CODE: &str = "RxmerPnmExportRange";
const FIXTURE_MESSAGE: &str = "Original Designer worker failure.";

// Independent requested spare text storage, not application or scientific limits.
const EXTRA_MESSAGE_BYTES: usize = 31;
const EXTRA_DIAGNOSTIC_BYTES: usize = 37;

enum DiagnosticText {
    Code,
    Title,
    Tooltip,
    FaqAnchor,
}

#[test]
fn actual_error_and_box_layout_count_once_while_each_text_capacity_delta_is_passive() {
    let mut message = String::with_capacity(FIXTURE_MESSAGE.len() + EXTRA_MESSAGE_BYTES);
    message.push_str(FIXTURE_MESSAGE);
    let mut error = AdmissionError {
        message,
        diagnostic: Some(Box::new(
            catalog_diagnostic(FIXTURE_DIAGNOSTIC_CODE).unwrap(),
        )),
    };
    let diagnostic = error.diagnostic.as_ref().unwrap();
    let mut expected = RetainedResources {
        payload_bytes: size_of::<AdmissionError>()
            + error.message.capacity()
            + size_of::<Diagnostic>()
            + diagnostic.code.capacity()
            + diagnostic.title.capacity()
            + diagnostic.tooltip.capacity()
            + diagnostic.faq_anchor.capacity(),
        ordered_entries: 0,
        shared_roots: 0,
    };
    // Serialization is only an independent value snapshot; no byte charge is
    // inferred from it and production metadata never serializes a diagnostic.
    let original = serde_json::to_value(diagnostic.as_ref()).unwrap();
    let diagnostic_pointer = std::ptr::from_ref(diagnostic.as_ref());
    let message_pointer = error.message.as_ptr();
    assert_eq!(error.retained_resources().unwrap(), expected);
    assert_eq!(error.retained_resources().unwrap(), expected);
    assert_eq!(error.message.as_ptr(), message_pointer);
    assert_eq!(error.to_string(), FIXTURE_MESSAGE);

    let old = error.message.capacity();
    error.message.reserve_exact(old + EXTRA_MESSAGE_BYTES);
    expected.payload_bytes += error.message.capacity() - old;
    assert_eq!(error.retained_resources().unwrap(), expected);
    for field in [
        DiagnosticText::Code,
        DiagnosticText::Title,
        DiagnosticText::Tooltip,
        DiagnosticText::FaqAnchor,
    ] {
        let diagnostic = error.diagnostic.as_mut().unwrap();
        let text = match field {
            DiagnosticText::Code => &mut diagnostic.code,
            DiagnosticText::Title => &mut diagnostic.title,
            DiagnosticText::Tooltip => &mut diagnostic.tooltip,
            DiagnosticText::FaqAnchor => &mut diagnostic.faq_anchor,
        };
        let old = text.capacity();
        text.reserve_exact(old + EXTRA_DIAGNOSTIC_BYTES);
        expected.payload_bytes += text.capacity() - old;
        assert_eq!(error.retained_resources().unwrap(), expected);
    }
    assert_eq!(error.retained_resources().unwrap(), expected);
    assert_eq!(error.to_string(), FIXTURE_MESSAGE);
    let diagnostic = error.diagnostic.as_ref().unwrap();
    assert_eq!(serde_json::to_value(diagnostic.as_ref()).unwrap(), original);
    assert_eq!(std::ptr::from_ref(diagnostic.as_ref()), diagnostic_pointer);
}

#[test]
fn missing_diagnostic_charges_only_the_error_root_and_actual_message_buffer() {
    let mut message = String::with_capacity(FIXTURE_MESSAGE.len() + EXTRA_MESSAGE_BYTES);
    message.push_str(FIXTURE_MESSAGE);
    let error = AdmissionError::from(message);
    let expected = RetainedResources {
        payload_bytes: size_of::<AdmissionError>() + error.message.capacity(),
        ordered_entries: 0,
        shared_roots: 0,
    };
    assert!(error.diagnostic.is_none());
    assert_eq!(error.retained_resources().unwrap(), expected);
    assert_eq!(error.retained_resources().unwrap(), expected);
    assert_eq!(error.to_string(), FIXTURE_MESSAGE);
    let empty = AdmissionError::from(String::new());
    assert_eq!(
        empty.retained_resources().unwrap(),
        RetainedResources {
            payload_bytes: size_of::<AdmissionError>(),
            ordered_entries: 0,
            shared_roots: 0,
        }
    );
    assert!(empty.diagnostic.is_none());
    assert!(empty.message.is_empty());
}
