/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Separate Rust-process metadata over the same original native input fixtures.
use docsis_ofdm_fec::api::{
    NamedCapture, PrbsSeed, SourceInput, TransmissionConfiguration, TransmitterPreviewControl,
    TransmitterPreviewRequest,
};
use pnm_input::{CaptureLoader, RxMerDesign, RxMerDesignPosition};
use serde_json::json;

const PROFILE: &[u8] =
    include_bytes!("../../../examples/measurements/single-channel/modulation_profile.json");
const RESPONSE: &[u8] =
    include_bytes!("../../../examples/measurements/single-channel/channel_estimation.json");
const RXMER: &[u8] = include_bytes!("../../../examples/measurements/single-channel/rxmer.json");
const DESIGN: &[u8] = include_bytes!("../tests/fixtures/designer-flat.json");
/// Original finite source includes high bits and a zero octet; no fill requested.
const FILE: &[u8] = &[0x81, 0x00, 0x5a];
/// Independent valid recurrence seed for the metadata fixture's idle TX preview.
const PREVIEW_SEED: u16 = 7;
/// Original encoded Profile A identity; independent of a vector position.
const FIXTURE_PROFILE_ID: u8 = 0;
/// One-based first original baseline selected for this fixture.
const DESIGN_CAPTURE_INDEX: usize = 1;
/// Complete original baseline collection contains one capture.
const DESIGN_CAPTURE_COUNT: usize = 1;
/// Small original sample prefix, independent of PSD/retention and source lengths.
const SAMPLE_PREFIX_COUNT: usize = 3;

/// Writes real owner metadata and unchanged public observations for native parity.
///
/// # Returns
/// JSON from public Rust owners, including Rust-only named/provenance roots.
///
/// # Errors
/// Propagates existing admission, generation, metadata and serialization errors.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = NamedCapture::new("profile", PROFILE.to_vec())?;
    let capture = CaptureLoader::profile_capture(&input)?;
    let response =
        CaptureLoader::channel_estimate(&NamedCapture::new("channel", RESPONSE.to_vec())?)?;
    let source = SourceInput::prbs15(PrbsSeed::new(PREVIEW_SEED)?);
    let file = SourceInput::file(NamedCapture::new("file", FILE.to_vec())?, None)?;
    let transmission = TransmissionConfiguration::new(Default::default())?;
    let request = TransmitterPreviewRequest::from_capture(
        &transmission,
        &capture,
        FIXTURE_PROFILE_ID,
        &source,
    )?;
    let control = TransmitterPreviewControl::new();
    let preview = request.generate(&control)?;
    let saved = RxMerDesign::from_bytes(&NamedCapture::new("design", DESIGN.to_vec())?)?;
    let imported = RxMerDesign::imported_capture(
        &capture,
        FIXTURE_PROFILE_ID,
        &NamedCapture::new("baseline", RXMER.to_vec())?,
        RxMerDesign::recipe_from_json(b"{}")?,
        RxMerDesignPosition::new(DESIGN_CAPTURE_INDEX, DESIGN_CAPTURE_COUNT)?,
    )?;
    println!(
        "{}",
        serde_json::to_string(&json!({
            "owners": {
                "source": source.retained_footprint()?,
                "file": file.retained_footprint()?,
                "capture": capture.retained_footprint()?,
                "response": response.retained_footprint()?,
                "request": request.retained_footprint()?,
                "preview": preview.retained_footprint()?,
                "saved": saved.retained_footprint()?,
                "imported": imported.retained_footprint()?,
            },
            "rust_only": {
                "named": input.retained_footprint()?,
                "provenance": capture.provenance().retained_footprint()?,
            },
            "preview_info": preview.info(),
            "preview_samples": preview.sample_view(0, SAMPLE_PREFIX_COUNT)?,
            "capture_provenance": capture.provenance(),
            "response_frame": response.frame(),
            "saved_original": saved.original_bytes(),
            "imported_original": imported.original_bytes(),
        }))?
    );
    Ok(())
}
