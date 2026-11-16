[speq-skill](../README.md) / [Docs](./index.md) / MCP Servers

---

# MCP Servers

speq-skill works well with [Serena](https://github.com/oraios/serena), an MCP (Model Context Protocol) server for code navigation, and [Context7](https://github.com/upstash/context7), an MCP server for library documentation. Both are optional. You install them once, globally. Your coding agent uses them through their own server instructions; the skills do not depend on them.

| Server | What your agent gets |
|--------|----------------------|
| Serena | Symbol-level code navigation and editing |
| Context7 | Current library documentation |

## Why Serena

- It finds symbols, references, and implementations, and edits one symbol at a time
- The agent reads less code and changes only what the plan names

## Set up Serena

The setup script asks before it installs Serena. It skips Serena when it is already registered. To do it yourself:

**Claude Code**

```bash
uv tool install -p 3.13 serena-agent
claude mcp add --scope user serena -- serena start-mcp-server --context claude-code --project-from-cwd
```

**Codex**

```bash
uv tool install -p 3.13 serena-agent
codex mcp add serena -- serena start-mcp-server --project-from-cwd --context=codex
```

See the [Serena documentation](https://github.com/oraios/serena) for advanced configuration.
