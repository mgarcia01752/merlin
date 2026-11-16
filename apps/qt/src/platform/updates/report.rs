/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Bounded, user-reviewed update diagnostics; never uploads or includes captures.
use serde_json::{json, Value};
use std::{
    io::{Read, Seek, SeekFrom},
    path::Path,
};
const LOG_LIMIT: u64 = 32 * 1024;

fn log_tail(path: &Path) -> std::io::Result<String> {
    let mut file = std::fs::File::open(path)?;
    let size = file.metadata()?.len();
    let mut bom = [0; 2];
    let read = file.read(&mut bom)?;
    let utf16 = read == 2 && bom == [0xff, 0xfe];
    let offset = size.saturating_sub(LOG_LIMIT);
    let offset = if utf16 { offset + offset % 2 } else { offset };
    file.seek(SeekFrom::Start(offset))?;
    let mut bytes = Vec::new();
    file.take(LOG_LIMIT).read_to_end(&mut bytes)?;
    let text = if utf16 {
        let words: Vec<_> = bytes
            .chunks_exact(2)
            .map(|b| u16::from_le_bytes([b[0], b[1]]))
            .collect();
        String::from_utf16_lossy(&words)
    } else {
        String::from_utf8_lossy(&bytes).into_owned()
    };
    Ok(format!(
        "{}{}",
        if offset > 0 {
            "[Log truncated to final 32 KiB]\n"
        } else {
            ""
        },
        text.trim_start_matches('\u{feff}')
    ))
}

fn redact(mut text: String, profile: &str) -> String {
    if !profile.is_empty() {
        for value in [profile.to_string(), profile.replace('\\', "/")] {
            // Windows paths are case-insensitive. Preserve offsets using ASCII folding.
            let needle = value.to_ascii_lowercase();
            let mut offset = 0;
            while let Some(relative) = text[offset..].to_ascii_lowercase().find(&needle) {
                let index = offset + relative;
                text.replace_range(index..index + value.len(), "%USERPROFILE%");
                offset = index + "%USERPROFILE%".len();
            }
        }
    }
    text
}

/// Prepares clipboard text and a fixed issue form; performs no external writes.
/// # Arguments
/// * `status` - Current update failure, never capture contents.
/// # Returns
/// Bounded JSON report with a log excerpt when available and explicit review steps.
pub(super) fn prepare(status: &str) -> Value {
    let log = if cfg!(target_os = "windows") {
        std::env::var_os("LOCALAPPDATA")
            .map(|root| {
                std::path::PathBuf::from(root)
                    .join("merlin")
                    .join("updates")
                    .join("last-install.log")
            })
            .and_then(|path| log_tail(&path).ok())
    } else {
        None
    };
    let text = format!("## MERLIN update failure\n\nVersion: {}\nPlatform: {} / {}\nStatus: {}\n\n## Installer log (may be from the last install attempt)\n\n{}\n\nReview for private information before submitting. Describe what happened before the failure.\n", env!("CARGO_PKG_VERSION"), std::env::consts::OS, std::env::consts::ARCH, status.chars().take(2048).collect::<String>(), log.unwrap_or_else(|| "Installer log unavailable; include the displayed error and reproduction steps.".into()));
    let text = redact(text, &std::env::var("USERPROFILE").unwrap_or_default());
    json!({"text":text,"url":"https://github.com/mgarcia01752/merlin/issues/new?title=MERLIN%20update%20failure&body=Paste%20the%20copied%20diagnostic%20report%20here%2C%20review%20it%20for%20private%20information%2C%20and%20describe%20what%20happened."})
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn profile_redaction_handles_case_and_separator_variants() {
        assert_eq!(
            redact(
                "C:\\Users\\Alice\\file C:/Users/Alice/file".into(),
                "c:\\users\\alice"
            ),
            "%USERPROFILE%\\file %USERPROFILE%/file"
        );
        assert_eq!(redact("unchanged".into(), ""), "unchanged");
    }
    #[test]
    fn bounded_log_decodes_windows_utf16_and_preserves_failure() {
        let path = std::env::temp_dir().join(format!("merlin-report-{}.log", std::process::id()));
        let text = format!("{}\nNote: 1: 1324 2: merlin/updates", "x".repeat(40000));
        let mut bytes = vec![0xff, 0xfe];
        bytes.extend(text.encode_utf16().flat_map(u16::to_le_bytes));
        std::fs::write(&path, bytes).unwrap();
        let result = log_tail(&path).unwrap();
        std::fs::write(&path, b"UTF-8 error 1619").unwrap();
        assert_eq!(log_tail(&path).unwrap(), "UTF-8 error 1619");
        std::fs::remove_file(&path).unwrap();
        assert!(log_tail(&path).is_err());
        assert!(result.starts_with("[Log truncated"));
        assert!(result.ends_with("1324 2: merlin/updates"));
        assert!(result.len() < 33000);
    }
}
