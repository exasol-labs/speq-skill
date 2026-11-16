[speq-skill](../README.md) / [Docs](./index.md) / Decision Log

---

# Decision Log

The decision log is the permanent record of durable decisions. It lives in `specs/_decision/` as one ADR fragment per plan or patch: `specs/_decision/NNN-<plan-name>.md`.

An ADR (Architecture Decision Record) is one durable decision. Most plans set none.

---

## Fragment format

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
speq decision-log validate   # file names, structure, required fields, status values, kebab-case slugs, slug references
speq decision-log show       # prints every fragment as one "Architecture Decision Records" view
```

Both read `specs/_decision/` and write nothing. An absent or empty directory is valid.
