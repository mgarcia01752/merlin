#!/usr/bin/env bash
#
# Copyright 2026 PNM Viewer contributors
# SPDX-License-Identifier: Apache-2.0
#
#
# Stage all non-ignored project changes and create a local commit.
# Require --commit-msg; never push. Git failures propagate to the caller.

set -euo pipefail

SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd -- "${SCRIPT_DIR}/../.." && pwd)"
COMMIT_MSG=""

# Print command usage to stdout without changing repository state.
# Arguments are unused unless described above; failures stop this script.
usage() {
  cat <<'EOF'
Usage: git-save.sh --commit-msg <message>

Stage all changes in the project and create a Git commit.

Options:
  --commit-msg <message>  Required commit subject.
  -h, --help              Show this help and exit.

Example:
  tools/git/git-save.sh --commit-msg "Feature: Add setup modules"
EOF
}

# Print the supplied error to stderr and exit with status 1.
# Arguments are unused unless described above; failures stop this script.
die() {
  echo "ERROR: $*" >&2
  exit 1
}

# Reject nested Git metadata and company-skill Gitlinks before staging.
# Arguments are unused unless described above; failures stop this script.
reject_nested_skill_checkouts() {
  local nested_git

  while IFS= read -r -d '' nested_git; do
    die "Nested Git metadata is not allowed under project skills: ${nested_git}"
  done < <(
    find "${PROJECT_DIR}/skills" -mindepth 2 \
      \( -type d -o -type f \) -name .git -print0
  )

  while IFS= read -r mode _ _ path; do
    if [[ "${mode}" == "160000" && -e "${PROJECT_DIR}/${path}" ]]; then
      die "Project skills must not be committed as a Gitlink: ${path}"
    fi
  done < <(
    git -C "${PROJECT_DIR}" ls-files --stage -- \
      skills
  )

}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --commit-msg)
      if [[ $# -lt 2 ]]; then
        die "--commit-msg requires a message"
      fi
      COMMIT_MSG="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *) die "Unknown argument: $1 (see --help)" ;;
  esac
done

if [[ -z "${COMMIT_MSG}" ]]; then
  die "--commit-msg is required"
fi
if ! git -C "${PROJECT_DIR}" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  die "Project directory is not a Git working tree: ${PROJECT_DIR}"
fi

reject_nested_skill_checkouts
git -C "${PROJECT_DIR}" add --all

if git -C "${PROJECT_DIR}" ls-files --stage -- \
    skills | awk '$1 == "160000" { found = 1 } END { exit !found }'; then
  die "Project skills Gitlink remains staged; remove it before committing"
fi

if git -C "${PROJECT_DIR}" diff --cached --quiet; then
  die "No staged changes to commit"
fi

echo "Staged changes:"
git -C "${PROJECT_DIR}" diff --cached --stat
echo
echo "Commit message: ${COMMIT_MSG}"

git -C "${PROJECT_DIR}" commit -m "${COMMIT_MSG}"
