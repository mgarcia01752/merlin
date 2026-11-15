#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Run Rust API examples with per-example wall time and retained failure evidence."""
import json
import subprocess
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
EXAMPLES = ("run_single_capture", "run_all_profiles", "run_single_capture_cm_chan_est")
TIMEOUT_SECONDS = 900


def run_example(command, log):
    """Run one bounded check and persist elapsed time even when the child fails.

    Args:
        command: Exact executable and arguments; no shell interpolation.
        log: Output path for the child's combined stdout/stderr.
    Returns:
        Timing/status record; elapsed time includes incremental compilation.
    Raises:
        Child execution errors after writing the adjacent JSON timing record.
    """
    started = time.perf_counter()
    record = {"command": command, "status": "failed"}
    log.parent.mkdir(parents=True, exist_ok=True)
    try:
        with log.open("w", encoding="utf-8") as output:
            subprocess.run(command, cwd=ROOT, stdout=output, stderr=subprocess.STDOUT,
                           check=True, timeout=TIMEOUT_SECONDS)
        record["status"] = "passed"
        return record
    finally:
        record["elapsed_seconds"] = time.perf_counter() - started
        log.with_suffix(".json").write_text(json.dumps(record, indent=2) + "\n", encoding="utf-8")
        print(f"API example {log.stem}: {record['elapsed_seconds']:.2f}s ({record['status']}; includes build)", flush=True)


def main():
    """Execute the fixture-backed Rust entry workflows and print total wall time."""
    started = time.perf_counter()
    try:
        for example in EXAMPLES:
            run_example(["bash", str(ROOT / "scripts/cargo-local.sh"), "run", "-p", "pnm-input",
                         "--example", example, "--locked"],
                        ROOT / "local/api-example-tests" / f"{example}.log")
    finally:
        print(f"API example checks total: {time.perf_counter() - started:.2f}s", flush=True)


if __name__ == "__main__":
    main()
