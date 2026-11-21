#!/usr/bin/env python3
#
# Copyright 2026 MERLIN contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Validate accepted implementation chains without rewriting historical evidence."""
import ast
from functools import lru_cache
import hashlib
import json
from pathlib import Path
import re
import subprocess

FIELDS = {'requirement_id', 'architecture_owner', 'feature_design', 'checklist_task',
          'implementing_crate', 'source_file', 'symbol', 'test_file', 'qualified_test_case',
          'validation_command', 'observed_result', 'tested_candidate', 'status', 'limitations'}


def local_path(root, name):
    """Resolve a repository-relative evidence path, rejecting escaped inputs.

    Args:
        root: Checkout or isolated fixture root.
        name: Plain relative file path, without a URL or fragment.
    Returns:
        Resolved path inside root.
    Raises:
        ValueError for absolute, parent-traversing or redirected paths.
    """
    path = Path(name)
    if path.is_absolute() or '..' in path.parts or not (root / path).resolve().is_relative_to(root.resolve()):
        raise ValueError(f'Invalid repository evidence path: {name}')
    return root / path


def git(root, *arguments):
    """Read bounded Git metadata without changing the index or worktree.

    Args:
        root: Git checkout.
        arguments: Exact argument vector; no shell interpretation is performed.
    Returns:
        Command stdout bytes.
    Raises:
        Git/timeout errors; absent history is not accepted as verified identity.
    """
    return subprocess.run(['git', '-C', str(root), *arguments], check=True,
                          capture_output=True, timeout=30).stdout


@lru_cache(maxsize=4096)
def blob_digest(root, identity):
    """Hash one immutable Git blob, caching only its digest.

    Args:
        root: Checkout path string.
        identity: Git object identity obtained from ls-tree.
    Returns:
        SHA-256 of the original committed bytes.
    """
    return hashlib.sha256(git(root, 'cat-file', 'blob', identity)).hexdigest()


def fingerprint(files):
    """Compute the existing ordered path/tab/SHA-256 candidate contract.

    Args:
        files: Candidate file records in their declared order.
    Returns:
        Whole-manifest SHA-256; duplicate paths are rejected by the caller.
    """
    return hashlib.sha256(''.join(f"{item['path']}\t{item['sha256']}\n"
                                  for item in files).encode()).hexdigest()


def candidate_revision(root_text, record_name, expected, head):
    """Verify a tested manifest against current bytes or its real Git descendant.

    Args:
        root_text: Checkout root string.
        record_name: Relative immutable candidate manifest.
        expected: Digest declared by the acceptance record.
        head: Current Git HEAD, also delimiting the bounded historical search.
    Returns:
        Working-tree or exact committed revision containing every tested file.
    Raises:
        ValueError for tampering, missing history or no matching candidate.
    """
    root = Path(root_text)
    data = json.loads(local_path(root, record_name).read_text())
    files = data['files']
    names = [item['path'] for item in files]
    if not files or len(files) > 1000 or len(set(names)) != len(names):
        raise ValueError(f'{record_name}: empty/duplicate/excessive candidate files')
    for name in names:
        local_path(root, name)
    if data['sha256'] != expected or fingerprint(files) != expected:
        raise ValueError(f'{record_name}: candidate fingerprint mismatch')
    base = data['base_revision']
    if not re.fullmatch(r'[0-9a-f]{40}', base):
        raise ValueError(f'{record_name}: invalid base revision')
    git(root, 'merge-base', '--is-ancestor', base, head)
    if all((root / item['path']).is_file() and
           hashlib.sha256((root / item['path']).read_bytes()).hexdigest() == item['sha256']
           for item in files):
        return 'working-tree'
    revisions = git(root, 'rev-list', '--reverse', f'{base}..{head}').decode().splitlines()
    if len(revisions) > 256:
        raise ValueError(f'{record_name}: historical search exceeds 256 commits')
    for revision in [base, *revisions]:
        entries = git(root, 'ls-tree', '-rz', revision, '--', *names).split(b'\0')
        blobs = {}
        for entry in filter(None, entries):
            metadata, name = entry.split(b'\t', 1)
            mode, kind, identity = metadata.decode().split()
            if kind == 'blob' and mode in ('100644', '100755'):
                blobs[name.decode()] = identity
        if set(blobs) == set(names) and all(blob_digest(root_text, blobs[item['path']]) == item['sha256'] for item in files):
            return revision
    raise ValueError(f'{record_name}: tested files match no current/descendant revision')


