# Plan Review Findings: add-patch-skill

Scope: the no-baseline revision only. That is § Requirements rows Test and Report, § PR body, decision-log [4], tasks 5.1 to 5.4, and the matching Parallelization, Dead Code Removal, and Manual Testing rows.

## Summary
- Axes checked: 6/6
- Total findings: 8 (Blockers: 2, Findings: 6)
- Intent Fidelity blockers: 0

## Failure Modes

1. The patch breaks a build or a test outside its changed files: a signature change breaks an unchanged caller, or a change in `src/` breaks an unchanged integration test that runs the binary. The Test row labels that failure "outside the patch". The run never fixes it and ships a ready PR with the regression listed as someone else's failure. Routed to Requirement Quality `[COMPLETENESS_GAP]` BLOCKER.
2. A suite whose only failure is outside the patch exits non-zero. The Test row imports `/speq-implement-pr` step 5's exit-code verdict and also defines red by log content. An orchestrator that follows the exit code counts the suite red, cannot fix the failure, and stops after 3 runs. The PR the user expects never opens. Routed to Requirement Quality `[REQUIREMENT_CONFLICT]` BLOCKER.
3. The revised behavior ships with no executed check. Task 5.4 runs a grep, a byte count, and the packaging rows. No task runs the new Manual Testing row. Routed to Task Breakdown `[TRACEABILITY_GAP]` FINDINGS.

## Intent Fidelity

No objection, axis checked: each clause of the request maps to the revision.
- "Remove the up-front baseline test run": task 5.1 deletes step 4 item 3 and renames step 4 to `### 4. Branch`.
- "after the change, run the suites and lint": Test row "After the change, run the real test suites and the lint command".
- "fix it only when it is in code the patch touched": Test row "A suite is red when its log has a failure in code the patch touched. Red → fix and rerun".
- "reported in the report and PR body, never fixed": Test row, Report row, § PR body `Verification` line, task 5.1 (step 11), task 5.2.
- "after 3 red runs of the same suite, stop and report": the Test row keeps that sentence.
- "Drop the baseline log files, baseline resume rules, and the PR body 'baseline failures' wording": task 5.1 removes `target/speq-baseline-<patch-name>-<suite>.log`, the step 4 item 2 resume sentence, and the step 8 baseline condition. Task 5.2 replaces the template's `baseline failures` field and rule.
- "Update docs/patch.md to match": task 5.3.

Interview lines 9 and 10 record both decisions. The added `<failing test or error location>` field and the Manual Testing row "Failure outside the patch" trace to "reported in the report and PR body". This is not scope creep. No clause is dropped.

## Feasibility

Checked: every task 5.1 anchor occurs exactly once in `.claude/skills/speq-patch/SKILL.md` (lines 50, 53, 54, 59, 85, 117). A simulation of the six edits gives 7953 bytes (cap 8000) and leaves no "baseline". Task 5.2's anchors are `pr-body-template.md:14,22`, and task 5.3's anchor is `docs/patch.md:22`. `docs/patch.md` § Resume (`:58-60`) has no baseline text. A repo-wide `grep -rni baseline .claude/ docs/ README.md scripts/ tests/` finds baseline text only in those three files, plus an unrelated use at `.claude/skills/speq-planning/SKILL.md:39`, so task 5.4's grep scope is sufficient.

#### [UNSTATED_ASSUMPTION] FINDINGS
- Location: plan.md § Requirements, rows "Test" and "Review"; `.claude/skills/speq-patch/SKILL.md` step 6
- Issue: The Test row decides "in code the patch touched" from "the changed files per `/speq-implement` Phase 4 step 1, with base `origin/<default-branch>`". Phase 4 step 1 runs `git diff --name-only <base>`, which compares the working tree with the current tip of `origin/<default-branch>`. The rule assumes that tip is still the branch point. A fetch moves it. `checkout` of a remote-only branch runs `git fetch` (`/speq-git-operations`, `checkout`), and so does a user fetch or pull between a stopped run and its resume. The list then contains every file changed upstream since the branch point. A failure in such a file counts as in code the patch touched, and the run fixes code the patch did not touch.
- Fix: In plan.md § Requirements rows "Test" and "Review", replace "with base `origin/<default-branch>`" with "with base `git merge-base origin/<default-branch> HEAD`". Add to task 5.1: "In step 6, replace ``with base `origin/<default-branch>`. Spawn`` with ``with base `git merge-base origin/<default-branch> HEAD`. Spawn``." With both BLOCKER fixes and the step 11 change below, the simulated file measures 7992 bytes.

