#!/usr/bin/env python3
#
# Copyright 2026 DOCSIS OFDM Channel Sim contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Validate exact API rows against Rust exports and executed acceptance cases."""
import argparse
import ast
import json
from pathlib import Path
import re

from integration_traceability import _requirement_block

ROOT = Path(__file__).resolve().parents[2]
MATRIX = Path("docs/development/features/high-level-api/operation-matrix.json")
COLUMNS = {
    "direction", "operation", "rust_namespace", "rust_type_method", "python_namespace",
    "python_type_method", "input_ownership", "units", "error_behavior", "lifecycle_behavior",
    "report_contract", "compatibility", "requirement", "checklist", "tests", "evidence", "status",
}
REFERENCES = {"input_ownership", "units", "error_behavior", "lifecycle_behavior", "compatibility", "evidence", "status"}


def exact_case(root, locator, language):
    """Resolve one declared test function without interpreting runtime outcomes.

    Args:
        root: Repository or independent test fixture root.
        locator: Repository-relative file followed by :: and an exact case.
        language: rust or python, selecting syntax-aware discovery.
    Returns:
        True only when the exact declared case exists in the selected source.
    """
    if "::" not in locator:
        return False
    path, case = locator.split("::", 1)
    source = root / path
    if not source.is_file():
        return False
    body = source.read_text(encoding="utf-8")
    if language == "rust":
        # Match a whole fn identifier: "test_one" must not match "test_one_more".
        return re.search(r"\bfn\s+" + re.escape(case) + r"\s*\(", body) is not None
    tree = ast.parse(body)
    for item in tree.body:
        if isinstance(item, ast.ClassDef):
            if any(isinstance(member, ast.FunctionDef) and f"{item.name}.{member.name}" == case
                   for member in item.body):
                return True
        if isinstance(item, ast.FunctionDef) and item.name == case:
            return True
    return False


def executed_cases(root, paths):
    """Read successful named cases from retained machine-readable run records.

    Args:
        root: Repository or synthetic root containing the records.
        paths: Repository-relative JSON run record paths.
    Returns:
        A set of exact file::case locators from exit-zero records only.
    Raises:
        OSError, ValueError or KeyError for missing/malformed records.
        A nonzero recorded run raises ValueError instead of becoming evidence.
    """
    result = set()
    for path in paths:
        record = json.loads((root / path).read_text(encoding="utf-8"))
        runs = record.get("runs", [record])
        for run in runs:
            if run.get("exit_code") != 0:
                raise ValueError(f"API execution record is not successful: {path}")
            cases = run.get("cases", [])
            if run.get("passed") != len(cases):
                raise ValueError(f"API execution count differs from named cases: {path}")
            named = {f"{case['test_file']}::{case['qualified_test_case']}" for case in cases}
            if len(named) != len(cases):
                raise ValueError(f"API execution record duplicates named cases: {path}")
            result.update(named)
    return result


