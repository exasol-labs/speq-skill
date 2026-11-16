# Decision Log: <plan-name>

<!--
STRUCTURAL TEMPLATE - DO NOT COPY-PASTE
Generate actual content from the clarifying interview and plan design.
-->

## Interview

<!-- Only what the user said. Acknowledged assumptions move here. -->

- Q: <question asked> → A: <user answer>
- Q: Assume <assumption>? → A: yes

## Assumptions

<!-- Choices the plan made without a user answer. Open until the user acknowledges them; /speq-plan step 6 empties this section. -->

### [A1] <Short assumption title>

- **Assumption:** What the plan takes as given or chose.
- **Why:** The gap it fills, and the evidence behind the choice.
- **Affects:** The deltas, tasks, or sections that depend on it.

## Design Decisions

### [1] <Short decision title>

- **Decision:** What was chosen.
- **Rationale:** Why this choice, citing the `## Interview` line it rests on.
- **Promotes to ADR:** no

<!-- Optional fields, add only when they carry a fact:
- **Alternatives:** A plausible alternative that was rejected, and why.
- **Consequences:** Effects, trade-offs, or corollary decisions.
-->

<!-- Rare: an entry that promotes to an ADR. Most plans have none.

### [2] <Short decision title>

- **Decision:** What was chosen.
- **Rationale:** Why this choice.
- **Supersedes:** <slug of the ADR this replaces> (optional; only when this decision replaces an existing ADR)
- **Architecture:** <§ Section[, § Section] | no change: <reason>> (required for some ADR entries)
- **Promotes to ADR:** yes
-->

## Review Findings

### [1] <Finding title>

- **Finding:** What the reviewer identified.
- **Direction change:** How the implementation was adjusted.
- **Promotes to ADR:** yes / no
