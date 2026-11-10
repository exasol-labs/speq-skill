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

## Review Findings