def rust_code(text):
    """Mask comments and literals before locating Rust module/impl declarations.

    Args:
        text: Original Rust source.
    Returns:
        Same-length text with non-code spans blanked; lifetimes are retained.
        This is bounded declaration discovery, not a Rust compiler replacement.
    """
    # Raw/escaped strings and character literals precede comment alternatives.
    # For example r#"mod fake {}"# and // fn fake() are masked, while 'input stays.
    pattern = r'r(\#*)"[\s\S]*?"\1|"(?:\\.|[^"\\])*"|\'(?:\\.|[^\'\\])\'|//[^\n]*|/\*[\s\S]*?\*/'
    return re.sub(pattern, lambda match: ' ' * len(match[0]), text)


def block(text, opening):
    """Read one balanced, literal-masked Rust declaration body.

    Args:
        text: Code with comments/literals already masked.
        opening: Index of its opening brace.
    Returns:
        The body excluding its braces, or an empty string for invalid balance.
    """
    depth = 0
    for index in range(opening, len(text)):
        if text[index] == '{':
            depth += 1
        elif text[index] == '}':
            depth -= 1
            if depth == 0:
                return text[opening + 1:index]
    return ''


def python_symbol(text, names):
    """Resolve exact Python class/function nesting from syntax without importing.

    Args:
        text: Source file.
        names: Qualified identifier components.
    Returns:
        Whether each component resolves to an actual declared class/function.
    """
    body = ast.parse(text).body
    for name in names:
        item = next((item for item in body if isinstance(item, (ast.ClassDef, ast.FunctionDef)) and item.name == name), None)
        if item is None:
            return False
        body = item.body
    return True


def rust_case(root, name, case):
    """Resolve qualified Rust test modules to their exact declaration file/body.

    Args:
        root: Source checkout.
        name: Test target source, including crate roots for unit cases.
        case: Whole Rust qualified case identifier.
    Returns:
        True for an actual function in the exact module path, never a suffix match.
    """
    path = local_path(root, name)
    text = rust_code(path.read_text())
    parts = case.split('::')
    for module in parts[:-1]:
        # Whole module declaration, e.g. mod tests { or pub mod spectrum;.
        match = re.search(r'\bmod\s+' + re.escape(module) + r'\s*([;{])', text)
        if match is None:
            return False
        if match[1] == '{':
            text = block(text, match.end() - 1)
        else:
            directory = path.parent if path.name in ('lib.rs', 'main.rs', 'mod.rs') else path.with_suffix('')
            candidates = [directory / (module + '.rs'), directory / module / 'mod.rs']
            path = next((path for path in candidates if path.is_file()), None)
            if path is None:
                return False
            text = rust_code(path.read_text())
    return re.search(r'\bfn\s+' + re.escape(parts[-1]) + r'\s*\(', text) is not None


def symbol_exists(root, name, symbol, exports):
    """Resolve source-owned Rust/Python symbols and compiler-known facade exports.

    Args:
        root: Checkout.
        name: Exact implementation source file.
        symbol: Qualified function or facade type.
        exports: Known public Rust type exports from the compiler/matrix inventory.
    Returns:
        True only for a concrete declaration or verified module re-export.
    """
    path = local_path(root, name)
    text = path.read_text()
    if path.suffix == '.py':
        return python_symbol(text, symbol.split('.'))
    code = rust_code(text)
    parts = symbol.split('::')
    if len(parts) > 1 and 'merlin::' + symbol in exports:
        return re.search(r'\bmod\s+' + re.escape(parts[0]) + r'\s*{', code) is not None and 'pub use ' in code
    if len(parts) > 1:
        owner = parts[-2]
        # Restrict member discovery to this exact impl/trait body, not another type.
        for match in re.finditer(r'\b(?:impl|trait)\b[^{}]*\b' + re.escape(owner) + r'\b[^{}]*{', code):
            body = block(code, match.end() - 1)
            if re.search(r'\bfn\s+' + re.escape(parts[-1]) + r'\s*(?:<[^;{}]*>)?\s*\(', body):
                return True
        return False
    return re.search(r'\bfn\s+' + re.escape(symbol) + r'\s*(?:<[^;{}]*>)?\s*\(', code) is not None


