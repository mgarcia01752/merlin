#!/usr/bin/env python3
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
"""Validate, version, tag and atomically push a DOCSIS OFDM Channel Sim release."""
import argparse
from pathlib import Path
import shlex
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'tools/release'))
from versions import LOCKS, MANIFESTS, bumped, synchronize, validate


def git(*arguments, check=True):
    """Run Git in this checkout and return captured output.

    arguments are separate Git arguments. With check enabled, failed commands
    raise RuntimeError with Git's actual diagnostic; no rollback is performed.
    """
    result = subprocess.run(['git', '-C', str(ROOT), *arguments],
                            capture_output=True, text=True)
    if check and result.returncode:
        raise RuntimeError(result.stderr.strip() or result.stdout.strip()
                           or f'git {arguments[0]} failed')
    return result


class CheckoutChangedError(RuntimeError):
    """Identify a checkout guard failure without matching its display wording."""


def ensure_checkout(commit, status=''):
    """Reject changes to the expected master commit or working-tree state.

    commit is the expected HEAD object; status is the expected porcelain output.
    Raise RuntimeError before further release mutations if either differs.
    """
    if (git('rev-parse', 'HEAD').stdout.strip() != commit
            or git('branch', '--show-current').stdout.strip() != 'master'
            or git('status', '--porcelain', '--untracked-files=all').stdout != status):
        raise CheckoutChangedError('Branch, commit or working tree changed; review local state before retrying')


def remote_refs(*references):
    """Read exact origin refs, propagating remote failures.

    references are full ref names (or a peeled annotated-tag ref). Return a
    ref-to-object map; an absent ref is distinct from an inaccessible remote.
    """
    output = git('ls-remote', 'origin', *references).stdout
    return dict((reference, object_id) for object_id, reference in
                (line.split() for line in output.splitlines()))


def resume_candidate(commit, part):
    """Recognize only a complete unstaged candidate derived from committed metadata.

    commit identifies the immutable base; part selects the requested bump.
    Return the base version and expected porcelain status. Reject unrelated,
    staged, partial, redirected or altered files without modifying the checkout.
    """
    if git('diff', '--cached', '--name-only').stdout:
        raise RuntimeError('--resume requires an unchanged index; review staged work first')
    current = validate(git('show', f'{commit}:VERSION').stdout.strip())
    version = bumped(current, part)
    names = ('VERSION', *MANIFESTS, *LOCKS)
    with tempfile.TemporaryDirectory(prefix='merlin-resume-') as temporary:
        candidate_root = Path(temporary)
        for name in names:
            destination = candidate_root / name
            destination.parent.mkdir(parents=True, exist_ok=True)
            destination.write_text(git('show', f'{commit}:{name}').stdout, encoding='utf-8')
        if synchronize(candidate_root, current):
            raise RuntimeError('Committed version metadata is inconsistent; cannot resume')
        synchronize(candidate_root, version, write=True)
        for name in names:
            actual = ROOT / name
            if (actual.is_symlink() or actual.resolve() != actual
                    or actual.read_bytes() != (candidate_root / name).read_bytes()):
                raise RuntimeError(f'--resume requires exact {version} version edits: {name}')
    expected = ''.join(f' M {name}\n' for name in sorted(names))
    ensure_checkout(commit, expected)
    return current, expected


