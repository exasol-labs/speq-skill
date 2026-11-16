## Revision Mode

Revise the plan from a feedback file: a plan-review findings file (`review/review.md`) or PR comments the user left (`notes/pr-feedback.md`).

- **Review findings:** address only the BLOCKER findings. Execute each one's `Fix:` line: an imperative naming the artifact, section, and concrete change. Revise exactly what it points to. Do not rewrite unrelated content. Do not act on FINDINGS-severity findings.
- **PR comments:** treat each comment as user feedback on the plan files. Make the change it asks for. If it conflicts with the user's intent or another comment, ask the user.
- When a fix makes or changes a choice the user did not make, add or rewrite its assumption in `intent.md` as `open`. Step 6 brings it to the user.
- If a fix changes a spec delta, re-derive the affected lines of `## Implementation Tasks`.
- Re-run `speq plan validate <plan-name>`.
- Keep one line per blocker or comment: `Resolved: <title> — <evidence>` or `Could not resolve: <title> — <why>`. Unresolved lines go to the user.
