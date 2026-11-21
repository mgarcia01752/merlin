#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Check integration requirement and architecture links in both directions."""
import argparse
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
