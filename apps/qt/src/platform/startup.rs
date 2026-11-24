/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Desktop startup requests; large workbenches are read by the background loader.
use crate::{
    shared::project_path::{self, ProjectPath},
    shell::RetentionBudget,
};
use std::{
    path::PathBuf,
    sync::{Mutex, OnceLock},
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum WorkbenchSource {
    File(PathBuf),
    Stdin,
}
#[derive(Default, Debug)]
pub(crate) struct Options {
    pub workbench: Option<WorkbenchSource>,
    pub debug_ui: bool,
}

/// Runtime source whose file path is charged before storage and worker capture.
#[derive(Debug)]
pub(crate) enum AdmittedSource {
    File(ProjectPath),
    Stdin,
}

#[derive(Debug)]
struct RuntimeOptions {
    workbench: Option<AdmittedSource>,
    debug_ui: bool,
}

static OPTIONS: OnceLock<Mutex<RuntimeOptions>> = OnceLock::new();

impl AdmittedSource {
    /// Converts a parsed temporary request into retained application ownership.
    ///
    /// # Arguments
    /// * `source` - Parsed file path or explicit stdin request; no file is read.
    /// * `budget` - The actual main-thread host later used by the loader.
    ///
    /// # Returns
    /// The original admitted path, or allocation-free stdin selection.
    ///
    /// # Errors
    /// Path admission rejection before runtime storage or loader dispatch.
    pub(crate) fn admit(source: WorkbenchSource, budget: &RetentionBudget) -> Result<Self, String> {
        match source {
            WorkbenchSource::File(path) => project_path::admit(path, budget).map(Self::File),
            WorkbenchSource::Stdin => Ok(Self::Stdin),
        }
    }
}

fn prepare(options: Options, budget: &RetentionBudget) -> Result<RuntimeOptions, String> {
    Ok(RuntimeOptions {
        workbench: options
            .workbench
            .map(|source| AdmittedSource::admit(source, budget))
            .transpose()?,
        debug_ui: options.debug_ui,
    })
}

/// Parses desktop startup options while leaving existing Qt switches intact.
/// # Arguments
/// * `arguments` - Arguments after the executable name.
/// # Returns
/// A bounded startup request; capture contents are not read here.
/// # Errors
/// Rejects missing/repeated workbenches and conflicting update recovery.
pub(crate) fn parse(arguments: &[String]) -> Result<Options, String> {
    let mut options = Options::default();
    let mut arguments_iter = arguments.iter();
    while let Some(argument) = arguments_iter.next() {
        match argument.as_str() {
            "--debug-ui" => options.debug_ui = true,
            "--workbench" => {
                if options.workbench.is_some() {
                    return Err("Provide only one --workbench input.".into());
                }
                let path = arguments_iter
                    .next()
                    .filter(|path| {
                        !path.is_empty() && (!path.starts_with('-') || path.as_str() == "-")
                    })
                    .ok_or("Use --workbench FILE or --workbench - < FILE.")?;
                options.workbench = Some(if path == "-" {
                    WorkbenchSource::Stdin
                } else {
                    WorkbenchSource::File(PathBuf::from(path))
                });
            }
            _ => {}
        }
    }
    if options.workbench.is_some()
        && arguments
            .iter()
            .any(|argument| argument == "--restore-update")
    {
        return Err("Do not combine --workbench with --restore-update.".into());
    }
    Ok(options)
}

/// Stores parsed startup options once before constructing the Qt application.
/// # Arguments
/// * `options` - Validated startup request and diagnostic display preference.
/// * `budget` - The same canonical host used by the future GUI adapters.
/// # Returns
/// No value; repeated configuration leaves the initial request unchanged.
/// # Errors
/// Rejects an inadmissible file path before it is retained by startup.
pub(crate) fn configure(options: Options, budget: &RetentionBudget) -> Result<(), String> {
    if OPTIONS.get().is_some() {
        return Ok(());
    }
    let runtime = prepare(options, budget)?;
    let _ = OPTIONS.set(Mutex::new(runtime));
    Ok(())
}

/// Takes the command-line workbench once when the GUI is ready.
/// # Returns
/// An owned input source, or None for ordinary desktop startup.
pub(crate) fn take_workbench() -> Option<AdmittedSource> {
    OPTIONS.get()?.lock().ok()?.workbench.take()
}

/// Reports whether diagnostic loading timing was explicitly requested.
/// # Returns
/// False for ordinary desktop startup and unconfigured test processes.
pub(crate) fn debug_ui() -> bool {
    OPTIONS
        .get()
        .and_then(|options| options.lock().ok().map(|options| options.debug_ui))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::shell::{ApplicationSessions, Direction, RetainedResources};

    // Independent host fixture ceilings; no production policy is duplicated.
    const LIMITS: RetainedResources = RetainedResources {
        payload_bytes: usize::MAX,
        ordered_entries: usize::MAX,
        shared_roots: usize::MAX,
        channel_count: usize::MAX,
        slot_count: usize::MAX,
        presentation_utf16_bytes: 0,
        presentation_entries: 0,
        retained_views: 0,
        open_views: 0,
    };

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).into()).collect()
    }
    #[test]
    fn startup_inputs_preserve_paths_and_require_explicit_stdin() {
        assert!(parse(&[]).unwrap().workbench.is_none());
        let options = parse(&args(&["--debug-ui", "--workbench", "a file.merlin.json"])).unwrap();
        assert!(options.debug_ui);
        assert_eq!(
            options.workbench,
            Some(WorkbenchSource::File("a file.merlin.json".into()))
        );
        assert_eq!(
            parse(&args(&["--workbench", "-"])).unwrap().workbench,
            Some(WorkbenchSource::Stdin)
        );
    }
    #[test]
    fn runtime_request_retains_original_path_and_moves_under_full_host_pressure() {
        let application = ApplicationSessions::new(LIMITS).unwrap();
        let budget = application.budget();
        let baseline = budget.usage();
        let key = application.snapshot(Direction::Downstream).key;
        let mut options = prepare(
            parse(&args(&["--workbench", "a file.merlin.json"])).unwrap(),
            budget,
        )
        .unwrap();
        let used = budget.usage();
        assert!(used.payload_bytes > baseline.payload_bytes);
        assert_eq!(used.ordered_entries, baseline.ordered_entries);
        assert_eq!(used.shared_roots, baseline.shared_roots);
        let pressure = budget
            .reserve_resources(RetainedResources {
                payload_bytes: LIMITS.payload_bytes - used.payload_bytes,
                ..Default::default()
            })
            .unwrap();
        let Some(AdmittedSource::File(path)) = options.workbench.take() else {
            panic!("Original startup file must move to the loader");
        };
        assert!(options.workbench.is_none());
        assert!(budget.owns(&path));
        assert_eq!(path.value(), &PathBuf::from("a file.merlin.json"));
        assert_eq!(application.snapshot(Direction::Downstream).key, key);
        assert_eq!(budget.usage().payload_bytes, LIMITS.payload_bytes);
        drop(pressure);
        assert_eq!(budget.usage(), used);
        drop(path);
        assert_eq!(budget.usage(), baseline);
    }

    #[test]
    fn rejected_runtime_path_does_not_retain_or_change_the_canonical_host() {
        let application = ApplicationSessions::new(LIMITS).unwrap();
        let budget = application.budget();
        let baseline = budget.usage();
        let before = application.snapshot(Direction::Downstream);
        let pressure = budget
            .reserve_resources(RetainedResources {
                payload_bytes: LIMITS.payload_bytes - baseline.payload_bytes,
                ..Default::default()
            })
            .unwrap();
        let full = budget.usage();
        assert!(prepare(
            parse(&args(&["--workbench", "a.merlin.json"])).unwrap(),
            budget
        )
        .is_err());
        assert_eq!(budget.usage(), full);
        assert_eq!(application.snapshot(Direction::Downstream), before);
        let stdin = prepare(parse(&args(&["--workbench", "-"])).unwrap(), budget).unwrap();
        assert!(matches!(stdin.workbench, Some(AdmittedSource::Stdin)));
        assert_eq!(budget.usage(), full);
        drop(pressure);
        assert_eq!(budget.usage(), baseline);
    }

    #[test]
    fn invalid_or_conflicting_requests_are_rejected_before_gui_startup() {
        for values in [
            vec!["--workbench"],
            vec!["--workbench", "--debug-ui"],
            vec!["--workbench", "a", "--workbench", "b"],
            vec!["--restore-update", "--workbench", "a"],
        ] {
            assert!(parse(&args(&values)).is_err());
        }
    }
}
