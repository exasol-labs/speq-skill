# Decision Log: remove-record-command

## Interview

- Q: Why drop `speq record`? → A: Skills merge deltas by hand and never call the CLI command
- Q: Plan with a sub-agent? → A: No, plan in session; review with a sub-agent
- Q: ADR for this plan? → A: No, no decision here is promoted
- Q: Where does delta parsing live? → A: Slim `src/delta.rs` (parser and anchor rules only)
- Q: What happens to the record specs? → A: Drop what skills do not use; keep what `plan validate` enforces

## Design Decisions

### [1] Keep delta parsing in a new `delta` module

- **Decision:** Move `parse_deltas`, `check_anchor_rules`, `find_delta_specs` and their types from `record.rs` to `src/delta.rs`, and delete the rest of `record.rs`.
- **Rationale:** `plan validate` still needs them. A module named `record` with no record command misleads, and `plan` would depend on a dead name. `delta` has one job and no other dependencies.
- **Consequences:** `validate_delta_markers` in `src/plan.rs` keeps its own marker-balance scan, so the marker syntax stays in both `plan` and `delta` in this plan.
- **Alternatives:** Fold into `plan.rs`: rejected, it adds about 300 lines of parsing to a module that owns plan results.
- **Promotes to ADR:** no

### [2] Move repeated-anchor rules to `plan-validate-anchors`

- **Decision:** `plan/record-anchors` is removed. Its two repeated-anchor reject scenarios are added to `plan/plan-validate-anchors`, because `plan validate` enforces them and no scenario covers them there yet.
- **Rationale:** The merge scenarios of `record-anchors` describe code that is deleted. The reject rules survive in `plan validate`.
- **Consequences:** ADR `plan-validate-skips-anchor-presence` still holds: `plan validate` stays target-independent, and the `/speq-record` skill (recorder-agent per `/speq-spec-merge`) checks anchor presence at record time. ADR `record-all-or-nothing-across-plan` describes the removed command and stays Accepted with no follow-up plan, by the user's choice in the interview to store no ADR change.
- **Promotes to ADR:** no

## Review Findings
