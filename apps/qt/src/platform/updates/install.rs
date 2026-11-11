/*
 * Copyright 2026 PNM Viewer contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Ubuntu update staging adapted from the viewer's GPL update architecture.
use super::release;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::{Duration, Instant},
};

const LAUNCHER: &str = "/usr/bin/merlin";
const EXECUTABLE: &str = "/usr/lib/merlin/merlin-qt";
const MAX_PACKAGE: u64 = 300_000_000;
// Linux open(2) flags: reject symlinks and avoid blocking on special inputs.
const OPEN_NOFOLLOW: i32 = 0x20000;
const OPEN_DIRECTORY: i32 = 0x10000;
const OPEN_NONBLOCK: i32 = 0x800;

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct Plan {
    version: String,
    package_url: String,
    size: u64,
    sha256: String,
}
impl Plan {
    fn validate(&self) -> Result<(), String> {
        release::version(&self.version)?;
        if self.version.starts_with('v')
            || self.package_url
                != format!(
                    "{}/releases/download/v{}/merlin_{}_ubuntu22.04_amd64.deb",
                    release::PROJECT,
                    self.version,
                    self.version
                )
            || !(1..=MAX_PACKAGE).contains(&self.size)
            || self.sha256.len() != 64
            || !self.sha256.bytes().all(|byte| byte.is_ascii_hexdigit())
        {
            return Err("Invalid update package identity.".into());
        }
        Ok(())
    }
}

struct Directory(PathBuf);
impl Directory {
    fn new() -> Result<Self, String> {
        use std::os::unix::fs::DirBuilderExt;
        for _ in 0..8 {
            let mut random = [0_u8; 16];
            File::open("/dev/urandom")
                .and_then(|mut input| input.read_exact(&mut random))
                .map_err(|error| error.to_string())?;
            let token = format!("{:032x}", u128::from_be_bytes(random));
            let path = PathBuf::from("/var/tmp").join(format!("merlin-update-{token}"));
            match fs::DirBuilder::new().mode(0o700).create(&path) {
                Ok(()) => return Ok(Self(path)),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(error.to_string()),
            }
        }
        Err("Unable to create private update storage.".into())
    }
}
impl Drop for Directory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

/// Checks installed Ubuntu identity without enabling elevation from a checkout.
/// # Returns
/// True only for the installed amd64 executable on Ubuntu 22.04.
pub(crate) fn supported() -> bool {
    std::env::consts::ARCH == "x86_64"
        && std::env::current_exe().ok().as_deref() == Some(Path::new(EXECUTABLE))
        && fs::read_to_string("/etc/os-release").is_ok_and(|text| {
            text.lines()
                .any(|line| line == "ID=ubuntu" || line == "ID=\"ubuntu\"")
                && text
                    .lines()
                    .any(|line| line == "VERSION_ID=\"22.04\"" || line == "VERSION_ID=22.04")
        })
}

fn trusted(address: &url::Url) -> bool {
    address.scheme() == "https"
        && address.username().is_empty()
        && address.password().is_none()
        && address.port().is_none()
        && matches!(
            address.host_str(),
            Some(
                "github.com"
                    | "release-assets.githubusercontent.com"
                    | "objects.githubusercontent.com"
            )
        )
}

fn copy_verified(
    mut input: impl Read,
    mut output: impl Write,
    plan: &Plan,
    progress: impl Fn(u64),
) -> Result<(), String> {
    let deadline = Instant::now() + Duration::from_secs(900);
    let mut digest = Sha256::new();
    let mut received = 0_u64;
    let mut buffer = [0_u8; 65_536];
    loop {
        if Instant::now() >= deadline {
            return Err("Update download timed out.".into());
        }
        let count = input.read(&mut buffer).map_err(|error| error.to_string())?;
        if count == 0 {
            break;
        }
        received += count as u64;
        if received > plan.size {
            return Err("Update exceeds its declared size.".into());
        }
        digest.update(&buffer[..count]);
        output
            .write_all(&buffer[..count])
            .map_err(|error| error.to_string())?;
        progress(received);
    }
    if received != plan.size
        || format!("{:x}", digest.finalize()) != plan.sha256.to_ascii_lowercase()
    {
        return Err("Update size or SHA-256 verification failed.".into());
    }
    Ok(())
}

/// Downloads a fresh verified stable DEB and requests normal polkit installation.
/// # Arguments
/// * `progress` - Worker-thread status callback, never accesses Qt.
/// # Returns
/// Success only after APT completed; caller may restart normally.
/// # Errors
/// Rejects source runs, network/hash/identity errors or canceled authentication.
pub(crate) fn update(progress: impl Fn(String)) -> Result<(), String> {
    if !supported() {
        return Err(
            "Automatic installation requires the installed Ubuntu 22.04 application.".into(),
        );
    }
    let identity = Command::new("/usr/bin/id")
        .arg("-u")
        .output()
        .map_err(|error| error.to_string())?;
    if !identity.status.success() || identity.stdout == b"0\n" {
        return Err("Start the update as your normal desktop user.".into());
    }
    let metadata = release::check(env!("CARGO_PKG_VERSION"))?;
    if metadata["available"] != true {
        return Err("No newer verified desktop update is available.".into());
    }
    let plan = Plan {
        version: metadata["version"]
            .as_str()
            .ok_or("Missing update version.")?
            .into(),
        package_url: metadata["package_url"]
            .as_str()
            .ok_or("Missing update URL.")?
            .into(),
        size: metadata["size"].as_u64().ok_or("Missing update size.")?,
        sha256: metadata["sha256"]
            .as_str()
            .ok_or("Missing update digest.")?
            .into(),
    };
    plan.validate()?;
    let directory = Directory::new()?;
    let agent = ureq::AgentBuilder::new()
        .redirects(0)
        .timeout_connect(Duration::from_secs(15))
        .timeout_read(Duration::from_secs(20))
        .build();
    let mut address = url::Url::parse(&plan.package_url).map_err(|error| error.to_string())?;
    let mut response = None;
    for _ in 0..=5 {
        if !trusted(&address) {
            return Err("Untrusted update download redirect.".into());
        }
        let received = agent
            .get(address.as_str())
            .set("User-Agent", "MERLIN")
            .call()
            .map_err(|_| "Unable to download the update.")?;
        match received.status() {
            200 => {
                response = Some(received);
                break;
            }
            301 | 302 | 303 | 307 | 308 => {
                address = address
                    .join(
                        received
                            .header("Location")
                            .ok_or("Missing update redirect.")?,
                    )
                    .map_err(|error| error.to_string())?;
            }
            _ => return Err("Unexpected update download response.".into()),
        }
    }
    let response = response.ok_or("Too many update redirects.")?;
    let mut output =
        File::create(directory.0.join("update.deb")).map_err(|error| error.to_string())?;
    copy_verified(response.into_reader(), &mut output, &plan, |received| {
        progress(format!(
            "Downloading update… {}%",
            received * 100 / plan.size
        ));
    })?;
    output.sync_all().map_err(|error| error.to_string())?;
    fs::write(
        directory.0.join("plan.json"),
        serde_json::to_vec(&plan).map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())?;
    progress("Approve the Ubuntu authentication prompt to install…".into());
    let status = Command::new("/usr/bin/pkexec")
        .arg(LAUNCHER)
        .arg("--install-update")
        .arg(&directory.0)
        .status()
        .map_err(|error| error.to_string())?;
    if !status.success() {
        return Err("Authentication canceled or installation failed. MERLIN remains open; check APT before retrying if installation began.".into());
    }
    Ok(())
}

/// Runs the authenticated installer before initializing Qt; never downloads.
/// # Arguments
/// * `directory` - Private downloaded update directory owned by PKEXEC_UID.
/// # Errors
/// Rejects unsafe input, package/hash/version mismatches or failed APT.
pub(crate) fn elevated(directory: &Path) -> Result<(), String> {
    use std::os::unix::{
        fs::{MetadataExt, OpenOptionsExt},
        io::AsRawFd,
    };
    if !supported() || std::env::var("PKEXEC_UID").is_err() {
        return Err("Use Update from the installed desktop application.".into());
    }
    let identity = Command::new("/usr/bin/id")
        .arg("-u")
        .output()
        .map_err(|error| error.to_string())?;
    if !identity.status.success() || identity.stdout != b"0\n" {
        return Err("Administrator authentication is required.".into());
    }
    let owner: u32 = std::env::var("PKEXEC_UID")
        .unwrap()
        .parse()
        .map_err(|_| "Invalid authenticated user.")?;
    // Open the directory itself without following links; subsequent reads use its
    // stable descriptor so renaming a caller-controlled path cannot redirect them.
    let directory_file = OpenOptions::new()
        .read(true)
        .custom_flags(OPEN_NOFOLLOW | OPEN_DIRECTORY)
        .open(directory)
        .map_err(|error| error.to_string())?; // Linux O_NOFOLLOW | O_DIRECTORY.
    let metadata = directory_file
        .metadata()
        .map_err(|error| error.to_string())?;
    if metadata.uid() != owner || metadata.mode() & 0o077 != 0 {
        return Err("Update directory must be private and owned by the requesting user.".into());
    }
    let descriptor_path = PathBuf::from(format!("/proc/self/fd/{}", directory_file.as_raw_fd()));
    let open = |name: &str| -> Result<File, String> {
        let file = OpenOptions::new()
            .read(true)
            .custom_flags(OPEN_NOFOLLOW | OPEN_NONBLOCK)
            .open(descriptor_path.join(name))
            .map_err(|error| error.to_string())?; // Linux O_NOFOLLOW | O_NONBLOCK.
        let metadata = file.metadata().map_err(|error| error.to_string())?;
        if !metadata.is_file() || metadata.uid() != owner {
            return Err("Unsafe update input file.".into());
        }
        Ok(file)
    };
    let mut bytes = Vec::new();
    open("plan.json")?
        .take(16385)
        .read_to_end(&mut bytes)
        .map_err(|error| error.to_string())?;
    if bytes.len() > 16384 {
        return Err("Update plan exceeds limits.".into());
    }
    let plan: Plan = serde_json::from_slice(&bytes).map_err(|_| "Invalid update plan.")?;
    plan.validate()?;
    let staging = Directory::new()?;
    let package = staging.0.join("update.deb");
    let mut output = File::create(&package).map_err(|error| error.to_string())?;
    copy_verified(open("update.deb")?, &mut output, &plan, |_| {})?;
    output.sync_all().map_err(|error| error.to_string())?;
    validate_package(&package, &plan.version)?;
    let installed = Command::new("/usr/bin/dpkg-query")
        .args(["-W", "-f=${Status}\n${Version}", "merlin"])
        .output()
        .map_err(|error| error.to_string())?;
    let installed_text = String::from_utf8_lossy(&installed.stdout);
    let Some(installed_version) = installed_text.strip_prefix("install ok installed\n") else {
        return Err("Repair the existing MERLIN package with APT before updating.".into());
    };
    if !installed.status.success()
        || !Command::new("/usr/bin/dpkg")
            .args([
                "--compare-versions",
                &plan.version,
                "gt",
                installed_version.trim(),
            ])
            .status()
            .map_err(|error| error.to_string())?
            .success()
    {
        return Err("Update must be newer than the installed package.".into());
    }
    // Only the already verified public package is made readable to APT's sandbox.
    use std::os::unix::fs::PermissionsExt;
    fs::set_permissions(&package, fs::Permissions::from_mode(0o644))
        .map_err(|error| error.to_string())?;
    fs::set_permissions(&staging.0, fs::Permissions::from_mode(0o755))
        .map_err(|error| error.to_string())?;
    let status = Command::new("/usr/bin/apt-get")
        .env_clear()
        .env("PATH", "/usr/sbin:/usr/bin:/sbin:/bin")
        .env("HOME", "/root")
        .env("DEBIAN_FRONTEND", "noninteractive")
        .env("LANG", "C.UTF-8")
        .args([
            "-y",
            "--no-remove",
            "-o",
            "DPkg::Lock::Timeout=60",
            "install",
        ])
        .arg(&package)
        .status()
        .map_err(|error| error.to_string())?;
    if !status.success() {
        return Err("APT installation failed; repair installed packages before retrying.".into());
    }
    Ok(())
}

fn validate_package(package: &Path, version: &str) -> Result<(), String> {
    for (field, expected) in [
        ("Package", "merlin"),
        ("Version", version),
        ("Architecture", "amd64"),
    ] {
        let actual = Command::new("/usr/bin/dpkg-deb")
            .arg("-f")
            .arg(package)
            .arg(field)
            .output()
            .map_err(|error| error.to_string())?;
        if !actual.status.success() || String::from_utf8_lossy(&actual.stdout).trim() != expected {
            return Err(format!("Update has an unexpected {field}."));
        }
    }
    Ok(())
}

/// Starts the installed app as the current desktop user after successful install.
/// # Errors
/// Reports launcher failure; the original window must remain open in that case.
pub(crate) fn restart() -> Result<(), String> {
    Command::new(LAUNCHER)
        .arg("--restore-update")
        .env_remove("LD_LIBRARY_PATH")
        .env_remove("QT_PLUGIN_PATH")
        .env_remove("QML_IMPORT_PATH")
        .env_remove("QML2_IMPORT_PATH")
        .stdin(Stdio::null())
        .spawn()
        .map_err(|error| error.to_string())?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    fn plan() -> Plan {
        Plan {
            version: "0.2.0".into(),
            package_url: format!(
                "{}/releases/download/v0.2.0/merlin_0.2.0_ubuntu22.04_amd64.deb",
                release::PROJECT
            ),
            size: 3,
            sha256: format!("{:x}", Sha256::digest(b"abc")),
        }
    }
    #[test]
    fn bounded_hash_copy_rejects_corruption_truncation_and_overflow() {
        let plan = plan();
        let mut bytes = Vec::new();
        copy_verified(&b"abc"[..], &mut bytes, &plan, |_| {}).unwrap();
        assert_eq!(bytes, b"abc");
        for data in [&b"ab"[..], &b"abcd"[..], &b"abd"[..]] {
            assert!(copy_verified(data, Vec::new(), &plan, |_| {}).is_err());
        }
    }
    #[test]
    fn identity_and_redirects_are_restricted() {
        plan().validate().unwrap();
        for address in [
            "http://github.com/file",
            "https://evil.invalid/file",
            "https://user@github.com/file",
            "https://github.com.evil.invalid/file",
        ] {
            assert!(!trusted(&url::Url::parse(address).unwrap()));
        }
        assert!(trusted(
            &url::Url::parse("https://release-assets.githubusercontent.com/file?sig=1").unwrap()
        ));
        let mut bad = plan();
        bad.size = MAX_PACKAGE + 1;
        assert!(bad.validate().is_err());
        let mut bad = plan();
        bad.sha256 = "a".into();
        assert!(bad.validate().is_err());
        let mut bad = plan();
        bad.version = "../bad".into();
        assert!(bad.validate().is_err());
        let mut bad = plan();
        bad.package_url = "https://evil.invalid".into();
        assert!(bad.validate().is_err());
    }
    #[test]
    fn private_staging_cleans_up_and_checkout_cannot_install() {
        use std::os::unix::fs::PermissionsExt;
        let directory = Directory::new().unwrap();
        let path = directory.0.clone();
        assert_eq!(
            fs::metadata(&path).unwrap().permissions().mode() & 0o777,
            0o700
        );
        drop(directory);
        assert!(!path.exists());
        assert!(!supported());
        assert!(elevated(Path::new("/nonexistent")).is_err());
    }
    #[test]
    fn actual_deb_identity_must_match_selected_release() {
        let directory = Directory::new().unwrap();
        let source = directory.0.join("package");
        fs::create_dir_all(source.join("DEBIAN")).unwrap();
        let archive = directory.0.join("package.deb");
        for (name, version, architecture, valid) in [
            ("merlin", "0.2.0", "amd64", true),
            ("other", "0.2.0", "amd64", false),
            ("merlin", "0.1.0", "amd64", false),
            ("merlin", "0.2.0", "arm64", false),
        ] {
            fs::write(source.join("DEBIAN/control"), format!("Package: {name}\nVersion: {version}\nArchitecture: {architecture}\nMaintainer: MERLIN tests\nDescription: Test identity\n")).unwrap();
            let output = Command::new("/usr/bin/dpkg-deb")
                .args(["--build", "--root-owner-group"])
                .arg(&source)
                .arg(&archive)
                .output()
                .unwrap();
            assert!(
                output.status.success(),
                "{}",
                String::from_utf8_lossy(&output.stderr)
            );
            assert_eq!(validate_package(&archive, "0.2.0").is_ok(), valid);
        }
    }
}
