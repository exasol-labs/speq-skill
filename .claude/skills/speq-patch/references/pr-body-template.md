# PR Body Template

Used by `/speq-patch` as the ready PR body.

## Skeleton

```markdown
## Current State
<1-2 sentences: what is broken, missing, or true today>

## What Changes
<1-2 sentences: the mechanism>

Verification: <suite>: pass[, <suite>: pass][; baseline failures: <suite>[, <suite>]]
ADRs: <none | slug (<rule-2 criterion>; ADR search: <result>)[, ...]>
Plans affected: <none | <plan-name> (<domain>/<feature>)[, ...]>
Architecture: <none | none (specs/architecture.md absent) | § Section (changed|new|removed)[, ...]>
```

## Rules

- **`Verification` is one line.** Name each test suite and the lint command that ran, with its result. Add every baseline failure from the run on the new branch.
- **`ADRs` is one line.** Write `none` when the patch writes no ADR, which is the usual case. Otherwise give one entry per new ADR: its slug, the `/speq-adr-rules` rule-2 criterion it meets, and the result of the ADR search.
- **`Plans affected` is one line.** Name each active plan that holds a delta for a feature the patch edits, with that feature.
- **`Architecture` is one line.** Write `none` when the patch leaves `specs/architecture.md` unchanged, or `none (specs/architecture.md absent)` in a repo without that file. Otherwise list each changed section with `changed`, `new`, or `removed`.
- **`Current State` and `What Changes` replace a list of edits.** State the problem, then the mechanism that fixes it. The diff holds the edits.
