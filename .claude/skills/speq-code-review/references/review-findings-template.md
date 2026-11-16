# Code Review Findings Template

The code reviewer writes this document to `specs/_plans/<plan-name>/code-review.md`.

## Rules

1. Group findings by file (`### <path>`). Every finding: `#### [TAG] <short title>` plus `Location`, `Issue`, `Fix`.
2. `Fix:` is an imperative instruction (file path, symbol, concrete change), phrased so it can be executed without re-reading `Issue`.
3. No findings → a single `[none]` line under `## Findings`.

## Skeleton

```markdown
# Code Review Findings: <plan-name>

## Summary
- Files reviewed: <N>
- Total findings: <M>

## Findings

### <path/to/file>

#### [<TAG>] <short title>
- Location: line <n>
- Issue: <defect, with evidence>
- Fix: <imperative instruction>
```
