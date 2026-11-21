#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Validate functional documentation links and the approved DOC-ORG-01 migration.

Normal mode checks current navigation/traceability. --migration also compares the
approved historical snapshot, exact file scope and checkbox states; use that mode
for this reorganization's acceptance, not as a freeze on future development.
"""
import argparse
import collections
import copy
import hashlib
import json
from pathlib import Path
import re
import subprocess
from urllib.parse import unquote, urlsplit

from integration_traceability import _has_link, _heading_block, audit as integration_audit

ROOT = Path(__file__).resolve().parents[2]
FEATURES = Path("docs/development/features")
INVENTORY = FEATURES / "unified-backend/source-inventory.json"
FROZEN = FEATURES / "unified-backend/feature-documentation-inventory.json"
ARCHITECTURE = Path("docs/architecture.md")
REQUIREMENTS = Path("docs/requirements.md")
CHECKLIST = Path("docs/development/checklist.md")
UI_CHECKLIST = Path("docs/development/ui-checklist.md")
FEC_AUDIT = "crates/fec-codec/src/upstream_ldpc/tests/source_audit.rs"
STALE_FEC_AUDIT = "crates/fec-codec/tests/upstream_source_audit.rs"

# Extract Markdown label/target groups: [API](api.md#run) -> API, api.md#run.
# Targets exclude whitespace/closing parentheses unless angle-quoted; empty labels
# are allowed, optional quoted titles are ignored, and matching is substring-based.
LINK = re.compile(r'!?\[([^\]\n]*)\]\((<[^>]+>|[^\s)]+)(?:\s+"[^"]*")?\)')


def _git(root, *args):
    return subprocess.check_output(["git", "-C", str(root), *args])


def _sha(data):
    return hashlib.sha256(data).hexdigest()


def _visible(text):

    # Mask fenced examples: ```md\n[x](missing.md)\n``` yields spaces/newlines.
    # Group 1 pairs the opening/closing fence; MULTILINE/DOTALL span the block.
    # The inner character substitution preserves newlines and masks all else.
    return re.sub(r'^\s*(```|~~~)[^\n]*\n.*?^\s*\1\s*$',
                  lambda match: re.sub(r'[^\n]', ' ', match[0]), text,
                  flags=re.MULTILINE | re.DOTALL)


def _slug(title):

    # Extract link labels within a heading: [API](x.md) -> API; group 1 is the
    # possibly empty label, excluding ], and the destination is discarded.
    title = re.sub(r'!?\[([^\]]*)\]\([^)]*\)', r'\1', title)

    # Remove embedded HTML substrings: <em>API</em> -> API, then lowercase.
    title = re.sub(r'<[^>]+>', '', title).lower()

    # Strip punctuation then hyphenate spaces: A: heading -> a-heading.
    # Word characters/hyphens survive; an empty heading produces an empty slug.
    return re.sub(r'[^\w\- ]', '', title).replace(' ', '-')


def anchors(text):
    """Return resolvable explicit/heading anchors and duplicate explicit IDs.

    Heading punctuation follows the repository's GitHub-style fragment convention;
    repeated headings receive numeric suffixes. Code fences do not declare anchors.
    """
    text = _visible(text)

    # Extract quoted nonempty anchor IDs: <a id="req-a"> -> req-a (group 1).
    # This scans visible text substrings and excludes quote delimiters.
    explicit = re.findall(r'<a\s+(?:id|name)=[\'"]([^\'"]+)', text)
    result = set(explicit)
    duplicates = [key for key, count in collections.Counter(explicit).items() if count > 1]
    seen = collections.Counter()

    # Extract ATX titles line-by-line: ## FEC -> FEC (group 1); at most three
    # leading spaces, one to six #, and optional trailing # are permitted.
    for title in re.findall(r'^ {0,3}#{1,6}\s+(.+?)\s*#*\s*$', text, re.MULTILINE):
        slug = _slug(title)
        index = seen[slug]
        seen[slug] += 1
        result.add(slug if not index else f"{slug}-{index}")
    return result, duplicates


def markdown_audit(root, paths):
    """Check local Markdown files/fragments and return diagnostics, edges and count.

    Args:
        root: Repository root; no network or external URL fetches occur.
        paths: Repository-relative Markdown paths to inspect.
    Returns:
        Errors, a local navigation graph, and the number of inspected links.
    """
    errors, edges, count = [], collections.defaultdict(set), 0
    cache = {}
    for path in paths:
        path = Path(path)
        text = (root / path).read_text(encoding="utf-8")
        own, duplicates = anchors(text)
        cache[path.resolve() if path.is_absolute() else (root / path).resolve()] = own
        if duplicates:
            errors.append(f"{path}: duplicate explicit anchors {duplicates}")
        for match in LINK.finditer(_visible(text)):
            url = urlsplit(match[2].strip('<>'))
            if url.scheme or url.netloc:
                continue
            count += 1
            target = ((root / path).parent / unquote(url.path)).resolve() if url.path else (root / path).resolve()
            try:
                relative = target.relative_to(root.resolve()).as_posix()
            except ValueError:
                errors.append(f"{path}: target escapes repository: {match[2]}")
                continue
            edges[path.as_posix()].add(relative)
            if not target.exists():
                errors.append(f"{path}: missing link {match[2]}")
            elif url.fragment and target.suffix == '.md':
                if target not in cache:
                    cache[target] = anchors(target.read_text(encoding="utf-8"))[0]
                if unquote(url.fragment) not in cache[target]:
                    errors.append(f"{path}: missing fragment {match[2]}")
    return errors, edges, count


def dependency_errors(graph):
    """Reject unknown prerequisites and cycles in a declared task graph."""
    errors, active, done = [], set(), set()

    def visit(task):
        if task in active:
            errors.append(f"Dependency cycle: {task}")
            return
        if task in done:
            return
        active.add(task)
        for dependency in graph[task]:
            if dependency not in graph:
                errors.append(f"Unknown prerequisite: {task} -> {dependency}")
            else:
                visit(dependency)
        active.remove(task)
        done.add(task)

    for task in graph:
        visit(task)
    return errors


def navigation_preservation_errors(root, path, before, after):
    """Preserve the prior index's local destinations and exact fragment routes."""
    def references(text):
        result = set()
        for match in LINK.finditer(_visible(text)):
            url = urlsplit(match[2].strip('<>'))
            if not url.scheme and not url.netloc:
                result.add((((root / path).parent / unquote(url.path)).resolve(), unquote(url.fragment)))
        return result

    return [f'Prior index navigation lost: {destination}#{fragment}'
            for destination, fragment in sorted(references(before) - references(after))]


