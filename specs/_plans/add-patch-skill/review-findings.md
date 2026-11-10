# Code Review Findings: add-patch-skill

## Summary
- Files reviewed: 3
- Total findings: 1 (standard: 1, expert: 0)

## Standard fixes

### .claude/skills/speq-patch/SKILL.md

#### [OUTDATED_COMMENT] Step 5 no longer names where the test and lint commands come from
- Location: line 58
- Issue: The deleted step 4 item 3 was the only text in the skill that named `specs/mission.md § Commands` as the source of the test suites and the lint command. Step 5 item 2 now says "Run the suites and the lint command with logs per `/speq-implement-pr` step 5". `/speq-implement-pr` step 5 names `specs/mission.md § Commands` for the test suites only and does not mention a lint command (`.claude/skills/speq-implement-pr/SKILL.md:77`). The skill therefore no longer states where the lint command comes from. The plan Requirements row Test requires "the real test suites and the lint command from `specs/mission.md § Commands`".
- Fix: In .claude/skills/speq-patch/SKILL.md step 5 item 2, replace "2. Run the suites and the lint command with logs per" with "2. Run the test suites and the lint command from `specs/mission.md § Commands` with logs per". Then run `wc -c .claude/skills/speq-patch/SKILL.md` and confirm it reports at most 8000 bytes.

## Expert fixes
[none]
