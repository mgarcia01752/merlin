/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Verified Windows MSI staging and normal-user installation handoff.
use super::{download, release};
use serde_json::{json, Value};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    sync::Mutex,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

const HELPER: &str = include_str!("windows_handoff.ps1");
static HANDOFF: Mutex<Option<Handoff>> = Mutex::new(None);

struct Handoff {
    child: Child,
    directory: PathBuf,
}
impl Drop for Handoff {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        let _ = fs::remove_dir_all(&self.directory);
    }
}

fn storage() -> Result<PathBuf, String> {
    let base = std::env::var_os("LOCALAPPDATA")
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
        .ok_or("No private Windows application data directory.")?;
    Ok(base.join("merlin").join("updates"))
}

fn bounded_text(path: &Path) -> Option<String> {
    let file = File::open(path).ok()?;
    if !file.metadata().ok()?.is_file() {
        return None;
    }
    let mut bytes = Vec::new();
    file.take(4097).read_to_end(&mut bytes).ok()?;
    if bytes.len() > 4096 {
        return None;
    }
    String::from_utf8(bytes).ok()
}

fn validate_metadata(metadata: &Value) -> Result<(&str, u64, &str), String> {
    let version = metadata["version"]
        .as_str()
        .ok_or("Missing update version.")?;
    release::version(version)?;
    let url = metadata["package_url"]
        .as_str()
        .ok_or("Missing update URL.")?;
    let size = metadata["size"].as_u64().ok_or("Missing update size.")?;
    let digest = metadata["sha256"]
        .as_str()
        .ok_or("Missing update digest.")?;
    if version.starts_with('v')
        || metadata["available"] != true
        || url
            != format!(
                "{}/releases/download/v{version}/merlin-desktop_{version}_windows_amd64.msi",
                release::PROJECT
            )
        || !(1..=300_000_000).contains(&size)
        || digest.len() != 64
        || !digest.bytes().all(|byte| byte.is_ascii_hexdigit())
    {
        return Err("Invalid Windows update package identity.".into());
    }
    Ok((url, size, digest))
}

fn helper_command(plan: &Path) -> Result<Command, String> {
    let system = std::env::var_os("SystemRoot")
        .map(PathBuf::from)
        .filter(|path| path.is_absolute())
        .ok_or("No Windows system directory.")?;
    let mut command = Command::new(system.join("System32/WindowsPowerShell/v1.0/powershell.exe"));
    command.args([
        "-NoLogo",
        "-NoProfile",
        "-NonInteractive",
        "-Command",
        HELPER,
    ]);
    command
        .env("MERLIN_UPDATE_PLAN", plan)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    // The ordinary helper stays unelevated; only its msiexec child requests UAC.
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(0x08000000); // CREATE_NO_WINDOW; no policy changes.
    }
    Ok(command)
}

/// Reports Windows availability; registered MSI ownership is checked by the worker.
/// # Returns
/// False outside Windows and for obvious source checkout executable locations.
pub(crate) fn supported() -> bool {
    cfg!(target_os = "windows")
        && std::env::consts::ARCH == "x86_64"
        && std::env::current_exe().is_ok_and(|path| {
            path.file_name().is_some_and(|name| name == "merlin-qt.exe")
                && !path
                    .ancestors()
                    .any(|parent| parent.join("Cargo.toml").is_file())
        })
}

/// Downloads an exact verified stable MSI and prepares a checked external helper.
/// # Arguments
/// * `progress` - Worker-thread status callback without Qt access.
/// # Errors
/// Rejects source runs, unverified bytes, missing MSI registration or failed handoff.
pub(crate) fn update(progress: impl Fn(String)) -> Result<(), String> {
    if !supported() {
        return Err("Automatic update requires the installed Windows MSI application.".into());
    }
    let metadata = release::check(env!("CARGO_PKG_VERSION"))?;
    let (url, size, digest) = validate_metadata(&metadata)?;
    let root = storage()?;
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let token = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|error| error.to_string())?
        .as_nanos();
    let directory = root.join(format!("staged-{}-{token}", std::process::id()));
    fs::create_dir(&directory).map_err(|error| error.to_string())?;
    let prepare = || -> Result<Handoff, String> {
        let package = directory.join("update.msi");
        let mut output = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&package)
            .map_err(|error| error.to_string())?;
        download::fetch(url, &mut output, size, digest, &progress)?;
        output.sync_all().map_err(|error| error.to_string())?;
        drop(output);
        // Retain the Internet-zone marker; installation keeps normal Windows
        // reputation and administrator approval instead of unblocking downloads.
        #[cfg(target_os = "windows")]
        {
            let mut zone_path = package.as_os_str().to_os_string();
            zone_path.push(":Zone.Identifier");
            fs::write(
                PathBuf::from(zone_path),
                format!("[ZoneTransfer]\r\nZoneId=3\r\nHostUrl={url}\r\n"),
            )
            .map_err(|error| error.to_string())?;
        }
        let executable = std::env::current_exe().map_err(|error| error.to_string())?;
        let location = executable
            .parent()
            .ok_or("Missing installation directory.")?;
        let plan = json!({"version":metadata["version"],"size":size,"sha256":digest,
            "package":package,"executable":executable,"directory":location,
            "parent_pid":std::process::id(),"staging":directory,
            "ready":directory.join("ready"),"failed":directory.join("failed"),
            "commit":directory.join("commit"),"result":root.join("last-result.json"),
            "log":root.join("last-install.log"),"desktop_shortcut":false});
        let plan_path = directory.join("plan.json");
        fs::write(&plan_path, plan.to_string()).map_err(|error| error.to_string())?;
        progress("Verifying the installed MSI and preparing Windows update…".into());
        let child = helper_command(&plan_path)?
            .spawn()
            .map_err(|error| error.to_string())?;
        let mut handoff = Handoff {
            child,
            directory: directory.clone(),
        };
        let deadline = Instant::now() + Duration::from_secs(30);
        loop {
            if let Some(message) = bounded_text(&directory.join("failed")) {
                return Err(message);
            }
            if directory.join("ready").is_file() {
                return Ok(handoff);
            }
            if handoff
                .child
                .try_wait()
                .map_err(|error| error.to_string())?
                .is_some()
            {
                return Err("Windows update helper could not start. MERLIN remains open.".into());
            }
            if Instant::now() >= deadline {
                return Err("Windows update preparation timed out. MERLIN remains open.".into());
            }
            std::thread::sleep(Duration::from_millis(100));
        }
    };
    match prepare() {
        Ok(handoff) => {
            *HANDOFF
                .lock()
                .map_err(|_| "Update handoff is unavailable.")? = Some(handoff);
            progress(
                "Closing MERLIN for installation. Approve the Windows administrator prompt…".into(),
            );
            Ok(())
        }
        Err(message) => {
            let _ = fs::remove_dir_all(directory);
            Err(message)
        }
    }
}

