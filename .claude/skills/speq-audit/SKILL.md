---
name: speq-audit
description: "Audits a speq project's health: validators, unrecorded plans, the architecture file, mission sync, and ADR noise. Use when asked to audit, health-check, or check a project built with speq-skill."
---

# Spec Auditor

## User Arguments

$ARGUMENTS

Runs read-only health checks over a speq project, diffs the mission against the spec library, judges the ADRs, prints a summary, and offers to fix each finding. **Always ask before a fix.**

Invoke `/speq-cli` and `/speq-writing-guardrails` first. **Read `references/checks.md`** for each check's detection, signal, and remediation.

## Workflow

### Phase 1: Preconditions

No `specs/` directory → STOP: "No spec library found. Run /speq-mission to bootstrap."

### Phase 2: Run Checks (read-only)

Run checks 1 to 5 in `references/checks.md`. Record `✓` / `✗` / `⚠` and a one-line detail for each. **Modify nothing in this phase.**

### Phase 3: Print the summary

Lead with the verdict, then the checks table, then numbered actions, each ending with the concrete next command. Quantify every result.

```
# speq:audit — <project>

| Result | Summary |
|--------|---------|
| **✓ healthy** _or_ **⚠ N findings** | <one line: what's wrong, most important first> |

## Checks
| Check                     | Status | Detail |
|---------------------------|--------|--------|
| Validators                | ✓ | 0 errors |
| Unrecorded plans          | ✗ | 1: add-export-command |
| Architecture file         | ✓ | present, 6 sections |
| mission.md ↔ spec library | ⚠ | 2 features unmentioned · 1 capability unbacked |
| ADR review                | ⚠ | 2 noise · 1 wordy of 12 ADRs |

## Recommended actions  (I ask before each change)
1. Record the finished plan → /speq-record add-export-command
2. Reconcile mission.md → /speq-mission (seeded): features `cli/export`, `cli/import` unmentioned; capability "Diff specs" unbacked
3. Clean up 3 ADRs: remove 2 noise, trim 1 wordy
```

A clean project prints `✓ healthy` and omits the actions section. List each noisy or wordy ADR under its action.

### Phase 4: Remediate (each finding gated)

For each actionable finding, ask the user (**Yes / No / Skip**) and apply the remediation in `references/checks.md`. **Never act without a Yes.** After a fix, re-run the affected validator and print its one-line result. Never edit `mission.md` directly: `/speq-mission` owns it.

### Phase 5: Close

Print the final status and any remaining manual next steps.
