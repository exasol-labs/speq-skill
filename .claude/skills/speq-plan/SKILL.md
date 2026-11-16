---
name: speq-plan
description: "Plans a feature or change through a clarifying interview, producing spec deltas, plan.md, and intent.md with adversarial review. Use to plan, spec, design, or scope a feature, change, removal, refactor, or fix, before any implementation."
---

# Spec Planner

## User Arguments

$ARGUMENTS

Run the clarifying interview, write the plan, then have a plan reviewer with a fresh context challenge it.

## Required Skills

Invoke before starting:
- `/speq-cli`: spec discovery and validation
- `/speq-code-tools`: codebase exploration
- `/speq-design-philosophy`: module boundaries, decision ownership, and dependency direction for the implementation design
- `references/decision-log-template.md`: the ADR rules. Default is no ADR
- `/speq-writing-guardrails`: prose style
- `/speq-ext-library`: on demand, for current library docs

## Guiding Principles

- **BDD (Gherkin syntax)**: scenarios use GIVEN/WHEN/THEN; integration tests by default, unit tests only for isolated pure computation
- **RFC 2119 keywords**: THEN steps use MUST, MUST NOT, SHALL, SHALL NOT, SHOULD, SHOULD NOT, MAY (uppercase)

## Rules

- **Read-only investigation**: planning reads existing code, tests, specs, and docs. It never builds the software, runs a test suite, starts Docker or an e2e run, or launches the software. Plan the commands to run; do not run them.
- **Short Background**: `## Background` is a concise summary of the context every scenario shares, 3-5 one-line bullets at most. It never encodes features, behavior, or requirements; each of those is a scenario (or a step in one). When a change adds behavior, add or change a scenario, not a Background line. When you rewrite a Background that has grown past this, cut it down in the same `DELTA:CHANGED` block.
- **Architecture by delta**: never edit `specs/architecture.md`; `/speq-record` merges the delta. When the plan changes no architecture, write no delta.
- **Proof is tests and manual runs**: `speq` commands (`speq plan validate`, `speq feature get`) check the plan, not the product, so never list them as test or manual evidence.
- **One copy**: never embed spec content in `plan.md` or `intent.md`; reference the delta files.
- **Specs describe the software, not its tests**: a spec delta states features, behavior, and constraints of the software. Never encode e2e tests, test suites, Docker setups, or the test environment as scenarios or Background. `specs/mission.md` encodes how to test; reference it from the plan's Verification section.
- **The user decides, the plan states its directions**: `## Interview` holds only what the user said. Every choice the plan makes without a user answer is an entry in `## Directions`, which the user reviews in step 6. Never bury a choice in a delta or a task.
- **Specs first, tasks last**: write the spec deltas before anything else in the plan, and the task list after everything else.

## Workflow

### 1. Discovery

```bash
speq domain list
speq feature list
grep -rhE -A3 '^## ADR:|^\*\*(ID|Status):\*\*' specs/_decision/ 2>/dev/null || echo none   # ADR digest, per /speq-cli
```

Name the features that relate to the request, per `/speq-cli`'s absence check. Collect only enough context for good interview questions, not a full exploration.

### 2. Clarifying Interview

Apply the Socratic Method by asking the user. Never assume. Decompose the problem space with MECE partitioning:

- **Probe**: surface hidden assumptions with open-ended questions
- **Partition**: present alternative solutions as MECE options
- **Challenge**: test design tradeoffs with counterexamples

**Gap hunt.** The goal of the interview is to find what the request leaves open, including what the user has not thought of. Before you close it, check each lens below against the request and the discovery results. Ask about every lens that applies and that the request does not settle. Skip a lens that does not apply.

- **Who and what is affected**: users, callers, and existing features that touch the change
- **Conflicts**: existing specs, ADRs, or behavior the request contradicts or changes
- **Edge cases and failures**: empty, invalid, or boundary input, and what happens on error
- **Boundaries**: what is explicitly out of scope
- **Compatibility**: breaking changes, migration of existing data or specs
- **Done**: how the user will judge that the change works

**Split check.** If the work spans more than one independent phase, or more than about 4 features, propose one plan per phase. Plan the first phase now. End its `plan.md` Context with a `Follow-up plans` list (name and one-line scope each).

Keep each exchange as one line: `- Q: <question> → A: <answer>`, a few words each.

### 3. Plan Name

Pattern: `<verb>-<feature-scope>[-<qualifier>]`

| Verb | When |
|------|------|
| `add` | New feature |
| `change` | Modify existing |
| `remove` | Deprecate/delete |
| `refactor` | Restructure, same behavior |
| `fix` | Bug or spec mismatch |

Once the name is set, write `specs/_plans/<plan-name>/intent.md` with the H1 `# Intent: <plan-name>` and an `## Interview` section holding the step 2 lines. This is the only copy of the interview.

### 4. Author the Plan

Write the artifacts in this order. Do not start a step before the previous one is done.

1. **Read the context.** `speq feature get <domain>/<feature>` for each related feature. If `specs/architecture.md` exists, read the sections the plan touches. Check the plan against every accepted ADR in the digest, per ADR rule 4 in `references/decision-log-template.md`; read the full fragment of any ADR the plan touches. Before you author a NEW feature or state that no spec covers a behavior, run `/speq-cli`'s absence check.

