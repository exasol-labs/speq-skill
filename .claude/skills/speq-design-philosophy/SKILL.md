---
name: speq-design-philosophy
description: "Design principles for complexity (deep modules, information hiding, dependency direction) used by /speq-plan, /speq-implement, and the plan and code reviewers. Not for general design work."
---

# Design Philosophy

Judge a design by whether it adds or removes complexity, not by line count or named patterns. Draws on J. Ousterhout (*A Philosophy of Software Design*) and R. Martin (*Clean Architecture*).

## Principles

- **Deep modules.** A module's interface is far easier to use than its internals would be to rebuild. Merge shallow pass-through modules.
- **Information hiding.** Each significant design decision (a format, a protocol, an execution-order split) has one owning module. The same decision in two modules is coupling: merge them, or give the decision its own module.
- **No needless configuration.** A parameter is a decision the module declined to make. Prefer a sensible default.
- **Dependencies point inward.** Business logic names no delivery mechanism, storage engine, or framework, performs no I/O, and reads no ambient state. The consumer defines the abstraction in its own vocabulary. Concrete implementations are built only at the entry point. No dependency cycles.

## Quick Diagnostic

When a change introduces a module, interface, or boundary, answer each question. A "no" names the fix.

| Question | If no |
|----------|-------|
| Does one sentence capture what the module is responsible for? | Split it, or merge it into the module it serves |
| Is calling it clearly easier than reimplementing it? | Hide more, expose less |
| Can its internals change without an edit outside it? | Pull the leaked decision into one owner |
| Does business logic depend only inward? | Invert the dependency |
