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


def validate(root, data, exports=None, executions=None, native=None):
    """Audit complete operation rows, exact signatures and optional executed scope.

    Args:
        root: Complete checkout or isolated synthetic contract.
        data: The authoritative seventeen-cell matrix and shared contract cells.
        exports: Optional fresh rustdoc item dictionary for exact export coverage.
        executions: Optional set of actually passed exact test locators.
        native: Optional actual installed namespace/class/operation inventory.
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
        for case in python.get("cases", []):
            if not exact_case(root, case, "python"):
                errors.append(f"{operation}: missing exact Python case {case}")
            if native is not None and executions is not None and case not in executions:
                errors.append(f"{operation}: Python case has no successful execution {case}")
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
    if native is not None:
        errors.extend(validate_native(root, data, native, executions))
    return errors


def validate_native(root, data, native, executions=None):
    """Require complete actual native exports, exact signatures and named cases.

    Args:
        root: Checkout or independent validation fixture root.
        data: Same authoritative matrix, including language-specific projections.
        native: Actual installed inventory; mocked metadata is unit-test input only.
        executions: Optional successful Rust/Python case locators.
    Returns:
        Diagnostics for omissions, stale aliases/signatures, owners or execution.
        Metadata checks establish availability, never scientific branch coverage.
    """
    errors, actual = [], {}
    expected_types = {}
    for direction, classes in native.get('namespaces', {}).items():
        expected_types[direction] = []
        for cls in classes:
            expected_types[direction].append({key: cls[key] for key in
                ('name', 'canonical_name', 'module', 'signature', 'constructible')})
            for item in cls['operations']:
                actual[f"merlin.{direction}.{cls['name']}.{item['name']}"] = item
    if data.get('native_type_exports') != expected_types:
        errors.append('Native class/alias/constructor/module inventory differs from matrix')
    declared, rust_rows = set(), {row['operation']: row for row in data['operations']}
    types = {item['export'] for items in data['type_exports'].values() for item in items}
    requirements = (root / 'docs/requirements.md').read_text(encoding='utf-8')
    checklist = (root / 'docs/development/checklist.md').read_text(encoding='utf-8')
    for row in data.get('python_operations', []):
        operation = row.get('operation', '<missing>')
        if set(row) != COLUMNS or any(value in (None, '', [], {}) for value in row.values()):
            errors.append(f'{operation}: missing/nonempty seventeen-cell native contract')
            continue
        if operation in declared:
            errors.append(f'{operation}: duplicate native operation')
        declared.add(operation)
        item = actual.get(operation)
        if item is None:
            errors.append(f'{operation}: no actual native export')
            continue
        direction = operation.split('.')[1]
        if row['direction'] != direction or row['python_namespace'] != 'merlin.' + direction:
            errors.append(f'{operation}: stale native namespace/direction')
        python = row['python_type_method']
        if any(python.get(key) != item[key] for key in ('name', 'kind', 'signature')):
            errors.append(f'{operation}: stale native signature/descriptor')
        rust = row['rust_type_method']
        source = root / rust['source_file']
        if not source.is_file() or rust['symbol'] not in source.read_text(encoding='utf-8'):
            errors.append(f'{operation}: missing native Rust conversion owner')
        if rust['export'] not in types or row['rust_namespace'] != rust['export'].rsplit('::', 1)[0]:
            errors.append(f'{operation}: missing public Rust type owner')
        delegated = rust.get('delegated_operation')
        if delegated is not None:
            target = rust_rows.get(delegated)
            if target is None or target['rust_type_method']['signature'] != rust['signature']:
                errors.append(f'{operation}: stale delegated Rust operation/signature')
        for target in rust.get('delegated_operations', []):
            actual_target = rust_rows.get(target['operation'])
            if actual_target is None or any(actual_target['rust_type_method'][key] != target[key]
                                            for key in ('signature', 'source_file')):
                errors.append(f'{operation}: stale composed Rust operation/owner')
        for field in REFERENCES:
            if not data.get('contracts', {}).get(row[field], '').strip():
                errors.append(f'{operation}: unresolved native {field} contract')
        for identifier in row['requirement']:
            if not _requirement_block(requirements, identifier):
                errors.append(f'{operation}: unknown native requirement {identifier}')
        for task in row['checklist']:
            if f'**{task}**' not in checklist:
                errors.append(f'{operation}: unknown native task {task}')
        python_cases = row['tests']['python'].get('cases', [])
        if not python_cases:
            errors.append(f'{operation}: missing native acceptance cases')
        for language, cases in [('rust', row['tests']['rust']), ('python', python_cases)]:
            for case in cases:
                if not exact_case(root, case, language):
                    errors.append(f'{operation}: missing exact {language} case {case}')
                if executions is not None and case not in executions:
                    errors.append(f'{operation}: native owner case has no successful execution {case}')
    errors.extend(f'{operation}: native export lacks a matrix row'
                  for operation in sorted(actual.keys() - declared))
    for row in data['operations']:
        operation = row['python_type_method'].get('native_operation')
        if operation is not None and operation not in declared:
            errors.append(f"{row['operation']}: Rust/Python operation reference is unresolved")
    return errors


def main():
    """Validate the current matrix, optionally requiring fresh exports/executions.

    Returns:
        None after reporting counts; exits nonzero for any inconsistent row.
        Optional records must be generated by actual matching validation runs.
    """
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--exports", type=Path)
    parser.add_argument("--native-exports", type=Path)
    parser.add_argument("--executed-record", type=Path, action="append", default=[])
    args = parser.parse_args()
    data = json.loads((ROOT / MATRIX).read_text(encoding="utf-8"))
    exports = json.loads(args.exports.read_text(encoding="utf-8")) if args.exports else None
    native = json.loads(args.native_exports.read_text(encoding="utf-8")) if args.native_exports else None
    executions = executed_cases(ROOT, args.executed_record) if args.executed_record else None
    errors = validate(ROOT, data, exports, executions, native)
    if errors:
        raise SystemExit("\n".join(errors))
    print(f"API operation matrix: {len(data['operations'])} Rust / "
          f"{len(data.get('python_operations', []))} native rows; declared scope passes")


if __name__ == "__main__":
    main()
