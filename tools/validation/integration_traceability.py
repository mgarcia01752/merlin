#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Check integration requirement and architecture links in both directions."""
import argparse
import ast
import hashlib
import json
from pathlib import Path
import re
from urllib.parse import unquote, urlsplit

ROOT = Path(__file__).resolve().parents[2]
CHECKLIST = Path("docs/development/checklist.md")
ARCHITECTURE = Path("docs/architecture.md")
REQUIREMENTS = Path("docs/requirements.md")
INVENTORY = Path("docs/development/features/unified-backend/source-inventory.json")


def _links(text):
    """Extract local Markdown link labels and paths from a contract block.

    For example, [REQ-X](../requirements.md#req-x) yields that label and target.
    Groups exclude closing brackets/parentheses and newlines; external URLs are
    retained for the caller to reject as canonical local requirement references.
    """
    return re.findall(r"\[([^\]\n]+)\]\(([^)\s]+)\)", text)


def _fragment(title):
    """Return the GitHub heading fragment for a plain canonical section title."""
    # Strip punctuation, preserve word characters/hyphens, and replace spaces:
    # "Unified PNM and capture admission" becomes "unified-pnm-and-capture-admission".
    return re.sub(r"[^\w\- ]", "", title.lower()).replace(" ", "-")


def _heading_block(text, fragment):
    """Return one heading's content through its next heading, or None if absent."""
    headings = list(re.finditer(r"^#{1,6} (.+)$", text, re.MULTILINE))
    for index, heading in enumerate(headings):
        if _fragment(heading[1]) == fragment:
            end = headings[index + 1].start() if index + 1 < len(headings) else len(text)
            return text[heading.start():end]
    return None


def _requirement_block(text, identifier):
    """Resolve a stable requirement to its defining table row or heading block."""
    marker = f'<a id="{identifier.lower()}"></a>'
    if text.count(marker) == 1:
        start = text.index(marker)
        return text[text.rfind("\n", 0, start) + 1:text.find("\n", start)]
    return _heading_block(text, identifier.lower())


def _has_link(text, source, destination, fragment):
    """Match a local link's resolved document and exact fragment, ignoring its label."""
    for _, target in _links(text):
        url = urlsplit(target)
        if url.scheme or url.netloc:
            continue
        resolved = (source.parent / unquote(url.path)).resolve()
        if resolved == destination.resolve() and unquote(url.fragment) == fragment:
            return True
    return False


def matrix_evidence_errors(root, path):
    """Resolve exact seed-matrix test cases without claiming they were executed.

    Args:
        root: Repository containing test source files.
        path: Repository-relative operation matrix.
    Returns:
        Diagnostics for missing Rust cases or Python class/method declarations.
        This is source discovery only, not native API or scientific acceptance.
    """
    errors = []
    text = (root / path).read_text(encoding="utf-8")
    # Capture exact backtick-qualified cases, e.g. `tests/a.py::Suite.test_run`.
    # Group 1 is a relative .rs/.py file; group 2 is a case with optional class.
    for name, case in re.findall(r"`([\w/.-]+\.(?:rs|py))::([\w.]+)`", text):
        source = root / name
        valid = False
        if source.is_file():
            body = source.read_text(encoding="utf-8")
            if source.suffix == ".py":
                tree = ast.parse(body)
                for item in tree.body:
                    if isinstance(item, ast.ClassDef):
                        valid |= any(isinstance(method, ast.FunctionDef)
                                     and f"{item.name}.{method.name}" == case
                                     for method in item.body)
            else:
                valid = bool(re.search(r"#\[test\]\s*fn " + re.escape(case) + r"\(", body))
        if not valid:
            errors.append(f"API matrix: missing exact test {name}::{case}")
    return errors


def master_prompt_errors(root, inventory):
    """Check every retained prompt section has a nonempty canonical delivery chain.

    Args:
        root: Repository directory; inspection never executes backend work.
        inventory: Existing integration inventory with optional master review.
    Returns:
        Structural diagnostics. Absence of the optional review preserves older
        minimal fixtures; the real repository declares it explicitly.
    """
    review = inventory.get("master_prompt_review")
    if review is None:
        if any(task["id"] == "INT-PLAN-01" for task in inventory["integration_tasks"]):
            return ["INT-PLAN-01: missing master prompt review inventory"]
        return []
    errors = []
    for field in ["source", "review", "api_matrix"]:
        if not (root / review[field]).is_file():
            errors.append(f"Master prompt: missing {field} file")
    if errors:
        return errors
    source = (root / review["source"]).read_bytes()
    if hashlib.sha256(source).hexdigest() != review["sha256"]:
        errors.append("Master prompt: retained source hash differs")
    text = source.decode("utf-8")
    # Match each actual source heading, including subordinate clauses. The first
    # group is heading depth, the second its title; section hashes include text
    # through the next heading so deleting a mandatory bullet is detected too.
    headings = list(re.finditer(r"^(#{1,3}) (.+)$", text, re.MULTILINE))
    clauses = review["clauses"]
    identifiers = [clause["id"] for clause in clauses]
    if len(clauses) != len(headings) or len(set(identifiers)) != len(identifiers):
        errors.append("Master prompt: missing or duplicate clause coverage")
    tasks = {task["id"]: task for task in inventory["integration_tasks"]}
    requirements = (root / REQUIREMENTS).read_text(encoding="utf-8")
    document = (root / review["review"]).read_text(encoding="utf-8")
    blocks = re.split(r"(?=^\| <a id=)", document, flags=re.MULTILINE)
    for index, clause in enumerate(clauses):
        identifier = clause["id"]
        if index < len(headings):
            heading = headings[index]
            end = headings[index + 1].start() if index + 1 < len(headings) else len(text)
            digest = hashlib.sha256(text[heading.start():end].encode()).hexdigest()
            line = text[:heading.start()].count("\n") + 1
            if (clause["heading"], clause["line"], clause["sha256"]) != (heading[2], line, digest):
                errors.append(f"{identifier}: source clause/line/hash differs")
        marker = f'<a id="{identifier.lower()}"></a>'
        block = next((part.splitlines()[0] for part in blocks if marker in part), "")
        if document.count(marker) != 1:
            errors.append(f"{identifier}: missing or duplicate review anchor")
        if not clause["requirements"] or not clause["tasks"]:
            errors.append(f"{identifier}: empty requirement/task mapping")
        delivered = set()
        for task_id in clause["tasks"]:
            task = tasks.get(task_id)
            if task is None:
                errors.append(f"{identifier}: unknown delivery task {task_id}")
                continue
            delivered.update(task["requirements"])
            if not _has_link(block, root / review["review"], root / CHECKLIST, task_id.lower()):
                errors.append(f"{identifier}: missing task link {task_id}")
            if not _has_link(block, root / review["review"], root / ARCHITECTURE, task["architecture"]):
                errors.append(f"{identifier}: missing architecture link for {task_id}")
        for requirement in clause["requirements"]:
            if (_requirement_block(requirements, requirement) is None
                    or requirement not in delivered
                    or not _has_link(block, root / review["review"], root / REQUIREMENTS, requirement.lower())):
                errors.append(f"{identifier}: invalid requirement delivery {requirement}")
    errors.extend(matrix_evidence_errors(root, Path(review["api_matrix"])))
    return errors


