# Code Review Findings: remove-record-command

## Summary
- Files reviewed: 22, plus the deleted `tests/fixtures/record/` tree
- Total findings: 8 (standard: 7, expert: 1)
- Evidence: `cargo clippy --all-targets` reports no issues, `cargo fmt --check` reports no changes, and `cargo test --test plan_validate --test cli_integration` plus `cargo test --lib delta` pass (45 and 32 tests). `cargo test` fails one test in `tests/install_mcp_servers.rs` because `/usr/bin/claude` exists on this machine. That file is outside this plan.

## Standard fixes

### src/delta.rs

#### [OUTDATED_COMMENT] DeltaAnchor doc describes the removed section-matching code
- Location: lines 38-39
- Issue: The doc says the anchor owns "the match mode, and the headings that end the section, so no caller passes a heading string or a stop prefix around". `matches_heading` and `stop_prefixes` stayed behind in the deleted `src/record.rs`. `DeltaAnchor` now only classifies a block and supplies a label for rule messages.
- Fix: In src/delta.rs, replace the second paragraph of the `DeltaAnchor` doc comment (lines 38-41) with: `/// The anchor owns the heading text of each recognized section, so rule messages name a section without a caller passing a heading string around. A block whose first non-empty line names none of the recognized sections is `Unrecognized` rather than absent, so the rule checks can report it.` Wrap it at the width of the surrounding doc lines.

#### [OUTDATED_COMMENT] check_anchor_rules doc claims the recorder agent runs this rule set
- Location: lines 212-215
- Issue: The doc says this function is "the sole owner of the `(DeltaKind, DeltaAnchor)` legality matrix, so `speq plan validate` and the recorder agent judge a delta file by one rule set". The recorder agent never calls this function. It applies the same rules by hand from the anti-pattern table in `.claude/skills/speq-spec-merge/SKILL.md`, and that skill runs only `speq feature validate` and `speq decision-log validate`. The rules therefore live in two places, and the doc hides that.
- Fix: In src/delta.rs, replace lines 212-215 of the `check_anchor_rules` doc comment with: `/// This function holds the `(DeltaKind, DeltaAnchor)` legality matrix and the repeated-anchor rule that `speq plan validate` enforces. The recorder agent applies the same rules by hand from the anti-pattern table in `/speq-spec-merge`, so a rule change here needs the same change there. The rules hold against the delta file alone and read no target spec. Whether an anchor exists in the target spec is settled at merge time instead.` Keep the last paragraph about messages naming no file unchanged.

#### [MISSING_DOC_COMMENT] Public DeltaError and parse_deltas have no doc comment
- Location: line 6 (`DeltaError`), line 102 (`parse_deltas`)
- Issue: `parse_deltas` is the entry point of the new module. Its contract is not visible from the signature: markers count only as a whole trimmed line, text outside a marker is ignored, and four marker layouts fail. `DeltaError` has no doc either.
- Fix: In src/delta.rs, add above `pub enum DeltaError` the doc comment `/// Why a plan's delta specs could not be found or a delta spec could not be parsed.` Add above `pub fn parse_deltas` the doc comment `/// Every `<!-- DELTA:* -->` block of a delta spec, in file order. A marker counts only when it is a whole line after trimming, and text outside any block is ignored. Fails with `MalformedDelta` naming the line on a nested open marker, a close marker with no open block, a close marker of another kind than the open one, or a block still open at the end of the file.`

#### [SHALLOW_MODULE] delta exposes its block data model to callers that never read it
- Location: lines 16, 43, 69-71
- Issue: `DeltaKind`, `DeltaAnchor`, and the fields `kind`, `content`, and `anchor` of `DeltaBlock` are `pub`. No code outside src/delta.rs names any of them (checked with references on `DeltaBlock` and `DeltaAnchor`). `src/plan.rs` only passes the `Vec<DeltaBlock>` from `parse_deltas` to `check_anchor_rules`. The public fields also let a caller build a `DeltaBlock` whose `anchor` disagrees with its `content`, which breaks the invariant the `DeltaBlock` doc states and `check_anchor_rules` relies on.
- Fix: In src/delta.rs, remove `pub` from `enum DeltaKind`, from `enum DeltaAnchor`, and from the three fields of `struct DeltaBlock`. Keep `DeltaError`, `DeltaBlock`, `find_delta_specs`, `parse_deltas`, and `check_anchor_rules` public. Run `cargo clippy --all-targets` and `cargo test --lib delta` and confirm no warnings and no failures.

