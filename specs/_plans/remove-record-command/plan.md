# Plan: remove-record-command

## Summary

Remove the `speq record` command and its merge, write and archive code. Keep delta parsing and anchor rules in a slim `delta` module for `speq plan validate`.

## Context

- The `/speq-record` skill and `recorder-agent` merge deltas by hand per `/speq-spec-merge`. No skill calls `speq record`.
- `speq plan validate` uses `parse_deltas`, `check_anchor_rules` and `find_delta_specs` from `src/record.rs`.
- Rejecting a repeated anchor is enforced by `plan validate` but only specified under `plan/record-anchors`.
- ADR `record-all-or-nothing-across-plan` describes the removed command and is left unchanged. ADR `plan-validate-skips-anchor-presence` still holds: the recorder checks anchor presence, not `plan validate`.

## Features

| Feature | Status | Spec |
|---------|--------|------|
| plan/plan-validate-anchors | CHANGED | `plan/plan-validate-anchors/spec.md` |
| plan/record | REMOVED | none: delete `specs/plan/record/` in task 3.1 |
| plan/record-anchors | REMOVED | none: delete `specs/plan/record-anchors/` in task 3.1 |

## Impact

Breaking: `speq record` is removed. Callers must use the `/speq-record` skill. Specs, deltas and `plan validate` behave as before.

## Implementation Tasks

1. Code
   - [ ] 1.1 Create `src/delta.rs` with `DeltaKind`, `DeltaAnchor`, `DeltaBlock`, `parse_deltas`, `check_anchor_rules`, `find_delta_specs`, a `DeltaError` enum limited to `FileReadError` and `MalformedDelta`, and their unit tests, moved from `src/record.rs`. Leave `PlanNotFound`, `FileWriteError`, `DirCreateError`, `DirMoveError`, `InvalidDeltaAnchor`, `DeltaAnchor::matches_heading` and `stop_prefixes` behind
   - [ ] 1.2 Delete `src/record.rs`; drop `pub mod record` and add `pub mod delta` in `src/lib.rs`; point `src/plan.rs` imports and doc comments at `delta`
   - [ ] 1.3 Remove the `Record` command from `src/cli.rs` and `handle_record_command` from `src/main.rs`
2. Tests
   - [ ] 2.1 Delete `mod record` in `tests/cli_integration.rs` and `tests/fixtures/record/`
   - [ ] 2.2 Add `plan_validate` tests and fixtures `prose-repeated-anchor` and `prose-mixed-kinds` for the two new scenarios, each with a target spec at `tests/fixtures/plan_validate/_targets/<fixture>/<domain>/<feature>/spec.md` holding the anchored scenario, and call `setup_target_specs` in both tests
3. Specs and docs
   - [ ] 3.1 Delete `specs/plan/record/` and `specs/plan/record-anchors/`
   - [ ] 3.2 Remove `speq record` from `docs/cli-reference.md` (table row and section), `.claude/skills/speq-cli/SKILL.md`, and the `record` line in the mock help text of `tests/docker/test-docker.sh`
   - [ ] 3.3 Reword `speq record` mentions in `.claude/skills/speq-plan/references/delta-template.md` (3 places) to name the `/speq-record` skill or `plan validate`. In `.claude/skills/speq-spec-merge/SKILL.md`, reword line 125 (missing scenario anchor) to name recorder-agent, never `plan validate`, and line 126 (heading outside the marker) to name `plan validate` and recorder-agent
   - [ ] 3.4 In `specs/mission.md`, delete Core Capability 5 and renumber capability 6 as 5. Change the Out of Scope bullet to `Version control — Does not manage spec history`

## Parallelization

| Group | Tasks | Depends on | Knowledge |
|-------|-------|------------|-----------|
| A: delta module | 1.1-1.3, 2.1-2.2 | none | spec delta `plan/plan-validate-anchors`; `src/record.rs`, `src/plan.rs`, `src/cli.rs`, `src/main.rs`, `src/lib.rs`, `tests/plan_validate.rs`, `tests/cli_integration.rs` |
| B: docs and skills | 3.1-3.4 | none | `docs/cli-reference.md`, `specs/plan/`, `specs/mission.md`, `.claude/skills/speq-cli/`, `.claude/skills/speq-spec-merge/SKILL.md`, `.claude/skills/speq-plan/references/` |

## Dead Code Removal

| Type | Location | Reason |
|------|----------|--------|
| Module | `src/record.rs` (merge, write, archive, `record_plan`) | Command removed |
| Function | `handle_record_command` in `src/main.rs` | Command removed |
| Test | `mod record` in `tests/cli_integration.rs` | Tests the removed command |
| Fixtures | `tests/fixtures/record/` | Used only by those tests |
| Error variants | `RecordError::{PlanNotFound, FileWriteError, DirCreateError, DirMoveError, InvalidDeltaAnchor}`, `DeltaAnchor::matches_heading`, `stop_prefixes` | Merge, write and archive only |
| Specs | `specs/plan/record/`, `specs/plan/record-anchors/` | Describe the removed command |

## Verification

### Scenario Coverage

| Scenario | Test Type | Test Location | Test Name |
|----------|-----------|---------------|-----------|
| Validate plan with a repeated anchor of the same kind reports an error | Integration | `tests/plan_validate.rs` | `fails_with_repeated_anchor_of_same_kind` |
| Validate plan with a repeated anchor of different kinds reports an error | Integration | `tests/plan_validate.rs` | `fails_with_repeated_anchor_of_mixed_kinds` |
| Existing `plan-validate-anchors` scenarios | Integration | `tests/plan_validate.rs` | unchanged, must still pass |

### Manual Testing

| Feature | Command | Expected Output |
|---------|---------|-----------------|
| plan/plan-validate-anchors | `./target/debug/speq plan validate <plan>` on a plan with two blocks on one anchor | Error naming the anchor, non-zero exit |
| record removal | `./target/debug/speq record x` | Clap error `unrecognized subcommand`, non-zero exit |
| record removal | `./target/debug/speq --help` | No `record` entry |

### Checklist

| Step | Command | Expected |
|------|---------|----------|
| Build | `cargo build --release` | Exit 0 |
| Test | `cargo test` | 0 failures |
| Lint | `cargo clippy` | 0 warnings |
| Format | `cargo fmt --check` | No changes |
