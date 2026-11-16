[speq-skill](../README.md) / [Docs](./index.md) / Decision Log

---

# Decision Log

The decision log is the permanent record of durable decisions: ADR fragments in `specs/_decision/`. Its source is a plan's `intent.md`, which records what the user said and what the plan assumed.

| Part | Location | Purpose |
|------|----------|---------|
| Plan intent | `specs/_plans/<plan-name>/intent.md` | The user's interview answers and the plan's assumptions |
| Permanent log | `specs/_decision/NNN-<plan-name>.md` | One fragment per plan or patch, holding its accepted ADRs |

An ADR (Architecture Decision Record) is one durable decision. Only a confirmed assumption marked `ADR: yes` becomes one. The `## Context` section of `plan.md` is a bullet list of the problem. It holds no decisions.

---

## Lifecycle

```
/speq:plan     writes intent.md. Every choice the user did not
               make is an assumption until the user confirms it.
               A confirmed assumption marked "ADR: yes" is an
               ADR candidate.

/speq:record   writes each candidate as an ADR with
               Status: Accepted into specs/_decision/NNN-<plan-name>.md.
               Recording the plan is the acceptance.

/speq:patch    writes an ADR with Status: Accepted into
               specs/_decision/NNN-<patch-name>.md.
               Committing the patch is the acceptance.
               Under /speq:patch-pr, merging the PR is.
```

To drop a candidate, ask for the ADR token's removal when `/speq:plan` confirms its assumptions. In a PR run, comment on the draft PR and run `/speq:plan-pr` again.

## Promotion rules

`/speq:adr-rules` owns the rules. In short, an assumption carries no ADR token by default. `ADR: yes` needs a named criterion and a prior ADR search (see `/speq:cli`).

## Plan intent

```markdown
# Intent: add-prose-delta-anchors

## Interview

- Q: New marker syntax to target a Background? → A: No, wrap the existing heading

## Assumptions

- [A1] confirmed: A delta block's anchor is its first non-empty line; a heading scan was rejected because a quoted heading would take over the anchor. ADR: yes, criterion 4, no existing ADR
```

Each assumption is one dense line with status `open` or `confirmed`. `/speq:plan` asks the user about every `open` one before the plan is ready, and `/speq:implement` refuses to start while one is open.

The file is optional. If present, `speq plan validate` checks its H1, its sections, and that every `ADR:` value is `yes` or `no`.

## Permanent log

A fragment is one Markdown file. This one comes from the plan `add-prose-delta-anchors`:

```markdown
# Decisions: add-prose-delta-anchors

## ADR: Anchor a delta block by its first non-empty line

**ID:** anchor-first-nonempty-line
**Plan:** add-prose-delta-anchors
**Status:** Accepted

### Context

A delta block must name the section it targets without ambiguity.

### Decision

A delta block's anchor is its first non-empty line. A Background or description delta wraps the existing heading in place, with no new marker syntax.

### Options Considered

- Scan the whole block for the first heading: rejected, a heading quoted inside the prose can take over the anchor.

### Consequences

- Plans need no new marker syntax to target a Background or description.
```

Each ADR needs `**ID:**`, `**Plan:**`, `**Status:**`, `### Context`, and `### Decision`. A later plan can replace this ADR by adding a line `**Supersedes:** anchor-first-nonempty-line` to its own ADR. The old fragment is never edited. `### Options Considered` and `### Consequences` are optional.

## Commands

```bash
speq decision-log validate   # structure, required fields, status values, slug references
speq decision-log show       # prints every fragment as one "Architecture Decision Records" view
```

Both read `specs/_decision/` and write nothing. An absent or empty directory is valid. `speq plan validate <plan-name>` checks the plan-level log.
