/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded display rows over backend-decoded descriptor meanings; no wire-code interpretation.
use crate::downstream::common::constants::{
    ASSIGNMENT_BIN_BYTES, ASSIGNMENT_CODE_BYTES, ASSIGNMENT_FIRST_BIN_START,
    ASSIGNMENT_LAST_BIN_START, DESCRIPTOR_TLV_HEADER_BYTES, QPSK_DISPLAY_ORDER,
};
use crate::shared::presentation::{bytes::hex, constants::HERTZ_PER_KILOHERTZ};
use merlin::downstream::{FullLoopDescriptorField, FullLoopDescriptorMeaning, PhysicalCarrierRole};
use serde_json::{json, Value};
use std::sync::OnceLock;

fn registry() -> &'static Value {
    static REGISTRY: OnceLock<Value> = OnceLock::new();
    REGISTRY.get_or_init(|| {
        serde_json::from_str(include_str!(
            "../../../../../../resources/registry/plc-descriptor-tlvs.json"
        ))
        .expect("Bundled descriptor display registry must be valid JSON")
    })
}

fn child(name: &str, value: impl Into<String>, offset: usize, bytes: &[u8]) -> Value {
    json!({"name":name,"value":value.into(),"offset":offset,"length":bytes.len(),"hex":hex(bytes),"children":[]})
}

fn yes_no(enabled: bool) -> &'static str {
    match enabled {
        true => "Yes",
        false => "No",
    }
}

fn assignment(
    summary: String,
    meaning: String,
    first: u16,
    last: u16,
    profile: Option<(bool, u8, &str)>,
    bytes: &[u8],
    offset: usize,
) -> Option<(String, Vec<Value>)> {
    let code = bytes.first()?;
    let first_bytes =
        bytes.get(ASSIGNMENT_FIRST_BIN_START..ASSIGNMENT_FIRST_BIN_START + ASSIGNMENT_BIN_BYTES)?;
    let last_bytes =
        bytes.get(ASSIGNMENT_LAST_BIN_START..ASSIGNMENT_LAST_BIN_START + ASSIGNMENT_BIN_BYTES)?;
    let mut fields = vec![child(
        "Assignment",
        format!("{code} · {meaning}"),
        offset,
        &bytes[..ASSIGNMENT_CODE_BYTES],
    )];
    if let Some((default, bits, modulation)) = profile {
        fields.push(child(
            "Default Assignment",
            yes_no(default),
            offset,
            &bytes[..ASSIGNMENT_CODE_BYTES],
        ));
        fields.push(child(
            "Bit Loading",
            format!("{bits} bits / subcarrier · {modulation}"),
            offset,
            &bytes[..ASSIGNMENT_CODE_BYTES],
        ));
    }
    fields.push(child(
        "First Subcarrier",
        first.to_string(),
        offset + ASSIGNMENT_FIRST_BIN_START,
        first_bytes,
    ));
    fields.push(child(
        "Last Subcarrier",
        last.to_string(),
        offset + ASSIGNMENT_LAST_BIN_START,
        last_bytes,
    ));
    Some((summary, fields))
}

fn formatted(
    meaning: &FullLoopDescriptorMeaning,
    bytes: &[u8],
    offset: usize,
) -> Option<(String, Vec<Value>)> {
    use FullLoopDescriptorMeaning as Meaning;
    let value = match meaning {
        Meaning::Numerology {
            fft_samples,
            spacing_hz,
        } => format!(
            "{fft_samples} samples · {:.0} kHz spacing",
            f64::from(*spacing_hz) / HERTZ_PER_KILOHERTZ
        ),
        Meaning::CyclicPrefix { samples } | Meaning::RollOff { samples } => {
            format!("{samples} samples")
        }
        Meaning::SubcarrierZeroFrequency { frequency_hz } => format!("{frequency_hz} Hz"),
        Meaning::InterleaverDepth { symbols } => symbols.to_string(),
        Meaning::PrimaryCapability { enabled } => yes_no(*enabled).into(),
        Meaning::PhysicalAssignment { role, first, last } => {
            let meaning = match role {
                PhysicalCarrierRole::ContinuousPilot => "Continuous Pilot",
                PhysicalCarrierRole::Excluded => "Excluded",
                PhysicalCarrierRole::Plc => "PLC",
                _ => return None,
            }
            .to_owned();
            return assignment(
                format!("{first}–{last} · {meaning}"),
                meaning,
                *first,
                *last,
                None,
                bytes,
                offset,
            );
        }
        Meaning::ProfileAssignment {
            default,
            bits_per_carrier,
            modulation_order,
            first,
            last,
        } => {
            let modulation = match modulation_order {
                Some(QPSK_DISPLAY_ORDER) => "QPSK".to_owned(),
                Some(order) => format!("{order}-QAM"),
                None => {
                    // Preserve the existing raw-only zero-loading tree presentation.
                    // Backend recognition is distinct from this renderer's supported labels.
                    return None;
                }
            };
            let suffix = match default {
                true => " · default",
                false => "",
            };
            let meaning = format!("{modulation}{suffix}");
            return assignment(
                format!("{first}–{last} · {meaning}"),
                meaning,
                *first,
                *last,
                Some((*default, *bits_per_carrier, &modulation)),
                bytes,
                offset,
            );
        }
        _ => return None,
    };
    Some((
        value.clone(),
        vec![child("Encoded Value", value, offset, bytes)],
    ))
}

