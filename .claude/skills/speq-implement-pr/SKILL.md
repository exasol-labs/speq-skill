---
name: speq-implement-pr
description: "Builds a planned feature on its feat/plan-name branch as a follow-up to /speq-plan-pr: implements, bumps the version, runs the real test suites, records the specs if green, comments the verification, and marks the PR ready. Run again to resume. Arg: plan name, PR number, or branch name."
---

# Spec Implementer, PR flow

## User Arguments

$ARGUMENTS

You run on top of `speq-implement`. Drive a plan on its existing `feat/<plan-name>` branch to a ready PR through the existing skills, run unchanged:
- `/speq-implement` builds and verifies. `/speq-record` merges the specs. You run every git/`gh` action yourself, per `/speq-git-operations`.
- Record only after the project's real test suites are green.
- Stop on the first failed step and report it. Running the skill again resumes the work.

## Required Skills

Invoke before starting:
- `/speq-cli`: spec discovery, to resolve plan names
- `/speq-writing-guardrails`: prose style for artifacts and GitHub text
- `/speq-git-operations`: the git/gh operation-to-command mapping, safety rules, and return formats. You run every operation directly

`speq-implement` and `speq-record` invoke their own required skills.

## Workflow

### 1. Resolve Target

`$1` empty → ask the user which plan. Otherwise check out the target (for a bare plan name, that is its `feat/<plan-name>` branch):

```
Run — operation: checkout (per /speq-git-operations)
  target: <$1: a plan name (→ feat/<plan-name>), PR number, or branch name>
```

If `checkout` reports not-found (the plan exists only locally and was never pushed):

```
Run — operation: create-branch (per /speq-git-operations)
  branch: feat/<plan-name>
```

If `specs/_recorded/*-<plan-name>/` exists and `specs/_plans/<plan-name>/` does not, the plan is already recorded: skip to step 6.

### 2. Implement

If `verification-report.md` exists in the plan directory and every task in `tasks.md` is checked, skip this step.

Otherwise invoke `/speq-implement <plan-name>` and let it run to completion. It resumes from the first unchecked task.

### 3. Bump Version

Skip this step if the version file already differs from the default branch.

Read the tech stack in `specs/mission.md` and bump the version in that stack's version file (for example `Cargo.toml` for Rust, `package.json` for Node, `pyproject.toml` for Python). Apply the next version per `/speq-git-operations` § PR title and version bump. Do not take the version from the plan's spec deltas. Run a build to keep the lockfile in sync, per `/speq-code-guardrails` § Running Suites.

### 4. Commit Evidence

Commit and push now, so the evidence artifacts reach git history before `/speq-record` archives the plan out of tracked space, and so the branch survives workspace loss. Derive `<type>(<scope>)` per `/speq-git-operations` § PR title and version bump:

```
Run — operation: commit (per /speq-git-operations)
  paths: implementation files, version bump, and the whole specs/_plans/<plan-name>/
         directory, not an itemized subset
  message: <type>(<scope>): implement <plan-name>

Run — operation: push (per /speq-git-operations)
```

### 5. Test and Record

1. **Run the suites.** Run the project's real test suites (per `specs/mission.md § Commands`, typically an integration suite and an end-to-end suite), per `/speq-code-guardrails` § Running Suites. If any suite is red, report the failed suites with their log tails, **stop**, and leave the plan unrecorded. After the user fixes the code, run this skill again.
2. **Record.** Invoke `/speq-record <plan-name>`. Recording accepts the ADR candidates on the plan PR. If it stops at a library threshold, relay its question to the user and **stop**. If it stops on a failure (for example a stale architecture delta base), **stop** and report it. Run this skill again after the user decides or fixes the plan.
3. Note the archive path from `/speq-record`'s `✓ Plan archived:` line.

### 6. Ship

1. **Ship.** One composite call. Step 4 already committed the implementation and evidence, so this commit covers only what `/speq-record` produced: the merge results and the archive's removal of the plan directory. Derive `<type>(<scope>)` and `<slug>` per `/speq-git-operations` § PR title and version bump. Do not rewrite the PR body `speq-plan-pr` set. Use `Implements <plan-name>.` as the body, which applies only if no PR exists yet:

   ```
   Run — operation: ship-ready (per /speq-git-operations)
     paths: permanent-spec merges (specs/<domain>/...), specs/architecture.md,
            specs/_decision/ additions, deletion of specs/_plans/<plan-name>/
     message: <type>(<scope>): record <plan-name>
     title: <type>(<scope>): <slug>
     body: Implements <plan-name>.
   ```

   `ship-ready` reuses the draft PR `speq-plan-pr` opened (or opens one if the plan was only implemented locally) and marks it ready. A commit with nothing to commit is a no-op, so running this step again is safe.

2. **Comment.** Post the verification summary. Build it from `<archive-path>/verification-report.md`, where `<archive-path>` is the archive path from step 5 or, when step 1 skipped ahead, the match of `specs/_recorded/*-<plan-name>/`. Compose it per `/speq-writing-guardrails` and `/speq-implement`'s `references/implementation-summary-template.md`:

   ```
   Run — operation: comment-pr (per /speq-git-operations)
     body: <the composed verification summary>
   ```

3. Report the finished PR and leave it for the user's review. Never merge it.
