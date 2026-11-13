/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Shared bounded filesystem reads for local workbench inputs.
use cxx_qt_lib::{QString, QUrl};
use std::{fs::File, io::Read};

thread_local! { static INPUT_PATHS: std::cell::RefCell<std::collections::BTreeSet<std::path::PathBuf>> = const { std::cell::RefCell::new(std::collections::BTreeSet::new()) }; }

/// Reads one local regular file with a caller-supplied byte ceiling.
/// # Arguments
/// * `url` - Local file URL; remote schemes reject before filesystem access.
/// * `limit` - Maximum bytes retained, checked before and during reading.
/// # Returns
/// Basename and exact original bytes, without conversion or writes.
/// # Errors
/// Rejects remote URLs, nonregular files, oversize input and filesystem errors.
pub(crate) fn read_local(url: &str, limit: usize) -> Result<(String, Vec<u8>), String> {
    let result = read_untracked(url, limit)?;
    let path = QUrl::from(&QString::from(url))
        .to_local_file()
        .ok_or("Choose a local file")?;
    let path = std::fs::canonicalize(path.to_string()).map_err(|e| e.to_string())?;
    INPUT_PATHS.with(|paths| -> Result<(), String> {
        let mut paths = paths.borrow_mut();
        if paths.len() >= 4096 && !paths.contains(&path) {
            return Err(
                "Too many original source paths tracked; reopen a saved workbench to release them."
                    .into(),
            );
        }
        paths.insert(path);
        Ok(())
    })?;
    Ok(result)
}

/// Reads a project document without treating its own Save destination as source input.
/// # Arguments
/// * `url` - Local project URI.
/// * `limit` - Maximum project bytes.
/// # Returns
/// Original bounded project bytes and basename.
/// # Errors
/// Reports invalid local paths, file kinds, length or read failures.
pub(crate) fn read_untracked(url: &str, limit: usize) -> Result<(String, Vec<u8>), String> {
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

/// Checks whether a proposed project destination would overwrite an imported source.
/// # Arguments
/// * `path` - Explicit project destination.
/// # Returns
/// True for a currently tracked imported original; missing destinations are false.
pub(crate) fn is_original(path: &std::path::Path) -> bool {
    std::fs::canonicalize(path)
        .ok()
        .is_some_and(|path| INPUT_PATHS.with(|paths| paths.borrow().contains(&path)))
}
/// Releases external source-path protection after adopting self-contained project originals.
pub(crate) fn clear_original_paths() {
    INPUT_PATHS.with(|paths| paths.borrow_mut().clear());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_reads_preserve_bytes_and_reject_remote_oversize_and_nonregular_inputs() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let source = root.join("examples/cli/payloads/message.txt");
        let url = url::Url::from_file_path(source.canonicalize().unwrap())
            .unwrap()
            .to_string();
        let (name, bytes) = read_local(&url, 1024).unwrap();
        assert_eq!(name, "message.txt");
        assert_eq!(
            bytes,
            include_bytes!("../../../../examples/cli/payloads/message.txt")
        );
        assert!(read_local(&url, 1).is_err());
        assert!(read_local("https://example.invalid/payload.bin", 1024).is_err());
        assert!(read_local(
            url::Url::from_directory_path(root.canonicalize().unwrap())
                .unwrap()
                .as_str(),
            1024
        )
        .is_err());
    }
}