fn format_field(
    ocd: bool,
    kind: u8,
    bytes: &[u8],
    offset: usize,
    meaning: Option<&FullLoopDescriptorMeaning>,
) -> Value {
    let family = match ocd {
        true => "OCD",
        false => "DPD",
    };
    let metadata = registry()["fields"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["message"] == family && entry["type"] == kind);
    let name = metadata
        .and_then(|entry| entry["name"].as_str())
        .unwrap_or("Unknown TLV");
    let description = metadata
        .and_then(|entry| entry["description"].as_str())
        .unwrap_or("No display interpretation for this TLV.");
    let value_offset = offset + DESCRIPTOR_TLV_HEADER_BYTES;
    let interpretation = meaning.and_then(|meaning| formatted(meaning, bytes, value_offset));
    let supported = interpretation.is_some();
    let (summary, mut children) =
        interpretation.unwrap_or_else(|| ("Unsupported value · inspect bytes".into(), Vec::new()));
    children.push(child("Value Bytes", hex(bytes), value_offset, bytes));
    json!({"name":name,"kind":kind,"length":bytes.len(),"offset":offset,"hex":hex(bytes),
        "value":summary,"description":description,"supported":supported,"children":children})
}

/// Presents one typed backend descriptor field without decoding or validating wire bytes.
/// # Arguments
/// * `ocd` - Selects the existing OCD or DPD display registry.
/// * `field` - Authoritative decoded meaning, raw value and absolute packet offset.
/// # Returns
/// Existing bounded summary/child rows and exact bytes. Unavailable future meanings
/// stay raw and unsupported; no inferred modulation, timing or assignment is introduced.
pub(super) fn row(ocd: bool, field: &FullLoopDescriptorField) -> Value {
    format_field(
        ocd,
        field.kind,
        &field.value,
        field.offset,
        Some(&field.meaning),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    // Independent display fixtures supply meanings explicitly. Backend-owner tests
    // validate scientific decoding; unavailable raw vectors never get guessed meanings.
    struct FieldFixture {
        kind: u8,
        value: Vec<u8>,
        meaning: Option<FullLoopDescriptorMeaning>,
    }
    fn row(ocd: bool, field: &FieldFixture, offset: usize) -> Value {
        format_field(
            ocd,
            field.kind,
            &field.value,
            offset,
            field.meaning.as_ref(),
        )
    }

    #[test]
    fn timing_fields_use_core_tables_and_preserve_reserved_values() {
        let prefix = row(
            true,
            &FieldFixture {
                kind: 1,
                value: vec![2],
                meaning: Some(FullLoopDescriptorMeaning::CyclicPrefix { samples: 512 }),
            },
            28,
        );
        assert_eq!(prefix["value"], "512 samples");
        assert_eq!(prefix["children"][0]["offset"], 30);
        assert_eq!(prefix["hex"], "02");
        for bytes in [vec![], vec![5], vec![2, 0]] {
            let reserved = row(
                true,
                &FieldFixture {
                    kind: 1,
                    value: bytes,
                    meaning: None,
                },
                28,
            );
            assert_eq!(reserved["supported"], false);
            assert!(reserved["value"].as_str().unwrap().contains("Unsupported"));
        }
        assert_eq!(
            row(
                true,
                &FieldFixture {
                    kind: 0,
                    value: vec![1],
                    meaning: Some(FullLoopDescriptorMeaning::Numerology {
                        fft_samples: 8192,
                        spacing_hz: 25000
                    }),
                },
                0
            )["value"],
            "8192 samples · 25 kHz spacing"
        );
        assert_eq!(
            row(
                true,
                &FieldFixture {
                    kind: 2,
                    value: vec![2],
                    meaning: Some(FullLoopDescriptorMeaning::RollOff { samples: 128 }),
                },
                0
            )["value"],
            "128 samples"
        );
    }

    #[test]
    fn assignment_ranges_default_loading_and_byte_offsets_remain_distinct() {
        let range = row(
            false,
            &FieldFixture {
                kind: 5,
                value: vec![0x28, 0, 0, 0x1f, 0xff],
                meaning: Some(FullLoopDescriptorMeaning::ProfileAssignment {
                    default: true,
                    bits_per_carrier: 8,
                    modulation_order: Some(256),
                    first: 0,
                    last: 8191,
                }),
            },
            46,
        );
        assert_eq!(range["value"], "0–8191 · 256-QAM · default");
        assert_eq!(range["children"][1]["value"], "Yes");
        assert_eq!(range["children"][3]["offset"], 49);
        assert_eq!(range["children"][3]["hex"], "00 00");
        assert_eq!(range["children"][4]["offset"], 51);
        let pilot = row(
            true,
            &FieldFixture {
                kind: 5,
                value: vec![1, 0, 10, 0, 12],
                meaning: Some(FullLoopDescriptorMeaning::PhysicalAssignment {
                    role: PhysicalCarrierRole::ContinuousPilot,
                    first: 10,
                    last: 12,
                }),
            },
            10,
        );
        assert_eq!(pilot["value"], "10–12 · Continuous Pilot");
        let unknown = row(
            false,
            &FieldFixture {
                kind: 0,
                value: vec![1],
                meaning: None,
            },
            10,
        );
        assert_eq!(unknown["name"], "Unknown TLV");
        assert_eq!(unknown["supported"], false);
        for bytes in [
            vec![0x28],
            vec![0x28, 0, 12, 0, 10],
            vec![0x38, 0, 0, 0, 10],
        ] {
            assert_eq!(
                row(
                    false,
                    &FieldFixture {
                        kind: 5,
                        value: bytes,
                        meaning: None,
                    },
                    0
                )["supported"],
                false
            );
        }
    }

    #[test]
    fn registry_covers_only_the_implemented_subset_with_unique_keys() {
        let fields = registry()["fields"].as_array().unwrap();
        assert_eq!(fields.len(), 8);
        let mut keys = std::collections::HashSet::new();
        for field in fields {
            assert!(keys.insert((
                field["message"].as_str().unwrap(),
                field["type"].as_u64().unwrap()
            )));
            assert!(!field["description"].as_str().unwrap().is_empty());
        }
        let frequency = row(
            true,
            &FieldFixture {
                kind: 3,
                value: 631_100_000u32.to_be_bytes().to_vec(),
                meaning: Some(FullLoopDescriptorMeaning::SubcarrierZeroFrequency {
                    frequency_hz: 631_100_000,
                }),
            },
            37,
        );
        assert_eq!(frequency["value"], "631100000 Hz");
        assert_eq!(frequency["children"][0]["length"], 4);
    }
    #[test]
    fn public_zero_loading_and_qpsk_keep_their_existing_distinct_display_states() {
        // Independent fixed-assignment vectors: bit 5 marks a default assignment;
        // zero loading is raw-only in the old viewer, while two bits means QPSK.
        let zero = FieldFixture {
            kind: 5,
            value: vec![0x20, 0, 0, 0, 31],
            meaning: Some(FullLoopDescriptorMeaning::ProfileAssignment {
                default: true,
                bits_per_carrier: 0,
                modulation_order: None,
                first: 0,
                last: 31,
            }),
        };
        let qpsk = FieldFixture {
            kind: 5,
            value: vec![0x22, 0, 0, 0, 31],
            meaning: Some(FullLoopDescriptorMeaning::ProfileAssignment {
                default: true,
                bits_per_carrier: 2,
                modulation_order: Some(4),
                first: 0,
                last: 31,
            }),
        };
        assert_eq!(row(false, &zero, 29)["supported"], false);
        assert_eq!(row(false, &zero, 29)["hex"], "20 00 00 00 1F");
        assert_eq!(row(false, &qpsk, 29)["supported"], true);
        assert_eq!(row(false, &qpsk, 29)["value"], "0–31 · QPSK · default");
    }
}
