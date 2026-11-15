/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Receiver API naming preserves saved configurations and existing callers.
use docsis_ofdm_fec::signal_processing::receiver::channel_estimation::ReceiverMode;

#[test]
fn cm_channel_name_preserves_default_alias_and_saved_configuration() {
    assert_eq!(ReceiverMode::default(), ReceiverMode::CmChannelEstimated);
    assert_eq!(
        ReceiverMode::SuppliedChannel,
        ReceiverMode::CmChannelEstimated
    );
    for spelling in ["supplied_channel", "cm_channel_estimated"] {
        let decoded: ReceiverMode = serde_json::from_str(&format!("\"{spelling}\"")).unwrap();
        assert_eq!(decoded, ReceiverMode::CmChannelEstimated);
    }
    assert_eq!(
        serde_json::to_string(&ReceiverMode::CmChannelEstimated).unwrap(),
        "\"supplied_channel\""
    );
    assert_eq!(
        serde_json::to_string(&ReceiverMode::PilotEstimated).unwrap(),
        "\"pilot_estimated\""
    );
}
