---
name: speq-minimal-change
description: "The smallest change that fully does the job: a decision ladder from reuse of the existing repository through the standard library or platform and installed dependencies to the minimum code that works, plus the boundaries a smaller diff never cuts. Triggered by planner-agent, implementer-agent, implementer-expert-agent, and code-reviewer before any planning, implementation, or review work."
---

# Minimal Change

Take the smallest change that fully does the job. The task's boundary decides what must exist. This skill decides how little work implements it.

## Scope

The task has a boundary: at plan time it is the user's request, at implement and review time it is the spec delta and the task list. Apply the ladder below inside that boundary. Never drop a behavior the spec delta names.

## Before You Write

Read the task and the code it touches. List every place the change must reach: callers, tests, fixtures, config, exports. Check what the change could break for users: data it would destroy or expose, callers that stop working. That is scope. Extra features are not.

## The Ladder

Take the first option that fully works:

1. Already in this repository: a helper, component, service, or pattern. Use it the way the surrounding code uses it.
2. The standard library or a platform feature. Use it, unless the project has its own. A house component beats a native widget.
3. An installed dependency. Use it. Never add a dependency for a few lines.
4. One line a reader understands at a glance. One line.
5. Otherwise: the minimum code that works.

- Be lazy about the solution, never about the change. Finish every part the task needs, including the callers, tests, and fixtures the change breaks.
- No abstraction, wrapper, type conversion, option, config, or boilerplate that nobody asked for. No "for later" code.
- Keep values in the form the platform already gives you. Deletion beats addition.
- Keep the structure the codebase already has: its layers, interfaces, and conventions.
- The shortest working diff wins, once you know everything it must touch. A one-liner that needs decoding is not short.
- Between options of equal size, take the one that is correct on edge cases.
- A shortcut with a known limit gets one comment: `shortcut: <the limit>, <when to upgrade>`.
- A small change still needs its test. `/speq-code-guardrails` owns the TDD cycle.

## Never Cut

A smaller diff never removes:

- validation at trust boundaries
- error handling that prevents data loss
- security
- accessibility
- calibration that real hardware needs
- anything the spec delta names

## Planning

At plan time the request itself is the task. Apply the ladder to it: leave out a feature, option, or flexibility the core job does not need, and name the omission in one line so the user can ask for it. An omission stated in the plan is a decision. The same omission unstated is the `[SCOPE_REDUCTION]` finding in `/speq-plan-review`.

## Reviewing

When you review a change, walk the ladder in reverse. For each changed line, ask which rung admitted it. A line that no rung justifies is a lean finding: name the existing helper, standard-library call, or installed dependency that replaces it. `/speq-code-review` owns the finding format.

## Attribution

Adapted from Ponytail (https://github.com/DietrichGebert/ponytail), MIT License. The decision ladder and the never-cut list come from that project's `.agents/rules/ponytail.md`.