#### [UNSTATED_ASSUMPTION] FINDINGS
- Location: plan.md § Verification, Manual Testing, row "Failure outside the patch"
- Issue: "In the same clone, push a failing test to the default branch". "The same clone" is the row Fix run's "throwaway clone of a speq project with a GitHub remote". A throwaway clone can still point at a shared remote. The row needs the push, because the skill branches from `origin/<default-branch>`. That push breaks the remote's CI for everyone, and branch protection blocks it on most protected repositories.
- Fix: In that row, replace "In the same clone, push a failing test to the default branch" with "In a clone whose `origin` is a throwaway GitHub repository that accepts direct pushes to its default branch, push a failing test to the default branch".

## Requirement Quality

#### [COMPLETENESS_GAP] BLOCKER
- Location: plan.md § Requirements, row "Test"; task 5.1 (step 5 item 2 replacement); § Verification, Manual Testing, row "Failure outside the patch"
- Issue: The row counts a failure as in code the patch touched only when the changed files "include the failing test's file, a file with code that test calls directly, or the file a lint or build error names". "Every other failure is outside the patch: name it in the report and the PR body, and never fix it", and the suite is not red. Three failures that the patch itself causes fall outside this rule:
  1. A build error in an unchanged caller. The patch changes a signature in a changed file, and the compiler reports the error at the call site in an unchanged file. "The file a lint or build error names" is that unchanged file. The run never fixes the error and ships a ready PR that does not compile.
  2. An integration test that runs the binary. The integration tests in `tests/` start the CLI through `Command::new(assert_cmd::cargo::cargo_bin!("speq"))` (for example `tests/cli_integration.rs:62`), so they call no project file directly. A change in `src/record.rs` that breaks an unchanged test in `tests/cli_integration.rs` counts as outside the patch.
  3. A transitive call. An unchanged test calls an unchanged function, which calls a changed one.

  In each case the defect is in code the patch changed. That breaks the intent "fix it only when it is in code the patch touched", and it contradicts decision-log [4]'s Rationale "a failure after the change is almost always in code the patch touched". The PR body then lists the patch's own regression under `failures outside the patch`.
- Fix:
  1. In plan.md § Requirements row "Test", replace "include the failing test's file, a file with code that test calls directly, or the file a lint or build error names" with "include the failing test's file, a file the failure output names, or a file with code that the failing test or the erroring line reaches, directly or through the binary under test".
  2. In task 5.1, replace "with the Test row's sentences on code the patch touched, failures outside the patch, and red suites" with this verbatim text: "A failure is in code the patch touched when the step 6 changed files include its test's file, a file its output names, or a file with code its test or erroring line reaches, directly or through the binary under test. Every other failure is outside the patch: name it in the report and the PR body, and never fix it." Follow it with the red-suite sentence from the `[REQUIREMENT_CONFLICT]` BLOCKER below. The reference to step 6 keeps a single changed-files definition in the skill. With both BLOCKER fixes, the simulated file measures 7945 bytes.
  3. In § Manual Testing row "Failure outside the patch", replace "calling only code the patch does not change" with "reaching only code the patch does not change".
  4. Add a § Manual Testing row. Feature: "Regression in a test the patch did not edit". Command: "In the same clone, run `/speq:patch change <output that an existing integration test asserts>`". Expected: "The diff updates that test or the code. The PR body `Verification` line lists no failure outside the patch".

#### [REQUIREMENT_CONFLICT] BLOCKER
- Location: plan.md § Requirements, row "Test"; task 5.1 (step 5 item 2 replacement)
- Issue: The row runs the suites "with logs and exit codes per `/speq-implement-pr` step 5". That step says "Judge green or red by exit code. If any suite is red, report the failed suites ..., **stop**" (`.claude/skills/speq-implement-pr/SKILL.md:77`). The same row then defines red differently: "A suite is red when its log has a failure in code the patch touched." Take a suite whose only failure is outside the patch. Its exit code is non-zero, so it is red under step 5 and not red under the row. The outcome depends on which definition the orchestrator follows: a ready PR under the row, or no fix possible and a stop after 3 runs under step 5. The stop breaks the intent that such a failure is "reported in the report and PR body", and it fails the new Manual Testing row "Failure outside the patch". This misreading has already happened: this plan's `code-reviewer` read the same reference as an exit-code verdict ("logs the baseline run per `/speq-implement-pr` step 5, which ... judges green or red by exit code", `review-findings.md:48`).
- Fix: In plan.md § Requirements row "Test", replace "with logs and exit codes per `/speq-implement-pr` step 5" with "with logs per `/speq-implement-pr` step 5". In the same row, replace "A suite is red when its log has a failure in code the patch touched." with "Whatever its exit code, a suite is red only when its log has a failure in code the patch touched." In task 5.1, use that same sentence as the red-suite sentence of the step 5 item 2 replacement.

