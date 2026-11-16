---
name: speq-record
description: "Merges implemented spec deltas into the permanent specs and archives the plan. Use after /speq-implement, or to record, merge, or archive a finished plan. Arg: plan-name."
---

# Spec Recorder

Verify the preconditions, merge the plan's deltas into the permanent specs, and archive the plan.

## Required Skills

Invoke before starting:
- `/speq-cli` — spec validation
- `/speq-spec-merge` — the delta-merge procedure, thresholds, and ADR promotion
- `/speq-adr-rules` — ADR rules and status lifecycle
- `/speq-git-discipline` — no git history writes
- `/speq-writing-guardrails` — prose style for ADR promotion

## Workflow

### Phase 1: Resolve Plan Name

Get the plan name from the user prompt. If none is given, ask the user to choose from the plans under `specs/_plans/`.

### Phase 2: Verify Preconditions

```
Check: specs/_plans/<plan-name>/verification-report.md exists?
├─ Yes → Proceed
└─ No  → STOP: "Run /speq-implement <plan-name> first."
```

### Phase 3: Merge

Merge the delta specs, and the architecture delta if present, per `/speq-spec-merge`. Validate between merges. Promote ADRs per `/speq-adr-rules`: recording is the acceptance, so write `Status: Accepted`.

- Merge deltas only. Do not rewrite scenarios for style.
- Leave no `DELTA:*` markers in permanent specs.
- Edit no file in `specs/_decision/` other than the new `NNN-<plan-name>.md` fragment.
- If any library threshold is exceeded (scenarios > 10, domain features > 8), stop before archiving and ask the user for their organizational decision. Never reorganize the library on your own.
- If the architecture delta fails (`Recording failed: architecture delta: ...`), do not fix the delta. The plan stays unarchived. Report the message and stop. The fix is a plan change: correct `architecture.md` in the plan, or refresh its BASE after a stale-base error.

### Phase 4: Archive

Archive the plan per `/speq-spec-merge` only when every validation passed and every threshold decision is resolved.

### Phase 5: Confirm Completion

Report to user:

```
✓ Verification report confirmed
✓ All deltas merged
✓ Spec library validated
✓ Plan archived: specs/_recorded/NNN-<plan-name>
✓ Architecture merged: <sections>   (omit when no delta)
✓ ADRs accepted: N — <slugs>   (omit when none)
```

## Anti-Patterns

| Pattern | Why Wrong |
|---------|-----------|
| Record without verification report | Implementation not proven |
| Archive after an architecture delta failure | The delta was not merged, so the plan must stay open |
