/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
use pnm::pnm::{
    analysis::upstream_pre_equalization::comparison::{precheck, CorrectionMode, PreEqComparison},
    parser::cm_us_ofdma_pre_eq::CmUsOfdmaPreEq,
};
fn synthetic(last: bool, pairs: &[[i16; 2]], time: u32) -> Vec<u8> {
    let mut bytes = if last {
        pnm_fixtures::UPSTREAM_PRE_EQ_LAST[..34].to_vec()
    } else {
        pnm_fixtures::UPSTREAM_PRE_EQ[..34].to_vec()
    };
    bytes[6..10].copy_from_slice(&time.to_be_bytes());
    bytes[30..34].copy_from_slice(&((pairs.len() * 4) as u32).to_be_bytes());
    for pair in pairs {
        for sample in pair {
            bytes.extend(sample.to_be_bytes());
        }
    }
    bytes
}
fn decode(bytes: &[u8]) -> CmUsOfdmaPreEq<'_> {
    CmUsOfdmaPreEq::parse(bytes, Default::default()).unwrap()
}
#[test]
fn precheck_requires_cm_cmts_grid_count_roles_and_strictly_newer_time() {
    let original = synthetic(false, &[[8192, 0]; 3], 100);
    let update = synthetic(true, &[[16384, 0]; 3], 101);
    let pair = precheck(&decode(&original), &decode(&update)).unwrap();
    assert_eq!(pair.file_time_difference_s, 1);
    assert_eq!(pair.sample_count, 3);
    let metadata = serde_json::to_value(&pair).unwrap();
    assert_eq!(metadata["mac_address"], "00:ca:00:00:00:01");
    assert_eq!(metadata["cmts_mac_address"], "00:ca:00:00:00:01");
    for (offset, field) in [
        (10, "channel_id"),
        (11, "mac_address"),
        (17, "cmts_mac_address"),
        (23, "subcarrier_zero_frequency_hz"),
        (28, "first_active_subcarrier_index"),
        (29, "subcarrier_spacing_khz"),
    ] {
        let mut wrong = update.clone();
        wrong[offset] ^= 1;
        assert_eq!(
            precheck(&decode(&original), &decode(&wrong))
                .unwrap_err()
                .field,
            field
        );
    }
    for time in [0, 99, 100] {
        let wrong = synthetic(true, &[[16384, 0]; 3], time);
        assert_eq!(
            precheck(&decode(&original), &decode(&wrong))
                .unwrap_err()
                .field,
            "capture_time"
        );
    }
    assert_eq!(
        precheck(&decode(&update), &decode(&original))
            .unwrap_err()
            .field,
        "original_type"
    );
    assert_eq!(
        precheck(&decode(&original), &decode(&original))
            .unwrap_err()
            .field,
        "correction_type"
    );
    assert_eq!(
        precheck(
            &decode(&original),
            &decode(&synthetic(true, &[[16384, 0]; 2], 101))
        )
        .unwrap_err()
        .field,
        "data_length"
    );
    assert_eq!(
        precheck(
            &decode(&synthetic(false, &[], 100)),
            &decode(&synthetic(true, &[], 101))
        )
        .unwrap_err()
        .field,
        "sample_count"
    );
    assert!(precheck(
        &decode(pnm_fixtures::UPSTREAM_PRE_EQ),
        &decode(pnm_fixtures::UPSTREAM_PRE_EQ_LAST)
    )
    .is_err());
}
#[test]
fn prediction_then_delta_normalization_and_phase_change_have_independent_expectations() {
    let original = synthetic(false, &[[8192, 0]; 3], 100);
    let half = synthetic(true, &[[8192, 0]; 3], 101);
    let result =
        PreEqComparison::calculate(&decode(&original), &decode(&half), CorrectionMode::Adjust)
            .unwrap();
    assert_eq!(result.points[0].predicted_iq, Some([0.5, 0.]));
    assert_eq!(result.points[0].delta_iq, Some([-0.5, 0.]));
    assert!((result.points[0].delta_db.unwrap() + 6.020599913279624).abs() < 1e-12);
    assert_eq!(result.points[0].aligned_delta_db, Some(0.));
    assert_eq!(result.summary.predicted_magnitude_span_db, Some(0.));
    assert_eq!(result.points[1].delta_delay_s, Some(0.));
    let phase = synthetic(true, &[[0, 16384]; 3], 101);
    let result =
        PreEqComparison::calculate(&decode(&original), &decode(&phase), CorrectionMode::Adjust)
            .unwrap();
    assert_eq!(result.points[0].delta_iq, Some([-1., 1.]));
    assert_eq!(result.points[0].delta_db, Some(0.));
    let doubled = synthetic(false, &[[16384, 0]; 3], 100);
    let set =
        PreEqComparison::calculate(&decode(&doubled), &decode(&half), CorrectionMode::Set).unwrap();
    assert_eq!(set.points[0].predicted_iq, Some([0.5, 0.]));
    assert_eq!(set.points[0].delta_iq, Some([-1.5, 0.]));
    let zero = synthetic(true, &[[0, 0]; 3], 101);
    let result =
        PreEqComparison::calculate(&decode(&original), &decode(&zero), CorrectionMode::Adjust)
            .unwrap();
    assert_eq!(result.points[0].predicted_iq, Some([0., 0.]));
    assert_eq!(result.points[0].predicted_db, None);
    assert_eq!(result.points[0].predicted_aligned_db, None);
    assert_eq!(result.points[0].delta_delay_s, None);
    let json: serde_json::Value = serde_json::from_str(&result.to_json(16384).unwrap()).unwrap();
    assert_eq!(json["points"][0]["delta_db"], serde_json::Value::Null);
    assert!(result.to_json(12).is_err());
}
