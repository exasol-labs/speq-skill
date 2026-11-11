[speq-skill](../README.md) / [Docs](./index.md) / Patch

---

# Patch

Apply a small fix, feature, or tweak in one run, without a plan. `/speq:patch` changes your working tree. `/speq:patch-pr` also ships the change as one ready pull request.

```
/speq:patch <intent>     →  code and tests, one review, specs and docs updated
                                   │
                                   ▼
                         you review and commit

/speq:patch-pr <intent>  →  branch, then /speq:patch, then version bump
                                   │
                                   ▼
                                ready PR
```

## What `/speq:patch` does

Run `/speq:patch <intent>`.

- Reads the existing specs and decisions, and asks only when the intent has more than one reading
- Writes the code and tests with TDD. A fix starts with a test that reproduces the defect
- Runs the test suites and the lint command after the change and fixes every failure. After 3 red runs of the same suite, it stops and reports the failing tests
- Has `code-reviewer` review the diff once, then fixes every finding
- Edits only the specs, ADRs, architecture, and docs the change needs
- Runs no git command that writes. You review the changes and commit them

## What `/speq:patch-pr` adds

Run `/speq:patch-pr <intent>`.

- Creates the branch `patch/<patch-name>` and runs `/speq:patch` on it
- Bumps the version and runs the test suites once more
- Commits the change and opens a ready PR. It never merges it

## When to use it instead of `/speq:plan`

- Use `/speq:patch` or `/speq:patch-pr` when the change is small and the expected behavior is clear
- Use [`/speq:plan`](./workflow.md) or [`/speq:plan-pr`](./pr-workflow.md) when the change needs design work, a review of the approach, or a new feature spec

## Size limit

A patch stays small. It exceeds the limit when it:

- needs a new feature spec or a new domain
- changes scenarios in more than 2 features
- adds or removes a component in `specs/architecture.md`
- touches more than 25 files, or more than 400 changed lines, in total, not counting the version file and the lockfile

The run names the limit and the measured value, recommends `/speq:plan` or `/speq:plan-pr`, and asks whether to continue. A "no" stops the run. Changes made so far stay in the working tree for you to keep or discard.

## Files a patch may change

- Feature specs in `specs/<domain>/<feature>/spec.md`, edited directly without delta markers
- At most one new ADR in `specs/_decision/`. The default is none
- `specs/architecture.md`, only where the change alters a component, boundary, interface, data flow, constraint, or external dependency
- The project's user docs, by default `README.md` and `docs/`

A patch never touches `specs/_recorded/` or `specs/mission.md`. In `specs/_plans/`, it uses only its own folder `specs/_plans/<patch-name>/`, which holds the change intent and the review findings during the run. The patch deletes that folder at the end of the run. `/speq:patch-pr` deletes it once the PR is open. When an active plan holds a delta for an edited feature, the report names the plan. `/speq:patch-pr` also names it in the PR body.

## Hooks

Create `.speq/patch-hook.md` to add project rules to `/speq:patch`, and `.speq/patch-pr-hook.md` for the PR steps of `/speq:patch-pr`. `/speq:patch-pr` reads both. A hook is authoritative and can override any step. See [Hooks](./hooks.md).

## Resume

If a run stops, for example at a usage limit or a red suite, run the same command with the same intent again. It continues where the work stopped. `/speq:patch-pr` reuses the branch `patch/<patch-name>`. When the working tree has uncommitted changes at the start, the run asks whether they belong to the patch. If they do not, it stops and asks you to commit or stash them. A stopped run can leave `specs/_plans/<patch-name>/` behind, and the next run reuses it. To abandon the patch, delete that folder, and the branch for `/speq:patch-pr`.