/// Commits the checked helper and permits the UI to exit without holding MSI files.
/// # Errors
/// Missing/failed handoff leaves the original application open.
pub(crate) fn restart() -> Result<(), String> {
    let mut pending = HANDOFF
        .lock()
        .map_err(|_| "Update handoff is unavailable.")?;
    let handoff = pending
        .as_mut()
        .ok_or("No verified Windows update is prepared.")?;
    if handoff
        .child
        .try_wait()
        .map_err(|error| error.to_string())?
        .is_some()
    {
        return Err("Windows update helper stopped. Try Update again.".into());
    }
    let mut commit = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(handoff.directory.join("commit"))
        .map_err(|error| error.to_string())?;
    commit
        .write_all(b"commit")
        .and_then(|()| commit.sync_all())
        .map_err(|error| error.to_string())?;
    // The external helper now owns cleanup and ordinary-user relaunch. Dropping
    // Child never terminates it; bypass Handoff's abort-only cleanup intentionally.
    if let Some(handoff) = pending.take() {
        std::mem::forget(handoff);
    }
    Ok(())
}

/// Rejects Linux's authenticated installer entry point on Windows.
/// # Arguments
/// * `_directory` - Unused directory; Windows uses only its verified MSI worker.
/// # Errors
/// Always rejects; no command from this internal entry point is executed.
pub(crate) fn elevated(_directory: &Path) -> Result<(), String> {
    Err("Use Update from the installed Windows application.".into())
}

/// Consumes one bounded, data-only installer result after normal-user restart.
/// # Returns
/// A plain diagnostic; absent, oversized or malformed results are ignored.
pub(crate) fn take_result() -> Option<String> {
    let path = storage().ok()?.join("last-result.json");
    let message = parse_result(&bounded_text(&path)?)?;
    fs::remove_file(path).ok()?;
    Some(message)
}

fn parse_result(text: &str) -> Option<String> {
    if text.len() > 4096 {
        return None;
    }
    let value: Value = serde_json::from_str(text.trim_start_matches('\u{feff}')).ok()?;
    let message = value["message"].as_str()?;
    if message.is_empty() || message.len() > 2048 {
        return None;
    }
    Some(message.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verified_msi_metadata_and_bounded_restart_result() {
        let mut metadata = json!({"available":true,"version":"0.3.0","package_url":format!("{}/releases/download/v0.3.0/merlin-desktop_0.3.0_windows_amd64.msi",release::PROJECT),"size":3,"sha256":"a".repeat(64)});
        assert!(validate_metadata(&metadata).is_ok());
        for key in ["version", "package_url", "sha256"] {
            let original = metadata[key].clone();
            metadata[key] = json!("../foreign");
            assert!(validate_metadata(&metadata).is_err());
            metadata[key] = original;
        }
        metadata["size"] = json!(300_000_001_u64);
        assert!(validate_metadata(&metadata).is_err());
        assert_eq!(
            parse_result("\u{feff}{\"message\":\"Update canceled\"}"),
            Some("Update canceled".into())
        );
        for text in ["bad", "{}", "{\"message\":\"\"}"] {
            assert!(parse_result(text).is_none());
        }
        assert!(parse_result(&"a".repeat(4097)).is_none());
    }
    #[test]
    fn checkout_never_installs_and_internal_entry_remains_closed() {
        assert!(!supported());
        assert!(update(|_| panic!("source must not download")).is_err());
        assert!(elevated(Path::new("unused")).is_err());
        assert!(restart().is_err());
        let _consume: fn() -> Option<String> = take_result;
    }
}
