# Decision Log Template

## ADR Rules

An ADR is a rare record of one durable decision in `specs/_decision/`. Most plans and patches write none.

1. **Default no.** The expected number of ADRs per plan is zero. Two or more in one plan get re-judged against rules 2 and 3.
2. **Criteria.** A decision qualifies only when one of these holds, and the ADR names it:
   - It affects multiple components.
   - It sets a long-lived constraint that binds future work.
   - It is a major technology, API, persistence, security, or deployment choice.
   - It rejects a plausible alternative, and the reason will matter to a later reader.
3. **Never an ADR.** Naming, file placement, or folder layout. A one-off bug fix. A scope trim or task order. Implementation detail: signatures, paths, flags, versions, data layout inside one module. A workaround for one plan's own mechanics. A corollary of another decision: add it to the parent ADR's Consequences. A fact that a spec scenario, `specs/architecture.md`, or `CLAUDE.md`/`AGENTS.md` already states.
4. **Search first.** Read the ADR digest (`/speq-cli`) and every fragment that matches. When an ADR already covers the decision, write none. When the decision changed, write a new ADR with `Supersedes: <slug>`. Never edit an old fragment. A plan that contradicts an accepted ADR conforms to it or supersedes it; a silent contradiction is a defect.
5. **Minimum text.** One decision per ADR. Present tense, no history: no earlier attempts, review rounds, plan names, or dates. Context is one or two sentences, Decision one to three. Options Considered and Consequences appear only when they carry content.

## Fragment Layout

One fragment per plan or patch: `specs/_decision/NNN-<plan-name>.md`. ID is a kebab-case slug, unique across every file in `specs/_decision/`.

```markdown
# Decisions: <plan-name>

## ADR: <Title>

**ID:** <kebab-case-slug>
**Plan:** <plan-name>
**Status:** Accepted
**Supersedes:** <superseded-slug>   (optional: only when this ADR replaces another)

### Context

<One or two sentences: the situation that forces the decision.>

### Decision

<One to three sentences: the chosen approach.>

### Options Considered

(optional: only when a real alternative was rejected)

| Option | Verdict |
|--------|---------|
| <chosen option> | ✓ Chosen — <brief rationale> |
| <other option> | ✗ Rejected — <brief reason> |

### Consequences

(optional: only when the decision has an effect on later work)

<Short bullets: what becomes easier or harder.>
```