def checkbox_history_errors(text, snapshot, allow_documentation_task=False):
    """Compare historical checkbox bytes, optionally removing only DOC-ORG-01.

    An ordinary accepted task becoming unchecked is an error. This does not change
    historical state; it is used only for the approved migration comparison.
    """

    # Extract complete checkbox lines: - [x] **A** Done. stays byte-identical;
    # MULTILINE includes indented details and both checked spellings, no groups.
    marks = re.findall(r'^[ \t]*- \[[ xX]\].*$', text, re.MULTILINE)
    if allow_documentation_task:
        marks = [line for line in marks if '**DOC-ORG-01**' not in line]
    if len(marks) != snapshot['checkbox_count'] or _sha('\n'.join(marks).encode()) != snapshot['checkbox_lines_sha256']:
        return ['Historical checkbox lines changed']
    return []


def owner_errors(root, owners, text):
    """Validate current versus explicitly planned module/test ownership locators."""
    errors = []
    for owner in owners:
        path = owner['path']
        state = owner['state']
        if state not in ('existing', 'planned'):
            errors.append(f"Unknown owner state: {path}")
        if state == 'existing' and not (root / path).exists():
            errors.append(f"Missing existing owner: {path}")
        if path not in text:
            errors.append(f"Owner absent from feature contract: {path}")
        if state == 'planned' and f'`{path}` (planned)' not in text:
            errors.append(f"Owner is not explicitly planned: {path}")
    return errors


def fec_owner_errors(root, record, definition):
    """Reject the stale current FEC locator without rewriting frozen provenance."""
    if record['test'] != FEC_AUDIT or FEC_AUDIT not in definition or STALE_FEC_AUDIT in definition or not (root / FEC_AUDIT).is_file():
        return ['REQ-COM-SYS-05: stale current source-audit owner']
    return []