def release(part, existing_tag, dry_run, force=False, skip_test=False, resume=False, test_quick=False):
    """Prepare a bumped release or push an existing tag at HEAD.

    part selects major/minor/patch; existing_tag selects recovery without a bump.
    force allows a new bump from an already published clean HEAD; it never
    bypasses validation, pending local release recovery or immutable tag checks.
    dry_run performs preflight and remote reads but no validation/build or writes
    to version files, commits, tags or remote refs. Real releases validate the
    exact candidate tree before committing and atomically uploading both refs.
    Failures retain local changes/commits/tags and never reset or overwrite refs.
    skip_test omits local Rust tests/smokes only; hosted checks remain mandatory.
    resume recognizes an exact unstaged interrupted bump and reruns all selected checks.
    test_quick omits only the reviewed local sweep cases; full tests stay default.
    """
    if Path(git('rev-parse', '--show-toplevel').stdout.strip()).resolve() != ROOT:
        raise RuntimeError('The release helper must belong to the checkout root')
    if git('branch', '--show-current').stdout.strip() != 'master':
        raise RuntimeError('Release modes require master, matching the tag CI ancestry rule')
    starting_commit = git('rev-parse', 'HEAD').stdout.strip()
    initial_status = ''
    if resume:
        current, initial_status = resume_candidate(starting_commit, part)
    ensure_checkout(starting_commit, initial_status)
    if git('config', '--bool', '--get', 'remote.origin.mirror', check=False).stdout.strip() == 'true':
        raise RuntimeError('Refusing origin configured as a mirror')
    if not resume:
        current = validate((ROOT / 'VERSION').read_text(encoding='utf-8').strip())
        if git('show', 'HEAD:VERSION').stdout.strip() != current or synchronize(ROOT, current):
            raise RuntimeError('Commit consistent VERSION, Cargo manifests and lockfiles before releasing')
    version = validate(existing_tag.removeprefix('v')) if existing_tag else bumped(current, part)
    tag = f'v{version}'
    if existing_tag and (existing_tag != tag or version != current):
        raise RuntimeError('Existing tag must be vMAJOR.MINOR.PATCH matching VERSION')
    branch_ref = 'refs/heads/master'
    tag_ref = f'refs/tags/{tag}'
    current_ref = f'refs/tags/v{current}'
    refs = remote_refs(branch_ref, tag_ref, current_ref, current_ref + '^{}')
    # Fetch without tags so origin ancestry is checked without creating local tags.
    if branch_ref in refs:
        git('fetch', '--no-tags', 'origin', branch_ref)
        if git('merge-base', '--is-ancestor', 'FETCH_HEAD', starting_commit, check=False).returncode:
            raise RuntimeError('master is behind or diverged from origin; reconcile it first')
    local_tag = git('show-ref', '--verify', '--hash', tag_ref, check=False)
    if existing_tag:
        if local_tag.returncode:
            raise RuntimeError(f'Local tag {tag} does not exist; no tag was created')
        tag_object = local_tag.stdout.strip()
        if git('rev-parse', tag_ref + '^{commit}').stdout.strip() != starting_commit:
            raise RuntimeError('Existing tag must point to HEAD')
        if tag_ref in refs and refs[tag_ref] != tag_object:
            raise RuntimeError('Remote tag differs; refusing to overwrite it')
        if refs.get(branch_ref) == starting_commit and refs.get(tag_ref) == tag_object:
            print(f'Nothing to push: origin already has master and {tag}')
            return
    else:
        published_current = refs.get(current_ref + '^{}', refs.get(current_ref))
        published_head = published_current == starting_commit
        if published_head and not force and not resume:
            print(f'Nothing to release: HEAD is already published as v{current}')
            return
        local_current = git('rev-parse', '--verify', current_ref + '^{commit}', check=False)
        if (local_current.returncode == 0 and local_current.stdout.strip() == starting_commit
                and not ((force or resume) and published_head)):
            raise RuntimeError(f'HEAD is already tagged v{current}; use git-push.sh --tag v{current} instead of bumping again')
        if local_tag.returncode == 0 or tag_ref in refs:
            raise RuntimeError(f'Tag {tag} already exists; never move a release tag')
        if (git('log', '-1', '--format=%s').stdout.strip() == f'Feature: Release DOCSIS OFDM Channel Sim v{current}'
                and not ((force or resume) and published_head)):
            raise RuntimeError(f'HEAD is a release commit without its tag; review and create v{current}, then use --tag v{current}')
    ensure_checkout(starting_commit, initial_status)
    print(f'DOCSIS OFDM Channel Sim {current} -> {version}; master; tag {tag}', flush=True)
    if dry_run:
        print('Dry run: no bump, checks, commit, tag or push performed')
        return
    # Confirm identity before changing files. No automatic commit uses unrelated work.
    if not existing_tag:
        git('var', 'GIT_AUTHOR_IDENT')
        git('var', 'GIT_COMMITTER_IDENT')
        ensure_checkout(starting_commit, initial_status)
        if resume:
            changes = ['VERSION', *MANIFESTS, *LOCKS]
            print(f'Resuming exact {tag} candidate; rerunning selected checks.', flush=True)
        else:
            changes = ['VERSION', *synchronize(ROOT, version, write=True)]
    else:
        changes = []
    expected_status = git('status', '--porcelain', '--untracked-files=all').stdout
    candidate = {name: (ROOT / name).read_bytes() for name in changes}
    print('Running local checks on the release candidate...', flush=True)
    check_arguments = []
    if skip_test:
        check_arguments.append('--skip-test')
    elif test_quick:
        check_arguments.append('--test-quick')
    subprocess.run(['bash', str(ROOT / 'scripts/check-before-push.sh'), *check_arguments],
                   cwd=ROOT, check=True)
    ensure_checkout(starting_commit, expected_status)
    if any((ROOT / name).read_bytes() != content for name, content in candidate.items()):
        raise RuntimeError('Version files changed during validation; review before retrying')
    if synchronize(ROOT, version):
        raise RuntimeError('Version metadata drifted during validation')
    if not existing_tag:
        git('add', '--', *changes)
        candidate_tree = git('write-tree').stdout.strip()
        git('commit', '-m', f'Feature: Release DOCSIS OFDM Channel Sim {tag}')
        release_commit = git('rev-parse', 'HEAD').stdout.strip()
        ensure_checkout(release_commit)
        if (git('rev-parse', 'HEAD^{tree}').stdout.strip() != candidate_tree
                or git('rev-parse', 'HEAD^').stdout.strip() != starting_commit):
            raise RuntimeError('Release commit differs from the validated candidate; no tag or push performed')
        git('tag', '-a', tag, release_commit, '-m', f'DOCSIS OFDM Channel Sim {version}')
        tag_object = git('rev-parse', tag_ref).stdout.strip()
    else:
        release_commit = starting_commit
    ensure_checkout(release_commit)
    if git('rev-parse', tag_ref).stdout.strip() != tag_object:
        raise RuntimeError('Local tag changed during validation; no push performed')
    try:
        # Explicit object IDs prevent accidentally uploading changed refs or extra tags.
        git('push', '--atomic', '--no-follow-tags', '--recurse-submodules=no', 'origin',
            f'{release_commit}:{branch_ref}', f'{tag_object}:{tag_ref}')
    except RuntimeError as error:
        recovery = shlex.join(['bash', str(ROOT / 'tools/git/git-push.sh'), '--tag', tag,
                              *check_arguments])
        raise RuntimeError(f'{error}\nLocal release commit and tag are retained. '
                           f'Inspect origin before retrying without a bump:\n{recovery}') from error
    print(f'Pushed {tag}; this helper does not publish GitHub Releases')


