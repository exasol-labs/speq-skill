# Verification Report: remove-record-command

## Verdict

| Result | Details |
|--------|---------|
| **PASS with one unrelated failure** | `speq record` is removed and `plan validate` behaves as specified. One test in `tests/install_mcp_servers.rs` fails because of the host environment, not this change. |
| Code review | 8 findings, 8 fixed |

| Check | Status |
|-------|--------|
| Build | ✓ |
| Tests | ✗ 1 failure, unrelated (see Notes) |
| Lint | ✓ |
| Format | ✓ |
| Scenario Coverage | ✓ |
| Manual Tests | ✓ |

## Test Evidence

### Coverage

Not measured. `cargo llvm-cov` stops with a non-zero exit because of the failing environment test and prints no totals.

| Type | Coverage % |
|------|------------|
| Unit | not measured |
| Integration | not measured |

### Test Results

`cargo test --no-fail-fast`, all binaries:

| Type | Run | Passed | Ignored |
|------|-----|--------|---------|
| Unit (lib) | 147 | 147 | 0 |
| Integration (all test binaries) | 99 | 98 | 0 |

The failing test is `offer_does_nothing_when_no_host_is_installed` in `tests/install_mcp_servers.rs`.

### Manual Tests

| Test | Result |
|------|--------|
| `./target/debug/speq record x` | ✓ `error: unrecognized subcommand 'record'` |
| `./target/debug/speq --help` | ✓ no `record` command; only `decision-log` mentions "records" |
| `plan validate` on a plan with two `DELTA:NEW` blocks on one anchor | ✓ error names the anchor `### Scenario: Existing one` (DELTA:NEW, DELTA:NEW) |
| `plan validate` on a plan with `DELTA:NEW` and `DELTA:CHANGED` on one anchor | ✓ error names the anchor (DELTA:NEW, DELTA:CHANGED) |

## Tool Evidence

### Linter

```
cargo clippy --all-targets: exit 0, 0 warnings
```

### Formatter

```
cargo fmt --check: exit 0, no changes
```

## Scenario Coverage

| Domain | Feature | Scenario | Test Location | Test Name | Passes |
|--------|---------|----------|---------------|-----------|--------|
| plan | plan-validate-anchors | Validate plan with a repeated anchor of the same kind reports an error | `tests/plan_validate.rs` | `fails_with_repeated_anchor_of_same_kind` | Pass |
| plan | plan-validate-anchors | Validate plan with a repeated anchor of different kinds reports an error | `tests/plan_validate.rs` | `fails_with_repeated_anchor_of_mixed_kinds` | Pass |
| plan | plan-validate-anchors | Existing scenarios (CHANGED Background or description, NEW/REMOVED on prose anchor, unrecognized anchor, new feature, malformed marker) | `tests/plan_validate.rs` | unchanged, 28 tests in the suite | Pass |

## Notes

- `offer_does_nothing_when_no_host_is_installed` asserts that `claude` is absent from `/usr/bin:/bin`. On this machine `/usr/bin/claude` exists, so the assertion fails. The test and the installer it covers are not touched by this plan. The failure was not reproduced on the base branch.
- `.claude/skills/speq-plan/references/delta-template.md` lines 13 and 21 still say bare `record` ("`record` and `plan validate` read only that line"). A reader may take this for the removed command. No review finding named it.
