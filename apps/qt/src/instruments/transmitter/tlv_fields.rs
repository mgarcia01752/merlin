/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded display rows for the existing fixed OCD/DPD implementation subset.
use docsis_ofdm_fec::{
    transmission::{
        plc::descriptors::DescriptorTlv,
        settings::{CYCLIC_PREFIX_SAMPLES, ROLL_OFF_SAMPLES},
        SubcarrierSpacing,
    },
    types::ofdm_modulation::OfdmModulation,
};
use serde_json::{json, Value};
use std::sync::OnceLock;

fn registry() -> &'static Value {
    static REGISTRY: OnceLock<Value> = OnceLock::new();
    REGISTRY.get_or_init(|| {
        serde_json::from_str(include_str!(
            "../../../../../resources/registry/plc-descriptor-tlvs.json"
        ))
        .expect("Bundled descriptor display registry must be valid JSON")
    })
}

fn hex(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|byte| format!("{byte:02X}"))
        .collect::<Vec<_>>()
        .join(" ")
}

fn child(name: &str, value: impl Into<String>, offset: usize, bytes: &[u8]) -> Value {
    json!({"name":name,"value":value.into(),"offset":offset,"length":bytes.len(),"hex":hex(bytes),"children":[]})
}

fn assignment(ocd: bool, bytes: &[u8], offset: usize) -> Option<(String, Vec<Value>)> {
    let [code, first_hi, first_lo, last_hi, last_lo] = bytes else {
        return None;
    };
    let first = u16::from_be_bytes([*first_hi, *first_lo]);
    let last = u16::from_be_bytes([*last_hi, *last_lo]);
    if first > last || code & 0xc0 != 0 {
        return None;
    }
    let mut fields = Vec::new();
    let meaning = if ocd {
        match code {
            1 => "Continuous Pilot".to_string(),
            16 => "Excluded".to_string(),
            20 => "PLC".to_string(),
            _ => return None,
        }
    } else {
        // Match the fixed receiver decoder's default bit and loading nibble.
        // Bit 4 is not used by MERLIN's generated subset; keep it unsupported.
        if code & 0x10 != 0 {
            return None;
        }
        let bits = code & 15;
        let modulation = if bits == 2 {
            "QPSK".to_string()
        } else {
            format!("{}-QAM", OfdmModulation::from_bits(bits)?.order)
        };
        let default = code & 0x20 != 0;
        fields.push(child(
            "Default Assignment",
            if default { "Yes" } else { "No" },
            offset,
            &bytes[..1],
        ));
        fields.push(child(
            "Bit Loading",
            format!("{bits} bits / subcarrier · {modulation}"),
            offset,
            &bytes[..1],
        ));
        format!("{modulation}{}", if default { " · default" } else { "" })
    };
    fields.insert(
        0,
        child(
            "Assignment",
            format!("{code} · {meaning}"),
            offset,
            &bytes[..1],
        ),
    );
    fields.push(child(
        "First Subcarrier",
        first.to_string(),
        offset + 1,
        &bytes[1..3],
    ));
    fields.push(child(
        "Last Subcarrier",
        last.to_string(),
        offset + 3,
        &bytes[3..5],
    ));
    Some((format!("{first}–{last} · {meaning}"), fields))
}

fn decoded(ocd: bool, kind: u8, bytes: &[u8], offset: usize) -> Option<(String, Vec<Value>)> {
    if kind == 5 {
        return assignment(ocd, bytes, offset);
    }
    if !ocd {
        return None;
    }
    let value = match (kind, bytes) {
        (0, [code]) => {
            let spacing = match code {
                0 => SubcarrierSpacing::Khz50,
                1 => SubcarrierSpacing::Khz25,
                _ => return None,
            };
            format!(
                "{} samples · {} kHz spacing",
                spacing.fft_samples(),
                spacing.frequency_hz() / 1000
            )
        }
        (1, [code]) => format!("{} samples", CYCLIC_PREFIX_SAMPLES.get(usize::from(*code))?),
        (2, [code]) => format!("{} samples", ROLL_OFF_SAMPLES.get(usize::from(*code))?),
        (3, [a, b, c, d]) => format!("{} Hz", u32::from_be_bytes([*a, *b, *c, *d])),
        (4, [depth]) => depth.to_string(),
        (6, [0]) => "No".to_string(),
        (6, [1]) => "Yes".to_string(),
        _ => return None,
    };
    Some((
        value.clone(),
        vec![child("Encoded Value", value, offset, bytes)],
    ))
}

