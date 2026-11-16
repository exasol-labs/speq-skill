## Revision Mode

Revise the plan from feedback: the plan-review findings (`plan-review.md`) or the PR comments the user left.

- **Review findings:** address only the BLOCKER findings. Execute each one's `Fix:` line: an imperative naming the artifact, section, and concrete change. Revise exactly what it points to. Do not rewrite unrelated content. Do not act on FINDINGS-severity findings.
- **PR comments:** treat each comment as user feedback on the plan files. Make the change it asks for. If it conflicts with the user's intent or another comment, ask the user.
- When a fix makes or changes a choice the user did not make, add or rewrite its direction in `intent.md`. Step 6 brings it to the user.
- Re-run `speq plan validate <plan-name>`.
- Keep one line per blocker or comment: `Resolved: <title> — <evidence>` or `Could not resolve: <title> — <why>`. Unresolved lines go to the user.
