# Plan: add-patch-skill

## Summary

Add the entry-point skill `/speq-patch` (plugin: `/speq:patch`), which ships a bug fix, small feature, or tweak in one run. It branches, implements and tests the change, has `code-reviewer` review the diff once, edits only the specs, ADRs, architecture, and docs the change needs, bumps the version, and opens a ready PR.

## Context

- A small change today runs through `/speq-plan`, `/speq-implement`, and `/speq-record`. That flow writes a plan directory, a plan review, `tasks.md`, and a verification report.
- Users replace that flow for small changes with a hand-written prompt: branch, fix and test, then derive the needed spec, ADR, and doc changes, then open a PR.
- `code-reviewer` reads `specs/_plans/<plan-name>/plan.md` and writes `review-findings.md` into that directory (`.claude/agents/code-reviewer.md`). A patch has no plan directory.
- `specs/mission.md` and `specs/architecture.md` describe the `speq` CLI only. Skill behavior lives in the skill files and `docs/`, so this plan has no spec delta and no architecture delta.
- An ADR fragment needs `**ID:**`, `**Plan:**`, `**Status:**`, `### Context`, and `### Decision` (`src/validate/decision_log.rs:380-400`). The validator checks that `**Plan:**` is present, not that a plan directory exists. Valid statuses are `Accepted`, `Deprecated`, and `Superseded by <slug>`.
- ADR `per-plan-decision-fragments` stores "one committed ADR fragment per plan". A patch fragment keeps the ADR's reason: each unit of work writes its own file. ADR `slug-identity-not-sequential-numbers` requires a slug that is unique across all fragments.
- `/speq-plan-pr` owns the PR-title derivation (`.claude/skills/speq-plan-pr/SKILL.md:85-90`), and `/speq-implement-pr` already reaches into it. `/speq-implement-pr` step 3 maps a change to a version bump for `feat`, `fix`, and breaking changes only. The patch skill would be the third consumer of both rules.
- `scripts/plugin/build.sh` copies, renames, and installs every `.claude/skills/speq-*` directory by glob. Hardcoded skill lists exist only in `set_codex_skill_model` (`:110`), a log line (`:295`), `scripts/release/test.sh:110,119`, `tests/docker/test-codex-plugin.sh:70`, and `docs/hooks.md:21-29`.

## Features

No spec delta. The change adds a skill and edits skill, agent, script, and doc files. No `speq` CLI behavior changes.

## Impact

- New command `/speq:patch <intent>` in the Claude Code and Codex plugins.
- New optional hook file `.speq/patch-hook.md`.
- A patch writes an accepted ADR straight into `specs/_decision/`. Merging the patch PR is its acceptance.
- `code-reviewer` accepts a patch brief. Its behavior for `/speq-implement` is unchanged.
- The PR-title and version-bump rules move to `/speq-git-operations` unchanged. The behavior of `/speq-plan-pr` and `/speq-implement-pr` is unchanged.
- No breaking change.

## Requirements

The behavior of `/speq-patch`. Task 1.1 implements each row. Task 5.1 applies the current Test and Report rows. The skill states each rule once and points to the owning skill instead of restating it.

