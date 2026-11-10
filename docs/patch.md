[speq-skill](../README.md) / [Docs](./index.md) / Patch

---

# Patch

Ship a small fix, feature, or tweak as one ready pull request, without a plan.

```
/speq:patch <intent>  →  branch, code and tests, one review, version bump
                                   │
                                   ▼
                      specs and docs updated  →  ready PR
```

## What one run does

Run `/speq:patch <intent>`.

- Reads the existing specs and decisions, and asks only when the intent has more than one reading
- Creates the branch `patch/<patch-name>`
- Writes the code and tests with TDD. A fix starts with a test that reproduces the defect
- Runs the test suites and the lint command after the change and fixes every failure. After 3 red runs of the same suite, it stops and reports the failing tests
- Has `code-reviewer` review the diff once, then fixes every finding
- Bumps the version and commits
- Edits only the specs, ADRs, architecture, and docs the change needs
- Opens a ready PR. It never merges it

## When to use it instead of `/speq:plan`

- Use `/speq:patch` when the change is small and the expected behavior is clear
- Use [`/speq:plan`](./workflow.md) or [`/speq:plan-pr`](./pr-workflow.md) when the change needs design work, a review of the approach, or a new feature spec

## Size limit

A patch stays small. It exceeds the limit when it:

- needs a new feature spec or a new domain
- changes scenarios in more than 2 features
- adds or removes a component in `specs/architecture.md`
- touches more than 10 files, or more than 400 changed lines, once committed

The run names the limit and the measured value, recommends `/speq:plan` or `/speq:plan-pr`, and asks whether to continue. A "no" stops the run. If the commit already exists, the branch stays local for you to keep or delete.

## Files a patch may change

- Feature specs in `specs/<domain>/<feature>/spec.md`, edited directly without delta markers
- At most one new ADR in `specs/_decision/`. The default is none
- `specs/architecture.md`, only where the change alters a component, boundary, interface, data flow, constraint, or external dependency
- The project's user docs, by default `README.md` and `docs/`

A patch never touches `specs/_recorded/` or `specs/mission.md`. In `specs/_plans/`, it uses only its own folder `specs/_plans/<patch-name>/`, which holds the change intent and the review findings during the run. The patch deletes that folder before it opens the PR, so the folder is never committed. When an active plan holds a delta for an edited feature, the report and the PR body name the plan.

## Hook

Create `.speq/patch-hook.md` to add project rules to a run. The hook is authoritative and can override any step. See [Hooks](./hooks.md).

## Resume

If a run stops, for example at a usage limit or a red suite, run `/speq:patch <intent>` again. It reuses the branch `patch/<patch-name>` and continues where the work stopped. When the current branch is not `patch/<patch-name>` and the working tree has uncommitted changes, the run asks whether they belong to the patch. If they do not, it stops and asks you to commit or stash them. A stopped run can leave `specs/_plans/<patch-name>/` behind, and the next run reuses it. To abandon the patch, delete that folder and the branch.