/// Builds an inspectable tree node without mutating or validating a packet.
/// # Arguments
/// * `ocd` - True for OCD; false for the existing DPD assignment subset.
/// * `tlv` - Exact retained type and value bytes from the core descriptor parser.
/// * `offset` - Absolute type-byte offset in the complete MAC packet.
/// # Returns
/// Summary, interpreted children and exact value bytes with absolute offsets.
/// Unsupported forms remain raw and explicitly marked rather than guessed.
pub(super) fn row(ocd: bool, tlv: &DescriptorTlv, offset: usize) -> Value {
    let family = if ocd { "OCD" } else { "DPD" };
    let metadata = registry()["fields"]
        .as_array()
        .unwrap()
        .iter()
        .find(|entry| entry["message"] == family && entry["type"] == tlv.kind);
    let name = metadata
        .and_then(|entry| entry["name"].as_str())
        .unwrap_or("Unknown TLV");
    let description = metadata
        .and_then(|entry| entry["description"].as_str())
        .unwrap_or("No display interpretation for this TLV.");
    let interpretation = decoded(ocd, tlv.kind, &tlv.value, offset + 2);
    let supported = interpretation.is_some();
    let (summary, mut children) = interpretation
        .unwrap_or_else(|| ("Unsupported value · inspect bytes".to_string(), Vec::new()));
    children.push(child(
        "Value Bytes",
        hex(&tlv.value),
        offset + 2,
        &tlv.value,
    ));
    json!({"name":name,"kind":tlv.kind,"length":tlv.value.len(),"offset":offset,"hex":hex(&tlv.value),"value":summary,"description":description,"supported":supported,"children":children})
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn timing_fields_use_core_tables_and_preserve_reserved_values() {
        let prefix = row(
            true,
            &DescriptorTlv {
                kind: 1,
                value: vec![2],
            },
            28,
        );
        assert_eq!(prefix["value"], "512 samples");
        assert_eq!(prefix["children"][0]["offset"], 30);
        assert_eq!(prefix["hex"], "02");
        for bytes in [vec![], vec![5], vec![2, 0]] {
            let reserved = row(
                true,
                &DescriptorTlv {
                    kind: 1,
                    value: bytes,
                },
                28,
            );
            assert_eq!(reserved["supported"], false);
            assert!(reserved["value"].as_str().unwrap().contains("Unsupported"));
        }
        assert_eq!(
            row(
                true,
                &DescriptorTlv {
                    kind: 0,
                    value: vec![1]
                },
                0
            )["value"],
            "8192 samples · 25 kHz spacing"
        );
        assert_eq!(
            row(
                true,
                &DescriptorTlv {
                    kind: 2,
                    value: vec![2]
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
            &DescriptorTlv {
                kind: 5,
                value: vec![0x28, 0, 0, 0x1f, 0xff],
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
            &DescriptorTlv {
                kind: 5,
                value: vec![1, 0, 10, 0, 12],
            },
            10,
        );
        assert_eq!(pilot["value"], "10–12 · Continuous Pilot");
        let unknown = row(
            false,
            &DescriptorTlv {
                kind: 0,
                value: vec![1],
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
                    &DescriptorTlv {
                        kind: 5,
                        value: bytes
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
            &DescriptorTlv {
                kind: 3,
                value: 631_100_000u32.to_be_bytes().to_vec(),
            },
            37,
        );
        assert_eq!(frequency["value"], "631100000 Hz");
        assert_eq!(frequency["children"][0]["length"], 4);
    }
}
