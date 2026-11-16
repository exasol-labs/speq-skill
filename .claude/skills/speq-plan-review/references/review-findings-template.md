# Plan Review Findings Template

The plan reviewer writes this document to `specs/_plans/<plan-name>/plan-review.md` and returns only a one-line verdict. Consumers: the `/speq-plan` session (its fix pass reads the BLOCKER findings and executes each `Fix:` line), the PR report (Findings section), and human PR reviewers.

## Rules

1. Emit all five axis sections in taxonomy order: Intent Fidelity, Feasibility, Requirement Quality, Task Breakdown, Design Depth.
2. Every finding: a `#### [TAG] SEVERITY` heading plus `Location`, `Issue`, `Fix`.
3. The Summary block carries three counts: total blockers, total advisories, and `Intent Fidelity blockers`. The verdict line's `INTENT:` field is the third count and must match the Summary exactly.

## Skeleton

```markdown
# Plan Review Findings: <plan-name>

## Summary
- Axes checked: 5/5
- Total findings: <M> (Blockers: <X>, Findings: <Y>)
- Intent Fidelity blockers: <I>

## Intent Fidelity
[no objection — axis checked: <evidence>]

#### [<TAG>] <BLOCKER|FINDINGS>
- Location: <artifact> § <section>
- Issue: <defect, quoting the offending line(s)>
- Fix: <imperative instruction>

## Feasibility
...

## Requirement Quality
...

## Task Breakdown
...

## Design Depth
...
```

## Example

```markdown
## Intent Fidelity

#### [SCOPE_REDUCTION] BLOCKER
- Location: plan.md § Out of Scope
- Issue: the user asked for "retry with backoff on all outbound calls"; plan.md defers backoff to "a follow-up plan" without user agreement
- Fix: Delete the backoff bullet from plan.md § Out of Scope, add a task "implement exponential backoff for outbound HTTP calls" to plan.md § Tasks, and add a DELTA:NEW scenario covering retry exhaustion to the http-client spec delta

## Feasibility
[no objection — axis checked: task 2.3's dependency verified at the pinned version]

## Requirement Quality

#### [REQUIREMENT_CONFLICT] BLOCKER
- Location: specs/_plans/add-x/cli/record/spec.md § Scenarios
- Issue: this delta's "Reject unknown anchor" scenario contradicts the recorded spec's existing "Unknown anchors are ignored" scenario; both apply to the same input
- Fix: Add a DELTA:CHANGED block replacing the recorded "Unknown anchors are ignored" scenario with wording consistent with this delta's new behavior
```
