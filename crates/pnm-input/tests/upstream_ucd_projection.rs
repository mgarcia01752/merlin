/*
 * Copyright 2026 MORGANA contributors
 * SPDX-License-Identifier: Apache-2.0
 * Adapted for MERLIN from revision
 * 69b38071cd3ebebc4eb96849f82b7871ab8da5ec.
 * Changes: local adapter imports; read authored JSON fixture directly.
 */
use pnm_input::upstream::reassemble_ucd;
const PLAN: &str =
    include_str!("../../../examples/measurements/upstream/measured/capture-driven.json");

#[test]
fn multipart_projection_has_literal_timestamp_enums_profiles_and_lossless_parts() {
    let plan: serde_json::Value = serde_json::from_str(PLAN).unwrap();
    let bytes: Vec<u8> = serde_json::from_value(plan["ucd_fragments"][0]["bytes"].clone()).unwrap();
    let bytes = &bytes;
    let parsed = reassemble_ucd(std::slice::from_ref(bytes)).unwrap();
    let channel = parsed.project_channel(20_000_000).unwrap();
    assert_eq!(channel.timestamp_snapshot.docsis_timestamp, 0x12345678);
    assert_eq!(channel.timestamp_snapshot.divide_by_20_msb, 9);
    assert_eq!(channel.timestamp_snapshot.minislot_count, 0x0abcdef0);
    assert_eq!(
        (
            channel.cyclic_prefix_samples,
            channel.rolloff_samples,
            channel.subcarrier_spacing_khz
        ),
        (192, 64, 50)
    );
    assert_eq!(
        channel.bursts[0].data_profile.as_ref().unwrap()[0].modulation_index,
        4
    );
    // Divide at a source TLV boundary, retaining repeated fixed fields per Fig30.
    let mut first = bytes[..78].to_vec();
    let mut second = bytes[..24].to_vec();
    second.extend_from_slice(&bytes[78..]);
    first[19] = 0x10;
    second[19] = 0x11;
    for part in [&mut first, &mut second] {
        let length = (part.len() - 14) as u16;
        part[12..14].copy_from_slice(&length.to_be_bytes());
    }
    let fragments = vec![first.clone(), second.clone()];
    assert_eq!(
        reassemble_ucd(&fragments)
            .unwrap()
            .project_channel(20_000_000)
            .unwrap(),
        channel
    );
    assert_eq!(fragments, vec![first.clone(), second.clone()]);
    assert!(reassemble_ucd(&[second.clone(), first.clone()]).is_err());
    assert!(reassemble_ucd(&[first.clone(), first.clone()]).is_err());
    second[21] ^= 1;
    assert!(reassemble_ucd(&[first, second]).is_err());
}
