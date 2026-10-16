---
name: speq-implement
description: "Orchestrate implementation of a reviewed plan: task breakdown, TDD sub-agents, code review, and verification report. Use when the user asks to implement, build, or execute a plan under specs/_plans/, after /speq-plan and before /speq-record. Arg: <plan-name>."
---

# Spec Implementer (Orchestrator)

Orchestrate implementation of the plan in `specs/_plans/<plan-name>`. Get the plan name from the user prompt. If none is given, ask.

Sub-agents do the heavy work. Each pins its own model and effort in its frontmatter:

| Sub-agent | When used |
|-----------|-----------|
| `implementer-agent` | Groups with no `[expert]` task (default) |
| `implementer-expert-agent` | Groups with at least one `[expert]` task |
| `code-reviewer` | Final review of all changed files |

## Required Skills

Invoke before starting:
- `/speq-cli`: spec discovery
- `/speq-writing-guardrails`: prose style for artifacts and GitHub text

Do not invoke coding skills (`/speq-code-tools`, `/speq-ext-research`, `/speq-code-guardrails`) yourself: the orchestrator never writes code. Sub-agents invoke their own required skills.

## Orchestrator Role

- Create and maintain `tasks.md` for persistence.
- Spawn sub-agents for parallel task groups.
- Update task status after each sub-agent completes.
- Never implement directly. Delegate all coding work.
- Rotate sub-agents to keep context windows fresh.
- Update `tasks.md` before the runtime task, so the persistent state is never behind. Mark a wave `[~]` before spawning it and apply its results after it returns.
- If a sub-agent fails, keep its tasks `[~]` and report the error. Never mark `[x]` while a test fails.

**Rotation rule:** when a sub-agent has completed `max_tasks_per_agent` (default 10) tasks, or returns `ROTATION NEEDED`: read tasks.md for current state, note the completed tasks from the return, and spawn a fresh agent of the SAME type with the remaining tasks. Repeat until the group is complete. Prefer handing the group's next task to the same child when the platform keeps children alive: a respawn re-pays the cold start, and the child still holds the group's mental model.

**Rotation hand-off:** the outgoing agent writes a hand-off note to `specs/_plans/<plan-name>/notes/<group-letter>.md` (its own duty, per the `## Process` section of its brief). `<group-letter>` is the group's letter only — the token before the `:` in its Parallelization-table `Group` entry (group `A: plan-log validation` → `notes/A.md`), never the full group name and never a slug of it. Add one line to the fresh agent's brief: `Orientation: read specs/_plans/<plan-name>/notes/<group-letter>.md first`. If the note is absent, omit the line. The note is working state inside the plan directory. It is never committed evidence, and it moves into the archive with the plan directory.

## Workflow

### Phase 0: Load Project Hook (orchestrator)

Load `.speq/implement-hook.md` per `/speq-plan`'s `references/hook-loading.md`. Pass its path (not its content) as a `Project Hook:` line to every sub-agent brief below; those agents read the hook themselves from that path.

### Phase 1: Load Plan

**Open-questions gate:** read `specs/_plans/<plan-name>/open-questions.md`. If it exists and is non-empty: **stop** and report that the plan has unresolved open questions pending human answers (resolve with `/speq-plan <plan-name>`, or via PR comments and `/speq-plan-pr <plan-name>` for a headless plan). This is the human-in-the-loop point, the same gate as `speq-implement-pr`'s blocker check.

```
Read: specs/_plans/<plan-name>/plan.md
Read: specs/_plans/<plan-name>/decision-log.md
```

Extract: feature specs, implementation tasks, parallelization groups, verification commands from plan.md, and the `## Design Decisions` entries from decision-log.md.

### Phase 2: Create Tasks

Decompose the plan into a **Work Breakdown Structure** in `specs/_plans/<plan-name>/tasks.md`.

**Lifecycle guard:** if `tasks.md` exists and contains a `## PR Lifecycle` section, `speq-implement-pr` pre-created it as its checkpoint (per its `references/checkpoint-protocol.md`). Preserve that section verbatim at the top and write the `## Phase N` sections below it. Never edit `## PR Lifecycle`: its writers are fixed by that protocol. Standalone runs get no lifecycle section. Create the file as below.

**Format:**

```markdown
# Tasks: <plan-name>

## Phase 2: Implementation (Group A)
- [ ] 2.1 <task from plan>
- [ ] 2.2 <task from plan> [expert]

## Phase 2: Implementation (Group B)
- [ ] 2.3 <task from plan>

## Phase 3: Verification
- [ ] 3.1 Run test suite
- [ ] 3.2 Run linter
```

**Status markers:**
- `[ ]` pending
- `[~]` started
- `[x]` completed

**Difficulty tags:**
- `[expert]`: tagged by `planner-agent` during planning. Routes the task's whole group to `implementer-expert-agent` (Phase 3 routing rule). Preserve the tag through every status transition.
- untagged: a group with only untagged tasks routes to `implementer-agent`.

If the plan tagged no tasks and a task clearly needs expert reasoning (for example concurrency, a cross-file refactor, a novel algorithm), you MAY add `[expert]` when materializing tasks.md. Tag sparingly: over-tagging wastes tokens.

