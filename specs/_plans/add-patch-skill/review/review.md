# Plan Review Findings: add-patch-skill

## Summary
- Axes checked: 6/6
- Total findings: 17 (Blockers: 2, Findings: 15)
- Intent Fidelity blockers: 0

## Failure Modes

1. A suite that is red on the default branch, or a `§ Commands` entry that is not a test suite, traps the run in the Test row's "Red → fix and rerun" loop. The patch then grows with unrelated fixes or does not end. Routed to Requirement Quality `[COMPLETENESS_GAP]` BLOCKER.
2. A correct build fails the plan's own verification: the "Command rename" check expects zero `/speq-` matches, but the Test row puts `target/speq-<suite>.log` into the skill, and the build keeps that path unchanged. Routed to Requirement Quality `[REQUIREMENT_CONFLICT]` BLOCKER.
3. Spec, ADR, and architecture text is written after the single `code-reviewer` pass. It reaches a ready PR with no review and no manual test. The user's main prose concern (bloat in specs and decisions) has no check. Routed to Requirement Quality `[COMPLETENESS_GAP]` findings on the Specs, Architecture, ADR, and Verification sections.

## Intent Fidelity

Checked: each of the 8 interview answers maps to a Requirements row (branch and PR → Branch, Ship; direct edits → Specs, ADR; ask on ambiguity → Questions; warn and continue → Patch size limit; inline coding and one review → Code, Review; version → Version; artifacts → Specs, ADR, Architecture, Docs; ready PR and hook → Ship, Hook). Task 1.3 serves the third PR-title consumer named in § Context, so it is not scope creep.

#### [SCOPE_REDUCTION] FINDINGS
- Location: plan.md § Requirements, row "Docs"
- Issue: The user asked to derive "any needed speq changes / decisions / docs", and the interview answer is "user docs". The row narrows this to "Edit `README.md` and `docs/` only where the change alters documented behavior". A project whose user docs live elsewhere (`doc/`, a docs-site directory, a root `USAGE.md`) gets no doc edit, and the row reads as a ban on those paths.
- Fix: In plan.md § Requirements, row "Docs", replace the path list with "Edit the project's user docs (by default `README.md` and `docs/`) only where the change alters documented behavior". Use the same wording in task 2.3's "the files a patch may change".

## Feasibility

#### [HIDDEN_DEPENDENCY] FINDINGS
- Location: plan.md § Requirements, rows "Prerequisite" and "Ship"
- Issue: The run needs an `origin` remote and an authenticated `gh`, because `ship-ready` runs `push` and `gh pr create`. The Prerequisite row checks only `specs/mission.md`. Without a remote or `gh` auth, the run fails at Ship, after branching, coding, a commit, and spec edits. `/speq-implement-pr` does not meet this case because `/speq-plan-pr` already pushed and opened the PR. `/speq-patch` is the first git contact.
- Fix: In plan.md § Requirements, row "Prerequisite", add: "`gh auth status` fails or the repository has no `origin` remote → stop before any edit and name the missing piece".

#### [HIDDEN_DEPENDENCY] FINDINGS
- Location: plan.md § Requirements, rows "Discovery" and "Specs"
- Issue: A patch edits `specs/<domain>/<feature>/spec.md` directly. An active plan in `specs/_plans/` can hold a `DELTA:CHANGED` or `DELTA:REMOVED` block that names a scenario in the same feature. If the patch renames or removes that scenario, the plan breaks. `speq plan validate` does not catch it, because ADR `plan-validate-skips-anchor-presence` keeps plan validation from reading the target spec. The break appears only when that plan's `speq record` rejects the missing anchor, at the end of its implement run. The Discovery row runs `speq feature list` and the ADR digest, not `speq plan list`.
- Fix: In plan.md § Requirements, row "Discovery", add `speq plan list`. In row "Specs", add: "An active plan holds a delta for a feature the patch edits → name the plan and the feature in the report and the PR body".

#### [UNSTATED_ASSUMPTION] FINDINGS
- Location: plan.md § Patch size limit, point 2; § Requirements, row "Branch"
- Issue: `git diff --shortstat <default-branch>...HEAD` assumes the local default branch is the base that `create-branch` used. `create-branch` runs `git fetch` and branches off the "repository default branch" without saying whether that is the local or the remote ref. If the branch starts at `origin/<default-branch>` while the local default branch is behind, the three-dot diff counts every upstream commit since the local tip. The 10-file or 400-line limit then fires on work the patch did not do.
- Fix: In plan.md § Requirements, row "Branch", pass `base: origin/<default-branch>` to `create-branch`. In § Patch size limit, point 2, measure `git diff --shortstat origin/<default-branch>...HEAD`.