| Requirement | Details |
|-------------|---------|
| Hook | Step 0 loads `.speq/patch-hook.md` with the wording of `/speq-implement-pr` step 0. The skill passes the hook path to `code-reviewer` |
| Prerequisite | Stop before any edit and name the missing piece when `specs/mission.md` is absent (point to `/speq-mission`), `gh auth status` fails, or the repository has no `origin` remote. The skill reads commands and the tech stack from `specs/mission.md` |
| Questions | No interview. Ask only when the intent has more than one reading or leaves the expected behavior open |
| Name | `<patch-name>` per `/speq-plan` step 3 |
| Discovery | `speq feature list`, `speq plan list`, the ADR digest, and the absence check per `/speq-cli` for the behavior the change touches |
| Size limit | See "Patch size limit" below |
| Branch | Branch `patch/<patch-name>` exists → `checkout` it and continue from the first missing output (implementation commit, spec commit, PR). Otherwise, before any edit: uncommitted changes in the working tree → ask whether they belong to the patch. They do not → stop and ask the user to commit or stash them. Never stash or discard them yourself. Then `create-branch` with `patch/<patch-name>` and `base: origin/<default-branch>` |
| Code | The orchestrator writes the code and tests itself, TDD per `/speq-code-guardrails`. A fix starts with a test that reproduces the defect |
| Test | After the change, run the real test suites and the lint command from `specs/mission.md § Commands`, with logs per `/speq-implement-pr` step 5, which judges green or red by exit code. Every failure counts as caused by the patch. Red → fix and rerun. After 3 red runs of the same suite, stop and report the suite, its failing tests, at most `tail -n 30` of its log, and the branch state |
| Review | Spawn `code-reviewer` once with the intent, the changed-files list (changed files per `/speq-implement` Phase 4 step 1), and the hook path. Fix every finding, then rerun the suites. No second review |
| Version | Bump the version file and sync the lockfile per `/speq-implement-pr` step 3, at the level `/speq-git-operations` § PR title and version bump gives |
| Implementation commit | `commit` with the code, tests, version file, and lockfile. Message: the PR title |
| Specs | A fix that restores behavior a scenario already states → no spec edit. Behavior that no scenario covers, or a wrong scenario → add or correct the scenario. Removed behavior → delete the scenario, and the feature directory when no scenario remains. The implementation commit already deletes the tests that covered it. A refactor with no behavior change → no spec edit. Edit `specs/<domain>/<feature>/spec.md` directly, without delta markers. A new feature uses `/speq-plan`'s `references/feature-template.md`. Follow the Short Background and Specs-describe-the-software rules in `/speq-planning` § Rules, and keep the feature description accurate. Every added or changed scenario has a test in the diff. `speq feature validate <domain>/<feature>` passes for each edited feature. An active plan holds a delta for a feature the patch edits → name the plan and the feature in the report and the PR body |
| ADR | Apply `/speq-adr-rules`. The default is none. At most one new fragment `specs/_decision/NNN-<patch-name>.md`: NNN per `/speq-spec-merge` "Promote ADRs to Permanent Decision Log" step 2, H1 `# Decisions: <patch-name>`, `**Plan:** <patch-name>`, `**Status:** Accepted`, layout per `/speq-plan`'s `references/decision-log-permanent-template.md`. Never edit another fragment. `speq decision-log validate` passes. Each new ADR gets an entry on the PR body `ADRs:` line with its rule-2 criterion and ADR search result |
| Architecture | Edit the affected section of `specs/architecture.md` only when the change alters a component, boundary, interface, data flow, constraint, or external dependency it describes. Keep the format of `/speq-plan`'s `references/architecture-template.md`, then run the post-check of `/speq-spec-merge` "Apply Architecture Delta" step 5. On failure, restore the file and report. Absent file → no edit |
| Docs | Edit the project's user docs (by default `README.md` and `docs/`) only where the change alters documented behavior |
| Never touched | `specs/_plans/`, `specs/_recorded/`, `specs/mission.md`. A change that makes `specs/mission.md` inaccurate is named in the report |
| Prose | `/speq-writing-guardrails`. State requirements and decisions. No process narration, no history, and no plan, notes, review, or report file in the commit |
| Library size | A feature or domain over the limits of `/speq-spec-merge` "Check Library Thresholds" after the edit → name it in the report. Never reorganize the library |
| Ship | `ship-ready` with the spec, ADR, architecture, and doc paths. Message `spec(<scope>): <patch-name>`, or `docs(<scope>): <patch-name>` when only docs change. Title per `/speq-git-operations` § PR title and version bump. Body per "PR body" below. Never merge |
| Report | PR link, new version, edited spec, ADR, architecture, and doc files, number of review findings fixed, the size limit the user chose to exceed with its measured value, and library-size notes |

### Patch size limit

The skill checks two points and asks at most once per run:

1. **Before branching**, from the intent and discovery: the change needs a new feature spec or a new domain, changes scenarios in more than 2 features, or adds or removes a component in `specs/architecture.md`.
2. **After the implementation commit**, measured with `git diff --shortstat origin/<default-branch>...HEAD`: more than 10 files changed, or more than 400 lines (insertions plus deletions). The skill measures after the commit so new untracked files count.

When a limit is exceeded, the skill names the limit and the measured value, recommends `/speq-plan` or `/speq-plan-pr`, and asks whether to continue. A "no" at point 1 stops before any branch exists. A "no" at point 2 stops with the implementation committed on the unpushed branch and reports the branch name and commit. The user keeps the branch for a `/speq-plan` run or deletes it.

### PR body

```markdown
## Current State
<1-2 sentences: what is broken, missing, or true today>

## What Changes
<1-2 sentences: the mechanism>

Verification: <suite>: pass[, <suite>: pass]
ADRs: <none | slug (<rule-2 criterion>; ADR search: <result>)[, ...]>
Plans affected: <none | <plan-name> (<domain>/<feature>)[, ...]>
Architecture: <none | none (specs/architecture.md absent) | § Section (changed|new|removed)[, ...]>
```

## Implementation Tasks

1. Skill and its dependencies
   - [ ] 1.1 Write `.claude/skills/speq-patch/SKILL.md` per the Requirements table, the size limit, and the PR body above. Frontmatter: `name: speq-patch`, a `description` that ends `Arg: change intent text.`, and no `model:` field (decision-log [3]). Required skills: `/speq-cli`, `/speq-code-guardrails`, `/speq-code-tools`, `/speq-adr-rules`, `/speq-writing-guardrails`, `/speq-git-operations`, `/speq-planning` (§ Rules only), and `/speq-ext-library` on demand. Number the workflow steps. Keep the file at most 8 KB. If it does not fit, move the PR body into `.claude/skills/speq-patch/references/pr-body-template.md` [expert]
   - [ ] 1.2 Edit `.claude/agents/code-reviewer.md` description (add `/speq-patch`), Input, and Scope Constraints: a patch brief carries the change intent instead of `plan.md`, and the agent returns its findings in its reply in the `/speq-code-review` format and writes no file (decision-log [1]). In `.claude/skills/speq-code-review/`, edit the path line and rule 3 of `references/review-findings-template.md`, and add one sentence to `SKILL.md` § Routing: for a patch brief, the findings go to the orchestrator in the reply
   - [ ] 1.3 Move the PR-title derivation from `.claude/skills/speq-plan-pr/SKILL.md:85-90` and the commit-type → version-bump rule from `.claude/skills/speq-implement-pr/SKILL.md:59` ("`feat` → minor, a purely `fix` plan → patch, a breaking change → major") into a `## PR title and version bump` section of `.claude/skills/speq-git-operations/SKILL.md`. Move both verbatim and add no mapping. Point `/speq-plan-pr` (`:81`, `:85-90`) and `/speq-implement-pr` (`:59`, `:63`, `:83`) at it. Add `/speq-patch` to the consumers in `.claude/skills/speq-git-operations/SKILL.md:3,8`
   - [ ] 1.4 Add one bullet to rule 8 of `.claude/skills/speq-adr-rules/SKILL.md`: `/speq-patch` writes an ADR with `Status: Accepted` into its own fragment, merging the patch PR is the acceptance, and a reviewer rejects it by asking for the fragment's removal on the PR. Add `/speq-patch` to the consumer list in the description at `:3`
   - [ ] 1.5 In `docs/decision-log.md`, change the permanent-log row to "One fragment per plan or patch" and add a `/speq:patch` entry to the Lifecycle block: it writes an ADR with `Status: Accepted` into `specs/_decision/NNN-<patch-name>.md`, and merging the PR is the acceptance
