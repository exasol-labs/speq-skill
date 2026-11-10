<div align="center">

<img src="assets/speq-skill-logo.svg" alt="speq-skill Logo" width="150">

# speq-skill

**A lightweight and straightforward workflow for spec-driven development with Claude Code or Codex**

[![spec|driven](https://img.shields.io/badge/spec-driven-blue)](specs/)
[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)
[![CI](https://github.com/exasol-labs/speq-skill/actions/workflows/ci.yml/badge.svg)](https://github.com/exasol-labs/speq-skill/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

[Getting Started](#getting-started) • [What](#what-this-is) • [How It Works](#how-does-it-work) • [Documentation](./docs/) • [Installation](./docs/installation.md)
</div>

---

## Getting Started

```bash
curl -fsSL https://raw.githubusercontent.com/exasol-labs/speq-skill/main/install.sh | bash
```

Then open Claude Code or Codex and start with the matching trigger: `/speq:mission` in Claude Code, or `$speq:mission` in Codex.

<details>
<summary>What does the installer do?</summary>

- Downloads a pre-built `speq` binary for your platform (Linux x86_64/ARM64, macOS Apple Silicon). On other platforms it stops and points to the [source build](./docs/installation.md#install-from-source)
- Installs the CLI to `~/.local/bin/speq`
- Installs plugin files to `~/.speq-skill/`
- Registers `/speq:*` skills for Claude Code and `$`-triggered skill suggestions for Codex when available
- Registers the local Codex marketplace when the Codex CLI is available

To uninstall, see [Installation — Uninstall](./docs/installation.md#uninstall).

</details>

---

## What this is

This is a plugin for Claude Code or Codex for lightweight spec-driven development with the following characteristics:

1. A workflow that is not tied to one language or framework (e.g., Python or TypeScript)
2. A straightforward repeatable workflow (`plan → implement → record`)
3. A **permanent** and growing spec library
4. A workflow that keeps the specs **small** to avoid context cluttering
5. A workflow that keeps **asking me instead of making assumptions**
6. **Semantic anchors** that ground AI behavior in established methodologies

It combines skills with a simple CLI called `speq` that lists and reads the permanent spec library. The coding agent lists the features, then reads only the relevant ones during planning and during the implementation. This avoids reading unnecessary specs into the context window.

Each skill is grounded in [semantic anchors](./docs/semantic-anchors.md) — named references to established methodologies (like London School TDD, BLUF, ADR) that steer AI behavior toward well-documented practices.

---

## How Does it Work?

```
/speq:mission → specs/mission.md (once per project)
                       │
      ┌────────────────┼────────────────┐
      ▼                ▼                ▼
/speq:plan  →  /speq:implement  →  /speq:record  (repeat)
```

1. **Mission** — Do it once. The coding agent explores your codebase (or interviews you for a greenfield project) and generates `specs/mission.md` and `specs/architecture.md`.
2. **Plan** — Describe what you want. The coding agent reads the existing specs, asks clarifying questions, and creates a plan with spec deltas.
3. **Implement** — The coding agent implements the plan, guided by guardrails for code quality, testing and more.
4. **Record** — The coding agent merges implemented spec deltas, and the architecture delta if the plan has one, into the permanent spec library.

Specs live in `specs/<domain>/<feature>/spec.md`. `specs/architecture.md` holds the current architecture. Plans stage in `specs/_plans/<plan-name>/`. The separation keeps your spec library clean while work is in progress.

For a small fix or tweak, `/speq:patch` skips the plan and ships one ready PR. See [Patch](./docs/patch.md).

---

## Documentation

| Guide                                          | Description                                                  |
|------------------------------------------------|--------------------------------------------------------------|
| [Installation](./docs/installation.md)         | Set up CLI and plugin                                        |
| [Workflow](./docs/workflow.md)                 | One-time mission setup, then Plan → Implement → Record cycle |
| [FAQ](./docs/faq.md)                           | Add features, fix bugs, start greenfield or brownfield       |
| [PR Workflow](./docs/pr-workflow.md)           | Plan and implement via a feat/ branch + Pull Request         |
| [Patch](./docs/patch.md)                       | Ship a small fix or tweak as one ready PR                    |
| [Decision Log](./docs/decision-log.md)         | Design decisions, ADR fragment format, validation, and show  |
| [CLI Reference](./docs/cli-reference.md)       | All CLI commands                                             |
| [MCP Servers](./docs/mcp-servers.md)           | Optional Serena and Context7 setup                           |
| [Semantic Anchors](./docs/semantic-anchors.md) | Named methodologies grounding each skill                     |
| [Hooks](./docs/hooks.md)                       | Repo-local `.speq/<name>-hook.md` customizations             |

---

## Important

`speq-skill` is a plugin for Claude Code, Codex, and other compatible AI coding agents. This project provides workflow structure and spec management only—**the coding agent generates all code, specs, other artifacts, and tool calls**.

## Companions

speq-skill works well with [Serena](https://github.com/oraios/serena) for code navigation and [Context7](https://github.com/upstash/context7) for library documentation. Both are optional. The skills use each one when it is installed and skip it when it is not. See [MCP Servers](./docs/mcp-servers.md).

> [!NOTE]
> Serena and Context7 are separate open-source projects with their own licenses and terms available in the linked repositories. Context7 sends your documentation queries to a hosted service and offers a free tier.

## Acknowledgments

- The `code-reviewer` agent's YAGNI / Over-Engineering review category was inspired by the tag taxonomy of [ponytail](https://github.com/DietrichGebert/ponytail).
- The [semantic anchor](./docs/semantic-anchors.md) approach — naming established methodologies (e.g. London School TDD) directly in skill instructions — draws on the [LLM-Coding/Semantic-Anchors](https://github.com/LLM-Coding/Semantic-Anchors) catalog.

## License

This project is licensed under the [MIT License](LICENSE).