#### [EFFORT_MISESTIMATION] FINDINGS
- Location: plan.md § Implementation Tasks, task 1.1
- Issue: "Keep the file at most 6 KB" conflicts with the content the task must carry. Measured in plan.md, the Requirements table is 4.0 KB, the size-limit section is 0.8 KB, and the PR body is 0.3 KB. The step-0 hook wording copied from `/speq-implement-pr` adds 0.36 KB. That is 5.5 KB before the frontmatter, the Required Skills list, the step headings, the `code-reviewer` brief, and the `create-branch`, `commit`, and `ship-ready` operation blocks (0.35 KB each in `/speq-implement-pr`). `/speq-implement-pr`, which delegates all coding, is 7.7 KB. No project rule sets a 6 KB limit. The implementer must drop requirements or break the cap.
- Fix: In plan.md task 1.1, replace "Keep the file at most 6 KB" with "Keep the file at most 8 KB. If it does not fit, move the PR body into `.claude/skills/speq-patch/references/pr-body-template.md`".

## Requirement Quality

#### [COMPLETENESS_GAP] BLOCKER
- Location: plan.md § Requirements, row "Test"
- Issue: "Run every suite in `specs/mission.md § Commands` ... Red → fix and rerun" has no baseline, no bound, and no defined command set.
  1. A suite that is already red or flaky on the default branch makes the skill "fix" code the patch did not cause. This breaks the user's "Ensure that changes are concise".
  2. The loop has no attempt limit and no stop-and-report. `/speq-implement-pr` step 5 stops on red.
  3. "every suite" is not actionable. This repository's `§ Commands` lists `cargo build --release`, `cargo test`, `cargo fmt && cargo clippy`, and `cargo llvm-cov`. The last is a coverage run, and `cargo fmt` rewrites files. The Prerequisite row says "build, test, and lint commands", the Test row says "every suite", and `/speq-implement-pr` says "the project's real test suites". No pass/fail check can be written for which commands run.
- Fix: In plan.md § Requirements, row "Test":
  1. Name the commands: the test suites and the lint command from `specs/mission.md § Commands`, per `/speq-implement-pr` step 5's "real test suites".
  2. Add: "Run the suites once on the new branch before the first edit. A suite red there is a baseline failure: name it in the report and the PR body, and never fix it. After the change, a suite passes when it has no failure beyond the baseline."
  3. Add: "After 3 red runs of the same suite, stop and report the suite with at most `tail -n 30` of its log, and the branch state."

#### [REQUIREMENT_CONFLICT] BLOCKER
- Location: plan.md § Verification, Manual Testing, row "Command rename"; § Requirements, row "Test"
- Issue: The row expects `grep -c '/speq-' dist/marketplace/plugins/speq-skill/skills/patch/SKILL.md` to print `0`. The Test row requires the skill to name `target/speq-<suite>.log`. `namespace_slash_commands` in `scripts/plugin/build.sh:69` skips `/speq-` when a path character precedes it, so `target/speq-build.log` survives the build unchanged. A test run of that regex on the text "log to target/speq-build.log" leaves `/speq-` in place, and `grep -c '/speq-'` prints `1`. A correct build fails this check. The implementer must either break the Test row or record a failed verification.
- Fix: In plan.md § Verification, Manual Testing, row "Command rename", change the command to `grep -cE '(^|[^A-Za-z0-9_./-])/speq-' dist/marketplace/plugins/speq-skill/skills/patch/SKILL.md` with expected output `0`. This pattern matches only slash commands the build failed to rename.

#### [COMPLETENESS_GAP] FINDINGS
- Location: plan.md § Requirements, rows "Branch" and "Report"; § Patch size limit, last paragraph
- Issue: Three states have no next step.
  1. A run that stops (usage limit, a red stop, a "no" at size point 2) leaves `patch/<patch-name>` with commits. A second `/speq-patch <intent>` reaches `create-branch` for a branch that already exists. No row says whether to check it out and continue. `/speq-implement-pr` and `/speq-plan-pr` both define a rerun.
  2. The Branch row asks whether uncommitted changes belong to the patch, but says nothing for "no". `create-branch` then carries the unrelated changes onto the patch branch.
  3. A "no" at point 2 recommends `/speq-plan`, but the implementation is already committed on the branch. The plan does not say what the user does with it.
- Fix: In plan.md § Requirements, row "Branch", add: "Branch `patch/<patch-name>` exists → `checkout` it and continue from the first missing output (implementation commit, spec commit, PR)". Add: "The changes do not belong → stop and ask the user to commit or stash them. Never stash or discard them yourself." In § Patch size limit, state that a "no" at point 2 reports the branch name and commit, so the user can keep the branch for a `/speq-plan` run or delete it. Add the rerun to the `docs/patch.md` outline in task 2.3.

