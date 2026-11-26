/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Bounded separate-process reference for passive original/fill source metadata.
use docsis_ofdm_fec::api::{FileFillPolicy, NamedCapture, PrbsSeed, SourceInput};
use serde_json::json;

/// Explicit nonzero register state of the reference continuous source, not a default.
const CONTINUOUS_SEED: u16 = 1;
/// Independent append recurrence state for the 11-bit partial-byte fixture.
const PARTIAL_FILL_SEED: u16 = 7;
/// Independent seed of the explicit zero-length append, separate from continuous PRBS.
const EQUAL_TARGET_FILL_SEED: u16 = 1;
/// Original high-bit-set octet shared by the three finite one-byte reference inputs.
const REFERENCE_FILE_OCTET: u8 = 0x80;
/// Meaningful total target for one actual octet plus three independently generated bits.
const PARTIAL_TARGET_BITS: u64 = 11;
/// Exact one-octet target exercises an explicit policy that appends zero bits.
const EQUAL_TARGET_BITS: u64 = 8;

/// Prints finite-source limits and original/fill summaries without running a generator.
/// # Returns
/// Bounded owned JSON; continuous PRBS remains distinct from named/unnamed/empty files.
/// # Errors
/// Propagates original source/seed/target admission or JSON transport failures.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sources = [
        SourceInput::prbs15(PrbsSeed::new(CONTINUOUS_SEED)?),
        SourceInput::file(NamedCapture::new("empty", Vec::new())?, None)?,
        SourceInput::unnamed_file(&[REFERENCE_FILE_OCTET], None)?,
        SourceInput::unnamed_file(
            &[REFERENCE_FILE_OCTET],
            Some(FileFillPolicy::new(
                PARTIAL_TARGET_BITS,
                PrbsSeed::new(PARTIAL_FILL_SEED)?,
            )?),
        )?,
        SourceInput::file(
            NamedCapture::new("equal", vec![REFERENCE_FILE_OCTET])?,
            Some(FileFillPolicy::new(
                EQUAL_TARGET_BITS,
                PrbsSeed::new(EQUAL_TARGET_FILL_SEED)?,
            )?),
        )?,
    ];
    let info = sources
        .iter()
        .map(SourceInput::file_info)
        .collect::<Vec<_>>();
    let extended = sources[2].with_fill(Some(FileFillPolicy::new(
        PARTIAL_TARGET_BITS,
        PrbsSeed::new(PARTIAL_FILL_SEED)?,
    )?))?;
    let reset = extended.with_fill(None)?;
    println!(
        "{}",
        serde_json::to_string(&json!({"limits":SourceInput::file_limits(),"info":info,
            "edits":[extended.file_info(),reset.file_info()]}))?
    );
    Ok(())
}