def audit(root):
    """Audit all declared integration tasks without modifying files or task states.

    Args:
        root: Repository directory containing canonical documents and inventory.
    Returns:
        A list of diagnostics; an empty list means every declared task has
        nonempty matching requirements and resolving forward/reverse links.
    Raises:
        OSError or ValueError for unreadable files or malformed JSON input.
    """
    checklist = (root / CHECKLIST).read_text(encoding="utf-8")
    architecture = (root / ARCHITECTURE).read_text(encoding="utf-8")
    requirements = (root / REQUIREMENTS).read_text(encoding="utf-8")
    inventory = json.loads((root / INVENTORY).read_text(encoding="utf-8"))
    tasks = inventory["integration_tasks"]
    # Match alphabetic integration areas, e.g. INT-PNM-01, excluding the existing
    # downstream cell-interleaver IDs INT-01..03. Groups are status and stable ID.
    starts = list(re.finditer(r"^- \[([ x])\] \*\*(INT-[A-Z][A-Z0-9-]*)\*\*", checklist, re.MULTILINE))
    declared = [task["id"] for task in tasks]
    errors = []
    if not tasks or len(set(declared)) != len(declared):
        errors.append("Integration inventory is empty or contains duplicate task IDs")
    if sorted(declared) != sorted(match[2] for match in starts):
        errors.append("Integration inventory/checklist task IDs differ")
    blocks = {}
    for match in starts:
        # Stop at the next top-level task, including a downstream task after the
        # integration section. This prevents borrowing a later task's references.
        following = re.search(r"^- \[", checklist[match.end():], re.MULTILINE)
        end = match.end() + following.start() if following else len(checklist)
        blocks[match[2]] = checklist[match.start():end]
    for task in tasks:
        identifier = task["id"]
        block = blocks.get(identifier, "")
        field = re.search(r"^  Requirements: (.*?)(?=\n  Architecture|\Z)", block, re.MULTILINE | re.DOTALL)
        references = _links(field[1]) if field else []
        expected = task.get("requirements", [])
        labels = [label.replace("**", "") for label, _ in references]
        if not expected or not references:
            errors.append(f"{identifier}: empty requirement references")
        if labels != expected or len(set(labels)) != len(labels):
            errors.append(f"{identifier}: checklist/inventory requirements differ or repeat")
        for label, target in references:
            requirement = label.replace("**", "")
            definition = _requirement_block(requirements, requirement)
            if definition is None or not _has_link(f"[reference]({target})", root / CHECKLIST,
                                                 root / REQUIREMENTS, requirement.lower()):
                errors.append(f"{identifier}: invalid requirement target {requirement}")
                continue
            if not _has_link(definition, root / REQUIREMENTS, root / CHECKLIST, identifier.lower()):
                errors.append(f"{identifier}: requirement {requirement} lacks task backlink")
        owner = task["architecture"]
        section = _heading_block(architecture, owner)
        if section is None or not _has_link(block, root / CHECKLIST, root / ARCHITECTURE, owner):
            errors.append(f"{identifier}: missing architecture owner/link {owner}")
        elif not _has_link(section, root / ARCHITECTURE, root / CHECKLIST, identifier.lower()):
            errors.append(f"{identifier}: architecture lacks task backlink")
        marker = f'<a id="{identifier.lower()}"></a>'
        if checklist.count(marker) != 1:
            errors.append(f"{identifier}: missing or duplicate stable task anchor")
    errors.extend(master_prompt_errors(root, inventory))
    return errors


def main():
    """Print audit failures and return a nonzero process status on traceability gaps.

    The optional repository argument supports isolated synthetic regression tests;
    the default is this checkout. No documents, acceptance states or Git state change.
    """
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=ROOT)
    args = parser.parse_args()
    errors = audit(args.root.resolve())
    for error in errors:
        print(error)
    if not errors:
        print("Integration traceability: all task, requirement and architecture links pass")
    return int(bool(errors))


if __name__ == "__main__":
    raise SystemExit(main())
