# Audit Checks

## 1. Validators
**Detect:** run `speq feature validate`, `speq decision-log validate`, and `speq plan validate <plan>` for each plan in `speq plan list` (call `/speq-cli`).
**Signal:** `✓ 0 errors` or `✗ N errors` naming the first few features, fragments, or plans.
**Remediate:** none. Report the validator messages. The user fixes the file.

## 2. Unrecorded plans
**Detect:** a `specs/_plans/*/` dir with `verification-report.md` is implemented but not archived. `tasks.md` without `verification-report.md` is in progress, and `plan.md` alone is not started. Note both, flag neither.
**Signal:** `✗ N unrecorded: <names>` or `✓ none`.
**Remediate:** point to `/speq-record <plan>`. Never archive here.

## 3. Architecture file
**Detect:** read-only, no CLI command exists. Check `specs/architecture.md` against the Rules in `/speq-plan`'s `references/architecture-template.md`.
**Signal:** `✓ present, N sections`, `✗ missing`, or `✗ bad structure: <first violation with line>`.
**Remediate:** missing: on Yes, run `/speq-mission`. Bad structure: report only. The user fixes the file or plans the change with `/speq-plan`. Never write `specs/architecture.md` here.

## 4. Mission sync
Run in SKILL.md Phase 3a. It yields (a) library domains and features the mission does not mention, (b) mission capabilities with no backing spec.
**Signal:** `✓ in sync`, `⚠ N features unmentioned · M capabilities unbacked`, or `✗ no mission.md`.
**Remediate (on Yes):** run `/speq-mission` seeded with the two lists. Never edit `mission.md`.

## 5. ADR review
Run in SKILL.md Phase 3b. Read the ADR rules in `/speq-plan`'s `references/decision-log-template.md` first.

**Detect:** skip (`— n/a`) when `specs/_decision/` holds no fragment. Otherwise read every fragment in full and judge each ADR by those rules.
- **Noise:** fails the rules. Remove it. Fold a corollary into its parent's `### Consequences`.
- **Wordy:** a real decision with more text than ADR rule 5 allows. Trim it.
- **Keep** everything else. When in doubt, keep. Never remove an ADR named in another's `Supersedes` or `Superseded by`.

**Signal:** `✓ N ADRs, clean` or `⚠ M noise · T wordy of N ADRs`, each listed with a one-line reason and `file:line` evidence.
**Remediate (on Yes):** ADR cleanup below.

---

## Remediation: ADR cleanup
Show the list first. The user can strike slugs. Apply the rest on Yes. This cleanup is the one exception to the rule that old fragments are never edited.
1. Noise: delete the ADR block (its `## ADR:` heading up to the next one or the file end). Delete a fragment left empty. Leave other `NNN-` prefixes unchanged.
2. Corollary: add its decision as one sentence to the parent's `### Consequences` first.
3. Wordy: rewrite to ADR rule 5 and the layout in `/speq-plan`'s `references/decision-log-template.md`. Keep the heading, `**ID:**`, `**Plan:**`, `**Status:**`, and supersede links.
4. Run `speq decision-log validate`. It MUST pass. On failure, undo and report.
5. Do not commit. The user reviews the diff.