#### [AMBIGUOUS_REQUIREMENT] FINDINGS
- Location: plan.md § PR body; task 5.2; § Requirements row "Report"; task 5.1 (step 11 replacement)
- Issue: The skeleton `Verification: <suite>: pass[, <suite>: pass][; failures outside the patch: ...]` allows only one result for a suite: `pass`. A suite whose only failures are outside the patch exits non-zero, yet the line reads, for example, "cargo test: pass; failures outside the patch: cargo test tests/a.rs::b". The template rule "with its result" leaves open whether to write `pass`. The two outputs also describe the same failure differently: the Report row and the step 11 replacement give "each failure outside the patch with its suite", while the PR body gives "<suite> <failing test or error location>". The report does not say which test failed.
- Fix: In plan.md § PR body and task 5.2, set the skeleton line to `Verification: <suite>: <pass | fail outside the patch>[, ...][; failures outside the patch: <suite> <failing test or error location>[, ...]]`. In § Requirements row "Report" and task 5.1's step 11 replacement, write "each failure outside the patch with its suite and test or error location".

#### [COMPLETENESS_GAP] FINDINGS
- Location: plan.md § Requirements, row "Test"; task 5.1
- Issue: "Every other failure is outside the patch" includes a suite that fails before any test runs. Examples are a build broken outside the patch, a crash, a timeout, or a missing tool. That log has no failure in code the patch touched, so the suite is not red. The run ships a ready PR in which that suite never ran any test, including the patch's own reproducing test.
- Fix: In plan.md § Requirements row "Test" and in task 5.1's red-suite sentence, write "Whatever its exit code, a suite is red only when its log has a failure in code the patch touched or it ran no test." A red suite whose cause is outside the patch is never fixed, so the run stops after 3 runs and reports. With every fix in this review applied, the simulated file measures 8010 bytes. To stay at most 8000, add to task 5.1: in step 5, shorten "name it in the report and the PR body, and never fix it" to "report it and never fix it". Step 10 (through `pr-body-template.md`) and step 11 already name these failures. The file then measures 7981 bytes.

## Task Breakdown

Checked: group C runs after A and B because it edits their files. Its Knowledge column lists exactly the three files tasks 5.1 to 5.3 edit. Tasks 5.1 to 5.3 each edit one file, and 5.4 verifies them. The Dead Code Removal rows match the anchors at `SKILL.md:50-59,85,117`, `pr-body-template.md:14,22`, and `docs/patch.md:22`.

#### [TRACEABILITY_GAP] FINDINGS
- Location: plan.md task 5.4; § Verification, Manual Testing, row "Failure outside the patch"
- Issue: Task 5.4 reruns "the Manual Testing rows Plugin packaging, Command rename, and Codex name". Those rows check packaging. The grep and the `wc -c` check prove that text was removed. No task runs the row "Failure outside the patch", which is the only check of the revised behavior. `verification-report.md` records the interactive rows as "not run", so the revision has no executed check of its new rule.
- Fix: In task 5.4, append "and the Manual Testing rows Failure outside the patch and Regression in a test the patch did not edit". If interactive rows stay unrun, replace that addition with "Record the Manual Testing rows Failure outside the patch and Regression in a test the patch did not edit as not run in `verification-report.md`".

## Design Depth

No objection, axis checked. The revision adds no module, interface, or boundary. It removes a step and its log files. Decision-log [4] has `Promotes to ADR: no`, which is correct: rule 3 excludes a workaround specific to one skill's mechanics, and no rule-2 criterion applies. The revision changes skill text only. `specs/architecture.md` describes the `speq` CLI only, so the revision needs no architecture delta and plan.md states none. The ADR digest lists 5 accepted ADRs (`anchor-first-nonempty-line`, `plan-validate-skips-anchor-presence`, `record-all-or-nothing-across-plan`, `per-plan-decision-fragments`, `slug-identity-not-sequential-numbers`). None concerns test runs, so there is no conflict. Task 5.1 as written repeats step 6's changed-files definition in step 5. The `[COMPLETENESS_GAP]` BLOCKER's fix replaces that repeat with a reference to step 6.

## Prose Quality

#### [PROSE_UNCLEAR] FINDINGS
- Location: plan.md § Implementation Tasks (groups 1, 2, 3, 5), task 5.1, § Dead Code Removal row 3
- Issue: plan.md skips from group 3 to group 5. It cites "review-fix tasks 4.6, 4.7, and 4.8" and "added by review-fix tasks 4.6-4.8", but those tasks exist only in `tasks.md` § Phase 4: Review Fixes. A reader of plan.md alone cannot resolve them.
- Fix: In task 5.1, replace "These edits remove the text of review-fix tasks 4.6, 4.7, and 4.8." with "These edits remove the text that `tasks.md` § Phase 4 tasks 4.6 to 4.8 added." In § Dead Code Removal row 3, replace "added by review-fix tasks 4.6-4.8" with "added by `tasks.md` § Phase 4 tasks 4.6 to 4.8".