def execution_index(root, names, candidates):
    """Load only successful, complete named MERLIN execution records.

    Args:
        root: Checkout.
        names: Existing record paths; historical source-only reports are excluded.
        candidates: Known SHA-256 to candidate-record mapping from implementation records.
    Returns:
        Exact file/case pairs with successful recorded execution.
    Raises:
        ValueError for failure, missing command, duplicates or count mismatches.
    """
    cases = set()
    for name in names:
        data = json.loads(local_path(root, name).read_text())
        for run in data.get('runs', [data]):
            if run.get('exit_code') != 0 or not run.get('command'):
                raise ValueError(f'{name}: failed/missing executed command')
            entries = [(case['test_file'], case['qualified_test_case']) for case in run.get('cases', [])]
            if run.get('passed') != len(entries) or len(set(entries)) != len(entries):
                raise ValueError(f'{name}: executed count/identity mismatch')
            if any(case.get('result', 'pass') not in ('pass', 'passed', 'ok') for case in run.get('cases', [])):
                raise ValueError(f'{name}: nonpassing case cannot become evidence')
            candidate = run.get('candidate_record', data.get('candidate_record'))
            digest = run.get('candidate_sha256', data.get('candidate_sha256'))
            if candidate is None:
                candidate = candidates.get(digest)
            if candidate is None:
                raise ValueError(f'{name}: execution lacks a known candidate identity')
            manifest = json.loads(local_path(root, candidate).read_text())
            if (digest is not None and manifest['sha256'] != digest
                    or fingerprint(manifest['files']) != manifest['sha256']):
                raise ValueError(f'{name}: execution candidate fingerprint mismatch')
            cases.update(entries)
    return cases


def compiled_cases(root, metadata, catalog):
    """Join actual nextest cases to exact Cargo target source paths.

    Args:
        root: Checkout whose metadata and binaries were compiled.
        metadata: Fresh locked cargo metadata output.
        catalog: Fresh nextest JSON discovery, without a test filter.
    Returns:
        Exact source-file/qualified-case identities, excluding ignored tests.
    Raises:
        ValueError for mismatched packages/targets or foreign source paths.
    """
    if Path(metadata['workspace_root']).resolve() != root.resolve():
        raise ValueError('Compiled metadata belongs to a different workspace')
    packages = {item['id']: item for item in metadata['packages']}
    result = set()
    for suite in catalog['rust-suites'].values():
        package = packages.get(suite['package-id'])
        if package is None or package['name'] != suite['package-name']:
            raise ValueError('Compiled case package identity differs from Cargo metadata')
        targets = [target for target in package['targets'] if target['name'] == suite['binary-name']
                   and suite['kind'] in target['kind']]
        if len(targets) != 1:
            raise ValueError('Compiled case has no exact Cargo target')
        source = Path(targets[0]['src_path']).resolve().relative_to(root.resolve()).as_posix()
        for name, case in suite['testcases'].items():
            if not case['ignored'] and case['filter-match']['status'] == 'matches':
                result.add((source, name))
    return result


