---
name: speq-implement
description: "Implements a reviewed plan: TDD through a task list it derives from the plan, one fresh-context code review, and a verification report. Use to build or execute a plan under specs/_plans/, after /speq-plan and before /speq-record. Arg: plan-name."
---

# Spec Implementer

## User Arguments

$ARGUMENTS

Implement the plan in `specs/_plans/<plan-name>`. Get the plan name from the user prompt. If none is given, ask. You run no git write: the user commits, or `/speq-implement-pr` does. A code reviewer with a fresh context reviews the result.

## Required Skills

Invoke before starting:
- `/speq-cli`: spec discovery
- `/speq-code-guardrails`: TDD
- `/speq-design-philosophy`: module depth and boundaries in the code you write
- `/speq-writing-guardrails`: prose style for the report

## Workflow

### Phase 1: Load Plan

Read `specs/_plans/<plan-name>/plan.md` and `intent.md`. The spec deltas they reference are the requirements. Note the merge base: `git rev-parse HEAD`.

### Phase 2: Implement

`specs/_plans/<plan-name>/tasks.md` is the task list. It exists to enable resume. Each task is a checkbox: `- [ ]` pending, `- [x]` done.

1. If `tasks.md` is absent, write it: break the spec deltas, Dead Code Removal, and Verification in `plan.md` into tasks in implementation order. Each task is small enough to verify as one unit and traces to one of them.
2. Mirror the unchecked tasks with TaskCreate, so progress is visible in the session.
3. Work through the tasks in order. For each task: TDD per `/speq-code-guardrails`, then check it off in `tasks.md` (`- [ ]` → `- [x]`) and mark the runtime task completed.
4. Never check a task while a test it touches fails. If you cannot finish a task, leave it unchecked and report why.

### Phase 3: Code Review

Code review runs ONCE per implementation.

1. **Collect changed files**: `git diff --name-only <base>` for tracked changes plus `git ls-files --others --exclude-standard` for new files. Leave out the plan directory. Work is uncommitted, so diff against the working tree. A commit-range diff (`<base>...HEAD`) would miss all of it.
2. **Spawn the code reviewer** as a sub-agent.
   Set its model to `opus`. <!-- claude-only -->
   Skip this if `code-review.md` already exists: a stopped run already reviewed. Prompt:
   ```
   Invoke /speq-code-review and follow it.

   ## Changed Files
   {changed_files_list}

   ## Context
   - Plan: specs/_plans/{plan_name}/plan.md
   - Write findings to specs/_plans/{plan_name}/code-review.md; return only the one-line verdict.
   ```
   It returns `CODE REVIEW: <n> findings — <path>`.
3. **Fix every finding** in `code-review.md`, TDD where behavior changes. Do not review again: Phase 4 verifies the fixes.

### Phase 4: Verification

#### 4a. Automated Checks

Execute commands from plan's `## Verification > Checklist`. Redirect each command's output to a log: `mkdir -p target && <command> > target/speq-<suite>.log 2>&1`, then branch on the exit code. If a report quotes output, quote at most `tail -n 30` of the log.

- Build → exit 0
- Test → 0 failures
- Lint → 0 errors
- Format → no changes

#### 4b. Scenario Coverage Audit

Cross-reference plan's `## Verification > Scenario Coverage` against test results. Every listed scenario needs a passing test. Flag any scenario without one as incomplete.

#### 4c. Manual Verification

Execute each step from plan's `## Verification > Manual Testing` against the built software. Capture actual output as evidence. Mark pass/fail per feature.

### Phase 5: Verification Report

Generate using `references/verification-template.md`, BLUF: verdict and summary first, evidence after. Fill the Verdict table's `Code review` row from Phase 3: a later session's condensed PR comment quotes the Verdict table.

Save to: `specs/_plans/<plan-name>/verification-report.md`

### Phase 6: Completion

Print the summary per `references/implementation-summary-template.md`, built from the `verification-report.md` Phase 5 just wrote, ending:

```
Ready for: /speq-record <plan-name>
```

## Resume

Running again resumes: start at the first unchecked task in `tasks.md`. With every task checked, continue at Phase 3.

## Anti-Patterns

| Pattern | Why Wrong |
|---------|-----------|
| Checking a task while its tests fail | Only verified work is done |
| A second code review after the fixes | One review; verification checks the fixes |
| Skipping the verification report | `/speq-record` gates on it |
