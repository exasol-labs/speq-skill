---
name: speq-plan
description: "Plans a feature or change through a clarifying interview, producing spec deltas, plan.md, and decision-log.md with adversarial review. Use to plan, spec, design, or scope a feature, change, removal, refactor, or fix, before any implementation."
---

# Spec Planner

Run the clarifying interview, write the plan yourself, then have `plan-reviewer` challenge it with a fresh context. `plan-reviewer` is the only sub-agent.

## Required Skills

Invoke before starting:
- `/speq-cli`: spec discovery and validation
- `/speq-code-tools`: codebase exploration
- `/speq-design-philosophy`: design principles for the decisions in `decision-log.md`
- `/speq-adr-rules`: when a decision becomes an ADR. Default is no
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
- **One copy**: never embed spec content in `plan.md` or `decision-log.md`; reference the delta files.
- **Specs describe the software, not its tests**: a spec delta states features, behavior, and constraints of the software. Never encode e2e tests, test suites, Docker setups, or the test environment as scenarios or Background. `specs/mission.md` encodes how to test; reference it from the plan's Verification section.
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

Once the name is set, write `specs/_plans/<plan-name>/decision-log.md` with the H1 `# Decision Log: <plan-name>` and an `## Interview` section holding the step 2 lines. This is the only copy of the interview.

### 4. Author the Plan

Write the artifacts in this order. Do not start a step before the previous one is done.

1. **Read the context.** `speq feature get <domain>/<feature>` for each related feature. If `specs/architecture.md` exists, read the sections the plan touches. Check the plan against every accepted ADR in the digest, per `/speq-adr-rules` rule 5; read the full fragment of any ADR the plan touches. Before you author a NEW feature or state that no spec covers a behavior, run `/speq-cli`'s absence check.

2. **Spec deltas.** For each feature in scope, write `specs/_plans/<plan-name>/<domain>/<feature>/spec.md`:
   - The feature exists → DELTA markers per `references/delta-template.md`.
   - It does not → a full spec per `references/feature-template.md`.

   **Prose drift check**: after a scenario delta on an existing feature, check that feature's `## Background` and `# Feature: <name>` description. When the scenario change makes either inaccurate, add a `DELTA:CHANGED` block for that section too.

3. **Architecture delta.** If the plan changes a component, boundary, interface, data flow, constraint, or external dependency, write `specs/_plans/<plan-name>/architecture.md` per `references/architecture-delta-template.md`. Copy each changed section from `specs/architecture.md`, edit it, and set the BASE hash with `git hash-object specs/architecture.md`. If `specs/architecture.md` is absent, write no delta and state `Architecture: no change: specs/architecture.md absent` in the decision log entry.

4. **Validate the deltas**: `speq plan validate <plan-name>`. Fix failures before you go on. Common fixes: close delta markers (`<!-- /CHANGED -->`, `<!-- /NEW -->`, `<!-- /REMOVED -->`), uppercase RFC 2119 keywords, bold step keywords (`*GIVEN*`, `*WHEN*`, `*THEN*`, `*AND*`).

5. **Decision log.** Complete `decision-log.md` from `references/decision-log-plan-template.md`. Keep `## Interview` unchanged.
   - **Design Decisions**: log a choice only when a reviewer or implementer cannot infer it from the deltas: a plausible alternative you rejected, or a constraint that is not obvious. Expect 0 to 3 entries. Never log naming, file placement, or task order. For a new abstraction or module boundary, justify it against `/speq-design-philosophy`'s Quick Diagnostic.
   - **Architecture field**: an entry with `Promotes to ADR: yes` whose Rationale names `/speq-adr-rules` rule-2 criterion 1, 2, or 3 needs an `- **Architecture:** <§ Section[, § Section] | no change: <reason>>` line. Every named section needs a block in the architecture delta.
   - **Promotion gate**: apply `/speq-adr-rules` to every entry. Default `Promotes to ADR: no`. For `yes`, name the rule-2 criterion and the ADR digest result in Rationale. Keep a promoted entry's Decision and Rationale short and factual: `/speq-record` derives the ADR from them.
   - **Review Findings**: leave empty; Revision Mode fills it.

6. **`plan.md` body.** Write every section of `references/plan-template.md` except `## Implementation Tasks`:
   - Context, Features (referencing the delta files), and Impact (breaking changes called out, or "None").
   - Dead Code Removal: existing code and tests the change or the new tests make redundant.
   - Verification. Scenario Coverage maps each new or changed scenario to an integration test (file path + test name), one test per scenario by default. Manual Testing gives concrete commands against the built software and their expected output, one per feature. Checklist commands come from `specs/mission.md`.

7. **`## Implementation Tasks`, last.** Derive the tasks from the finished deltas, Dead Code Removal, and Verification, in implementation order, as `- [ ]` checkboxes. Every task traces to one of them.

8. **Final validation**: `speq plan validate <plan-name>`.

### 5. Adversarial Plan Review

Spawn `plan-reviewer` once. There is one review round.

```
Review <plan-name>

## Plan Name
<plan-name>

## User Intent
<verbatim original request>

## Clarifying Interview Results
Read `## Interview` in `specs/_plans/<plan-name>/decision-log.md`.

## Plan Artifacts
plan.md, decision-log.md, every specs/_plans/<plan-name>/**/spec.md delta, and specs/_plans/<plan-name>/architecture.md when present
```

It writes `specs/_plans/<plan-name>/review/review.md` and returns `PLAN REVIEW: BLOCKERS: <n>, FINDINGS: <n>, INTENT: <n> — <path>`. `INTENT` counts the BLOCKERs on the Intent Fidelity axis alone.

- **`INTENT > 0`:** the reviewer holds that the plan solves a different problem than the one asked. Ask the user about those findings before any fix. The user accepts the plan as-is, or gives guidance and you revise.
- **`INTENT == 0` and `BLOCKERS > 0`:** fix the blockers per `references/revision-mode.md`. Do not review again.
- **Any blocker you could not resolve, or a failed `speq plan validate`:** ask the user, one line of context each plus a pointer to the review file.
- **Non-blocking findings** never loop and are never persisted. Carry them into step 6's report.

### 6. Explain Next Steps

Print to the terminal:

- `plan.md`'s `## Impact` section
- `ADR candidates: none`, or the title of each `Promotes to ADR: yes` entry in `decision-log.md`
- The non-blocking findings from step 5
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
| Writing tasks before the deltas exist | The tasks then describe a design the specs do not hold |
| Running a second review after the fix pass | One review, one fix pass; unresolved findings go to the user |
| Persisting non-blocking findings or looping on them | Report-only; they never gate |
