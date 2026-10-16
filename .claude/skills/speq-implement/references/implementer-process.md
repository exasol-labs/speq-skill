# Implementer Process

Shared by `implementer-agent` and `implementer-expert-agent`. The orchestrator inlines this file as the `## Process` section of every implementer brief. It owns the steps common to both agents; each agent file owns only its routing-specific parts (fix-task mode and output format).

## Required Skills

Load these three before any implementation work. They govern every task:
- `/speq-code-tools`: semantic code navigation and editing
- `/speq-code-guardrails`: TDD workflow and guardrails
- `/speq-minimal-change`: the smallest change that fully does the job

Load a skill from this table only when its trigger holds for the task in hand. A skill you do not need is a turn you do not spend:

| Skill | Load when |
|-------|-----------|
| `/speq-ext-research` | The task needs a library or API whose contract the repo does not show |
| `/speq-design-philosophy` | The task introduces or changes a module seam, public interface, or data format |
| `/speq-cli` | You need spec text the brief's `Knowledge:` entry does not name |

**Git history is read-only for you.** Do not run `git commit`, `git checkout`, `git reset`, `git stash`, `git rebase`, or any other history write; the orchestrator and the headless pipeline own those. Read-only git (`status`, `diff`, `log`, `show`) is fine. Files you write under `specs/_plans/<plan_name>/notes/` are local working state and are never committed. This rule replaces loading `/speq-git-discipline`.

## For Each Assigned Task

### 1. Orient

If the brief has an `Orientation:` line, read that hand-off note first: it is your predecessor's mental model of this group. If the brief has a `Knowledge:` line, read the spec deltas and files it names next. These two lines replace a cold search.

### 2. Fill Gaps Only

Task details are already inline in `## Your Tasks`. If the brief has a `## Rationale` section, treat it as a verbatim excerpt from `decision-log.md` and do not re-read the log for it. Open `specs/_plans/{plan_name}/plan.md` or `specs/_plans/{plan_name}/decision-log.md` yourself only when you need context the brief does not cover (for example another group's decisions).

### 3. Search Specs

Use these for gaps the `Knowledge:` entry does not cover:

```bash
speq search query "<relevant terms>"
speq feature get '<domain>/<feature>/<scenario>'
```

### 4. TDD Cycle

Follow the LIST → RED → GREEN → REFACTOR → REVISE cycle per `/speq-code-guardrails`. An expert task targets the invariants, failure modes, concurrent interactions, and edge cases that would break a naive implementation, not only the happy path.

**Run only the test you changed.** `cargo test --test <binary>`, `pytest <file>::<test>`, or the project's equivalent single-test invocation. Never run the full suite in this loop: the orchestrator runs it once in Phase 5, and a full cycle per task costs more than the implementation. If the project offers no single-test invocation, run the smallest target that contains the test.

### 5. Report Progress

Do not edit `specs/_plans/{plan_name}/tasks.md`: during implementation the orchestrator is its only writer, so a concurrent wave has no write conflicts. Report each task's outcome in your return instead, in the Output Format your agent file specifies. Name every task you did not complete, with the state it is in.

## Early Termination

If context is running low or you have hit your task limit, write a hand-off note to `specs/_plans/{plan_name}/notes/<group-letter>.md` (create the directory if absent). `<group-letter>` is the token before the `:` in the group's Parallelization-table `Group` entry (group `A: plan-log validation` gives `notes/A.md`), never the full group name and never a slug of it. Keep it under one page under five headings: files that matter, invariants established, conventions observed, test list, dead ends. Under `test list`, record the items not yet crossed off, in order. Your successor reads this note instead of rebuilding your mental model from cold files. Then return:

```
ROTATION NEEDED

Progress at termination:
- X.1: completed
- X.2: in progress (describe state)

Remaining tasks:
- X.3: <task>

State: tasks.md is up to date
Hand-off: specs/_plans/{plan_name}/notes/<group-letter>.md
```

## Scope Constraints

- Implement only the tasks in your assignment. In an expert group, the untagged tasks in the list are part of that assignment.
- Leave `tasks.md` to the orchestrator during implementation. Fix-Task Mode is the one exception: no other agent is in flight then, so you append your own `## Phase 4: Review Fixes` lines and mark them.
- Keep every change inside the spec deltas named in your brief.
- Never edit a `## PR Lifecycle` section in `tasks.md`: it is the headless pipeline's checkpoint, not a work item.
- Leave unrelated code unchanged.
