# Intent: <plan-name>

<!--
STRUCTURAL TEMPLATE - DO NOT COPY-PASTE
-->

## Interview

- Q: <question> → A: <user answer>

## Directions

- [D1] <what the plan chose>; <why, in one clause>
- [D2] <what>; <why>. ADR: yes, criterion <n>, <ADR search result>[, supersedes <slug>][, § <architecture section>]

## Rules

- `## Interview` holds only what the user said, one line per exchange, a few words each.
- A direction is a choice the plan made without a user answer that changes behavior, scope, or design. One line, at most about 30 words: the choice, then why. A rejected alternative appears only when it is the reason for the choice.
- Never a direction: naming, file placement, task order, anything a spec delta or task already states, and anything the user answered.
- Merge directions that share one reason. Delete one that the plan no longer depends on.
- Mark `ADR: yes` only on a direction that sets a durable decision per `/speq-adr-rules`. It is rare: name the rule-2 criterion and the ADR search result. Add `§ <section>` when the choice changes `specs/architecture.md` (criterion 1, 2, or 3), and `supersedes <slug>` when it replaces an accepted ADR. Without `ADR: yes`, write no ADR token.