#### [COMPLETENESS_GAP] FINDINGS
- Location: plan.md § Requirements, row "Specs"; task 1.1 Required skills
- Issue: The row covers restored, uncovered, wrong, and unchanged behavior. It does not cover removed behavior, although `remove` is a valid patch verb in the Name row. The row also keeps only one of the spec-authoring rules that `/speq-planning` § Rules owns: "Background at most 5 bullets". It drops "Background never encodes features, behavior, or requirements" and "Specs describe the software, not its tests". `speq feature validate` checks neither rule. In the plan flow, `plan-reviewer`'s `[IMPLEMENTATION_DETAIL]` check catches these defects. A patch has no reviewer for spec text, and the user named spec bloat as the main concern.
- Fix: In plan.md § Requirements, row "Specs", add: "Removed behavior → delete the scenario and the tests that covered it, and delete the feature directory when no scenario remains". Replace "Background at most 5 bullets" with "Follow the Short Background and Specs-describe-the-software rules in `/speq-planning` § Rules". In task 1.1, add to Required skills: "`/speq-planning`, § Rules only".

#### [COMPLETENESS_GAP] FINDINGS
- Location: plan.md § Requirements, row "Architecture"
- Issue: The row edits `specs/architecture.md` in place with no format rule and no check. In the plan flow, `/speq-spec-merge` "Apply Architecture Delta" step 5 post-checks the result: H1 `# Architecture`, the six canonical sections once each and in order, no duplicate `##`, and the line rules of `/speq-plan`'s `references/architecture-template.md`. A direct edit by a patch can break that format, and nothing reports it before the PR.
- Fix: In plan.md § Requirements, row "Architecture", add: "Keep the format of `/speq-plan`'s `references/architecture-template.md`. Then run the post-check of `/speq-spec-merge` 'Apply Architecture Delta' step 5. On failure, restore the file and report."

#### [COMPLETENESS_GAP] FINDINGS
- Location: plan.md § Requirements, row "ADR"; § PR body
- Issue: `/speq-adr-rules` rule 2 requires an ADR's Rationale to name the rule-2 criterion and state the rule-5 search result. A patch writes the ADR straight into `specs/_decision/` with no decision-log entry, so neither fact is recorded anywhere. In the plan flow, `plan-reviewer` checks `[ADR_OVERPROMOTION]` against those two facts. In a patch, the PR reviewer sees only `ADRs: <slug>`. The user asked for "real architectural decisions if any", and an over-promoted ADR becomes `Status: Accepted` when the PR merges.
- Fix: In plan.md § PR body, change the ADR line to `ADRs: <none | slug (<rule-2 criterion>; ADR search: <result>)[, ...]>`. In § Requirements, row "ADR", require that line for every new ADR.

#### [COMPLETENESS_GAP] FINDINGS
- Location: plan.md § Verification, Manual Testing
- Issue: The plan has no spec delta, so Manual Testing is its only behavioral proof. The "Fix run" row uses a fix, which per the Specs row edits no spec when a scenario already states the behavior. No row exercises the path the user asked for: a change that adds or corrects a scenario, writes an ADR, or edits `specs/architecture.md` or docs. No row covers size point 2, the uncommitted-changes question, or the `docs(<scope>)` ship message.
- Fix: In plan.md § Verification, Manual Testing, add three rows. (1) `/speq:patch change <behavior no scenario covers>` → one scenario added with a test in the diff, `speq feature validate` and `speq decision-log validate` pass, the second commit is `spec(<scope>): <patch-name>`, and `specs/_plans/` stays absent. (2) A change over 400 lines → the point-2 warning names the measured value before any push. (3) A run started with an uncommitted edit → the skill asks before it creates the branch.

## Task Breakdown

#### [EFFORT_MISESTIMATION] FINDINGS
- Location: plan.md § Implementation Tasks, task 1.2
- Issue: "Edit `.claude/skills/speq-code-review/SKILL.md` only where it fixes the findings path to the plan directory" leaves three statements that contradict patch mode.
  1. `.claude/skills/speq-code-review/references/review-findings-template.md:3` also fixes the path: "`code-reviewer` writes this document to `specs/_plans/<plan-name>/review-findings.md`". Its rule 3 addresses `Fix:` to an implementer agent that appends to `tasks.md`.
  2. `SKILL.md` § Routing says "The orchestrator never sees them individually". In a patch, the orchestrator reads every finding.
  3. The `code-reviewer.md` frontmatter description says "spawned by /speq-implement ... against the plan".
  If the reviewer follows the template, it creates `specs/_plans/<patch-name>/`, which the interview and the "Never touched" row exclude.
- Fix: In plan.md task 1.2, extend the edit list to `references/review-findings-template.md` (the path line and rule 3), `SKILL.md` § Routing (one sentence: for a patch brief, the findings go to the orchestrator in the reply), and the `code-reviewer.md` description (add `/speq-patch`).

