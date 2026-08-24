#!/usr/bin/env bash
#
# Commits the given files through GitHub's GraphQL API instead of `git commit`.
#
# Commits authored this way are signed by GitHub and show up as Verified.
#
# Usage:
#   VERSION=1.2.3 ./scripts/create-signed-commit.sh [file...]
#
# Environment:
#   VERSION             required unless MESSAGE is set; used in the message
#   MESSAGE             commit message (default: "chore(release): prepare for $VERSION")
#   BRANCH              branch to commit onto (default: main)
#   GITHUB_REPOSITORY   owner/name (set automatically in Actions)
#   GH_TOKEN            token for `gh`; must NOT be a user PAT or the commit
#                       will be unsigned
#   GITHUB_OUTPUT       if set, the commit SHA is appended as `commit=<sha>`
#
# Requirements: gh, jq.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BRANCH="${BRANCH:-main}"

# Single-quoted on purpose, since these are GraphQL variables.
# shellcheck disable=SC2016
QUERY='
mutation (
  $repository: String!,
  $branch: String!,
  $expectedHeadOid: GitObjectID!,
  $message: String!,
  $files: [FileAddition!]!
) {
  createCommitOnBranch(input: {
    branch: {repositoryNameWithOwner: $repository, branchName: $branch},
    expectedHeadOid: $expectedHeadOid,
    message: {headline: $message},
    fileChanges: {additions: $files}
  }) {
    commit {
      oid
      signature { wasSignedByGitHub }
    }
  }
}'

FILES=("$@")
if [[ ${#FILES[@]} -eq 0 ]]; then
  FILES=(Cargo.toml Cargo.lock CHANGELOG.md)
fi

if [[ -z "${MESSAGE:-}" ]]; then
  MESSAGE="chore(release): prepare for ${VERSION:?VERSION or MESSAGE must be set}"
fi

cd "$ROOT"

if git diff --quiet HEAD -- "${FILES[@]}" &&
   ! git ls-files --others --exclude-standard -- "${FILES[@]}" | grep -q .; then
  echo "::error::No changes to commit in: ${FILES[*]}" >&2
  exit 1
fi

for f in "${FILES[@]}"; do
  if [[ ! -f "$f" ]]; then
    echo "::error::File does not exist: ${f}" >&2
    exit 1
  fi
done

files_json="$(
  for f in "${FILES[@]}"; do
    jq -n --arg path "$f" --rawfile content "$f" \
      '{path: $path, contents: ($content | @base64)}'
  done | jq -s '.'
)"

result="$(jq -n \
  --arg query "$QUERY" \
  --arg repository "${GITHUB_REPOSITORY:?GITHUB_REPOSITORY must be set}" \
  --arg branch "$BRANCH" \
  --arg expectedHeadOid "$(git rev-parse HEAD)" \
  --arg message "$MESSAGE" \
  --argjson files "$files_json" \
  '{query: $query, variables: {
     repository: $repository,
     branch: $branch,
     expectedHeadOid: $expectedHeadOid,
     message: $message,
     files: $files
   }}' \
  | gh api graphql --input - \
      --jq '[.data.createCommitOnBranch.commit.oid,
             .data.createCommitOnBranch.commit.signature.wasSignedByGitHub] | @tsv')"

read -r COMMIT_SHA SIGNED <<<"$result"

if [[ -z "$COMMIT_SHA" || "$COMMIT_SHA" == "null" ]]; then
  echo "::error::createCommitOnBranch returned no commit SHA." >&2
  exit 1
fi

if [[ "$SIGNED" != "true" ]]; then
  echo "::error::Commit ${COMMIT_SHA} was not signed by GitHub (wasSignedByGitHub=${SIGNED:-null})." >&2
  exit 1
fi

echo "$COMMIT_SHA"
if [[ -n "${GITHUB_OUTPUT:-}" ]]; then
  echo "commit=$COMMIT_SHA" >> "$GITHUB_OUTPUT"
fi