Also create runtime tasks:
```
For each task in tasks.md:
  TaskCreate(subject, description, activeForm)
```

### Phase 3: Implement (Orchestrated)

**Waves.** Independent groups run concurrently. A group is ready when every group its `Depends on` column names has finished. Take all ready groups as one wave and run the wave's agents in parallel; a group's dependency puts it in a later wave. When the platform caps concurrency, chunk a large wave and keep the chunks in flight back to back.

**Single writer.** During Phase 3 the orchestrator is the only writer of `tasks.md`. Sub-agents report their per-task status in their return and never edit the file. This is what makes a concurrent wave safe in one working tree.

For each wave:

1. **Route each group by its hardest task.** Any `[expert]` task routes the whole group to `implementer-expert-agent`. Otherwise the whole group goes to `implementer-agent`. One agent per group. Never split a group by tag: a group is one knowledge cluster, and each extra agent rebuilds the same mental model, which costs more than the model-price difference.
2. **Mark the wave started**: update tasks.md, `[ ]` → `[~]`, for every task in the wave.
3. **Spawn one sub-agent per group in the wave, in one parallel batch.** Issue every delegate call of the wave together (one parallel or background delegate call, or the platform's equivalent) and only then wait. Never await one group before spawning the next: that is the serialization this phase exists to avoid. Give each child a wall-clock limit and its acceptance criteria when the platform supports them. Per child:
   - If the plan's Parallelization table has a `Knowledge` column, copy the group's entry into the brief's `Knowledge:` line.
   - If `decision-log.md`'s `## Design Decisions` has entries that touch this group's tasks, paste their Decision and Rationale verbatim into the brief's `## Rationale` section. Omit the section entirely when no entry touches the group.
   - Inline the full content of `references/implementer-process.md` as the brief's `## Process` section. The orchestrator already read the plan and its decision log once in Phase 1, so the sub-agent does not reread them for this.
4. **Await the wave**: collect every child's return.
5. **Handle rotation**: apply the Rotation rule and Rotation hand-off above. A rotating group's successor joins the next wave.
6. **Apply reported status**: write each child's per-task result to tasks.md, `[~]` → `[x]` (preserve the `[expert]` tag). A task the child did not report complete stays `[~]`.
7. **Update TaskTools**: `TaskUpdate(taskId, status: "completed")` for each completed task.
8. **Next wave**: once a group's dependencies are complete.

**Build-lock caveat.** Concurrent children in one working tree share one build directory. Two `cargo`-style builds then serialize on the build lock, so the wave waits rather than corrupting. That wait is still shorter than a whole agent run, but when the plan's Verification checklist is the dominant cost, expect the wave to spend its tail there.

**Subagent invocation** (`implementer-agent`, or `implementer-expert-agent` for a group with an `[expert]` task):

```
Delegate to <implementer-agent | implementer-expert-agent>: Implement <group-name>

## Your Tasks (the whole group)

{group_task_list}

## Rationale

<verbatim Decision and Rationale of the decision-log.md ## Design Decisions entries that touch this group's tasks; omit this section if none do>

## Process

<inline the full content of references/implementer-process.md here: required skills, per-task steps, early termination, scope constraints>

## Context

- Plan: specs/_plans/{plan_name}/plan.md
- Tasks file: specs/_plans/{plan_name}/tasks.md
- Knowledge: <the group's Knowledge entry: read these spec deltas and files first; omit if the plan has no Knowledge column>
- Orientation: read specs/_plans/{plan_name}/notes/<group-letter>.md first <rotation respawns only>
- Project Hook: <if active, ".speq/implement-hook.md: read it and apply it"; otherwise omit this line>
```

### Phase 4: Code Review

Review all changed files after implementation completes. Code review runs ONCE per implementation: after fix tasks complete, proceed to Phase 5 (its checks verify the fixes). Do not respawn `code-reviewer` for a second round.

1. **Collect changed files**: `git diff --name-only <base>` for tracked changes plus `git ls-files --others --exclude-standard` for new files. Implementation work is uncommitted at this point, so diff against the working tree. A commit-range diff (`<base>...HEAD`) would miss all of it.
2. **Spawn code-reviewer agent:**
   ```
   Delegate to code-reviewer — Review implementation quality

   ## Changed Files

   {changed_files_list}

   ## Context

   - Plan: specs/_plans/{plan_name}/plan.md
   - Review for: guardrail violations, dead code, test quality, bad comments, optimizations, YAGNI/over-engineering, error handling, design depth
   - Write findings to specs/_plans/{plan_name}/review-findings.md per your output format, partitioned into `## Standard fixes` and `## Expert fixes`; return only the one-line verdict.
   - Project Hook: <if active, ".speq/implement-hook.md — read it and apply it"; otherwise omit this line>
   ```
   It returns one line: `CODE REVIEW: <n> findings — standard: <n>, expert: <n> — <path>`. You never see the findings themselves.
3. **Process findings**: branch on the two counts in the verdict. If both are 0, go to Phase 5. One agent applies the whole fix pass, routed by its hardest finding: the findings cluster on the files just written, and a second agent there re-orients into the same code and can collide with the first.
   - **standard > 0, expert == 0**: spawn `implementer-agent`:
     ```
     Delegate to implementer-agent — Apply standard review fixes

     ## Your Assignment (fix-task mode)

     Read specs/_plans/{plan_name}/review-findings.md, section `## Standard fixes`.
     Append one fix task per finding to specs/_plans/{plan_name}/tasks.md under a
     `## Phase 4: Review Fixes` group, deriving each task line from the finding's
     `Fix:` field, then execute them.

     ## Context

     - Plan: specs/_plans/{plan_name}/plan.md
     - Project Hook: <if active, ".speq/implement-hook.md — read it and apply it"; otherwise omit this line>
     ```
   - **expert > 0**: spawn `implementer-expert-agent` with the same brief shape, naming section `## Expert fixes`, and also `## Standard fixes` when standard > 0. It tags only the tasks derived from `## Expert fixes` with `[expert]`.
4. **Proceed to Phase 5.**

### Phase 5: Verification

#### 5a. Automated Checks

Execute the plan's `## Verification > Checklist` as **one chained call**, redirecting each step's output to its own log:

```bash
mkdir -p target && \
  <build-command> > target/speq-build.log 2>&1 && \
  <test-command>  > target/speq-test.log 2>&1 && \
  <lint-command>  > target/speq-lint.log 2>&1 && \
  <format-command> > target/speq-format.log 2>&1
```

One call, one turn, one log per step. Then branch on the exit code. This is the only full-suite run of the implementation: Phase 3 sub-agents run the single test binary they changed, never the whole suite. If a report quotes output, quote at most `tail -n 30` of the log. Raw build or test output never lands verbatim in the transcript.

- Build → exit 0
- Test → 0 failures
- Lint → 0 errors
- Format → no changes

#### 5b. Scenario Coverage Audit

Cross-reference plan's `## Verification > Scenario Coverage` against test results:

- Every listed scenario → corresponding test exists and passes
- Flag any scenario without a passing test as incomplete

#### 5c. Manual Verification

Execute each step from plan's `## Verification > Manual Testing`:

- Run documented commands against the built software
- Capture actual output as evidence for the verification report
- Mark pass/fail per feature

Update tasks.md verification tasks as completed.

### Phase 6: Verification Report

Generate using `references/verification-template.md`. Structure the report **BLUF (Bottom Line Up Front)**: verdict and summary first, evidence after. Fill the Verdict table's `Code review` row from Phase 4's verdict line: a later session's condensed PR comment quotes the Verdict table, and that row is how the counts survive.

Save to: `specs/_plans/<plan-name>/verification-report.md`

### Phase 7: Completion

Print the summary per `references/implementation-summary-template.md`, built from the `verification-report.md` Phase 6 just wrote.

Append a `## Timing` section to `tasks.md`, one line per phase, so the next plan can see where its wall clock went:

```markdown
## Timing
- Phase 3 (implement): <groups> groups, <waves> waves, <m> min
- Phase 4 (review + fixes): <m> min
- Phase 5 (verification): <m> min
- Total: <m> min
```

End with:

```
Ready for: /speq-record <plan-name>
```

## Context Recovery

If context is lost or compacted:

1. Read `specs/_plans/<plan-name>/tasks.md`
2. Find incomplete tasks (`[ ]` or `[~]`). Scan only `## Phase N` sections, never `## PR Lifecycle` (its unnumbered entries are `speq-implement-pr` checkpoints, not work items)
3. Resume from the first incomplete task. A `[~]` task means its sub-agent was interrupted: respawn that group's agent
4. Continue the orchestration workflow

## References

| File | Use When |
|------|----------|
| `references/implementer-process.md` | Inlined into every sub-agent brief's `## Process` section |
| `references/verification-template.md` | Phase 6 report generation |
| `references/implementation-summary-template.md` | Phase 7 summary |

## Anti-Patterns

| Pattern | Why Wrong |
|---------|-----------|
| Orchestrator invokes coding skills | The orchestrator never writes code, and that text dilutes the most expensive session for work it never does |
| Awaiting one group before spawning the next | Independent groups are the only parallelism the plan gives you; running them one at a time turns N agent lifetimes into the wall clock |
| Letting a Phase 3 sub-agent edit `tasks.md` | A concurrent wave then has several writers on one file; the orchestrator applies the statuses the children report |
| Running the full suite inside a Phase 3 TDD loop | It costs a full build and test cycle per task; the changed test binary answers the RED/GREEN question, and Phase 5 runs the suite once |
| Splitting one group between two agents by tag | The group is one knowledge cluster; each extra agent re-derives the same mental model |
| Proceeding past a non-empty open-questions.md | The plan is blocked on human answers |
| Marking `[x]` without a sub-agent completion return | Only verified completions are done |
| Skipping the verification report | `/speq-record` gates on it |
| Pointing a sub-agent at decision-log.md for rationale instead of inlining the group's excerpt | Full-file reads repeat per sub-agent; the orchestrator already read the decision log once in Phase 1 |
