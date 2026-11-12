# Tasks: remove-record-command

## Phase 2: Implementation (Group A: delta module)
- [x] 1.1 Create `src/delta.rs` with `DeltaKind`, `DeltaAnchor`, `DeltaBlock`, `parse_deltas`, `check_anchor_rules`, `find_delta_specs`, a `DeltaError` enum limited to `FileReadError` and `MalformedDelta`, and their unit tests, moved from `src/record.rs`. Leave `PlanNotFound`, `FileWriteError`, `DirCreateError`, `DirMoveError`, `InvalidDeltaAnchor`, `DeltaAnchor::matches_heading` and `stop_prefixes` behind
- [x] 1.2 Delete `src/record.rs`; drop `pub mod record` and add `pub mod delta` in `src/lib.rs`; point `src/plan.rs` imports and doc comments at `delta`
- [x] 1.3 Remove the `Record` command from `src/cli.rs` and `handle_record_command` from `src/main.rs`
- [x] 2.1 Delete `mod record` in `tests/cli_integration.rs` and `tests/fixtures/record/`
- [x] 2.2 Add `plan_validate` tests and fixtures `prose-repeated-anchor` and `prose-mixed-kinds` for the two new scenarios, each with a target spec at `tests/fixtures/plan_validate/_targets/<fixture>/<domain>/<feature>/spec.md` holding the anchored scenario, and call `setup_target_specs` in both tests

## Phase 2: Implementation (Group B: docs and skills)
- [x] 3.1 Delete `specs/plan/record/` and `specs/plan/record-anchors/`
- [x] 3.2 Remove `speq record` from `docs/cli-reference.md` (table row and section), `.claude/skills/speq-cli/SKILL.md`, and the `record` line in the mock help text of `tests/docker/test-docker.sh`
- [x] 3.3 Reword `speq record` mentions in `.claude/skills/speq-plan/references/delta-template.md` (3 places) to name the `/speq-record` skill or `plan validate`. In `.claude/skills/speq-spec-merge/SKILL.md`, reword line 125 (missing scenario anchor) to name recorder-agent, never `plan validate`, and line 126 (heading outside the marker) to name `plan validate` and recorder-agent
- [x] 3.4 In `specs/mission.md`, delete Core Capability 5 and renumber capability 6 as 5. Change the Out of Scope bullet to `Version control — Does not manage spec history`

## Phase 3: Verification
- [x] 4.1 Run build, test, clippy, fmt
- [x] 4.2 Manual verification

## Phase 4: Review Fixes
- [x] 4.3 In `src/delta.rs`, replace the second paragraph of the `DeltaAnchor` doc comment with the text stating that the anchor owns the heading text of each recognized section and that an unmatched block is `Unrecognized`, wrapped at the width of the surrounding doc lines
- [x] 4.4 In `src/delta.rs`, replace the `check_anchor_rules` doc paragraph that names the recorder agent as a caller with the text stating that `speq plan validate` enforces the legality matrix and repeated-anchor rule and that `/speq-spec-merge` mirrors them by hand. Keep the paragraph about messages naming no file
- [x] 4.5 In `src/delta.rs`, add a doc comment above `pub enum DeltaError` and above `pub fn parse_deltas` stating the error purpose and the parse contract (whole-line markers, text outside blocks ignored, the four `MalformedDelta` layouts)
- [x] 4.6 In `src/delta.rs`, remove `pub` from `enum DeltaKind`, `enum DeltaAnchor`, and the three fields of `struct DeltaBlock`. Keep `DeltaError`, `DeltaBlock`, `find_delta_specs`, `parse_deltas`, and `check_anchor_rules` public. Confirm `cargo clippy --all-targets` and `cargo test --lib delta` report no warnings and no failures
- [x] 4.7 In the `tests` module of `src/delta.rs`, add `find_delta_specs_fails_naming_the_directory_when_it_is_missing`, asserting `DeltaError::FileReadError` whose `path` is the missing directory
- [x] 4.8 In `.claude/skills/speq-plan/references/delta-template.md`, change the Background and Description examples to `# Feature: Plan Delta Anchor Validation` with the plan-validation description and Background bullets. Leave the Anti-Pattern example unchanged
- [x] 4.9 In `.claude/skills/speq-spec-merge/SKILL.md`, add below the "Scenario-anchor markers" table the instruction to check that the target spec holds every scenario a `DELTA:CHANGED` or `DELTA:REMOVED` block names, and to stop and report the delta file and scenario name otherwise
- [x] 4.10 Prepend `# Feature: Test`, a description, `## Background`, and `## Scenarios` to the delta specs of fixtures `prose-repeated-anchor` and `prose-mixed-kinds`, and add `.stdout(predicate::str::contains("ERROR: Missing").not())` to `fails_with_repeated_anchor_of_same_kind` and `fails_with_repeated_anchor_of_mixed_kinds` in `tests/plan_validate.rs` [expert]
