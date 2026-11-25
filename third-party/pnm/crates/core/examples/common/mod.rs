/*
 * Copyright 2026 PNM Viewer contributors
 * SPDX-License-Identifier: Apache-2.0
 */
//! Input handling for runnable examples; parsers themselves perform no I/O.
use std::{borrow::Cow, io::Read};

pub type ExampleError = Box<dyn std::error::Error + Send + Sync>;
const MAX_INPUT_BYTES: u64 = 16 * 1024 * 1024;

/// Loads an optional CLI file or borrows the bundled default capture.
///
/// # Arguments
/// * `default_capture` - Static fixture bytes; None requires a file argument.
/// # Returns
/// Owned file bytes or a borrowed fixture, bounded to 16 MiB for file inputs.
/// # Errors
/// Reports missing/extra arguments, file access failures or excessive input size.
pub fn capture_bytes(
    default_capture: Option<&'static [u8]>,
) -> Result<Cow<'static, [u8]>, ExampleError> {
    let mut arguments = std::env::args_os().skip(1);
    let path = arguments.next();
    if arguments.next().is_some() {
        return Err("expected at most one capture file path".into());
    }
    let Some(path) = path else {
        return default_capture
            .map(Cow::Borrowed)
            .ok_or_else(|| "a capture file path is required for this example".into());
    };
    let mut bytes = Vec::new();
    std::fs::File::open(path)?
        .take(MAX_INPUT_BYTES + 1)
        .read_to_end(&mut bytes)?;
    if bytes.len() as u64 > MAX_INPUT_BYTES {
        return Err("capture exceeds example input limit".into());
    }
    Ok(Cow::Owned(bytes))
}
