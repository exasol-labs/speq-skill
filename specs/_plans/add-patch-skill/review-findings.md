# Code Review Findings: add-patch-skill

## Summary
- Files reviewed: 19
- Total findings: 8 (standard: 5, expert: 3)

## Standard fixes

### .claude/skills/speq-patch/SKILL.md

#### [SIDE_EFFECT] Naming pointer pulls in a decision-log write under specs/_plans/
- Location: line 45
- Issue: Step 2 item 3 names the patch "per `/speq-plan` step 3". That step also writes `specs/_plans/<plan-name>/decision-log.md` once the name is set (`.claude/skills/speq-plan/SKILL.md:58`). The patch Rules forbid any write under `specs/_plans/` (line 24), and the plan's Manual Testing expects no `specs/_plans/` directory after a run. The pointer imports a write that the skill forbids.
- Fix: In .claude/skills/speq-patch/SKILL.md step 2 item 3, replace "Name the patch per `/speq-plan` step 3." with "Name the patch per `/speq-plan` step 3, without its decision log."

### .claude/skills/speq-code-review/SKILL.md

#### [OUTDATED_COMMENT] Routing says the orchestrator never sees findings, then says it does
- Location: line 106
- Issue: § Routing opens with "The orchestrator never sees them individually." The sentence added at the end of the same paragraph says that for a `/speq-patch` brief the orchestrator receives the findings in the reply and applies every fix. The first claim is now false for a patch brief.
- Fix: In .claude/skills/speq-code-review/SKILL.md § Routing, replace "The orchestrator never sees them individually." with "For a plan brief, the orchestrator never sees them individually."

#### [OUTDATED_COMMENT] Frontmatter description names only /speq-implement
- Location: line 3
- Issue: The description reads "for the code-reviewer agent during /speq-implement". `code-reviewer` now also loads this skill when `/speq-patch` spawns it, and § Output Format and § Routing carry patch-brief rules.
- Fix: In .claude/skills/speq-code-review/SKILL.md frontmatter, replace "during /speq-implement." with "during /speq-implement or /speq-patch."

### .claude/agents/code-reviewer.md

#### [OUTDATED_COMMENT] Purpose line and skill list still assume a plan brief
- Location: lines 18, 23
- Issue: Line 18 says the agent finds issues "for the implementer agents to fix". For a patch brief, the `/speq-patch` orchestrator applies the fixes (line 49). Line 23 says `/speq-writing-guardrails` sets the prose style "for `review-findings.md`". A patch brief writes no file and returns the findings document as the reply (line 43).
- Fix: In .claude/agents/code-reviewer.md, replace "identify issues for the implementer agents to fix." with "identify issues for the implementer agents or the `/speq-patch` orchestrator to fix.", and replace "prose style for `review-findings.md`" with "prose style for the findings document".

### docs/patch.md

#### [OUTDATED_COMMENT] Resume section misstates the uncommitted-changes check
- Location: line 60
- Issue: The doc says "Uncommitted changes on another branch make it stop and ask you to commit or stash them." The skill first asks whether the changes belong to the patch, and stops only when they do not (`.claude/skills/speq-patch/SKILL.md` line 53). The check also runs on a first run, not only on a resume.
- Fix: In docs/patch.md § Resume, replace the sentence "Uncommitted changes on another branch make it stop and ask you to commit or stash them." with "When the current branch is not `patch/<patch-name>` and the working tree has uncommitted changes, the run asks whether they belong to the patch. If they do not, it stops and asks you to commit or stash them."

## Expert fixes

### .claude/skills/speq-patch/SKILL.md

#### [SWALLOWED_ERROR] Baseline log is overwritten, so a new failure in a baseline-red suite passes
- Location: lines 55, 60
- Issue: Step 4 item 3 logs the baseline run per `/speq-implement-pr` step 5, which writes `target/speq-<suite>.log` and judges green or red by exit code. Step 5 item 2 runs the suites "as in step 4" and writes the same path, so the first post-change run overwrites the baseline log. A suite red at baseline stays red by exit code after the change. The rule "no failure beyond the baseline" then has no baseline failure list to compare against, and a failure the patch introduces in that suite passes unseen.
- Fix: In .claude/skills/speq-patch/SKILL.md step 4 item 3, replace "with logs and exit codes per `/speq-implement-pr` step 5." with "with logs and exit codes per `/speq-implement-pr` step 5, but log to `target/speq-baseline-<patch-name>-<suite>.log`." In step 5 item 2, replace "A suite passes when it has no failure beyond the baseline." with "A suite passes when each test that fails in its log also fails in its baseline log. No baseline log → every failure counts."

#### [SWALLOWED_ERROR] Baseline taken with the user's patch edits in the working tree
- Location: lines 53, 55
- Issue: Step 4 item 1 lets uncommitted changes that belong to the patch carry onto the new branch. Step 4 item 3 then runs the baseline on that tree. A failure caused by those changes, for example a reproducing test the user already wrote, is labelled a baseline failure. The skill never fixes a baseline failure and lists it in the PR body as pre-existing.
- Fix: In .claude/skills/speq-patch/SKILL.md step 4 item 3, replace "On the new branch, before the first edit," with "On a new branch with a clean working tree, before the first edit,". Apply this together with the baseline-log finding above, so a run that skips the baseline has no baseline log and counts every failure.

#### [TACTICAL_SHORTCUT] A resumed run drops the baseline and cannot pass a baseline-red suite
- Location: lines 54, 86
- Issue: Step 4 item 2 says "A resumed run has no baseline." A suite that is red on `origin/<default-branch>` then counts as red in the resumed run. Step 5 item 2 makes the agent either fix code unrelated to the patch or stop after 3 red runs. Step 8 stops on its first red run ("Red → stop and report as in step 5."). A stopped patch on a repository with a baseline failure never reaches a PR, which contradicts docs/patch.md § Resume ("continues where the work stopped").
- Fix: In .claude/skills/speq-patch/SKILL.md step 4 item 2, replace "A resumed run has no baseline." with "A resumed run reuses the baseline logs of step 4 item 3, or has no baseline when they are absent." In step 8, replace "Red → stop and report as in step 5." with "A failure beyond the baseline → stop and report as in step 5." After applying every finding for this file, run `wc -c .claude/skills/speq-patch/SKILL.md`. It must report at most 8192 bytes (task 1.1). If it reports more, delete the Rules bullet "- `<default-branch>` is the repository default branch." and the step 8 sentence "The user keeps the branch for a `/speq-plan` run or deletes it."
