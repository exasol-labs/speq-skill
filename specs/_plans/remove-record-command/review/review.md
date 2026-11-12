# Plan Review Findings: remove-record-command

## Summary
- Axes checked: 6/6
- Total findings: 11 (Blockers: 2, Findings: 9)
- Intent Fidelity blockers: 0

## Ways This Plan May Fail

1. `specs/mission.md` still lists "Record plan deltas" as a CLI capability. `AGENTS.md` loads that file into every agent session, so agents keep reaching for a removed command, and `/speq-audit` reports mission drift. Routed to `[TRACEABILITY_GAP]` (Task Breakdown).
2. ADR `plan-validate-skips-anchor-presence` names `speq record` as the component that checks anchor presence. The plan deletes the command and does not name this ADR. Task 3.3 also allows the rewording to credit `plan validate` with the check, but the ADR keeps `plan validate` target-independent. The recorder-agent would then rely on a check that no component runs. Routed to `[ADR_CONFLICT]` (Design Depth).
3. Task 2.2 adds fixtures but no `_targets/` target specs. `plan validate` skips anchor checks when the target spec is missing, so both new tests get exit 0 instead of the expected error. Routed to `[TRACEABILITY_GAP]` (Task Breakdown).

## Intent Fidelity

no objection — axis checked: "drop record" maps to tasks 1.3, 3.1 and 3.2. The claim "no skill calls `speq record`" holds: a grep over `.claude/skills/` and `.claude/agents/` finds the command only in the `speq-cli` command table and in the reference texts that task 3.3 rewords. "Do not promote as ADR" holds: `decision-log.md` has no `Promotes to ADR: yes` entry. "Plan in session, review with a sub-agent" matches interview Q2, and no planner hand-off note exists.

## Feasibility

no objection — axis checked: `src/plan.rs:4` imports only `check_anchor_rules`, `find_delta_specs` and `parse_deltas` from `record`. No module in `src/validate/`, `src/feature.rs` or `src/tree.rs` imports `record`. `repeated_anchor_messages` in `src/record.rs:469-500` already names the anchor and every marker, so both new scenarios hold against current code. `./target/debug/speq plan validate remove-record-command` passes.

## Requirement Quality

#### [IMPLEMENTATION_DETAIL] FINDINGS
- Location: specs/_plans/remove-record-command/plan/plan-validate-anchors/spec.md § Background
- Issue: the CHANGED Background has 8 bullets, over the limit of 5. Bullet 8, "A delta file that fails delta parsing produces an error naming the file, and its anchor checks are skipped", states behavior. The "anchor checks are skipped" half has no scenario that tests it.
- Fix: Cut the Background to at most 5 bullets. Merge bullets 3 and 4 into "The anchor of a delta block is its first non-empty line: `### Scenario: <name>`, `## Background`, or `# Feature: <name>`". Delete bullet 7, because the "skips anchor checks for a new feature" scenario's THEN already states it. Delete bullet 8, because the "malformed delta marker" scenario's THEN already states its error half.

## Task Breakdown

#### [TRACEABILITY_GAP] BLOCKER
- Location: plan.md § Implementation Tasks
- Issue: no task updates `specs/mission.md`, which per `AGENTS.md` "describes the `speq` CLI only". After this plan, mission.md still claims Core Capability 5, "Record plan deltas — Merge approved delta specs from `_plans/` into permanent specs and archive the plan". Its Out of Scope line still says "Does not manage spec history beyond archiving recorded plans", but the CLI will no longer archive anything.
- Fix: Add task 3.4 to plan.md § Implementation Tasks: "In `specs/mission.md`, delete Core Capability 5 and renumber capability 6 as 5. Change the Out of Scope bullet to `Version control — Does not manage spec history`." Add `specs/mission.md` to Group B's Knowledge column.

#### [TRACEABILITY_GAP] FINDINGS
- Location: plan.md § Implementation Tasks, task 2.2; spec delta § Scenarios
- Issue: task 2.2 names fixtures `prose-repeated-anchor` and `prose-mixed-kinds`, but the scenarios run `speq plan validate repeat-same` and `speq plan validate repeat-mixed`. In every other scenario of this feature, the plan name equals the fixture directory name. Task 2.2 also omits target specs. `setup_target_specs` in `tests/plan_validate.rs` copies `tests/fixtures/plan_validate/_targets/<fixture>/` only when that directory exists. Without it, anchor checks are skipped and both tests get exit 0.
- Fix: Use one name per case in both the scenario and the fixture. For example, change the GIVEN and WHEN steps to "prose-repeated-anchor" and "prose-mixed-kinds". Extend task 2.2 with: "add `tests/fixtures/plan_validate/_targets/<fixture>/<domain>/<feature>/spec.md` holding the anchored scenario, and call `setup_target_specs` in both tests".

#### [TRACEABILITY_GAP] FINDINGS
- Location: plan.md § Dead Code Removal; task 1.1
- Issue: task 1.1 moves "their errors" without naming which ones. The moved functions return only `FileReadError` (from `find_delta_specs`) and `MalformedDelta` (from `parse_deltas`). The variants `PlanNotFound`, `FileWriteError`, `DirCreateError`, `DirMoveError` and `InvalidDeltaAnchor` serve only merge, write and archive. Public enum variants raise no dead-code warning, so the clippy step will not catch them if they move unchanged.
- Fix: In task 1.1, name the new error enum (for example `DeltaError`) and limit it to `FileReadError` and `MalformedDelta`. Add a Dead Code Removal row: "`RecordError` variants `PlanNotFound`, `FileWriteError`, `DirCreateError`, `DirMoveError`, `InvalidDeltaAnchor`, and `DeltaAnchor::matches_heading` and `stop_prefixes` | merge, write and archive only".

