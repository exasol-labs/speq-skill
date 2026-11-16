---
name: speq-mission
description: "Creates or updates specs/mission.md and specs/architecture.md through an interview, for brownfield or greenfield projects. Use to bootstrap a speq project, revise the mission, or when /speq-audit reports mission drift."
---

# Mission Creator

## User Arguments

$ARGUMENTS

Create or update `specs/mission.md` and `specs/architecture.md` through an interview with the user.

**Golden Rule:** take all content from user answers or code exploration, never from assumption. Later plans are checked against the mission, so a guessed fact turns into a false requirement.

Invoke `/speq-cli` and `/speq-writing-guardrails` first.

## Workflow

### 1. Detect the Mode

- **Brownfield** when source code exists, **greenfield** otherwise. Read any existing `specs/`.
- **Update mode:** `specs/architecture.md` exists. Skip the architecture topics (architecture, technical and performance constraints, external dependencies) and tell the user: "Change the architecture through /speq-plan."
- **Migration mode:** `specs/architecture.md` is missing and `specs/mission.md` still holds `## Architecture`, `## External Dependencies`, or technical constraint lines. Interview the architecture topics once, seeded with the step 2 draft ("The code shows [X]. mission.md said [Y]. Which is current?"), or with the old content when there is no code. Write the approved file to `specs/architecture.md`, remove the old sections from `specs/mission.md`, and add `Architecture: see specs/architecture.md.` after Out of Scope.
- **Create mode** otherwise.

### 2. Explore (brownfield)

Before the interview, read the manifests, build and test commands, top-level structure, README, and docs.

In Create and Migration mode, draft `specs/architecture.md` from the code per `architecture-template.md` in `/speq-formats`. For a large codebase, hand the mapping to the built-in Explore agent.
- Derive each section from evidence only: components from modules and imports, data flow from an entry point to storage or output, interfaces from CLI definitions, routes, and file formats, external dependencies from clients and drivers, constraints only from what code or config enforces. Never invent one.
- Where the old mission sections disagree with the code, the draft follows the code and the item goes on a `Conflicts` list. What the code cannot show (performance targets, failure impact) goes on a `Questions` list. Skipped directories go on a `Not explored` list.

### 3. Interview

Ask the user about every section of `references/mission-template.md` and, outside Update mode, of the architecture template. Never fill a section without an answer. Brownfield: present what step 2 found and ask the user to confirm or correct it, instead of asking cold. With an architecture draft, ask one question per `Conflicts` and `Questions` item and name what `Not explored` lists.

Routing:
- Technical and performance constraints and external dependencies go to `specs/architecture.md`.
- Business constraints (GDPR, multi-tenant) stay in `specs/mission.md`.

### 4. Generate

1. Write `specs/mission.md` per `references/mission-template.md`.
2. Write `specs/architecture.md` per `architecture-template.md` in `/speq-formats`, unless it already exists. With a draft, the draft plus the interview corrections is the file.
3. Fill both files with the collected information only, no placeholders.
4. Check `specs/architecture.md` against the Rules in the architecture template.
5. Name the files written.
