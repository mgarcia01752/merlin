/*
 * Copyright 2026 PNM Viewer contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

mod bridge;
mod help;
mod inputs;
mod instruments;
mod platform;
mod simulation;
mod workbench;
use cxx_qt_lib::{QGuiApplication, QQmlApplicationEngine, QString, QUrl};
pub(crate) use instruments::{spectrum as spectrum_view, transmitter as transmitter_instruments};

/// Starts the native workbench and its Rust simulation adapters.
///
/// # Panics
/// Terminates if Qt cannot construct its application or engine.
fn main() {
    let arguments = std::env::args().skip(1).collect::<Vec<_>>();
    if arguments
        .first()
        .is_some_and(|argument| argument == "--install-update")
    {
        let result = arguments
            .get(1)
            .ok_or("Provide the update directory.".to_string())
            .and_then(|path| platform::updates::install::elevated(std::path::Path::new(path)));
        if let Err(message) = result {
            eprintln!("{message}");
            std::process::exit(1);
        }
        return;
    }
    if arguments
        .first()
        .is_some_and(|argument| argument == "--version")
    {
        println!("MERLIN {}", env!("CARGO_PKG_VERSION"));
        return;
    }
    if arguments
        .first()
        .is_some_and(|argument| argument == "--check-workbench")
    {
        let check = || -> Result<(), String> {
            let path = arguments.get(1).ok_or("Provide a workbench JSON path.")?;
            workbench::project_file::check_file(std::path::Path::new(path))
        };
        if let Err(message) = check() {
            eprintln!("{message}");
            std::process::exit(1);
        }
        println!("Workbench inputs accepted.");
        return;
    }
    // GTK's native chooser runs before Qt starts, in a separate presentation-only
    // process, so the two toolkits cannot consume each other's pending UI events.
    if matches!(
        std::env::args().nth(1).as_deref(),
        Some(
            "--native-file-picker"
                | "--native-file-picker-multiple"
                | "--native-save-picker"
                | "--native-workbench-picker"
        )
    ) {
        platform::native_file_picker::run_helper(
            &std::env::args().nth(2).unwrap_or_default(),
            std::env::args().nth(1).as_deref() == Some("--native-file-picker-multiple"),
            std::env::args().nth(1).as_deref() == Some("--native-save-picker"),
            std::env::args().nth(1).as_deref() == Some("--native-workbench-picker"),
        );
        return;
    }
    if arguments
        .iter()
        .any(|argument| argument == "--help" || argument == "-h")
    {
        println!("Usage: merlin-qt [--workbench FILE|-] [--debug-ui]\n\n  --workbench FILE  Open a workbench without a file chooser\n  --workbench -     Read workbench JSON from standard input\n  --debug-ui        Show loading elapsed time\n  --check-workbench FILE  Validate a workbench without opening the UI\n  --version        Print the application version\n\nExample: merlin-qt --workbench - < workbench.merlin.json");
        return;
    }
    match platform::startup::parse(&arguments) {
        Ok(options) => platform::startup::configure(options),
        Err(message) => {
            eprintln!("{message}");
            std::process::exit(1);
        }
    }
    if let Err(error) = platform::logging::initialize(platform::startup::debug_ui()) {
        eprintln!("Cannot initialize logging: {error}");
        std::process::exit(1);
    }
    std::env::set_var("QT_QUICK_CONTROLS_STYLE", "Fusion");
    let mut application = QGuiApplication::new();
    application
        .as_mut()
        .expect("Qt application unavailable")
        .set_application_name(&QString::from("merlin"));
    QGuiApplication::set_desktop_file_name(&QString::from("merlin"));
    let mut engine = QQmlApplicationEngine::new();
    engine
        .as_mut()
        .expect("Qt QML engine unavailable")
        .load(&QUrl::from("qrc:/qt/qml/Merlin/Workbench/qml/Main.qml"));
    application
        .as_mut()
        .expect("Qt application unavailable")
        .exec();
}
