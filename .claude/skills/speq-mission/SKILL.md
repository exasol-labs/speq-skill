---
name: speq-mission
description: "Creates or updates specs/mission.md and specs/architecture.md through an interview, for brownfield or greenfield projects. Use to bootstrap a speq project, revise the mission, or when /speq-audit reports mission drift."
---

# Mission Creator

You are creating a project mission file (`specs/mission.md`) and the first architecture file (`specs/architecture.md`) through an interactive interview.

**Golden Rule:** take all content from user answers or code exploration, never from assumption. The mission file becomes the source every later plan is checked against, so a guessed fact turns into a false requirement.

## Required Skills

Invoke before starting:
- `/speq-ext-library`: library docs
- `/speq-cli`: spec structure
- `/speq-code-tools`: code exploration for the architecture draft
- `/speq-writing-guardrails`: prose style for artifacts and GitHub text

## Workflow

### 1. Project Detection

Determine project type:

```
Cargo.toml, package.json, go.mod, etc. exists?
├─ Yes → Brownfield (existing code)
└─ No  → Greenfield (new project)

specs/ or similar directory exists?
├─ Yes → Has existing specs (read them)
└─ No  → No specs yet

specs/architecture.md exists?
├─ Yes → Update mode: skip topics 4.9 to 4.11 (see below)
└─ No  → mission.md still has ## Architecture, ## External Dependencies, or technical constraints?
         ├─ Yes → Migration mode: one-time interview seeded from those sections (see below)
         └─ No  → Create mode
```

**Update mode.** `specs/architecture.md` exists. Skip the interview topics for Architecture, technical and performance constraints, and External Dependencies. Tell the user: "Change the architecture through /speq-plan." Interview the remaining topics as usual.

Brownfield and not Update mode: step 2 drafts `specs/architecture.md` from the code. Greenfield and Update mode: no draft.

**Migration mode.** `specs/architecture.md` is missing and `specs/mission.md` still holds the old sections. Run topics 4.9 to 4.11 once, seeded with the step 2 draft, which already reconciles the old sections with the code ("The code shows [X]. mission.md said [Y]. Which is current?"). Without a draft (no source files), seed with the old content ("mission.md lists [X] as the architecture. Is this still accurate?"). Place the approved file at `specs/architecture.md`. Remove the old Architecture and External Dependencies sections and the technical and performance constraint lines from `specs/mission.md`. Add the line `Architecture: see specs/architecture.md.` after Out of Scope.

### 2. Brownfield Exploration

For existing projects, gather context BEFORE interviewing:

1. **Tech stack**: read the manifest(s): `Cargo.toml`, `package.json`, `go.mod`, `pyproject.toml`/`requirements.txt`, `pom.xml`/`build.gradle`. The list is not exhaustive; a project can use other languages and several technologies at once.
2. **Commands**: look for existing scripts: `package.json` scripts, Makefile targets, `Cargo.toml` aliases, `pyproject.toml` scripts.
3. **Structure**: list top-level directories; find the main source directories (`src`, `lib`, `app`).
4. **Docs**: read `README.md`, `docs/`, and any existing `specs/`.
5. **Architecture draft**: applies in brownfield Create and Migration mode when source files exist beyond manifests. Skip it in Update mode and in greenfield. Draft `specs/architecture.md` from the code per `/speq-plan`'s `references/architecture-template.md`. For a large codebase, you may hand the code mapping to the built-in Explore agent.
   - Map breadth-first: manifests and workspace members, then the source roots two levels deep, then the entry points (main, bin, server, router, handler registries). Skip test, vendor, generated, and build directories, and note what you skipped under `Not explored`.
   - Derive each section from evidence only. Components from modules and their imports; Data Flow from an entry point to storage or output; Interfaces from CLI definitions, routes, public APIs, and file formats; External Dependencies from SDK clients, endpoints, and drivers; Constraints only from what code or config enforces.
   - Compare each claim in the old mission sections (if any) with the code. When they disagree, the draft follows the code and the conflict goes on a `Conflicts` list. What the code cannot show (performance targets, failure impact) goes on a `Questions` list unless the old sections state it.
   - Never invent a component, dependency, or constraint without evidence.

   Keep `Conflicts`, `Questions`, and `Not explored` for step 4.

### 3. Research Phase

