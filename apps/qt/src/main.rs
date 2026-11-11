/*
 * Copyright 2026 PNM Viewer contributors
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
mod bridge;
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
