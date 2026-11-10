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
        fn chooseNativeFiles(title: &QString) -> QString;
        fn chooseNativeSave(title: &QString) -> QString;
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
    select(title, false)
}

/// Selects multiple original inputs through the same isolated OS-native picker.
/// # Arguments
/// * `title` - Native picker title.
/// # Returns
/// At most 64 local URIs; empty means Cancel.
/// # Errors
/// Reports missing native integration or malformed/oversize selection.
pub(crate) fn choose_many(title: &str) -> Result<Vec<String>, String> {
    let selected = select(title, true)?.unwrap_or_default();
    if selected.len() > 262144 {
        return Err("Native selection is too large.".into());
    }
    Ok(selected.lines().map(str::to_owned).collect())
}

/// Opens the OS-native project destination chooser with overwrite confirmation.
/// # Arguments
/// * `title` - User-facing native window title.
/// # Returns
/// Local destination URI or cancellation.
/// # Errors
/// Reports unavailable native integration without a Qt browser fallback.
pub(crate) fn save(title: &str) -> Result<Option<String>, String> {
    select_mode(title, "--native-save-picker")
}

fn select(title: &str, multiple: bool) -> Result<Option<String>, String> {
    select_mode(
        title,
        if multiple {
            "--native-file-picker-multiple"
        } else {
            "--native-file-picker"
        },
    )
}
fn select_mode(title: &str, mode: &str) -> Result<Option<String>, String> {
    let output =
        std::process::Command::new(std::env::current_exe().map_err(|error| error.to_string())?)
            .args([mode, title])
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
/// * `multiple` - Whether the native dialog permits multiple selected files.
/// * `save` - Select a destination and confirm overwriting through GTK.
/// # Returns
/// Writes only a URI, empty cancellation or error marker to the internal pipe.
/// GTK owns transient selection; no input bytes or scientific data are processed.
pub(crate) fn run_helper(title: &str, multiple: bool, save: bool) {
    use std::io::Write;
    let selected = if title.len() <= 512 {
        if save {
            bridge::chooseNativeSave(&cxx_qt_lib::QString::from(title)).to_string()
        } else if multiple {
            bridge::chooseNativeFiles(&cxx_qt_lib::QString::from(title)).to_string()
        } else {
            bridge::chooseNativeFile(&cxx_qt_lib::QString::from(title)).to_string()
        }
    } else {
        "ERROR: Native picker title is too long.".into()
    };
    let _ = std::io::stdout().write_all(selected.as_bytes());
}
