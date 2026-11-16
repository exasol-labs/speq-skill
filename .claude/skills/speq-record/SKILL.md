---
name: speq-record
description: "Merges implemented spec deltas into the permanent specs and archives the plan. Use after /speq-implement, or to record, merge, or archive a finished plan. Arg: plan-name."
---

# Spec Recorder

## User Arguments

$ARGUMENTS

Verify the preconditions, merge the plan's deltas into the permanent specs, promote its ADRs, and archive the plan. Recording is mechanical: merge what the deltas say, nothing more. You run no git write: the user commits, or `/speq-implement-pr` does.

## Required Skills

Invoke before starting:
- `/speq-cli` — spec validation
- `/speq-plan`'s `references/decision-log-template.md` — ADR rules and fragment layout
- `/speq-writing-guardrails` — prose style for promoted ADRs

## Workflow

### Phase 1: Resolve Plan Name

Get the plan name from the user prompt. If none is given, ask the user to choose from the plans under `specs/_plans/`.

### Phase 2: Verify Preconditions

```
Check: specs/_plans/<plan-name>/verification-report.md exists?
├─ Yes → Proceed
└─ No  → STOP: "Run /speq-implement <plan-name> first."
```

Read `plan.md`, list the delta specs (`specs/_plans/<plan-name>/**/spec.md`), and run `speq feature list`.

### Phase 3: Check the Architecture Delta

Skip this phase when `specs/_plans/<plan-name>/architecture.md` is absent, and report `Architecture: no delta`. Otherwise the delta changes `specs/architecture.md`. The `speq` CLI does not see it, so you check and apply it by hand. Its format is in `/speq-plan`'s `references/architecture-delta-template.md`, the target format in `references/architecture-template.md`.

1. **Already applied?** The delta is applied when each CHANGED section in `specs/architecture.md` already equals its block body, each NEW section exists with its block body, and each REMOVED section is gone. This happens on a rerun after a threshold decision. Then report `Architecture: already merged` and go to Phase 4.
2. **Pre-check (read-only).**
   - The H1 is `# Architecture Delta: <plan-name>`. DELTA markers are balanced. Each block's first non-empty line is `## <Section>` and is its only `##` line. No two blocks share an anchor. NEW and REMOVED target non-canonical sections only.
   - `specs/architecture.md` exists, each CHANGED and REMOVED anchor exists in it, and no NEW anchor exists in it.
   - Line rules: outside blocks only the H1, the BASE comment, and blank lines. Inside blocks only `- ` bullets, fenced blocks, and blank lines. No `###`, no tables, no prose paragraphs.
3. **Base check.** Compare `git hash-object specs/architecture.md` with the BASE comment. If they differ, run `git cat-file -p <BASE>` and compare each CHANGED and REMOVED section with the same section in the current file. A differing section, or a missing blob, stops the record with a stale-base error: a stale BASE would overwrite another plan's changes.

Failure format: `Recording failed: architecture delta: architecture.md:<line>: <rule>`. On failure, do not fix the delta. The plan stays unarchived. Report the message and stop. The fix is a plan change: correct the plan's `architecture.md`, or refresh its BASE.

### Phase 4: Merge Feature Deltas

For each delta spec at `specs/_plans/<plan-name>/<domain>/<feature>/spec.md`:

```
specs/<domain>/<feature>/spec.md exists?
├─ No  → Copy delta (strip markers)
└─ Yes → Merge using markers below
```

A delta block's anchor is the first non-empty line inside the block. `### Scenario: <name>` and `## Background` match exactly. `# Feature: <name>` matches by the prefix `# Feature`, so a description delta MAY rename the feature.

Before you merge a delta spec, check that the target spec holds every scenario a `DELTA:CHANGED` or `DELTA:REMOVED` block names. If one is missing, stop and report the delta file and the scenario name.

