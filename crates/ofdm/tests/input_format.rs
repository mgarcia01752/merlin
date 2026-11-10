/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
use docsis_ofdm_fec::input_format::{detect, json_bytes, InputFormat};

#[test]
fn contents_determine_format_without_filename_assumptions() {
    for (bytes, expected) in [
        (&b"PNN\x0a\x01\x00"[..], InputFormat::BinaryPnm),
        (&b" \n{\"pnm_header\":{}}"[..], InputFormat::JsonText),
        (&b"\xef\xbb\xbf\t{}"[..], InputFormat::JsonText),
        (&b"plain ASCII text"[..], InputFormat::UnsupportedText),
        (&b"\x00\xff\x01"[..], InputFormat::UnsupportedBinary),
        (&b" \r\n"[..], InputFormat::Empty),
    ] {
        assert_eq!(detect(bytes), expected);
    }
    assert_eq!(json_bytes(b"\xef\xbb\xbf{}"), b"{}");
}
