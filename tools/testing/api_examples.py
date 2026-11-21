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
EXAMPLES = (
    ('pnm-input', 'run_single_capture', ()),
    ('pnm-input', 'run_all_profiles', ()),
    ('pnm-input', 'run_single_capture_cm_chan_est', ()),
    ('merlin', 'upstream_session', ()),
    ('docsis-ofdma', 'upstream_loop', ()),
    ('docsis-ofdma', 'upstream_measurements', ()),
    ('docsis-ofdma', 'upstream_document', ()),
    ('docsis-ofdma', 'upstream_document', ('synthetic',)),
    ('pnm-input', 'inspect_ucd', ()),
    ('pnm-input', 'inspect_rxmer', ()),
    ('pnm-input', 'upstream_captures', ()),
    ('pnm-input', 'upstream_captures', ('25',)),
)
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
        for package, example, arguments in EXAMPLES:
            command = ['bash', str(ROOT / 'scripts/cargo-local.sh'), 'run', '-p', package,
                       '--example', example, '--locked']
            if arguments:
                command += ['--', *arguments]
            suffix = '-'.join(arguments)
            name = example + ('-' + suffix if suffix else '')
            run_example(command, ROOT / 'local/api-example-tests' / (name + '.log'))
    finally:
        print(f"API example checks total: {time.perf_counter() - started:.2f}s", flush=True)


if __name__ == "__main__":
    main()