2. Packaging and docs
   - [ ] 2.1 Add `/speq:patch` to the "Workflow skills" log line in `scripts/plugin/build.sh:295`. Leave `set_codex_skill_model` unchanged (decision-log [3])
   - [ ] 2.2 Add `skills/patch/SKILL.md` to both file loops in `scripts/release/test.sh:110,119`, and `patch` to the skill loop in `tests/docker/test-codex-plugin.sh:70`
   - [ ] 2.3 Write `docs/patch.md` in the style of `docs/pr-workflow.md`: breadcrumb, what one run does, when to use it instead of `/speq:plan`, the size limit, the files a patch may change (feature specs, ADRs, `specs/architecture.md`, and the project's user docs, by default `README.md` and `docs/`), the hook, and the rerun after a stop
   - [ ] 2.4 Add a Patch row to the guide tables in `docs/index.md` and `README.md:79-89`. Add one sentence that points to `docs/patch.md` to `README.md` "How Does it Work?" and to `docs/workflow.md`
   - [ ] 2.5 Add `/speq:patch` → `.speq/patch-hook.md` to the table in `docs/hooks.md:21-27`, and change "7 entry-point skills" to 8 (`:29`)
   - [ ] 2.6 Add a bullet to `docs/faq.md` "How do I fix a bug?" that points to `/speq:patch` for a small fix
3. Verification
   - [ ] 3.1 Run the Checklist and the Manual Testing commands below
5. Remove the baseline run
   - [ ] 5.1 In `.claude/skills/speq-patch/SKILL.md`, apply the Requirements rows Test and Report and decision-log [4]. Rename step 4 to `### 4. Branch`. Delete the step 4 item 2 sentence "A resumed run reuses the item 3 baseline logs." and step 4 item 3. Set step 5 item 2 to "2. Run the suites and the lint command with logs per `/speq-implement-pr` step 5, which judges green or red by exit code. Every failure counts as caused by the patch. Red → fix and rerun. After 3 red runs of the same suite, stop and report the suite, its failing tests, at most `tail -n 30` of its log, and the branch state." In step 8, replace "A failure beyond the baseline → stop and report as in step 5." with "Red → stop and report as in step 5." In step 11, delete "the baseline failures, ". These edits remove the text that `tasks.md` § Phase 4 tasks 4.6 to 4.8 added. Keep the file at most 8000 bytes (`wc -c`)
   - [ ] 5.2 In `.claude/skills/speq-patch/references/pr-body-template.md`, set the skeleton's `Verification` line to the one in "PR body" above. In § Rules, delete "Add every baseline failure from the run on the new branch."
   - [ ] 5.3 In `docs/patch.md`, delete the baseline bullet at `:22`. After the TDD bullet, add "Runs the test suites and the lint command after the change and fixes every failure. After 3 red runs of the same suite, it stops and reports the failing tests". § Resume holds no baseline text and stays unchanged
   - [ ] 5.4 Run `grep -rni baseline .claude/skills/speq-patch/ docs/patch.md` (no output) and `wc -c .claude/skills/speq-patch/SKILL.md` (at most 8000). Rerun the Checklist and the Manual Testing rows Plugin packaging, Command rename, and Codex name. Record the interactive Manual Testing rows (Claude load through Hook) as not run in `verification-report.md`

## Parallelization

| Group | Tasks | Depends on | Knowledge |
|-------|-------|------------|-----------|
| A: patch skill | 1.1-1.5 | — | `plan.md` Requirements; `.claude/skills/speq-implement-pr/SKILL.md`, `.claude/skills/speq-git-operations/SKILL.md`, `.claude/skills/speq-plan-pr/SKILL.md`, `.claude/agents/code-reviewer.md`, `.claude/skills/speq-code-review/`, `.claude/skills/speq-adr-rules/SKILL.md`, `docs/decision-log.md` |
| B: packaging and docs | 2.1-2.6, 3.1 | A (docs describe `.claude/skills/speq-patch/SKILL.md`) | `.claude/skills/speq-patch/SKILL.md`; `scripts/plugin/build.sh`, `scripts/release/test.sh`, `tests/docker/test-codex-plugin.sh`, `docs/`, `README.md` |
| C: baseline removal | 5.1-5.4 | A, B (edits files they wrote) | `plan.md` Requirements rows Test and Report, "PR body", decision-log [4]; `.claude/skills/speq-patch/SKILL.md`, `.claude/skills/speq-patch/references/pr-body-template.md`, `docs/patch.md` |

## Dead Code Removal

| Type | Location | Reason |
|------|----------|--------|
| Skill text | `.claude/skills/speq-plan-pr/SKILL.md:85-90` | PR-title derivation moves to `/speq-git-operations` (task 1.3) |
| Skill text | `.claude/skills/speq-implement-pr/SKILL.md:59`, the bump rule | Moves to `/speq-git-operations` (task 1.3) |
| Skill text | `.claude/skills/speq-patch/SKILL.md` step 4 item 3 and the baseline sentences in step 4 item 2, step 5 item 2, step 8, and step 11, added by `tasks.md` § Phase 4 tasks 4.6 to 4.8 | No baseline run (decision-log [4], task 5.1) |
| Template text | `.claude/skills/speq-patch/references/pr-body-template.md:14,22`, the baseline failures | No baseline run (task 5.2) |
| Doc text | `docs/patch.md:22`, the baseline bullet | No baseline run (task 5.3) |

## Verification

### Scenario Coverage

No spec delta, so no scenario maps to a test. The packaging tests below are the automated proof: `scripts/release/test.sh` and `tests/docker/test-codex-plugin.sh` fail when `skills/patch/SKILL.md` is missing from the Claude or Codex plugin.

### Manual Testing

| Feature | Command | Expected Output |
|---------|---------|-----------------|
| Plugin packaging | `./scripts/plugin/build.sh && ls dist/marketplace/plugins/speq-skill/skills/patch/SKILL.md dist/marketplace/codex/plugins/speq-skill/skills/patch/SKILL.md` | Both files listed |
| Command rename | `grep -c -e '^/speq-' -e '[^A-Za-z0-9_./-]/speq-' dist/marketplace/plugins/speq-skill/skills/patch/SKILL.md` | `0`: no slash command left unrenamed. Paths such as `target/speq-<suite>.log` do not match |
| Codex name | `grep -e '^name:' -e '^model:' dist/marketplace/codex/plugins/speq-skill/skills/patch/SKILL.md` | `name: speq:patch` and no `model:` line |
| Claude load | `claude --plugin-dir dist/marketplace/plugins/speq-skill`, then type `/speq:patch` | The skill is listed with its description |
| Fix run | In a throwaway clone of a speq project with a GitHub remote, run `/speq:patch fix <symptom>` | Branch `patch/fix-<scope>`; suite logs under `target/`; one `code-reviewer` spawn; version bumped by one patch level; ready PR with the PR body sections; no `specs/_plans/` directory |
| Spec change run | In the same clone, run `/speq:patch change <behavior no scenario covers>` | One scenario added, with a test in the diff; `speq feature validate` and `speq decision-log validate` pass; the second commit is `spec(<scope>): <patch-name>`; `specs/_plans/` stays absent |
| Size warning, point 1 | In the same clone, run `/speq:patch add <capability that needs a new domain>` | The skill names the exceeded limit and asks to continue before a branch exists |
| Size warning, point 2 | In the same clone, run `/speq:patch` on a change of more than 400 lines | The point-2 warning names the measured value before any push |
| Uncommitted edit | Start `/speq:patch fix <symptom>` with an uncommitted edit in the working tree | The skill asks whether the edit belongs to the patch before it creates the branch |
| Hook | Add `.speq/patch-hook.md`, run `/speq:patch fix <symptom>` | First output line: `Loaded project hook: .speq/patch-hook.md` |

### Checklist

| Step | Command | Expected |
|------|---------|----------|
| Build | `cargo build --release` | Exit 0 |
| Test | `cargo test` | 0 failures |
| Lint | `cargo clippy` | 0 errors or warnings |
| Format | `cargo fmt --check` | No changes |
| Plugin build | `./scripts/plugin/build.sh` | Exit 0 |
| Release artifact | `./scripts/release/build.sh v<version> && ./scripts/release/test.sh v<version>`, `<version>` from `get_version` in `scripts/lib/version.sh` | Exit 0, `OK skills/patch/SKILL.md` and `OK codex/skills/patch/SKILL.md` |
| Docker | `./tests/docker/test-docker.sh` | `PASS: Codex skills exist` |