def validate(root, data, exports=None, executions=None):
    """Audit complete operation rows, exact signatures and optional executed scope.

    Args:
        root: Complete checkout or isolated synthetic contract.
        data: The authoritative seventeen-cell matrix and shared contract cells.
        exports: Optional fresh rustdoc item dictionary for exact export coverage.
        executions: Optional set of actually passed exact test locators.
    Returns:
        Diagnostics; an empty list means the supplied scope is consistent.
        Source/symbol checks alone never establish runtime acceptance.
    """
    errors = []
    seen = set()
    requirements = (root / "docs/requirements.md").read_text(encoding="utf-8")
    checklist = (root / "docs/development/checklist.md").read_text(encoding="utf-8")
    # Read stable bold task identifiers from checkbox lines, e.g. **INT-RUST-01**.
    tasks = set(re.findall(r"^- \[[ x]\] \*\*([A-Z][A-Z0-9-]+)\*\*", checklist, re.MULTILINE))
    contracts = data.get("contracts", {})
    source_methods = {}
    source_namespaces = {}
    if exports is not None:
        for items in exports.values():
            for item in items:
                for method in item["methods"]:
                    source_methods[method["operation"]] = method
                    source_namespaces[method["operation"]] = item["export"].rsplit("::", 1)[0]
    for row in data.get("operations", []):
        operation = row.get("operation", "<missing>")
        if set(row) != COLUMNS or any(value in (None, "", [], {}) for value in row.values()):
            errors.append(f"{operation}: missing/nonempty seventeen-cell contract")
            continue
        if operation in seen:
            errors.append(f"{operation}: duplicate operation")
        seen.add(operation)
        if row["direction"] not in {"downstream", "upstream", "common"}:
            errors.append(f"{operation}: invalid direction")
        if not operation.startswith(row["rust_namespace"] + "::"):
            errors.append(f"{operation}: incorrect Rust namespace")
        for field in REFERENCES:
            if row[field] not in contracts or not contracts[row[field]].strip():
                errors.append(f"{operation}: unresolved {field} contract")
        for identifier in row["requirement"]:
            if not _requirement_block(requirements, identifier):
                errors.append(f"{operation}: unknown requirement {identifier}")
        for task in row["checklist"]:
            if task not in tasks:
                errors.append(f"{operation}: unknown task {task}")
        declared = row["rust_type_method"]
        source = root / declared["source_file"]
        if not source.is_file():
            errors.append(f"{operation}: missing source owner")
        for case in row["tests"]["rust"]:
            if not exact_case(root, case, "rust"):
                errors.append(f"{operation}: missing exact Rust case {case}")
            if executions is not None and case not in executions:
                errors.append(f"{operation}: Rust case has no successful execution {case}")
        python = row["tests"]["python"]
        if not python.get("status"):
            errors.append(f"{operation}: missing Python availability/test status")
        if python.get("status", "").startswith("existing"):
            for case in python.get("cases", []):
                if not exact_case(root, case, "python"):
                    errors.append(f"{operation}: missing exact Python case {case}")
        if exports is not None and operation in source_methods:
            actual = source_methods[operation]
            if row["rust_namespace"] != source_namespaces[operation]:
                errors.append(f"{operation}: stale public namespace")
            if declared["signature"] != actual["signature"] or declared["source_file"] != actual["source_file"]:
                errors.append(f"{operation}: stale public signature/owner")
        elif exports is not None:
            if "::serde_limits::" in operation and source.is_file():
                name = operation.rsplit("::", 1)[1]
                body = source.read_text(encoding="utf-8")
                marker = "pub fn " + name
                if marker not in body:
                    errors.append(f"{operation}: missing hidden public helper")
                else:
                    start = body.index(marker)
                    signature = " ".join(body[start:body.index("{", start)].split())
                    if declared["signature"] != signature:
                        errors.append(f"{operation}: stale hidden helper signature")
            else:
                errors.append(f"{operation}: no matching public Rust export")
    if exports is not None:
        errors.extend(f"{operation}: exported operation lacks a matrix row"
                      for operation in sorted(source_methods.keys() - seen))
    return errors


def main():
    """Validate the current matrix, optionally requiring fresh exports/executions.

    Returns:
        None after reporting counts; exits nonzero for any inconsistent row.
        Optional records must be generated by actual matching validation runs.
    """
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--exports", type=Path)
    parser.add_argument("--executed-record", type=Path, action="append", default=[])
    args = parser.parse_args()
    data = json.loads((ROOT / MATRIX).read_text(encoding="utf-8"))
    exports = json.loads(args.exports.read_text(encoding="utf-8")) if args.exports else None
    executions = executed_cases(ROOT, args.executed_record) if args.executed_record else None
    errors = validate(ROOT, data, exports, executions)
    if errors:
        raise SystemExit("\n".join(errors))
    print(f"API operation matrix: {len(data['operations'])} complete rows; declared scope passes")


if __name__ == "__main__":
    main()
