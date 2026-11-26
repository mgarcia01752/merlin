/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Shared bounded filesystem reads for local workbench inputs.
use crate::shell::{ApplicationSessions, Direction};
use cxx_qt_lib::{QString, QUrl};
use std::{fs::File, io::Read};

/// Reads one local regular file with a caller-supplied byte ceiling.
/// # Arguments
/// * `application` - Exact canonical host whose Save operations protect originals.
/// * `direction` - Explicit importing owner; no tab or thread-global inference.
/// * `url` - Local file URL; remote schemes reject before filesystem access.
/// * `limit` - Maximum bytes retained, checked before and during reading.
/// # Returns
/// Basename and exact original bytes, without conversion or writes.
/// # Errors
/// Rejects remote URLs, nonregular files, oversize input, filesystem errors and
/// registry admission/cap failure; previously protected paths remain unchanged.
pub(crate) fn read_local(
    application: &ApplicationSessions,
    direction: Direction,
    url: &str,
    limit: usize,
) -> Result<(String, Vec<u8>), String> {
    let result = read_untracked(url, limit)?;
    let path = QUrl::from(&QString::from(url))
        .to_local_file()
        .ok_or("Choose a local file")?;
    let path = std::fs::canonicalize(path.to_string()).map_err(|e| e.to_string())?;
    application.register_input_path(direction, path)?;
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
/// * `application` - Actual saving host; protection is the union of its directions.
/// * `path` - Explicit project destination.
/// # Returns
/// True for a currently tracked imported original; missing destinations are false.
pub(crate) fn is_original(application: &ApplicationSessions, path: &std::path::Path) -> bool {
    std::fs::canonicalize(path)
        .ok()
        .is_some_and(|path| application.protects_input_path(&path))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn local_reads_preserve_bytes_and_reject_remote_oversize_and_nonregular_inputs() {
        let application = ApplicationSessions::desktop().unwrap();
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
        let source = root.join("examples/cli/payloads/message.txt");
        let url = url::Url::from_file_path(source.canonicalize().unwrap())
            .unwrap()
            .to_string();
        let (name, bytes) = read_local(&application, Direction::Downstream, &url, 1024).unwrap();
        assert_eq!(name, "message.txt");
        assert_eq!(
            bytes,
            include_bytes!("../../../../examples/cli/payloads/message.txt")
        );
        assert!(read_local(&application, Direction::Downstream, &url, 1).is_err());
        assert!(read_local(
            &application,
            Direction::Downstream,
            "https://example.invalid/payload.bin",
            1024
        )
        .is_err());
        assert!(read_local(
            &application,
            Direction::Downstream,
            url::Url::from_directory_path(root.canonicalize().unwrap())
                .unwrap()
                .as_str(),
            1024
        )
        .is_err());
    }

    #[cfg(unix)]
    #[test]
    fn canonical_symlink_protection_is_host_scoped_and_preserves_direction_union() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../docs/development/local/source-path-admission");
        std::fs::create_dir_all(&root).unwrap();
        let root = root.canonicalize().unwrap();
        let original = root.join(format!("read-original-{}.bin", std::process::id()));
        let alias = root.join(format!("read-alias-{}.bin", std::process::id()));

        // Exact original bytes; symlink naming does not change the read payload.
        const ORIGINAL: &[u8] = b"exact\0original\n";
        std::fs::write(&original, ORIGINAL).unwrap();
        std::os::unix::fs::symlink(original.canonicalize().unwrap(), &alias).unwrap();
        let application = ApplicationSessions::desktop().unwrap();
        let independent = ApplicationSessions::desktop().unwrap();
        let alias_url = url::Url::from_file_path(&alias).unwrap().to_string();
        let original_url = url::Url::from_file_path(original.canonicalize().unwrap())
            .unwrap()
            .to_string();
        let (_, bytes) = read_local(
            &application,
            Direction::Downstream,
            &alias_url,
            ORIGINAL.len(),
        )
        .unwrap();
        assert_eq!(bytes, ORIGINAL);
        assert!(is_original(&application, &original));
        assert!(is_original(&application, &alias));
        assert!(!is_original(&independent, &original));
        read_local(
            &application,
            Direction::Upstream,
            &original_url,
            ORIGINAL.len(),
        )
        .unwrap();
        application.clear_input_paths(Direction::Downstream);
        assert!(is_original(&application, &alias));
        application.clear_input_paths(Direction::Upstream);
        assert!(!is_original(&application, &original));
        assert_eq!(std::fs::read(&original).unwrap(), ORIGINAL);
        std::fs::remove_file(alias).unwrap();
        std::fs::remove_file(original).unwrap();
    }
}
