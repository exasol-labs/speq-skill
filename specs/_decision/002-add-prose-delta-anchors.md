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

## ADR: Validate plans without reading the target spec

**ID:** plan-validate-skips-anchor-presence
**Plan:** add-prose-delta-anchors
**Status:** Accepted

### Context

Whether an anchor exists in the target spec depends on that spec, and another plan can change it before this plan is recorded.

### Decision

`speq plan validate` checks only the rules that need no target spec. `/speq-record` checks anchor presence against the current target.

### Options Considered

- Read the target spec during validation: rejected, the result can be stale by record time.
