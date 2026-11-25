/*
 * Copyright 2026 MERLIN contributors
 * SPDX-License-Identifier: GPL-3.0-only
 */
//! Explicit native-GUI artifact gate, run separately from ordinary unit fixtures.
use merlin::upstream::ReportJson;
use merlin::{
    common::{Project, ProjectDirection},
    downstream::FullLoopControl,
};
use serde_json::{json, Value};
use std::{io::Write, path::PathBuf};

#[test]
#[ignore = "requires completed actual native GUI file-probe artifacts"]
fn actual_gui_project_files_load_and_run_through_public_rust_for_native_python_parity() {
    let input = PathBuf::from(
        std::env::var_os("MERLIN_QT_GUI_PROJECT_DIRECTORY")
            .expect("Actual GUI output directory is required"),
    );
    let output = PathBuf::from(
        std::env::var_os("MERLIN_QT_PROJECT_REFERENCE_DIRECTORY")
            .expect("Fresh reference directory is required"),
    );
    assert!(input.is_dir() && output.is_dir());
    for name in [
        "downstream-a.merlin.json",
        "downstream-open.merlin.json",
        "upstream-a.merlin.json",
    ] {
        let project = Project::from_file(input.join(name)).unwrap();
        let report = match project.direction() {
            ProjectDirection::Downstream => {
                let request = project.downstream_plan().unwrap().request(0).unwrap();
                let result = request.run(&FullLoopControl::new()).unwrap();
                let report = serde_json::to_value(result.report()).unwrap();
                assert_eq!(report["status"], "complete");
                json!({"status":report["status"],"summary":report["summary"],
                    "error":report["error"],"unrecovered_generated_payloads":report["unrecovered_generated_payloads"]})
            }
            ProjectDirection::Upstream => {
                let mut session = project.upstream_simulation().unwrap();
                let report = session.run().unwrap();
                serde_json::from_str::<Value>(&report.to_json().unwrap()).unwrap()
            }
        };
        let record = json!({"project":std::str::from_utf8(project.original_document()).unwrap(),"reference":report});
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(output.join(name))
            .unwrap();
        file.write_all(&serde_json::to_vec(&record).unwrap())
            .unwrap();
    }
}
