---
name: speq-patch
description: "Applies a bug fix, small feature, or tweak in the working tree in one run: implements and tests it, has code-reviewer review the diff once, and edits only the specs, ADRs, architecture, and docs it needs. Runs no git writes. Arg: change intent text."
---

# Patch (Orchestrator)

You take one small change to a tested, reviewed working tree. You write the code and tests yourself. You run no git write: the user commits, or `/speq-patch-pr` ships the change as a PR. Stop on the first failed step and report it. Running it again resumes the work.

## Required Skills

Invoke before starting:
- `/speq-cli`: spec discovery and validation
- `/speq-code-guardrails`: TDD
- `/speq-code-tools`: code navigation and editing
- `/speq-adr-rules`: whether a change earns an ADR
- `/speq-writing-guardrails`: prose style
- `/speq-planning`: apply only the Short Background and Specs-describe-the-software rules of its § Rules
- `/speq-ext-library`: on demand, for library APIs

## Rules

- Never touch `specs/_recorded/` or `specs/mission.md`. In `specs/_plans/`, touch only the patch folder `specs/_plans/<patch-name>/`.
- Prose states requirements and decisions. No process narration, no history.
- Ask about the size limit at most once per run. After a yes, report a later exceeded limit without asking.

## Workflow

### 0. Load Project Hook

Check for `.speq/patch-hook.md` in the repo root.
- **Present:** read it. Announce "Loaded project hook: .speq/patch-hook.md". Its content is authoritative: it can add to, change, or override any part of this workflow. If the hook conflicts with this workflow, the hook wins.
- **Absent:** continue without mention.

### 1. Check Prerequisites

`$1` empty → ask the user for the change intent. `specs/mission.md` absent → stop before any edit and point to `/speq-mission`. The working tree has uncommitted changes → ask whether they belong to the patch. They do not → stop and ask the user to commit or stash them. Never stash or discard them yourself.

### 2. Discover and Name

1. Run `speq plan list` and the absence check per `/speq-cli` for the behavior the change touches.
2. Do not interview. Ask only when the intent has more than one reading or leaves the expected behavior open.
3. Name the patch per `/speq-plan` step 3, without its decision log. A stopped run for this intent left the patch folder or the branch `patch/<name>` → reuse `<name>`. Otherwise never pick a name that `speq plan list` shows.

### 3. Size Check Before Coding

The change exceeds the size limit when it needs a new feature spec or a new domain, changes scenarios in more than 2 features, or adds or removes a component in `specs/architecture.md`. Then name the limit and the measured value, recommend `/speq-plan` or `/speq-plan-pr`, and ask whether to continue. A "no" stops the run before any edit.

### 4. Implement and Test

1. Write the code and tests, TDD per `/speq-code-guardrails`. A fix starts with a test that reproduces the defect. A removal deletes the code and the tests that covered it.
2. Run the test suites and the lint command from `specs/mission.md § Commands`. Redirect each command's output to a log: `mkdir -p target && <command> > target/speq-<suite>.log 2>&1`, and judge green or red by exit code. Every failure counts as caused by the patch. Red → fix and rerun. After 3 red runs of the same suite, stop and report the suite, its failing tests, and at most `tail -n 30` of its log.

### 5. Review

1. Collect the changed files per `/speq-implement` Phase 4 step 1, with base `HEAD`, leaving out the patch folder.
2. Write the patch folder's `plan.md`: H1 `# Plan: <patch-name>`, then `## Summary` with the change intent `$1`.
3. `review-findings.md` absent from the patch folder → spawn `code-reviewer` once with `/speq-implement` Phase 4 step 2's brief, `<plan-name>` set to `<patch-name>` and the hook line naming `.speq/patch-hook.md`. Present → a stopped run already reviewed: never spawn a second review.
4. Read `specs/_plans/<patch-name>/review-findings.md`. Fix every finding that is not yet fixed, then rerun the suites per step 4.

### 6. Specs, ADRs, Architecture, Docs

Edit only what the change needs.

- **Specs.** Edit `specs/<domain>/<feature>/spec.md` directly, without delta markers.
  - A fix that restores behavior a scenario already states, or a refactor with no behavior change → no spec edit.
  - Behavior that no scenario covers, or a wrong scenario → add or correct the scenario.
  - Removed behavior → delete the scenario, and the feature directory when no scenario remains.
  - A new feature starts from `/speq-plan`'s `references/feature-template.md`.
  - Keep the feature description accurate. Every added or changed scenario has a test in the diff. `speq feature validate <domain>/<feature>` passes for each edited feature.
  - An active plan other than the patch folder holds a delta for an edited feature → name the plan and the feature in the report.
  - A feature or domain over the limits of `/speq-spec-merge` "Check Library Thresholds" → name it in the report. Never reorganize the library.
- **ADR.** Apply `/speq-adr-rules`. The default is none. Write at most one new fragment `specs/_decision/NNN-<patch-name>.md`: NNN per `/speq-spec-merge` "Promote ADRs to Permanent Decision Log" step 2, H1 `# Decisions: <patch-name>`, `**Plan:** <patch-name>`, `**Status:** Accepted`, layout per `/speq-plan`'s `references/decision-log-permanent-template.md`. Never edit another fragment. `speq decision-log validate` passes.
- **Architecture.** `specs/architecture.md` absent → no edit. Otherwise edit its affected section only when the change alters a component, boundary, interface, data flow, constraint, or external dependency it describes. Keep the format of `/speq-plan`'s `references/architecture-template.md`, then run the post-check of `/speq-spec-merge` "Apply Architecture Delta" step 5. On failure, restore the file and report.
- **Docs.** Edit the project's user docs (by default `README.md` and `docs/`) only where the change alters documented behavior.

### 7. Size Check After the Change

Measure the whole change against `HEAD`, leaving out the patch folder, the version file, and the lockfile: `git diff --shortstat HEAD` plus each untracked file from `git ls-files --others --exclude-standard` and its line count. More than 25 files changed, or more than 400 lines (insertions plus deletions), exceeds the size limit. Ask as in step 3. A "no" stops the run with the change left in the working tree: report the changed files.

### 8. Clean Up and Report

Delete the patch folder `specs/_plans/<patch-name>/`. Then report:

- the patch name, the changed files, and the number of review findings fixed
- each suite and the lint command with its result
- the new ADR with its `/speq-adr-rules` rule-2 criterion and ADR search result, or none
- the changed `specs/architecture.md` sections, or none
- the size limit the user chose to exceed with its measured value, the affected plans, the library-size notes, and any `specs/mission.md` inaccuracy

End with: review the changes and commit them.
