---
name: speq-patch
description: "Ships a bug fix, small feature, or tweak in one run: branches, implements and tests it, has code-reviewer review the diff once, edits only the specs, ADRs, architecture, and docs it needs, bumps the version, and opens a ready PR. Arg: change intent text."
---

# Patch (Orchestrator)

You take one small change to a ready PR on the branch `patch/<patch-name>`. You write the code and tests yourself. Stop on the first failed step and report it. Running it again resumes the work.

## Required Skills

Invoke before starting:
- `/speq-cli`: spec discovery and validation
- `/speq-code-guardrails`: TDD
- `/speq-code-tools`: code navigation and editing
- `/speq-adr-rules`: whether a change earns an ADR
- `/speq-writing-guardrails`: prose style
- `/speq-git-operations`: git/`gh` writes, PR title, version bump
- `/speq-planning`: apply only the Short Background and Specs-describe-the-software rules of its § Rules
- `/speq-ext-library`: on demand, for library APIs

## Rules

- Never touch `specs/_plans/`, `specs/_recorded/`, or `specs/mission.md`.
- Prose states requirements and decisions. No process narration, no history. Commit no plan, notes, review, or report file.
- Ask about the size limit at most once per run. After a yes, report a later exceeded limit without asking.

## Workflow

### 0. Load Project Hook

Check for `.speq/patch-hook.md` in the repo root.
- **Present:** read it. Announce "Loaded project hook: .speq/patch-hook.md". Its content is authoritative: it can add to, change, or override any part of this workflow. If the hook conflicts with this workflow, the hook wins.
- **Absent:** continue without mention.

### 1. Check Prerequisites

`$1` empty → ask the user for the change intent. Stop before any edit and name the missing piece when `specs/mission.md` is absent (point to `/speq-mission`), `gh auth status` fails, or the repository has no `origin` remote.

### 2. Discover and Name

1. Run `speq plan list` and the absence check per `/speq-cli` for the behavior the change touches.
2. Do not interview. Ask only when the intent has more than one reading or leaves the expected behavior open.
3. Name the patch per `/speq-plan` step 3, without its decision log. On the branch `patch/<name>` of a stopped run for this intent, reuse `<name>`.

### 3. Size Check Before Branching

The change exceeds the size limit when it needs a new feature spec or a new domain, changes scenarios in more than 2 features, or adds or removes a component in `specs/architecture.md`. Then name the limit and the measured value, recommend `/speq-plan` or `/speq-plan-pr`, and ask whether to continue. A "no" stops the run before any branch exists.

### 4. Branch

1. The current branch is not `patch/<patch-name>` and the working tree has uncommitted changes → ask whether they belong to the patch. They do not → stop and ask the user to commit or stash them. Never stash or discard them yourself.
2. Run `checkout` with `patch/<patch-name>`. Found → resume at step 8 when a commit on `origin/<default-branch>..HEAD` has the step 7 PR title as subject, otherwise at step 5. Not found → `create-branch` with `patch/<patch-name>` and `base: origin/<default-branch>`.

### 5. Implement and Test

1. Write the code and tests, TDD per `/speq-code-guardrails`. A fix starts with a test that reproduces the defect. A removal deletes the code and the tests that covered it.
2. Run the test suites and the lint command from `specs/mission.md § Commands` with logs per `/speq-implement-pr` step 5, which judges green or red by exit code. Every failure counts as caused by the patch. Red → fix and rerun. After 3 red runs of the same suite, stop and report the suite, its failing tests, at most `tail -n 30` of its log, and the branch state.

### 6. Review

Collect the changed files per `/speq-implement` Phase 4 step 1, with base `origin/<default-branch>`. Spawn `code-reviewer` once:

```
Delegate to code-reviewer: Review patch <patch-name>
- Change intent: <$1>
- Changed files: <the list>
- Write findings to target/speq-review-<patch-name>.md per your output format; return only the one-line verdict.
- Project Hook: <if active: .speq/patch-hook.md, read it and apply it; else omit this line>
```

Read the findings file. Fix every finding, then rerun the suites per step 5. Never spawn a second review.

### 7. Bump Version and Commit

1. Bump the version file and sync the lockfile per `/speq-implement-pr` step 3, at the level `/speq-git-operations` § PR title and version bump gives.
2. Derive the PR title from `<patch-name>` per that same section.
3. Run `commit` with the code, the tests, the version file, and the lockfile. Message: the PR title.

### 8. Size Check After Commit

Measure `git diff --shortstat origin/<default-branch>...HEAD`. More than 10 files changed, or more than 400 lines (insertions plus deletions), exceeds the size limit. Ask as in step 3. A "no" stops the run with the implementation committed on the unpushed branch: report the branch name and the commit.

A resumed run runs the suites here once for the PR body. Red → stop and report as in step 5.

### 9. Specs, ADRs, Architecture, Docs

Edit only what the change needs.

- **Specs.** Edit `specs/<domain>/<feature>/spec.md` directly, without delta markers.
  - A fix that restores behavior a scenario already states, or a refactor with no behavior change → no spec edit.
  - Behavior that no scenario covers, or a wrong scenario → add or correct the scenario.
  - Removed behavior → delete the scenario, and the feature directory when no scenario remains.
  - A new feature starts from `/speq-plan`'s `references/feature-template.md`.
  - Keep the feature description accurate. Every added or changed scenario has a test in the diff. `speq feature validate <domain>/<feature>` passes for each edited feature.
  - An active plan holds a delta for an edited feature → name the plan and the feature in the report and the PR body.
  - A feature or domain over the limits of `/speq-spec-merge` "Check Library Thresholds" → name it in the report. Never reorganize the library.
- **ADR.** Apply `/speq-adr-rules`. The default is none. Write at most one new fragment `specs/_decision/NNN-<patch-name>.md`: NNN per `/speq-spec-merge` "Promote ADRs to Permanent Decision Log" step 2, H1 `# Decisions: <patch-name>`, `**Plan:** <patch-name>`, `**Status:** Accepted`, layout per `/speq-plan`'s `references/decision-log-permanent-template.md`. Never edit another fragment. `speq decision-log validate` passes.
- **Architecture.** `specs/architecture.md` absent → no edit. Otherwise edit its affected section only when the change alters a component, boundary, interface, data flow, constraint, or external dependency it describes. Keep the format of `/speq-plan`'s `references/architecture-template.md`, then run the post-check of `/speq-spec-merge` "Apply Architecture Delta" step 5. On failure, restore the file and report.
- **Docs.** Edit the project's user docs (by default `README.md` and `docs/`) only where the change alters documented behavior.

### 10. Ship

```
Run operation: ship-ready (per /speq-git-operations)
  paths: the edited specs, ADRs, architecture, and docs
  message: spec(<scope>): <patch-name>, or docs(<scope>): <patch-name> when only docs change
  title: <the step 7 PR title>
  body: per references/pr-body-template.md
```

Never merge the PR.

### 11. Report

Give the PR link, the new version, the edited files, and the number of review findings fixed. Name the size limit the user chose to exceed with its measured value, the affected plans, the library-size notes, and any `specs/mission.md` inaccuracy.
