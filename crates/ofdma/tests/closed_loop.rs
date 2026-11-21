/*
 * Copyright 2026 DOCSIS OFDM Channel Sim contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Integrated public-session coverage beyond the retained source matrix.
mod common;
use docsis_ofdma::api::*;
use fec_codec::upstream_ldpc::DocsisLdpcCode;

fn recover(mode: Numerology, loading: u8, frames: usize, family: Option<DocsisLdpcCode>) {
    let pattern = if mode == Numerology::Khz50 { 1 } else { 8 };
    let config = ExperimentConfig::new(
        common::configuration(mode, loading, pattern, frames, 8),
        0x1234,
        ReceiverConfig::new(Estimator::FrameFlatLs, 0.02, 1e-12, 50).unwrap(),
    )
    .unwrap();
    let mut session = Simulation::new(ExperimentOptions {
        retained_points: 0,
        retained_taps: vec![],
        ..Default::default()
    })
    .unwrap();
    session.configure(config).unwrap();
    let preview = session.preview().unwrap();
    if let Some(family) = family {
        assert!(preview
            .grant_plan
            .codewords
            .iter()
            .any(|word| word.code == family));
    }
    let report = session.run().unwrap();
    assert_eq!(report.status, RunStatus::Completed);
    assert!(report.reference_recovered);
    assert_eq!(report.bit_errors, 0);
    assert_eq!(report.generated_source_bits, preview.source_bits);
    assert_eq!(report.compared_source_bits, preview.source_bits);
    assert!(report
        .grants
        .iter()
        .all(|grant| grant.unverified_source_bits == 0));
}

#[test]
fn khz50_qam_bits_2_recovers() {
    recover(Numerology::Khz50, 2, 1, None);
}

#[test]
fn khz50_qam_bits_3_recovers() {
    recover(Numerology::Khz50, 3, 1, None);
}

#[test]
fn khz50_qam_bits_4_recovers() {
    recover(Numerology::Khz50, 4, 1, None);
}

#[test]
fn khz50_qam_bits_5_recovers() {
    recover(Numerology::Khz50, 5, 1, None);
}

#[test]
fn khz50_qam_bits_6_recovers() {
    recover(Numerology::Khz50, 6, 1, None);
}

#[test]
fn khz50_qam_bits_7_recovers() {
    recover(Numerology::Khz50, 7, 1, None);
}

#[test]
fn khz50_qam_bits_8_recovers() {
    recover(Numerology::Khz50, 8, 1, None);
}

#[test]
fn khz50_qam_bits_9_recovers() {
    recover(Numerology::Khz50, 9, 1, None);
}

#[test]
fn khz50_qam_bits_10_recovers() {
    recover(Numerology::Khz50, 10, 1, None);
}

#[test]
fn khz50_qam_bits_11_recovers() {
    recover(Numerology::Khz50, 11, 1, None);
}

#[test]
fn khz50_qam_bits_12_recovers() {
    recover(Numerology::Khz50, 12, 1, None);
}

#[test]
fn khz50_short_family_recovers() {
    recover(Numerology::Khz50, 4, 1, Some(DocsisLdpcCode::UpstreamShort));
}

#[test]
fn khz50_medium_family_recovers() {
    recover(
        Numerology::Khz50,
        4,
        3,
        Some(DocsisLdpcCode::UpstreamMedium),
    );
}

#[test]
fn khz50_long_family_recovers() {
    recover(Numerology::Khz50, 4, 12, Some(DocsisLdpcCode::UpstreamLong));
}

#[test]
fn khz25_qam_bits_2_recovers() {
    recover(Numerology::Khz25, 2, 1, None);
}

#[test]
fn khz25_qam_bits_3_recovers() {
    recover(Numerology::Khz25, 3, 1, None);
}

#[test]
fn khz25_qam_bits_4_recovers() {
    recover(Numerology::Khz25, 4, 1, None);
}

#[test]
fn khz25_qam_bits_5_recovers() {
    recover(Numerology::Khz25, 5, 1, None);
}

#[test]
fn khz25_qam_bits_6_recovers() {
    recover(Numerology::Khz25, 6, 1, None);
}

#[test]
fn khz25_qam_bits_7_recovers() {
    recover(Numerology::Khz25, 7, 1, None);
}

#[test]
fn khz25_qam_bits_8_recovers() {
    recover(Numerology::Khz25, 8, 1, None);
}

#[test]
fn khz25_qam_bits_9_recovers() {
    recover(Numerology::Khz25, 9, 1, None);
}

#[test]
fn khz25_qam_bits_10_recovers() {
    recover(Numerology::Khz25, 10, 1, None);
}

#[test]
fn khz25_qam_bits_11_recovers() {
    recover(Numerology::Khz25, 11, 1, None);
}

#[test]
fn khz25_qam_bits_12_recovers() {
    recover(Numerology::Khz25, 12, 1, None);
}

#[test]
fn khz25_short_family_recovers() {
    recover(Numerology::Khz25, 4, 1, Some(DocsisLdpcCode::UpstreamShort));
}

#[test]
fn khz25_medium_family_recovers() {
    recover(
        Numerology::Khz25,
        4,
        3,
        Some(DocsisLdpcCode::UpstreamMedium),
    );
}

#[test]
fn khz25_long_family_recovers() {
    recover(Numerology::Khz25, 4, 12, Some(DocsisLdpcCode::UpstreamLong));
}