def recovery_hint(arguments):
    """Suggest a read-only inspection or an exactly recognized bump retry.

    arguments contains the parsed release options. Return copyable shell text;
    preserve selected test/preview flags. Never mutate files or query origin.
    Unknown, staged or unrelated edits cannot receive a resume recommendation.
    """
    prefix = f'cd {shlex.quote(str(ROOT))} &&\n'
    if arguments.bump and not arguments.resume:
        try:
            commit = git('rev-parse', 'HEAD').stdout.strip()
            resume_candidate(commit, arguments.bump)
        except (OSError, ValueError, RuntimeError):
            pass
        else:
            command = ['tools/git/git-push.sh', f'--tag-{arguments.bump}', '--resume']
            if arguments.skip_test:
                command.append('--skip-test')
            if arguments.test_quick:
                command.append('--test-quick')
            if arguments.dry_run:
                command.append('--dry-run')
            return ('Exact pending version bump detected. Resume with:\n'
                    + prefix + shlex.join(command) + '\n')
    return 'Inspect local changes before retrying:\n' + prefix + 'git status --short && git diff\n'


def main():
    """Parse one explicit release mode; report failures without discarding work."""
    parser = argparse.ArgumentParser(description=__doc__)
    modes = parser.add_mutually_exclusive_group(required=True)
    modes.add_argument('--bump', choices=('major', 'minor', 'patch'))
    modes.add_argument('--tag', help='Push an existing matching tag at HEAD without bumping')
    parser.add_argument('--dry-run', action='store_true')
    testing = parser.add_mutually_exclusive_group()
    testing.add_argument('--test-quick', action='store_true',
                         help='Run representative tests; omit reviewed sweep cases')
    testing.add_argument('--skip-test', action='store_true',
                        help='Omit local Rust tests/smokes; retain other checks and hosted CI')
    parser.add_argument('--resume', action='store_true',
                        help='Resume an exact unstaged interrupted bump; rerun selected checks')
    parser.add_argument('--force', action='store_true',
                        help='Allow a bump from an already published clean HEAD; all checks still run')
    arguments = parser.parse_args()
    if arguments.force and not arguments.bump:
        parser.error('--force requires --bump')
    if arguments.resume and not arguments.bump:
        parser.error('--resume requires --bump')
    try:
        release(arguments.bump, arguments.tag, arguments.dry_run, arguments.force,
                arguments.skip_test, arguments.resume, arguments.test_quick)
    except (OSError, ValueError, RuntimeError, subprocess.CalledProcessError) as error:
        hint = ''
        if isinstance(error, (CheckoutChangedError, subprocess.CalledProcessError)):
            hint = recovery_hint(arguments)
        parser.exit(1, f'ERROR: {error}\n{hint}Review any local release state; nothing is reset automatically.\n')


if __name__ == '__main__':
    main()
