# Intent: <plan-name>

<!--
STRUCTURAL TEMPLATE - DO NOT COPY-PASTE
-->

## Interview

- Q: <full question, understandable without the conversation>
  A: <chosen option with its description, or the user's own words exactly as written>
  Rejected: <offered options the user did not pick>

## Directions

- [D1] <what the plan chose>; <why, in one clause>
- [D2] <what>; <why>. ADR: yes, criterion <n>, <ADR search result>[, supersedes <slug>][, § <architecture section>]

## Rules

- `## Interview` holds only the interview exchanges, never the plan's own choices: one entry per exchange, 2 to 3 lines each. Add `Rejected:` only when the user chose between real alternatives.
- A direction is a choice the plan made without a user answer that changes behavior, scope, or design. One line, at most about 30 words: the choice, then why. A rejected alternative appears only when it is the reason for the choice.
- Never a direction: naming, file placement, anything a spec delta already states, and anything the user answered.
- Merge directions that share one reason. Delete one that the plan no longer depends on.
- Mark `ADR: yes` only on a direction that sets a durable decision per the ADR rules in `adr-rules.md` in `/speq-formats`. It is rare: name the rule-2 criterion and the ADR search result. Add `§ <section>` when the choice changes `specs/architecture.md` (criterion 1, 2, or 3), and `supersedes <slug>` when it replaces an accepted ADR. Without `ADR: yes`, write no ADR token.
