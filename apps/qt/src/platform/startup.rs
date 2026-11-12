/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Desktop startup requests; large workbenches are read by the background loader.
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
static OPTIONS: OnceLock<Mutex<Options>> = OnceLock::new();

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
/// # Returns
/// No value; repeated configuration leaves the initial request unchanged.
pub(crate) fn configure(options: Options) {
    let _ = OPTIONS.set(Mutex::new(options));
}

/// Takes the command-line workbench once when the GUI is ready.
/// # Returns
/// An owned input source, or None for ordinary desktop startup.
pub(crate) fn take_workbench() -> Option<WorkbenchSource> {
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