#### [CLUSTER_INCOHERENCE] FINDINGS
- Location: plan.md § Parallelization, group B; tasks 1.4 and 2.7
- Issue: Group B is a layer slice. Tasks 2.1-2.2 edit packaging scripts, tasks 2.3-2.7 edit docs, and the two halves share no file. Task 2.7 restates the acceptance rule that task 1.4 writes into `/speq-adr-rules` rule 8 ("merging the PR is the acceptance"). It sits in another group, and group B's Knowledge lacks `.claude/skills/speq-adr-rules/SKILL.md`. Two agents author the same fact.
- Fix: In plan.md § Parallelization, move task 2.7 into group A after task 1.4.

#### [TRACEABILITY_GAP] FINDINGS
- Location: plan.md § Implementation Tasks, task 1.4
- Issue: Task 1.4 makes `/speq-patch` a consumer of `/speq-adr-rules`, but the skill's description still reads "Use inside a /speq-plan, /speq-record, or /speq-audit run". Task 1.3 updates the consumer list of `/speq-git-operations` for the same reason. Task 1.4 does not.
- Fix: In plan.md task 1.4, add: "Add /speq-patch to the consumer list in the description at `.claude/skills/speq-adr-rules/SKILL.md:3`".

## Design Depth

Checked: `[ADR_OVERPROMOTION]`: decisions [1], [2], and [3] are all `Promotes to ADR: no`. `[ARCHITECTURE_DRIFT]`: there is no `yes` entry, plan.md names no CLI component, interface, or data-flow change, and `specs/architecture.md` describes the CLI only (AGENTS.md). `[ADR_CONFLICT]`: all five accepted ADRs checked. `per-plan-decision-fragments`: a patch writes its own fragment, which keeps the ADR's reason (no shared file). `slug-identity-not-sequential-numbers`: the NNN prefix only orders fragments; `render_permanent_log` sorts by prefix then name, and only duplicate slugs fail validation. `record-all-or-nothing-across-plan` and `anchor-first-nonempty-line` are not touched. `plan-validate-skips-anchor-presence` is not contradicted; its interaction is under Feasibility. `[SHALLOW_DESIGN]`: the dual-mode `code-reviewer` adds one input shape and one output path, and the review taxonomy keeps one owner.

#### [INFORMATION_COUPLING] FINDINGS
- Location: plan.md § Requirements, intro sentence and rows "Name", "Version", "Test", "Review", "Library size"
- Issue: The intro says "The skill states each rule once and points to the owning skill instead of restating it". Five rows restate rules that other skills own.
  1. Name restates the verb table of `/speq-plan` step 3.
  2. Test restates the log convention of `/speq-implement-pr` steps 3 and 5.
  3. Library size restates the thresholds of `/speq-spec-merge`.
  4. Review omits how to collect the changed-files list. `/speq-implement` Phase 4 step 1 owns that rule, including untracked files.
  5. Version restates `/speq-implement-pr` step 3 and diverges from it. The patch maps `refactor` → patch and `remove` → minor. `/speq-implement-pr` names only `feat` and `fix`. The patch's PR title maps `remove` → `chore`, and Conventional Commits does not bump on `chore`.
  Task 1.3 already moves the verb → commit-type table to `/speq-git-operations`. The verb → version-bump rule derives from the same verb and stays split across two skills.
- Fix: In plan.md task 1.3, add the verb → version-bump mapping to the new `/speq-git-operations` section next to the verb → type mapping, and settle `remove` there once. Point `/speq-implement-pr` step 3 and the patch Version row at it. In § Requirements, change rows Name, Test, and Library size to pointers: "per `/speq-plan` step 3", "per `/speq-implement-pr` step 5", and "per `/speq-spec-merge` thresholds". In row Review, add "changed files per `/speq-implement` Phase 4 step 1".

## Prose Quality

#### [PROSE_UNCLEAR] FINDINGS
- Location: plan.md § Context, the bullets starting "ADR `per-plan-decision-fragments`" and "`scripts/plugin/build.sh` copies"; § Requirements, row "Report"
- Issue:
  1. The first bullet says the ADR "gives each unit of work its own fragment file". The ADR says "Store one committed ADR fragment per plan". The paraphrase widens the ADR to patches without saying so.
  2. The second bullet cites the log line as `:296`, and task 2.1 edits `:295`. Line 295 is "Workflow skills" and line 296 is "PR pipeline", so the reader cannot tell which line the patch joins.
  3. "a confirmed size warning" in the Report row does not say what is reported. It means the size limit the user chose to exceed.
- Fix: In plan.md § Context, quote the ADR ("one committed ADR fragment per plan") and add "A patch fragment keeps the ADR's reason: each unit of work writes its own file". In the build.sh bullet, change `:296` to `:295`. In the Report row, replace "a confirmed size warning" with "the size limit the user chose to exceed, with its measured value".
