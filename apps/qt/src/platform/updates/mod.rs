/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Application-global preferences and a bounded release-check worker.
mod release;
use serde_json::{json, Value};
use std::io::Read;
use std::{
    path::PathBuf,
    sync::mpsc::{self, Receiver, TryRecvError},
};

/// One transient worker/status and the application-global startup preference.
pub(crate) struct Model {
    pending: Option<Receiver<Result<Value, String>>>,
    state: Value,
    preferences: Option<PathBuf>,
}
impl Default for Model {
    fn default() -> Self {
        let preferences = std::env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .filter(|path| path.is_absolute())
            .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".config")))
            .map(|base| base.join("merlin/preferences.json"));
        let enabled = preferences
            .as_ref()
            .and_then(|path| {
                let metadata = std::fs::metadata(path).ok()?;
                if !metadata.is_file() || metadata.len() > 4096 {
                    return None;
                }
                let mut bytes = Vec::new();
                std::fs::File::open(path)
                    .ok()?
                    .take(4097)
                    .read_to_end(&mut bytes)
                    .ok()?;
                if bytes.len() > 4096 {
                    return None;
                }
                let value: Value = serde_json::from_slice(&bytes).ok()?;
                value.get("check_updates_at_startup")?.as_bool()
            })
            .unwrap_or(true);
        Self {
            pending: None,
            preferences,
            state: json!({"enabled":enabled,"busy":false,"available":false,"status":"Check for a newer MERLIN release.","url":""}),
        }
    }
}
impl Model {
    /// Starts at most one background lookup without accessing Qt or captures.
    /// # Returns
    /// Current display state; another check is ignored while a worker is active.
    pub(crate) fn start(&mut self) -> Value {
        if self.pending.is_none() {
            let (sender, receiver) = mpsc::sync_channel(1);
            self.pending = Some(receiver);
            self.state["busy"] = json!(true);
            self.state["status"] = json!("Checking for updates…");
            std::thread::spawn(move || {
                let _ = sender.send(release::check(env!("CARGO_PKG_VERSION")));
            });
        }
        self.state.clone()
    }
    /// Polls completion without blocking or waiting for a network deadline.
    /// # Returns
    /// Bounded display state with the persisted preference included.
    pub(crate) fn poll(&mut self) -> Value {
        if let Some(receiver) = &self.pending {
            match receiver.try_recv() {
                Ok(result) => {
                    let enabled = self.state["enabled"].clone();
                    self.state = result.unwrap_or_else(
                        |message| json!({"busy":false,"available":false,"status":message,"url":""}),
                    );
                    self.state["enabled"] = enabled;
                    self.pending = None;
                }
                Err(TryRecvError::Disconnected) => {
                    self.state["busy"] = json!(false);
                    self.state["status"] = json!("Update check ended. Try again later.");
                    self.pending = None;
                }
                Err(TryRecvError::Empty) => {}
            }
        }
        self.state.clone()
    }
    /// Atomically saves the global startup preference outside workbench files.
    /// # Arguments
    /// * `enabled` - Whether a future launch performs one quiet update lookup.
    /// # Returns
    /// Current state, or a diagnostic if the preference could not be saved.
    pub(crate) fn set_enabled(&mut self, enabled: bool) -> Value {
        let save = || -> Result<(), String> {
            let path = self
                .preferences
                .as_ref()
                .ok_or("No application configuration directory.")?;
            std::fs::create_dir_all(path.parent().unwrap())
                .map_err(|_| "Unable to save update preference.")?;
            let temporary = path.with_extension(format!("{}.tmp", std::process::id()));
            let result = (|| {
                std::fs::write(
                    &temporary,
                    json!({"check_updates_at_startup":enabled}).to_string(),
                )?;
                std::fs::rename(&temporary, path)
            })();
            if result.is_err() {
                let _ = std::fs::remove_file(temporary);
            }
            result.map_err(|_| "Unable to save update preference.".to_string())
        };
        match save() {
            Ok(()) => self.state["enabled"] = json!(enabled),
            Err(message) => self.state["status"] = json!(message),
        }
        self.state.clone()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn completion_preserves_preference_and_failure_is_nonblocking() {
        let (sender, receiver) = mpsc::sync_channel(1);
        let mut model = Model {
            pending: Some(receiver),
            preferences: None,
            state: json!({"enabled":false,"busy":true}),
        };
        assert_eq!(model.poll()["busy"], true);
        sender.send(Err("Offline".into())).unwrap();
        let result = model.poll();
        assert_eq!(result["busy"], false);
        assert_eq!(result["enabled"], false);
        assert_eq!(result["available"], false);
        assert!(model.pending.is_none());
        assert_eq!(model.set_enabled(true)["enabled"], false);
    }
}
