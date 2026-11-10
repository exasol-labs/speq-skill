# Tasks: add-patch-skill

## Phase 2: Implementation (Group A: patch skill)
- [x] 1.1 Write `.claude/skills/speq-patch/SKILL.md` per the plan Requirements table, size limit, and PR body [expert]
- [x] 1.2 Edit `code-reviewer.md` and `speq-code-review` for the patch brief (findings in reply, no file)
- [x] 1.3 Move PR-title derivation and version-bump rule into `speq-git-operations` `## PR title and version bump`; repoint `speq-plan-pr` and `speq-implement-pr`
- [x] 1.4 Add `/speq-patch` bullet to rule 8 of `speq-adr-rules` and to its description
- [x] 1.5 Update `docs/decision-log.md` permanent-log row and Lifecycle block

## Phase 2: Implementation (Group B: packaging and docs)
- [x] 2.1 Add `/speq:patch` to the "Workflow skills" log line in `scripts/plugin/build.sh`
- [x] 2.2 Add `skills/patch/SKILL.md` to both loops in `scripts/release/test.sh` and `patch` to `tests/docker/test-codex-plugin.sh`
- [x] 2.3 Write `docs/patch.md`
- [x] 2.4 Add Patch row to `docs/index.md` and `README.md` guide tables; add pointer sentences
- [x] 2.5 Add `/speq:patch` to `docs/hooks.md` table; 7 → 8 entry-point skills
- [x] 2.6 Add bullet to `docs/faq.md` "How do I fix a bug?"

## Phase 3: Verification
- [x] 3.1 Run the Checklist and Manual Testing commands

## Phase 4: Review Fixes
- [x] 4.1 In `.claude/skills/speq-patch/SKILL.md` step 2 item 3, name the patch per `/speq-plan` step 3 without its decision log
- [x] 4.2 In `.claude/skills/speq-code-review/SKILL.md` § Routing, limit "The orchestrator never sees them individually." to a plan brief
- [x] 4.3 In `.claude/skills/speq-code-review/SKILL.md` frontmatter, name `/speq-patch` beside `/speq-implement` in the description
- [x] 4.4 In `.claude/agents/code-reviewer.md`, name the `/speq-patch` orchestrator as a fixer and call the output "the findings document" in the skill list
- [x] 4.5 In `docs/patch.md` § Resume, state that the run asks whether uncommitted changes belong to the patch and stops only when they do not
- [x] 4.6 In `.claude/skills/speq-patch/SKILL.md`, log the step 4 baseline to `target/speq-baseline-<patch-name>-<suite>.log` and pass a step 5 suite only when each failure in its log also appears in its baseline log [expert]
- [x] 4.7 In `.claude/skills/speq-patch/SKILL.md` step 4 item 3, run the baseline only on a new branch with a clean working tree [expert]
- [x] 4.8 In `.claude/skills/speq-patch/SKILL.md`, reuse the baseline logs on a resumed run, stop step 8 only on a failure beyond the baseline, and keep the file at most 8000 bytes [expert]

## Phase 2: Implementation (Group C: baseline removal)
- [x] 5.1 Apply the simple test rule in `.claude/skills/speq-patch/SKILL.md` and remove the baseline text (steps 4, 5, 8, 11)
- [x] 5.2 Remove the baseline field and rule from `references/pr-body-template.md`
- [x] 5.3 Replace the baseline bullet in `docs/patch.md`
- [x] 5.4 Grep for leftover baseline text, check size, rerun Checklist and packaging rows

## Phase 4: Review Fixes (Group C)
- [x] 6.1 In `.claude/skills/speq-patch/SKILL.md` step 5 item 2, name `specs/mission.md § Commands` as the source of the test suites and the lint command, and keep the file at most 8000 bytes