| Anchor | Marker | Action |
|--------|--------|--------|
| `### Scenario:` | `DELTA:NEW` | Append the scenario |
| `### Scenario:` | `DELTA:CHANGED` | Replace the scenario with the same name |
| `### Scenario:` | `DELTA:REMOVED` | Delete the scenario with the same name |
| `## Background`, `# Feature` | `DELTA:CHANGED` | Replace the whole section (heading plus body) in place. A line the block omits is deleted |
| `## Background`, `# Feature` | `DELTA:NEW`, `DELTA:REMOVED` | Rejected: both sections are required and always exist |

After each merge, strip all `<!-- DELTA:* -->` markers and run `speq feature validate <domain>/<feature>`. If validation fails, stop and report. Do not guess fixes.

### Phase 5: Merge the Architecture Delta

Skip when Phase 3 skipped or found the delta already applied.

1. Build the merged text in memory. CHANGED replaces the heading and body up to the next `##`, and a line the block omits is deleted. NEW appends the section at the end. REMOVED deletes the section. Strip all DELTA markers. Write the file once.
2. **Post-check.** The H1 is `# Architecture`. The six canonical sections appear once each, in order. No `##` heading is duplicated. No DELTA text is left. The line rules hold. On failure, restore the original text, report it in the failure format, and do not archive.

Report line: `Architecture: § Components (CHANGED), § Deployment (NEW)`. List the lines each CHANGED block removed, so the user can spot an accidental deletion.

### Phase 6: Check Library Thresholds

A spec with more than 10 scenarios, or a domain with more than 8 features, needs a user decision. Stop before archiving and ask a concrete question about how to reorganize. Never reorganize the library on your own.

### Phase 7: Promote ADRs

Read `## Directions` in `specs/_plans/<plan-name>/intent.md`. If it is absent or no direction is marked `ADR: yes`, skip this phase.

For each direction marked `ADR: yes`, write an ADR per `/speq-plan`'s `references/decision-log-template.md`:

- **Title**: a short noun phrase for the choice
- **ID**: a kebab-case slug of the title, unique across every file in `specs/_decision/`
- **Plan**: `<plan-name>`
- **Status**: `Accepted`. Recording the plan is the acceptance
- **Supersedes** (optional): the slug from the direction's `supersedes <slug>`. It must name an existing ADR
- **Context**: one or two sentences from the direction's reason: the situation that forces the choice, as present fact, per ADR rule 5
- **Decision**: the direction's choice, in one to three sentences
- **Options Considered**: only when the direction names a rejected alternative
- **Consequences**: only when the direction states an effect on later work
- The criterion and `§ <section>` are not mapped into the ADR

Write all of them into ONE new fragment `specs/_decision/NNN-<plan-name>.md` (NNN = existing fragment count plus one, zero-padded to 3 digits), H1 `# Decisions: <plan-name>`, one `## ADR: <Title>` block per direction in `intent.md` order. Edit no other fragment: a supersede is a one-way pointer from the new ADR. Run `speq decision-log validate`.

### Phase 8: Archive

Only when every validation passed and every threshold decision is resolved:

1. Run `speq feature validate`.
2. `mv specs/_plans/<plan-name> specs/_recorded/NNN-<plan-name>`, where NNN = existing entry count in `specs/_recorded/` plus one, zero-padded to 3 digits. The architecture delta moves with the plan.

### Phase 9: Confirm Completion

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
| Rewriting scenario wording or `specs/architecture.md` beyond the delta | Recording is mechanical |
| Merging a CHANGED or REMOVED block whose scenario is absent from the target | It silently merges nothing; stop instead |
| A heading left outside the marker, so the block's first line is not an anchor | Nothing can tell what the block targets |
| Two blocks of one delta file sharing an anchor | The result would depend on block order; one block carries the final text |
| `Status: Proposed` on a promoted ADR | Recording is the acceptance |
| Archiving after an architecture delta failure | The delta was not merged, so the plan stays open |