Look up the libraries discovered or mentioned per `/speq-ext-library`. Use the results to inform interview questions and validate user choices.

### 4. Clarifying Interview

Conduct a Socratic interview by asking the user about every section below. Never fill in content without asking. Each question reveals assumptions, surfaces contradictions, or narrows scope. Brownfield: present what step 2 discovered and ask the user to confirm or correct it ("I found [X]. Is this accurate? What would you add or change?") instead of asking cold.

#### 4.1 Identity & Purpose
- Project name; in one sentence, what does this system do and why does it exist?
- What problem does this solve, and who experiences it?
- Why do existing solutions fall short?
- Brownfield: "The README says [Y]. Is this still the current purpose?"

#### 4.2 Target Users
- Who are the primary users? What are they trying to achieve? What is their typical workflow?
- Brownfield: "Based on the code, it seems targeted at [X]. Is this correct?"

#### 4.3 Core Capabilities
Apply **User Story Mapping** (Patton): identify activities, then decompose into capabilities.
- What are the 3-5 core capabilities this system provides? (What it does, not how.)
- Brownfield: "I found these main modules: [X, Y, Z]. What capabilities do they represent?"

#### 4.4 Out of Scope
- What does this project explicitly NOT do?
- What features might users expect but won't be supported?

#### 4.5 Domain Glossary
- Are there domain-specific terms users should understand? Any terms used differently than their common meaning?
- Brownfield: "I noticed these terms in the code: [X, Y]. What do they mean in this context?"

#### 4.6 Tech Stack
- Language/runtime, framework, database, testing framework. Greenfield: ask each. Brownfield: confirm the discovered stack ("I found: Rust with tokio, clap for CLI, no database. Correct?").

#### 4.7 Commands
- Build, test, lint/format, and coverage commands. Greenfield: ask each. Brownfield: confirm discovered commands and ask for any missing ones ("No coverage command found. What should it be?").

#### 4.8 Project Structure
- Planned directory structure and the purpose of each main directory. Brownfield: present the discovered structure and ask for clarification on purpose.

#### 4.9 Architecture
Answers go to `specs/architecture.md`.
- With a draft: present it. Ask one question per `Conflicts` item and replace the cold question with each `Questions` item. Tell the user what `Not explored` lists and ask about it. Apply the corrections to the draft.
- Without a draft: high-level architecture pattern (layered, hexagonal, event-driven, etc.)? Key components and their responsibilities? How does data flow through the system?

#### 4.10 Constraints
- Technical (browser-only, offline-first) and performance (response time, memory limits): answers go to the Constraints section of `specs/architecture.md`.
- Business (GDPR, multi-tenant): answers stay in `specs/mission.md`.

#### 4.11 External Dependencies
Answers go to `specs/architecture.md`. With a draft, topics 4.10 (technical and performance) and 4.11 are covered by the same review and the same `Conflicts` and `Questions` items. Ask only what the draft leaves open.
- What external services/APIs does this depend on? What happens if each dependency is unavailable?

### 5. Generate Mission

After collecting ALL information:

1. Create `specs/` directory if needed
2. Generate `specs/mission.md` using `references/mission-template.md` as structure
3. Generate `specs/architecture.md` using `references/architecture-template.md` from `/speq-plan` as structure, unless it already exists (update mode); with a step 2 draft, the draft plus the interview corrections is the file
4. Fill the generated files with ACTUAL collected information (no placeholders)
5. Check `specs/architecture.md` against the Rules in `/speq-plan`'s `references/architecture-template.md`
6. Name the files written

## Interview Guidelines

### Question Batching

Group questions into MECE partitions (max 3-4 per question round). Each group covers one dimension without overlap:

| Phase | Questions to Group |
|-------|-------------------|
| Identity | Name, summary, problem |
| Users | Personas, goals, workflows |
| Capabilities | Core features, out of scope |
| Technical | Stack, commands, structure |
| Constraints | Technical, business, performance (technical and performance go to architecture.md) |

### Adaptive Depth

| Project Complexity | Interview Depth |
|-------------------|-----------------|
| Simple CLI tool | Minimal (architecture.md with Overview and Components only, other sections `- None`) |
| Web application | Standard (all sections) |
| Distributed system | Deep (detailed architecture, failure modes) |
