---
name: speq-code-guardrails
description: "TDD cycle and the code rules speq holds stricter than the usual defaults, for /speq-implement, /speq-patch, and the code-review sub-agent. Not for general coding."
---

# Code Guardrails

## Rules

- **Test first.** No production code without a failing test. Before you change untested code, add a test that captures its current behavior.
- **Evidence.** No claim without evidence: run the command, show the output, then claim.
- **Dependencies.** No new dependency until the standard library and the installed dependencies are ruled out.

## TDD Cycle

RED: write the test, run it, show the failure. GREEN: the least code that passes, run the test. REFACTOR: clean up, run the test and the linter. Run only the tests you touched, not the full suite.

## Tests

- Pure logic is tested without doubles. Orchestration is tested with doubles at its own abstractions. Adapters are tested against the real thing; a temporary directory is fine.
- Tests are repeatable: no shared mutable state, real clock, network, or unseeded randomness.
- Cover each failure path and the boundaries, not only the happy path.
- Assert observable behavior, never internal state.
- A skipped or ignored test is a defect. Fix it or delete it.

## Code

- No boolean or selector arguments: split the function.
- Signal failure with the language's error mechanism, never a sentinel value. An error states what was attempted, the input, and the violated constraint.
- Translate a third-party error into the module's own error type at the boundary.
- No role-suffix names without a responsibility (Manager, Processor, Handler, Util). Keep transport, vendor, and storage format out of names.
- No TODO, FIXME, or ticket references in code.
- Inline an abstraction with one implementation, unless it is a seam over I/O, nondeterminism, or a third party.