#### [TRACEABILITY_GAP] FINDINGS
- Location: plan.md § Implementation Tasks, task 3.2
- Issue: the mock binary in `tests/docker/test-docker.sh:83` still prints `record        Record approved plan deltas to permanent specs` in its help text. The plan removes the command but leaves this line.
- Fix: Extend task 3.2: "Remove the `record` line from the mock help text in `tests/docker/test-docker.sh`."

#### [CLUSTER_INCOHERENCE] FINDINGS
- Location: plan.md § Parallelization
- Issue: the Knowledge columns omit files that the group's tasks edit. Group A omits `src/cli.rs`, `src/main.rs`, `src/lib.rs` and `tests/cli_integration.rs`. Group B omits `.claude/skills/speq-spec-merge/SKILL.md` and `specs/plan/`.
- Fix: Add `src/cli.rs`, `src/main.rs`, `src/lib.rs` and `tests/cli_integration.rs` to Group A's Knowledge. Add `.claude/skills/speq-spec-merge/SKILL.md` and `specs/plan/` to Group B's Knowledge.

## Design Depth

`[ADR_OVERPROMOTION]`: no objection — axis checked: decision-log entries [1] and [2] are both `Promotes to ADR: no`.
`[ARCHITECTURE_DRIFT]`: no objection — axis checked: the BASE comment `441addd…` equals `git hash-object specs/architecture.md` at HEAD. Every block anchors on a canonical `##` section with CHANGED and holds only bullets and one fenced block. Each fact plan.md states (new `delta` component, `plan -> delta` dependency, CLI without `record`) appears in the delta.

#### [ADR_CONFLICT] BLOCKER
- Location: plan.md § Context; decision-log.md § [2] Consequences; plan.md task 3.3
- Issue: accepted ADR `plan-validate-skips-anchor-presence` decides "`speq plan validate` checks only the rules that need no target spec. `speq record` checks anchor presence against the current target." The architecture delta removes `speq record`, and no plan artifact names this ADR. plan.md Context names only `record-all-or-nothing-across-plan`. Task 3.3 also lets the implementer reword the `/speq-spec-merge` anti-pattern "`record` errors instead of silently merging nothing" to name `plan validate`. That wording credits `plan validate` with the anchor-presence check, which the ADR excludes from it.
- Fix: Conform to the ADR without promoting a new one, because the interview ruled out ADR promotion. Add to plan.md § Context and to decision-log [2] Consequences: "ADR `plan-validate-skips-anchor-presence` still holds. `plan validate` stays target-independent, and the `/speq-record` skill (recorder-agent per `/speq-spec-merge`) checks anchor presence at record time." Split task 3.3: reword `.claude/skills/speq-spec-merge/SKILL.md` line 125 (missing scenario anchor) to name recorder-agent and never `plan validate`. Reword line 126 ("`record` cannot tell what the block targets") to name `plan validate` and recorder-agent. Task 3.3 currently misses line 126.

#### [TACTICAL_SHORTCUT] FINDINGS
- Location: decision-log.md § [2] Consequences
- Issue: "`/speq-audit` or a later plan can supersede it" leaves ADR `record-all-or-nothing-across-plan` Accepted with no scheduled follow-up. After this plan, no recording component writes all-or-nothing. `/speq-spec-merge` validates each feature after its merge, so earlier features are already written when a later one fails.
- Fix: Add a `## Follow-up plans` section to plan.md that names a plan to supersede `record-all-or-nothing-across-plan`. As an alternative, state in decision-log [2] Consequences that the ADR stays Accepted with no follow-up, by the user's choice in the interview.

#### [INFORMATION_COUPLING] FINDINGS
- Location: decision-log.md § [1]; architecture.md § Components
- Issue: the delta adds `delta` as the module that "parses delta markers". `validate_delta_markers` in `src/plan.rs:179-215` keeps a second, separate scan of the same marker syntax for open and close balance. After this plan, the marker format still lives in two modules, and the plan does not say so.
- Fix: Add a Consequences line to decision-log [1]: "`validate_delta_markers` in `src/plan.rs` keeps its own marker-balance scan, so the marker syntax stays in both `plan` and `delta` in this plan."

## Prose Quality

#### [PROSE_UNCLEAR] FINDINGS
- Location: specs/_plans/remove-record-command/plan/plan-validate-anchors/spec.md § Background, bullet 6
- Issue: "A recognized anchor MAY appear in at most one delta block per delta file" uses the permission keyword MAY for a prohibition. A reader can take it as optional.
- Fix: Replace the bullet with "A recognized anchor MUST NOT appear in more than one delta block of a delta file, whatever the marker kinds".

#### [PROSE_UNCLEAR] FINDINGS
- Location: specs/_plans/remove-record-command/plan/plan-validate-anchors/spec.md § Feature description
- Issue: "Catches a delta block that cannot be merged into a permanent spec before implementation begins" claims too much. A CHANGED block whose anchor is missing from the target spec cannot be merged, and `plan validate` passes it by design.
- Fix: Replace the sentence with "Catches an unrecognized, illegal, or repeated anchor before implementation begins."
