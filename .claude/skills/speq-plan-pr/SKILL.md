---
name: speq-plan-pr
description: "Plans a feature through the same clarifying interview as /speq-plan, then creates the feat/plan-name branch, commits the plan, and opens a draft pull request. Arg: feature intent text, plan name, PR number, or branch name."
---

# Spec Planner, PR flow

## User Arguments

$ARGUMENTS

This skill runs `/speq-plan` and ships the result as a draft PR.

## Required Skills

Invoke before starting:
- `/speq-cli`: spec discovery
- `/speq-writing-guardrails`: prose style for artifacts and GitHub text
- `/speq-git-operations`: the git/gh operation-to-command mapping, safety rules, and return formats.

## Workflow

### 1. Resolve Target

If the argument is a plan name, PR number, or branch, check it out:

```
Run — operation: checkout (per /speq-git-operations)
  target: <a PR number or branch; for a bare plan name, feat/<plan-name>>
```

- **Found, with a PR:** go to step 3 (revise).
- **Not found, or the argument is feature intent text:** go to step 2 (new plan).

### 2. New Plan

1. Run `/speq-plan` steps 1 to 5: discovery, the clarifying interview, the plan name, plan authoring, and the one review with its fix pass. Skip its step 6.
2. Create the branch. The plan files are uncommitted, so they carry over:
   ```
   Run — operation: create-branch (per /speq-git-operations)
     branch: feat/<plan-name>
   ```
3. Ship the draft, per [Ship](#ship).

### 3. Revise from PR Comments

Run this when step 1 found a PR for the target.

1. Read the feedback:
   ```
   Run — operation: read-comments (per /speq-git-operations)
     since: <timestamp of the last `spec(plan)` commit on this branch>
   ```
2. If no comment is new, report that and stop.
3. Treat every comment as user feedback on the files the PR changed, with its file and line when it has them. Revise the plan per `/speq-plan`'s `references/revision-mode.md`. If a comment is ambiguous, ask the user.
4. Run `/speq-plan` step 5 (the plan review) again only when the revision changed a spec delta or the plan's scope.
5. Commit and push, and answer on the PR:
   ```
   Run — operation: commit (per /speq-git-operations)
     paths: the plan directory
     message: spec(plan): address PR feedback for <plan-name>

   Run — operation: push (per /speq-git-operations)

   Run — operation: comment-pr (per /speq-git-operations)
     body: one line per comment, `Resolved` or `Could not resolve` with the reason
   ```

### Ship

Commit the plan and open the draft PR with one composite call:

```
Run — operation: ship-draft (per /speq-git-operations)
  paths: the plan directory
  message: spec(plan): <plan-name>
  title: <type>(<scope>): <slug>, derived per /speq-git-operations § PR title and version bump
  body: per references/pr-body-template.md, ending "Draft pending implementation."
```

Derive the PR title from `<plan-name>` per `/speq-git-operations` § PR title and version bump.

The PR stays a draft. `speq-implement-pr` is the only skill that marks it ready.

### Report

Give the PR link. Print `plan.md`'s `## Impact` section, the `ADR candidates:` line, and the `Architecture:` line from the PR body. Mention the non-blocking findings from `specs/_plans/<plan-name>/plan-review.md`, read from the file, not from memory. If the fix pass ran, name it in one line ("N review findings fixed"). Tell the user to review the PR, comment on it, and run `/speq-plan-pr <PR number>` again to apply the comments, or `/speq-implement-pr <plan-name>` to build.