def _task_block(text, identifier):

    # Locate a named top-level task: - [ ] **INT-X-01** matches that exact ID.
    # The escaped ID cannot supply regex syntax; MULTILINE anchors each line.
    match = re.search(r'^- \[[ xX]\] \*\*' + re.escape(identifier) + r'\*\*', text, re.MULTILINE)
    if not match:
        return ''

    # Find the next top-level checkbox prefix, e.g. - [, to bound this task
    # without borrowing a later task's links. No match means end-of-file.
    following = re.search(r'^- \[', text[match.end():], re.MULTILINE)
    end = match.end() + following.start() if following else len(text)
    return text[match.start():end]


def _requirement_block(text, identifier, fragment):
    marker = f'<a id="{fragment}"></a>'
    if marker in text:
        position = text.index(marker)
        return text[text.rfind('\n', 0, position) + 1:text.find('\n', position)]
    return _heading_block(text, identifier.lower()) or ''


def feature_errors(root, record, requirement_anchors):
    """Check one feature's canonical round trips, delivery gates and local owners."""
    path = Path(record['proposed_path'])
    text = (root / path).read_text(encoding='utf-8')
    architecture = (root / ARCHITECTURE).read_text(encoding='utf-8')
    requirements = (root / REQUIREMENTS).read_text(encoding='utf-8')
    checklist = (root / CHECKLIST).read_text(encoding='utf-8')
    errors = []
    for owner in record['architecture_references']:
        _, _, fragment = owner.partition('#')
        section = _heading_block(architecture, fragment) or ''
        if not _has_link(text, root / path, root / ARCHITECTURE, fragment):
            errors.append(f"{path}: missing architecture forward link")
        if not _has_link(section, root / ARCHITECTURE, root / path, ''):
            errors.append(f"{path}: missing architecture return link")
    for identifier in record['requirement_ids']:
        fragment = requirement_anchors.get(identifier)
        if not fragment:
            errors.append(f"{path}: unknown requirement {identifier}")
            continue
        if not _has_link(text, root / path, root / REQUIREMENTS, fragment):
            errors.append(f"{path}: missing requirement {identifier}")
        definition = _requirement_block(requirements, identifier, fragment)
        if not _has_link(definition, root / REQUIREMENTS, root / path, ''):
            errors.append(f"{path}: requirement lacks return link: {identifier}")
    for task in record['checklist_ids']:
        if not _has_link(text, root / path, root / CHECKLIST, task.lower()):
            errors.append(f"{path}: missing delivery link: {task}")
        if not _has_link(_task_block(checklist, task), root / CHECKLIST, root / path, ''):
            errors.append(f"{path}: task lacks return link: {task}")
    errors.extend(owner_errors(root, record['implementation_and_test_owners'], text))
    return errors