2. **Spec deltas.** For each feature in scope, write `specs/_plans/<plan-name>/<domain>/<feature>/spec.md`:
   - The feature exists → DELTA markers per `references/delta-template.md`.
   - It does not → a full spec per `references/feature-template.md`.

   **Prose drift check**: after a scenario delta on an existing feature, check that feature's `## Background` and `# Feature: <name>` description. When the scenario change makes either inaccurate, add a `DELTA:CHANGED` block for that section too.

3. **Architecture delta.** Shape every new module, interface, or boundary per `/speq-design-philosophy`'s Quick Diagnostic. If the plan changes a component, boundary, interface, data flow, constraint, or external dependency, write `specs/_plans/<plan-name>/architecture.md` per `references/architecture-delta-template.md`. Copy each changed section from `specs/architecture.md`, edit it, and set the BASE hash with `git hash-object specs/architecture.md`. If `specs/architecture.md` is absent, write no delta.

4. **Validate the deltas**: `speq plan validate <plan-name>`. Fix failures before you go on. Common fixes: close delta markers (`<!-- /CHANGED -->`, `<!-- /NEW -->`, `<!-- /REMOVED -->`), uppercase RFC 2119 keywords, bold step keywords (`*GIVEN*`, `*WHEN*`, `*THEN*`, `*AND*`).

5. **Directions.** Add every choice you made without a user answer to `## Directions` in `intent.md`, per `references/intent-template.md`. Gaps you found while authoring land here too. Keep the section short: the template's rules say what earns an entry.

6. **`plan.md` body.** Write every section of `references/plan-template.md` except `## Implementation Tasks`:
   - Context, Features (referencing the delta files), and Impact (breaking changes called out, or "None").
   - Dead Code Removal: existing code and tests the change or the new tests make redundant.
   - Verification. Scenario Coverage maps each new or changed scenario to an integration test (file path + test name), one test per scenario by default. Manual Testing gives concrete commands against the built software and their expected output, one per feature. Checklist commands come from `specs/mission.md`.

7. **`## Implementation Tasks`, last.** Derive the tasks from the finished deltas, Dead Code Removal, and Verification, in implementation order, as `- [ ]` checkboxes. Every task traces to one of them.

8. **Final validation**: `speq plan validate <plan-name>`.

### 5. Adversarial Plan Review

Spawn the plan reviewer once as a sub-agent. There is one review round.
Set its model to `opus`. <!-- claude-only -->

Prompt:

```
Invoke /speq-plan-review and follow it.

## Plan Name
<plan-name>

## User Intent
<verbatim original request>

## Clarifying Interview Results
Read `specs/_plans/<plan-name>/intent.md`.

## Plan Artifacts
plan.md, intent.md, every specs/_plans/<plan-name>/**/spec.md delta, and specs/_plans/<plan-name>/architecture.md when present
```

It writes `specs/_plans/<plan-name>/plan-review.md` and returns `PLAN REVIEW: BLOCKERS: <n>, FINDINGS: <n>, INTENT: <n> — <path>`. `INTENT` counts the BLOCKERs on the Intent Fidelity axis alone.

- **`INTENT > 0`:** the reviewer holds that the plan solves a different problem than the one asked. Ask the user about those findings before any fix. The user accepts the plan as-is, or gives guidance and you revise.
- **`INTENT == 0` and `BLOCKERS > 0`:** fix the blockers per `references/revision-mode.md`. Do not review again.
- **Any blocker you could not resolve, or a failed `speq plan validate`:** ask the user, one line of context each plus a pointer to the review file.
- **Non-blocking findings** never loop and are never persisted. Carry them into step 7's report. An `[UNSTATED_ASSUMPTION]` or `[INTENT_GAP]` finding becomes an entry in `## Directions`, whatever its severity.

### 6. Review Directions

Present every direction to the user, grouped by topic, each with what it affects in the plan. Ask which ones to change. For each correction, rewrite the direction with the user's answer, revise the deltas, `plan.md`, and tasks it affects, and run `speq plan validate <plan-name>`. A direction the user does not correct stands.

### 7. Explain Next Steps

Print to the terminal:

- `plan.md`'s `## Impact` section
- `ADR candidates: none`, or each `ADR: yes` direction
- The non-blocking findings from step 5
- The number of directions the user corrected in step 6
- If step 5 ran a fix pass, one line ("N review findings fixed")
- Tell the user to run `/speq-implement <plan-name>` to continue, after `/clear` for a fresh context window

## Spec Hierarchy (reference)

```
specs/
├── <domain>/<feature>/spec.md     # Permanent
├── _plans/<plan-name>/            # Active
└── _recorded/<plan-name>/         # Archived
```

## Anti-Patterns

| Pattern | Why Wrong |
|---------|-----------|
| Skipping the clarifying interview | Content comes from user answers, never assumptions |
| Deciding a gap silently | The user never sees the choice; it surfaces as a defect after implementation |
| Writing tasks before the deltas exist | The tasks then describe a design the specs do not hold |
| Running a second review after the fix pass | One review, one fix pass; unresolved findings go to the user |
| Persisting non-blocking findings or looping on them | Report-only; they never gate |
