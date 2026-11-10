# Verification Report: add-patch-skill

## Verdict

| Result | Details |
|--------|---------|
| **PASS** | `/speq:patch` builds into the Claude and Codex plugins, and every automated suite passes |
| Code review | 8 findings — 8 fixed |

| Check | Status |
|-------|--------|
| Build | ✓ |
| Tests | ✓ |
| Lint | ✓ |
| Format | ✓ |
| Scenario Coverage | ✓ (no spec delta) |
| Manual Tests | ✓ (packaging rows); interactive rows not run |

## Test Evidence

### Coverage

| Type | Coverage % |
|------|------------|
| Unit | not measured (no Rust change) |
| Integration | not measured (no Rust change) |

### Test Results

| Type | Run | Passed | Ignored |
|------|-----|--------|---------|
| `cargo test` (unit and integration) | 302 | 302 | 0 |
| Release artifact (`scripts/release/test.sh v0.28.0`) | 1 | 1 | 0 |
| Docker (`tests/docker/test-docker.sh`) | 32 | 32 | 0 |

### Manual Tests

| Test | Result |
|------|--------|
| Plugin packaging: `skills/patch/SKILL.md` in the Claude and Codex plugins | ✓ both files listed |
| Command rename: unrenamed `/speq-` commands in the built patch skill | ✓ count 0 |
| Codex name: frontmatter of the Codex patch skill | ✓ `name: speq:patch`, no `model:` line |
| Release artifact: patch skill in both plugins | ✓ `OK skills/patch/SKILL.md`, `OK codex/skills/patch/SKILL.md` |
| Docker: Codex skills present | ✓ `PASS: Codex skills exist` |
| Claude load, fix run, spec change run, size warnings, uncommitted edit, hook | not run: each needs an interactive session or a throwaway project with a GitHub remote |

## Tool Evidence

### Linter

```
cargo clippy: exit 0
```

### Formatter

```
cargo fmt --check: exit 0, no changes
```

## Scenario Coverage

The plan has no spec delta, so no scenario maps to a test. `scripts/release/test.sh` and `tests/docker/test-codex-plugin.sh` fail when `skills/patch/SKILL.md` is missing from either plugin. Both pass.

## Notes

- The interactive Manual Testing rows are unverified. A reviewer runs them before relying on `/speq:patch`.
- `.claude/skills/speq-patch/SKILL.md` is 7996 bytes, under the 8000-byte cap.
- `docs/patch.md` says the run takes a test baseline once. The skill takes the baseline only on a new branch with a clean working tree.
- `get_version` in `scripts/lib/version.sh` returns an empty string when it is sourced from zsh, because it reads `BASH_SOURCE`.