def audit(root, migration=False):
    """Audit current functional documentation and optionally its approved migration.

    Args:
        root: MERLIN checkout directory.
        migration: Compare exact approved file/ID/state history and mutation scope.
    Returns:
        A JSON-serializable result with actual counts and diagnostics.
    Raises:
        OSError, ValueError or subprocess errors for unreadable/inconsistent inputs.
    """
    inventory = json.loads((root / INVENTORY).read_text())
    frozen = json.loads((root / FROZEN).read_text())
    organization = inventory['documentation_organization']
    files = set(_git(root, 'ls-files', '-z').decode().split('\0'))
    files.update(_git(root, 'ls-files', '--others', '--exclude-standard', '-z').decode().split('\0'))
    files.discard('')
    markdown = sorted(p for p in files if p.endswith('.md') and '/local/' not in p and '/references/sections/' not in p and (root / p).is_file())
    errors, edges, link_count = markdown_audit(root, markdown)
    errors.extend(integration_audit(root))
    backfill = json.loads((root / FEATURES / 'downstream-requirements/traceability.json').read_text())
    groups = ['requirements', 'api_requirements', 'feature_scoped_requirements', 'engineering_requirements', 'retained_named_requirements', 'broad_preservation_requirements']
    identities = [(r['id'], r['anchor']) for group in groups for r in backfill[group]]
    identities += [(r['id'], r['id'].lower()) for r in inventory['source_requirements']]
    identities += [(r['id'], r['anchor']) for r in inventory.get('master_prompt_review', {}).get('requirements', [])]
    requirement_anchors = dict(identities)
    if len(requirement_anchors) != len(identities):
        errors.append('Duplicate canonical requirement ID')
    defined = anchors((root / REQUIREMENTS).read_text())[0]
    for identifier, fragment in identities:
        if fragment not in defined:
            errors.append(f"Missing canonical requirement anchor: {identifier}")

    # Existing downstream routes are current locators; historical source paths in
    # frozen inventories have a different role and are not reinterpreted here.
    for route_name, route in backfill['routes'].items():
        name, _, fragment = route['architecture'].partition('#')
        path = root / name
        if not path.exists() or (fragment and fragment not in anchors(path.read_text())[0]):
            errors.append(f'Invalid downstream architecture route: {route_name}')
        for kind in ['owners', 'tests', 'records']:
            for owner in route[kind]:
                if not (root / owner.partition('#')[0]).exists():
                    errors.append(f'Invalid downstream {kind} route: {owner}')
    downstream_task_mappings = 0
    for requirement in backfill['requirements']:
        definition = _requirement_block((root / REQUIREMENTS).read_text(), requirement['id'], requirement['anchor'])
        for task in requirement['tasks']:
            downstream_task_mappings += 1
            source = root / task['path']
            text = source.read_text()
            marker = f'<a id="{task["anchor"]}"></a>'
            if marker not in text:
                errors.append(f'Missing downstream task anchor: {task["anchor"]}')
                continue
            start = text.index(marker)
            end = text.find('<a id=', start + len(marker))
            block = text[start:end if end >= 0 else len(text)]
            if not _has_link(block, source, root / REQUIREMENTS, requirement['anchor']):
                errors.append(f'Downstream task lacks requirement link: {task["anchor"]}')
            if not _has_link(definition, root / REQUIREMENTS, source, task['anchor']):
                errors.append(f'Downstream requirement lacks task link: {requirement["id"]}')
    for record in organization['functional_records']:
        errors.extend(feature_errors(root, record, requirement_anchors))
    checklist = (root / CHECKLIST).read_text()
    doc_task = _task_block(checklist, 'DOC-ORG-01')
    for identifier, fragment in [('DOC-R3', 'ds-doc-r3'), ('REQ-COM-DOC-01', 'req-com-doc-01')]:
        if not _has_link(doc_task, root / CHECKLIST, root / REQUIREMENTS, fragment):
            errors.append(f'DOC-ORG-01: missing requirement {identifier}')
        definition = _requirement_block((root / REQUIREMENTS).read_text(), identifier, fragment)
        if not _has_link(definition, root / REQUIREMENTS, root / CHECKLIST, 'doc-org-01'):
            errors.append(f'DOC-ORG-01: missing requirement backlink {identifier}')
    doc_owner = organization['architecture']
    section = _heading_block((root / ARCHITECTURE).read_text(), doc_owner) or ''
    if not _has_link(doc_task, root / CHECKLIST, root / ARCHITECTURE, doc_owner) or not _has_link(section, root / ARCHITECTURE, root / CHECKLIST, 'doc-org-01'):
        errors.append('DOC-ORG-01: missing architecture round trip')
    graph = {task['id']: task['dependencies'] for task in inventory['integration_tasks']}
    graph.update({'DS-REQ-01': ['INT-DOC-01'], 'DS-GAP-01': ['DS-REQ-01'], 'DS-GAP-02': ['DS-REQ-01'], 'DS-GAP-03': ['DS-REQ-01'], 'SKILL-US-01': ['DEV-GUIDE-01'], 'DEV-GUIDE-01': [], 'DOC-ORG-01': organization['dependencies']})
    errors.extend(dependency_errors(graph))
    for task in inventory['integration_tasks'] + [{'id': 'DOC-ORG-01', 'dependencies': organization['dependencies']}]:
        block = _task_block(checklist, task['id'])

        # Extract group 1 across wrapped prerequisite lines, stopping before
        # Acceptance/Design: 'Depends on: A.\n  Acceptance:' -> 'A.'.
        dependency = re.search(r'Depends on: ([\s\S]+?)\n  (?:Acceptance|Design):', block)

        # Extract task-ID substrings: 'INT-DOC-01, DS-REQ-01.' -> those IDs.
        # Word boundaries exclude surrounding prose; absent fields yield [].
        actual = re.findall(r'\b(?:INT|DS|SKILL|DEV|DOC)-[A-Z-]+-\d+\b', dependency[1]) if dependency else []
        if actual != task['dependencies']:
            errors.append(f"Dependency inventory/checklist mismatch: {task['id']}")
    for name in [CHECKLIST, UI_CHECKLIST]:
        text = (root / name).read_text()

        # Extract named task labels per line: - [x] **UI-DOC-02** -> UI-DOC-02.
        # Group 1 excludes *; anonymous checkboxes are outside this ID check.
        labels = re.findall(r'^- \[[ xX]\] \*\*([^*]+)\*\*', text, re.MULTILINE)
        duplicates = {key: n for key, n in collections.Counter(labels).items() if n > 1}
        if duplicates != frozen['baseline']['legacy_duplicate_labels'][name.as_posix()]:
            errors.append(f'Unexpected task label collision: {name}')
    for fragment in organization['historical_aliases']['UI-DOC-02']:
        if fragment not in anchors(checklist)[0]:
            errors.append(f'Missing historical alias anchor: {fragment}')
    fec = next(r for r in inventory['source_requirements'] if r['id'] == 'REQ-COM-SYS-05')
    definition = _requirement_block((root / REQUIREMENTS).read_text(), fec['id'], fec['id'].lower())
    errors.extend(fec_owner_errors(root, fec, definition))
    feature_paths = {p for p in files if p.startswith(FEATURES.as_posix() + '/') and (root / p).is_file()}
    reached = {ARCHITECTURE.as_posix(), REQUIREMENTS.as_posix(), CHECKLIST.as_posix(), 'docs/development/README.md', (FEATURES / 'README.md').as_posix(), 'README.md', 'AGENTS.md', 'skills/README.md'}
    queue = list(reached)
    while queue:
        for destination in edges[queue.pop()] - reached:
            reached.add(destination)
            queue.append(destination)
    for path in sorted(feature_paths - reached):
        errors.append(f'Unreachable feature document: {path}')
    skills = [p for p in (root / 'skills').iterdir() if (p / 'SKILL.md').is_file()]
    for skill in skills:
        text = (skill / 'SKILL.md').read_text()

        # Check nonempty metadata lines, e.g. name: public-api; anchored
        # MULTILINE searches supplement the separately executed YAML validator.
        if not text.startswith('---\n') or not re.search(r'^name: .+', text, re.MULTILINE) or not re.search(r'^description: .+', text, re.MULTILINE):
            errors.append(f'Invalid skill frontmatter: {skill.name}')
        for discovery in ['.agents/skills', '.claude/skills']:
            path = root / discovery / skill.name
            if not path.is_symlink() or not path.exists() or path.resolve() != skill.resolve():
                errors.append(f'Invalid skill discovery: {path}')

    # Parse all original feature JSON files, including historical snapshot roles.
    for record in frozen['files']:
        if record['existing_path'].endswith('.json'):
            json.loads((root / record['existing_path']).read_text())
    if migration:
        approved = organization['approved_baseline']
        for path in [organization['frozen_plan'], organization['frozen_inventory']]:
            if (root / path).read_bytes() != _git(root, 'show', f'{approved}:{path}'):
                errors.append(f'Frozen proposal changed: {path}')
        expected = {r['existing_path'] for r in frozen['files']} | {organization['frozen_plan'], organization['frozen_inventory']} | {r['proposed_path'] for r in organization['functional_records']}
        if feature_paths != expected or len(expected) != 181:
            errors.append('Approved feature-file set/count differs from 181')
        expected_mapping = [{'path': r['existing_path'], 'destination': r['proposed_path'], 'disposition': r['disposition'], 'baseline_sha256': r['sha256']} for r in frozen['files']]
        if organization['existing_file_mapping'] != expected_mapping:
            errors.append('Current original-file mapping differs from approved snapshot')
        for record in frozen['files']:
            if record['existing_path'] != record['proposed_path']:
                errors.append('Unexpected original file move')
            if record['disposition'] != 'Adapt' and _sha((root / record['existing_path']).read_bytes()) != record['sha256']:
                errors.append(f"Protected original changed: {record['existing_path']}")
            before = _git(root, 'show', f"{frozen['baseline']['commit']}:{record['existing_path']}").decode()
            after = (root / record['existing_path']).read_text()
            if not anchors(before)[0] <= anchors(after)[0]:
                errors.append(f"Original feature anchor lost: {record['existing_path']}")
        index = FEATURES / 'README.md'
        errors.extend(navigation_preservation_errors(root, index, _git(root, 'show', f'{approved}:{index}').decode(), (root / index).read_text()))
        if set(requirement_anchors) != set(frozen['baseline']['canonical_requirement_ids']):
            errors.append('Approved canonical requirement ID set changed')
        current_requirements = (root / REQUIREMENTS).read_text()
        old_requirements = _git(root, 'show', f'{approved}:{REQUIREMENTS}').decode()

        # Extract exact API ID/text pairs: - REQ-1: keep -> (REQ-1, keep).
        # MULTILINE anchors whole clause lines; empty behavior text is retained.
        old_api = re.findall(r'^- (REQ-\d+): (.*)$', old_requirements, re.MULTILINE)

        # Apply the same clause extractor to current text; compare ordered pairs.
        if old_api != re.findall(r'^- (REQ-\d+): (.*)$', current_requirements, re.MULTILINE):
            errors.append('Original numbered API clauses changed')
        if not anchors(old_requirements)[0] <= anchors(current_requirements)[0]:
            errors.append('Original canonical requirement anchor lost')
        for requirement in inventory['source_requirements']:
            if requirement['behavior'] not in current_requirements or 'Acceptance: ' + requirement['acceptance'] not in current_requirements:
                errors.append(f"Canonical source clause changed: {requirement['id']}")
        for name in [CHECKLIST, UI_CHECKLIST]:
            errors.extend(f'{name}: {e}' for e in checkbox_history_errors((root / name).read_text(), frozen['baseline']['checkbox_snapshots'][name.as_posix()], name == CHECKLIST))
            before = _git(root, 'show', f'{approved}:{name}').decode()
            if not anchors(before)[0] <= anchors((root / name).read_text())[0]:
                errors.append(f'Historical task anchor lost: {name}')
        old = json.loads(_git(root, 'show', f'{approved}:{INVENTORY}'))
        current = copy.deepcopy(inventory)
        current.pop('documentation_organization')
        for row in current['source_requirements']:
            if row['id'] == 'REQ-COM-SYS-05':
                row['test'] = STALE_FEC_AUDIT
        for row in current['integration_tasks']:
            if row['id'] == 'INT-DOC-02':
                row['dependencies'].remove('DOC-ORG-01')
        if current != old:
            errors.append('Original source inventory history/clauses changed beyond approved routing')
        allowed = {r['existing_path'] for r in frozen['files'] if r['disposition'] == 'Adapt'} | {r['proposed_path'] for r in organization['functional_records']} | {ARCHITECTURE.as_posix(), REQUIREMENTS.as_posix(), CHECKLIST.as_posix(), 'tools/validation/feature_documentation.py', 'tools/validation/test_feature_documentation.py'}
        changed = set(_git(root, 'diff', '--name-only', approved).decode().splitlines())
        changed.update(_git(root, 'ls-files', '--others', '--exclude-standard').decode().splitlines())
        for path in sorted(changed - allowed):
            errors.append(f'Change outside approved documentation/tooling scope: {path}')
    return {
        'errors': errors,
        'mode': 'migration' if migration else 'current',
        'feature_files': len(feature_paths),
        'markdown_links': link_count,
        'canonical_requirement_ids': len(identities),
        'source_requirements': len(inventory['source_requirements']),
        'downstream_routes': len(backfill['routes']),
        'downstream_task_mappings': downstream_task_mappings,
        'functional_records': len(organization['functional_records']),
        'functional_requirement_mappings': sum(len(r['requirement_ids']) for r in organization['functional_records']),
        'functional_unique_requirements': len({i for r in organization['functional_records'] for i in r['requirement_ids']}),
        'functional_task_mappings': sum(len(r['checklist_ids']) for r in organization['functional_records']),
        'dependency_nodes': len(graph),
        'skills': len(skills),
        'discovery_links': len(skills) * 2,
        'protected_original_records': sum(r['disposition'] != 'Adapt' for r in frozen['files']),
        'historical_archives': sum(r['disposition'] == 'Archive' for r in frozen['files']),
    }


def main():
    """Print structural validation as JSON and fail on any unresolved diagnostic."""
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, default=ROOT)
    parser.add_argument('--migration', action='store_true')
    args = parser.parse_args()
    result = audit(args.root.resolve(), args.migration)
    print(json.dumps(result, indent=2))
    return int(bool(result['errors']))


if __name__ == '__main__':
    raise SystemExit(main())
