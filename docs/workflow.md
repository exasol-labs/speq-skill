[speq-skill](../README.md) / [Docs](./index.md) / Workflow

---

# Workflow Guide

A one-time `/speq:mission` to setup, then a repeating Plan → Implement → Record cycle.

```
/speq:mission → specs/mission.md (once per project)
                       │
/speq:plan    →  /speq:implement  →  /speq:record        (repeat)
```

## Set up the project

Run `/speq:mission` once per project.

- Interviews you about the project's purpose
- Writes `specs/mission.md` and `specs/architecture.md`
- For an existing codebase, drafts the architecture from the existing codebase

## Plan a feature, change or fix

Run `/speq:plan <intent>`.

- Interviews you about the change
- Writes a plan in `specs/_plans/<plan-name>/`: `plan.md`, spec deltas, `intent.md`, and an optional `architecture.md` delta
- Has a reviewer agent challenge the plan first
- Shows you every direction it took where you gave none, so you can correct it

## Implement it

Run `/speq:implement <plan-name>`.

- Works through the plan's task list with TDD
- Has a fresh-context sub-agent review the result
- Runs the build, tests, and lint checks
- Writes `verification-report.md`

## Record it

Run `/speq:record <plan-name>`.

- Merges the spec deltas into `specs/`
- Accepts the proposed ADRs
- Merges the architecture delta (if present)
- Moves the plan to `specs/_recorded/NNN-<plan-name>/`

## Ship a small change

Run `/speq:patch <intent>` for a small fix or tweak.

- Skips the plan and applies the change in one run
- Writes code and tests with TDD; a fix starts with a test that reproduces the defect
- Has a fresh-context sub-agent review the diff once, then fixes every finding
- Edits specs directly, without delta markers, plus at most one ADR, `specs/architecture.md`, and the docs the change needs
- Runs no git write; you review and commit

## Check the library

Run `/speq:audit` to run the validators and check unrecorded plans, the architecture file, mission drift, and ADR noise.

## Good to know

Plan names follow `<verb>-<scope>`, with the verb `add`, `change`, `remove`, `refactor`, or `fix`.

`/speq:record` stops and asks you when a feature has more than 10 scenarios or a domain has more than 8 features. It never reorganizes without your decision.
