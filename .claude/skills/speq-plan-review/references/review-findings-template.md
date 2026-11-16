# Plan Review Findings Template

The plan reviewer writes this document to `specs/_plans/<plan-name>/plan-review.md` and returns only a one-line verdict. Consumers: the `/speq-plan` session (its fix pass reads the BLOCKER findings and executes each `Fix:` line), the PR report (Findings section), and human PR reviewers.

## Rules

1. Emit all four axis sections in taxonomy order: Intent Fidelity, Feasibility, Requirement Quality, Design Depth.
2. Every finding: a `#### [TAG] SEVERITY` heading plus `Location`, `Issue`, `Fix`.
3. The Summary block carries three counts: total blockers, total FINDINGS, and `Intent Fidelity blockers`. The verdict line's `INTENT:` field is the third count and must match the Summary exactly.

## Skeleton

```markdown
# Plan Review Findings: <plan-name>

## Summary
- Axes checked: 4/4
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

## Design Depth
...
```