#### [UNTESTED_ERROR_PATH] find_delta_specs read failure has no test
- Location: lines 85-88
- Issue: `find_delta_specs_recursive` returns `DeltaError::FileReadError` when a directory cannot be read. No unit or integration test reaches this path. `find_delta_specs_returns_paths_in_a_stable_order` covers only the success path.
- Fix: In the `tests` module of src/delta.rs, add the test `find_delta_specs_fails_naming_the_directory_when_it_is_missing`. Create a `TempDir`, set `let missing = tmp.path().join("missing");`, call `find_delta_specs(&missing)`, and assert `matches!(result, Err(DeltaError::FileReadError { path }) if path == missing.display().to_string())`.

### .claude/skills/speq-plan/references/delta-template.md

#### [OUTDATED_COMMENT] Background and Description examples describe the removed record command
- Location: lines 70-78 and 90-92
- Issue: Both examples use `# Feature: CLI Record`. The Background example keeps the description "The CLI SHALL provide a command to record approved plan deltas into permanent specs" and the bullet "Merges every delta spec of the plan before writing any permanent spec", but its command bullet now reads `speq plan validate <plan-name>`. The Description example says "The CLI SHALL merge every approved delta of a plan into permanent specs in one atomic write, then archive the plan". The examples now describe a command that no longer exists, and the Background example mixes two commands.
- Fix: In .claude/skills/speq-plan/references/delta-template.md, change the Background example (lines 70-78) to `# Feature: Plan Delta Anchor Validation`, a blank line, `Validates the anchors of a plan's spec delta blocks during plan validation.`, a blank line, then the `DELTA:CHANGED` block holding `## Background`, a blank line, `* Command syntax: `speq plan validate <plan-name>``, and `* A recognized anchor MUST NOT appear in more than one delta block of a delta file`. Change the Description example (lines 90-92) to `# Feature: Plan Delta Anchor Validation`, a blank line, and `Validates the anchors of a plan's spec delta blocks during plan validation. Catches an unrecognized, illegal, or repeated anchor before implementation begins.` Leave the Anti-Pattern example unchanged.

### .claude/skills/speq-spec-merge/SKILL.md

#### [OUTDATED_COMMENT] Anti-pattern row promises a stop that the merge procedure never instructs
- Location: line 125, and the "Apply Deltas" section at lines 20-50
- Issue: The row now says a `DELTA:CHANGED` or `DELTA:REMOVED` block naming a scenario absent from the target spec is rejected because "recorder-agent stops instead of silently merging nothing". Before this change, `speq record` enforced that. The "Apply Deltas" procedure that recorder-agent follows has no step that checks the scenario exists or tells it to stop. Decision [2] of the plan relies on recorder-agent checking anchor presence at record time.
- Fix: In .claude/skills/speq-spec-merge/SKILL.md, directly below the "Scenario-anchor markers" table (after line 38), add the line: `Before you merge a delta spec, check that the target spec holds every scenario a `DELTA:CHANGED` or `DELTA:REMOVED` block names. If one is missing, stop and report the delta file and the scenario name. Do not merge that delta spec.`

## Expert fixes

### tests/plan_validate.rs

#### [ASSERTION_FREE_TEST] Exit-code assertion of the repeated-anchor tests passes without the anchor rule
- Location: `fails_with_repeated_anchor_of_same_kind` and `fails_with_repeated_anchor_of_mixed_kinds` (lines 407-436); fixtures `tests/fixtures/plan_validate/prose-repeated-anchor/test/feature/spec.md` and `tests/fixtures/plan_validate/prose-mixed-kinds/test/feature/spec.md`
- Issue: Both delta fixtures contain only the two delta blocks, with no `# Feature:` heading, description, `## Background`, or `## Scenarios`. Running `speq plan validate prose-repeated-anchor` on the fixture prints the anchor error and also `ERROR: Missing feature description`, `ERROR: Missing Background section`, and `ERROR: Missing Scenarios section`, then exits 1. The `.code(1)` assertion is therefore satisfied by the structure errors alone. If the anchor rule stopped failing the plan, both tests would still pass, so the scenario step "the system SHALL exit with non-zero code" is not verified for the anchor rule. With the four sections prepended, the same command prints only the anchor error and still exits 1.
- Fix: In both fixture files, prepend these lines above the first `<!-- DELTA:` marker: `# Feature: Test`, blank, `Description.`, blank, `## Background`, blank, `* Context.`, blank, `## Scenarios`, blank. In tests/plan_validate.rs, add `.stdout(predicate::str::contains("ERROR: Missing").not())` to the assertion chain of both `fails_with_repeated_anchor_of_same_kind` and `fails_with_repeated_anchor_of_mixed_kinds`. Run `cargo test --test plan_validate delta_anchors` and confirm both tests pass.
