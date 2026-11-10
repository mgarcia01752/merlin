/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Shared bounded filesystem reads for local workbench inputs.
use cxx_qt_lib::{QString, QUrl};
use std::{fs::File, io::Read};

/// Reads one local regular file with a caller-supplied byte ceiling.
/// # Arguments
/// * `url` - Local file URL; remote schemes reject before filesystem access.
/// * `limit` - Maximum bytes retained, checked before and during reading.
/// # Returns
/// Basename and exact original bytes, without conversion or writes.
/// # Errors
/// Rejects remote URLs, nonregular files, oversize input and filesystem errors.
pub(crate) fn read_local(url: &str, limit: usize) -> Result<(String, Vec<u8>), String> {
    let path = QUrl::from(&QString::from(url))
        .to_local_file()
        .ok_or("Choose a local file.")?;
    let path = std::path::PathBuf::from(path.to_string());
    let metadata =
        std::fs::metadata(&path).map_err(|error| format!("Cannot inspect input: {error}"))?;
    if !metadata.is_file() {
        return Err("Choose a regular input file.".into());
    }
    if metadata.len() > limit as u64 {
        return Err(format!("Input exceeds the {limit}-byte file limit."));
    }
    let file = File::open(&path).map_err(|error| format!("Cannot open input: {error}"))?;
    if !file
        .metadata()
        .map_err(|error| error.to_string())?
        .is_file()
    {
        return Err("Choose a regular input file.".into());
    }
    let mut bytes = Vec::new();
    file.take((limit + 1) as u64)
        .read_to_end(&mut bytes)
        .map_err(|error| format!("Cannot read input: {error}"))?;
    if bytes.len() > limit {
        return Err(format!("Input exceeds the {limit}-byte file limit."));
    }
    Ok((
        path.file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned(),
        bytes,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_reads_preserve_bytes_and_reject_remote_oversize_and_nonregular_inputs() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let source = root.join("examples/payloads/message.txt");
        let url = format!("file://{}", source.display());
        let (name, bytes) = read_local(&url, 1024).unwrap();
        assert_eq!(name, "message.txt");
        assert_eq!(
            bytes,
            include_bytes!("../../../examples/payloads/message.txt")
        );
        assert!(read_local(&url, 1).is_err());
        assert!(read_local("https://example.invalid/payload.bin", 1024).is_err());
        assert!(read_local(&format!("file://{}", root.display()), 1024).is_err());
    }
}