def audit(root, inventory, compiled=None):
    """Require complete chains for newly accepted integration tasks.

    Args:
        root: Checkout with historical Git objects and current canonical documents.
        inventory: Existing source inventory, including its additive evidence catalog.
        compiled: Optional fresh compiled source-file/qualified-case identities.
    Returns:
        Diagnostics. Historical exceptions are verified against their old checklist;
        source and recorded execution checks never imply a fresh scientific run.
    """
    catalog = inventory.get('implementation_acceptance')
    if catalog is None:
        return []
    if catalog.get('completion_authority') != 'docs/development/checklist.md':
        return ['Implementation catalog does not retain the official completion authority']
    from integration_traceability import _heading_block, _requirement_block
    errors = []
    checklist = (root / 'docs/development/checklist.md').read_text()
    architecture = (root / 'docs/architecture.md').read_text()
    requirements = (root / 'docs/requirements.md').read_text()
    # Exact top-level checked integration IDs; INT-01 historical DS tasks excluded.
    completed = set(re.findall(r'^- \[x\] \*\*(INT-[A-Z][A-Z0-9-]*)\*\*', checklist, re.MULTILINE))
    tasks = {task['id']: task for task in inventory['integration_tasks']}
    try:
        candidates = {}
        for name in catalog['evidence_files']:
            for record in json.loads(local_path(root, name).read_text())['records']:
                candidate = record.get('tested_candidate', {})
                if isinstance(candidate, dict) and 'sha256' in candidate and 'record' in candidate:
                    candidates[candidate['sha256']] = candidate['record']
        executed = execution_index(root, catalog['execution_files'], candidates)
        head = git(root, 'rev-parse', 'HEAD').decode().strip()
        exports = {item['export'] for items in json.loads((root / catalog['operation_matrix']).read_text())['type_exports'].values() for item in items}
    except (OSError, ValueError, KeyError, subprocess.SubprocessError) as cause:
        return [f'Implementation evidence: {cause}']
    historical = set()
    for entry in catalog['historical_tasks']:
        try:
            previous = git(root, 'show', entry['revision'] + ':docs/development/checklist.md').decode()
            identifier = entry['task']
            if identifier not in tasks or f'- [x] **{identifier}**' not in previous:
                raise ValueError('historical exception was not already accepted')
            if not local_path(root, entry['evidence'].split('#')[0]).is_file() or not entry['reason']:
                raise ValueError('historical exception lacks retained evidence/reason')
            historical.add(identifier)
        except (OSError, ValueError, KeyError, subprocess.SubprocessError) as cause:
            errors.append(f'Historical implementation evidence: {cause}')
    coverage = set()
    seen = set()
    for name in catalog['evidence_files']:
        try:
            data = json.loads(local_path(root, name).read_text())
            if data.get('completion_authority') != 'docs/development/checklist.md':
                raise ValueError('record does not retain the official completion authority')
            for record in data['records']:
                identity = (record.get('checklist_task'), record.get('requirement_id'), record.get('source_file'), record.get('symbol'))
                if (set(record) != FIELDS or not isinstance(record.get('tested_candidate'), dict)
                        or not record['tested_candidate'] or any(not isinstance(record[field], str)
                        or not record[field].strip() for field in FIELDS - {'tested_candidate'})):
                    errors.append(f'{name}: missing/nonempty fourteen-field record {identity}')
                    continue
                if identity in seen:
                    errors.append(f'{name}: duplicate implementation identity {identity}')
                seen.add(identity)
                task, requirement = record['checklist_task'], record['requirement_id']
                if task not in tasks or _requirement_block(requirements, requirement) is None:
                    errors.append(f'{name}: unknown requirement/task {identity}')
                    continue
                requirement_block = _requirement_block(requirements, requirement)
                requirement_owner = record['architecture_owner'].removeprefix('docs/')
                if (record['architecture_owner'] != 'docs/architecture.md#' + tasks[task]['architecture']
                        and requirement_owner not in requirement_block):
                    errors.append(f'{name}: wrong architecture owner {identity}')
                for field in ('architecture_owner', 'feature_design'):
                    path, _, fragment = record[field].partition('#')
                    source = local_path(root, path)
                    if not source.is_file() or fragment and _heading_block(source.read_text(), fragment) is None:
                        errors.append(f'{name}: unresolved {field} {identity}')
                if not symbol_exists(root, record['source_file'], record['symbol'], exports):
                    errors.append(f'{name}: missing exact implementation symbol {identity}')
                source = local_path(root, record['source_file'])
                owner = None
                for parent in source.parents:
                    if not parent.is_relative_to(root):
                        break
                    manifest = parent / 'Cargo.toml'
                    if manifest.is_file():
                        # Whole Cargo name assignment, e.g. name = "docsis-ofdma".
                        found = re.search(r'^name\s*=\s*"([^"]+)"', manifest.read_text(), re.MULTILINE)
                        if found:
                            owner = found[1]
                            break
                if owner is not None and owner not in record['implementing_crate']:
                    errors.append(f'{name}: incorrect implementing Cargo owner {identity}')
                if owner is None and not any(word in record['implementing_crate'].lower() for word in ('tool', 'validation')):
                    errors.append(f'{name}: development owner is not identified as tooling {identity}')
                test = local_path(root, record['test_file'])
                exists = False
                if test.is_file():
                    exists = python_symbol(test.read_text(), record['qualified_test_case'].split('.')) if test.suffix == '.py' else rust_case(root, record['test_file'], record['qualified_test_case'])
                if not exists:
                    errors.append(f'{name}: missing exact acceptance declaration {identity}')
                if test.suffix == '.rs' and compiled is not None and (record['test_file'], record['qualified_test_case']) not in compiled:
                    errors.append(f'{name}: acceptance case is not in compiled discovery {identity}')
                if (record['test_file'], record['qualified_test_case']) not in executed:
                    errors.append(f'{name}: acceptance case has no successful execution {identity}')
                candidate = record['tested_candidate']
                candidate_revision(str(root), candidate['record'], candidate['sha256'], head)
                if not record['status'].startswith('integrated-validated') or record['observed_result'].lower().startswith(('fail', 'not run')):
                    errors.append(f'{name}: unaccepted/contradictory observed status {identity}')
                coverage.add((task, requirement))
        except (OSError, ValueError, KeyError, SyntaxError, subprocess.SubprocessError) as cause:
            errors.append(f'{name}: invalid implementation evidence: {cause}')
    for task in sorted(completed - historical):
        for requirement in tasks.get(task, {}).get('requirements', []):
            if (task, requirement) not in coverage:
                errors.append(f'{task}: accepted task lacks executed requirement chain {requirement}')
    return errors
