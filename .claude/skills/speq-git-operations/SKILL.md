---
name: speq-git-operations
description: The git and gh operations, safety rules, and PR-title and version-bump rules for /speq-plan-pr, /speq-implement-pr, and /speq-patch-pr.
---

# Git Operations

## Rules

- Write commit messages as Conventional Commits. The type `spec` marks spec changes.
- Stage only the paths you name. Touch only the refs and PRs you name.
- A no-op (nothing to commit, PR already exists, already ready) is not an error. Report it and continue.
- Report a not-found or ambiguous target. Do not guess.
- Never merge a PR. Never force-push. Never rewrite history.

## PR Title and Version Bump

PR title from `<plan-name>`: `<type>(<scope>): <slug>`.
- **type**: `add`/`change` → `feat`, `remove` → `chore`, `refactor` → `refactor`, `fix` → `fix`, `chore` for an unparseable name.
- **scope**: the token after the verb. Without one, emit `<type>: <slug>`.
- **slug**: the plan name with hyphens turned into spaces.

Example: `add-plan-export` → `feat(plan): add plan export`.

Version bump: major for a breaking change (a `!` type or a `BREAKING CHANGE` in the plan's Impact), minor for `feat`, patch otherwise.

## Operations

| Operation | What it does |
|-----------|--------------|
| `create-branch` | Fetch, then create `branch` off `base` (default `origin/<default-branch>`) and check it out |
| `checkout` | A PR number or URL → `gh pr checkout`. A local or remote branch → check it out. Otherwise not-found. Creates nothing. Report the branch and its PR, if any |
| `commit` | Stage `paths`, commit with `message` |
| `push` | Push the current branch, with `-u origin <branch>` on first push |
| `create-pr` | `gh pr create` with `title`, `body`, and `--draft` when asked. If a PR exists for the branch, return it unchanged |
| `ready-pr` | `gh pr ready` when the PR is a draft |
| `comment-pr` | `gh pr comment` with `body` |
| `read-comments` | All three PR comment kinds posted at or after `since`: conversation comments and review summaries (`gh pr view --json comments,reviews`) and inline comments (`gh api repos/{owner}/{repo}/pulls/<n>/comments`, with path and line) |
| `ship-draft` | `commit` → `push` → `create-pr` (draft) |
| `ship-ready` | `commit` → `push` → `create-pr` (draft) → `ready-pr` |

A composite operation stops at the first failing step and reports which steps completed.
