/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! OS-native selection only; filesystem access remains in Rust adapters.
#[cxx::bridge]
mod bridge {
    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
        include!("native_file_picker.h");
        fn chooseNativeFile(title: &QString) -> QString;
    }
}

/// Runs the OS-native chooser without mixing GTK and Qt event dispatchers.
/// # Arguments
/// * `title` - Native picker title, independent of input interpretation.
/// # Returns
/// Selected file URI, or None on cancellation without changing draft input.
/// # Errors
/// Reports unavailable OS-native picker integration.
pub(crate) fn choose(title: &str) -> Result<Option<String>, String> {
    let output =
        std::process::Command::new(std::env::current_exe().map_err(|error| error.to_string())?)
            .args(["--native-file-picker", title])
            .output()
            .map_err(|error| format!("Cannot open OS-native file picker: {error}"))?;
    if !output.status.success() {
        return Err("OS-native file picker did not complete successfully.".into());
    }
    let selected = String::from_utf8(output.stdout)
        .map_err(|_| "OS-native file picker returned an invalid URI.".to_string())?;
    if let Some(error) = selected.strip_prefix("ERROR: ") {
        return Err(error.into());
    }
    if selected.is_empty() {
        return Ok(None);
    }
    Ok(Some(selected))
}

/// Handles the internal native-picker process before any Qt application exists.
/// # Arguments
/// * `title` - Bounded picker title supplied by the parent application.
/// # Returns
/// Writes only a URI, empty cancellation or error marker to the internal pipe.
/// GTK owns transient selection; no input bytes or scientific data are processed.
pub(crate) fn run_helper(title: &str) {
    use std::io::Write;
    let selected = if title.len() <= 512 {
        bridge::chooseNativeFile(&cxx_qt_lib::QString::from(title)).to_string()
    } else {
        "ERROR: Native picker title is too long.".into()
    };
    let _ = std::io::stdout().write_all(selected.as_bytes());
}
