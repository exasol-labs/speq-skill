---
name: speq-patch-pr
description: "Ships a bug fix, small feature, or tweak as one ready PR: runs /speq-patch on the patch/patch-name branch, bumps the version, commits, and opens a ready pull request. Run again to resume. Arg: change intent text."
---

# Patch, PR flow (Orchestrator)

This skill runs `/speq-patch` on its own branch and ships the result as a ready PR. `/speq-patch` writes the code, tests, specs, and docs. You run every git/`gh` action yourself, per `/speq-git-operations`. Stop on the first failed step and report it. Running it again resumes the work.

## Required Skills

Invoke before starting:
- `/speq-writing-guardrails`: prose style for GitHub text
- `/speq-git-operations`: git/`gh` writes, PR title, version bump

`/speq-patch` invokes its own required skills.

## Workflow

### 0. Load Project Hook

Check for `.speq/patch-pr-hook.md` in the repo root.
- **Present:** read it. Announce "Loaded project hook: .speq/patch-pr-hook.md". Its content is authoritative: it can add to, change, or override any part of this workflow. If the hook conflicts with this workflow, the hook wins.
- **Absent:** continue without mention.

### 1. Check Prerequisites and Name

1. `gh auth status` fails, or the repository has no `origin` remote → stop before any edit and name the missing piece.
2. Run `/speq-patch` steps 0 to 3: its hook, prerequisites, discovery, the patch name, and the first size check.

### 2. Branch

Run `checkout` with `patch/<patch-name>`.
- **Found:** a commit on `origin/<default-branch>..HEAD` has the step 4 PR title as subject → go to step 5. Otherwise go to step 3.
- **Not found:** run `create-branch` with `patch/<patch-name>` and `base: origin/<default-branch>`. Uncommitted changes the user assigned to the patch carry over.

### 3. Patch

Run `/speq-patch` steps 4 to 8. Keep its report for the PR body. Skip its closing line.

### 4. Bump Version

1. Derive the PR title from `<patch-name>` per `/speq-git-operations` § PR title and version bump.
2. Bump the version file and sync the lockfile per `/speq-implement-pr` step 3, at the level that same section gives.

### 5. Ship

A run that resumed here stops when the working tree has uncommitted changes: name the files and ask the user to commit or stash them. Never ship them unreviewed. Otherwise it runs the suites once per `/speq-patch` step 4 and reads the ADR, architecture, and plan lines from the commit's diff. Red → stop and report as in `/speq-patch` step 4.

```
Run operation: ship-ready (per /speq-git-operations)
  paths: every file /speq-patch changed, the version file, and the lockfile
  message: <the step 4 PR title>
  title: <the step 4 PR title>
  body: per references/pr-body-template.md
```

Never merge the PR.

### 6. Report

Give the PR link, the new version, and the `/speq-patch` report.
