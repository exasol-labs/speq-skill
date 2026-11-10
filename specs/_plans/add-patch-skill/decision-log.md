# Decision Log: add-patch-skill

## Interview

- Q: Git flow? → A: Always branch + PR
- Q: How to write spec/ADR changes? → A: Edit specs/ and specs/_decision/ directly, then validate; no plan dir, no archive
- Q: Ceremony before coding? → A: Ask only on ambiguity
- Q: Change too big for a patch? → A: Warn and continue on confirmation
- Q: Who codes? → A: Orchestrator inline, then code-reviewer once on the diff
- Q: Version bump? → A: Yes; fix → patch, small feature → minor
- Q: Artifacts patch may update? → A: Feature specs, ADRs, architecture.md, user docs
- Q: PR state and hook? → A: Ready PR; support .speq/patch-hook.md
- Q: Run the suites once before the first edit as a baseline? → A: No, the default branch is green in almost every case (CI); nobody asked for it
- Q: A suite fails in code the patch did not touch? → A: Report it in the report and the PR body, never fix it
- Q: Plan review shows "touched" cannot be decided by file (callers, binary-level tests). Keep the touched rule or simplify? → A: Simplify: every failure after the change counts as caused by the patch and is fixed; after 3 red runs of a suite, stop and report the failing tests

## Design Decisions

### [1] code-reviewer returns patch findings in its reply

- **Decision:** In a patch run, `code-reviewer` receives the change intent, the changed-files list, and the hook path. It returns its findings in its reply, in the `/speq-code-review` format, and writes no file.
- **Rationale:** A patch has no plan directory to hold `review-findings.md`. The orchestrator that fixes the findings is the same agent that reads the reply.
- **Alternatives:** Write the findings to `specs/_plans/<patch-name>/review-findings.md` and delete the file after the fix: rejected, it creates the plan directory the interview excludes, and a missed delete commits review output.
- **Promotes to ADR:** no

### [2] Fixed patch size limit with a warning

- **Decision:** A patch exceeds its size limit when it needs a new feature spec or domain, changes scenarios in more than 2 features, adds or removes a component in `specs/architecture.md`, or its implementation commit changes more than 10 files or more than 400 lines. The skill names the exceeded limit and continues only after the user confirms. A project changes the limit through `.speq/patch-hook.md`.
- **Rationale:** The spec and architecture limits are checkable from discovery before any code exists. The line limit matches the 200 to 400 lines per review session above which defect detection drops in the SmartBear/Cisco code review study, and `code-reviewer` reviews a patch once.
- **Alternatives:** A threshold in a separate settings file: rejected, the hook already overrides any step, and a second mechanism adds a file format to learn.
- **Promotes to ADR:** no

### [3] speq-patch pins no model

- **Decision:** The `speq-patch` frontmatter has no `model:` field, so the skill runs on the session model.
- **Rationale:** `/speq-plan-pr` and `/speq-implement-pr` pin `model: sonnet` because sub-agents write their code. The patch orchestrator writes the code itself. With no `model: sonnet` line, `set_codex_skill_model` in `scripts/plugin/build.sh` has nothing to rewrite, so `speq-patch` stays off its list.
- **Alternatives:** Pin `model: sonnet` like the other PR orchestrators: rejected, it fixes the coding model to Sonnet whatever model the user's session runs.
- **Promotes to ADR:** no

### [4] No baseline run, every failure is fixed

- **Decision:** `/speq-patch` runs no test suite before the first edit. After the change, it counts every failure as caused by the patch and fixes it. After 3 red runs of the same suite, it stops and reports.
- **Rationale:** CI keeps the default branch green, so a failure after the change is almost always caused by the patch. A per-file "touched" test cannot find failures in unchanged callers or in tests that run the binary.
- **Alternatives:** (a) Run the suites once before the first edit as a baseline: rejected, it adds a full suite run to every patch for a case CI almost always rules out. (b) Fix only failures in code the patch touched: rejected, it misses regressions in unchanged callers and in tests that run the binary.
- **Promotes to ADR:** no

## Review Findings

### [plan-review] "Touched" cannot be decided by file

- **Finding:** `plan-reviewer` raised two BLOCKERs against the Test row. A per-file rule labels the patch's own regression in an unchanged caller or in a test that runs the binary as a failure outside the patch. The row's log-based red verdict also conflicts with the exit-code verdict of `/speq-implement-pr` step 5.
- **Direction change:** Per user decision, the simple rule in [4] replaces the touched rule. A suite is green or red by exit code, every failure counts as caused by the patch and is fixed, and the run stops after 3 red runs of the same suite.
- **Promotes to ADR:** no
