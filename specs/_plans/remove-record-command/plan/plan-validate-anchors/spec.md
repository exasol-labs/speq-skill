<!-- DELTA:CHANGED -->
# Feature: Plan Delta Anchor Validation

Validates the anchors of a plan's spec delta blocks during plan validation. Catches an unrecognized, illegal, or repeated anchor before implementation begins.
<!-- /DELTA:CHANGED -->

<!-- DELTA:CHANGED -->
## Background

* Plans are stored in `specs/_plans/<plan-name>/`
* A delta spec at `<domain>/<feature>/spec.md` in the plan targets `specs/<domain>/<feature>/spec.md`
* The anchor of a delta block is its first non-empty line: `### Scenario: <name>`, `## Background`, or `# Feature: <name>`
* `DELTA:NEW` and `DELTA:REMOVED` MAY target a `### Scenario: <name>` anchor only
* A recognized anchor MUST NOT appear in more than one delta block of a delta file, whatever the marker kinds
<!-- /DELTA:CHANGED -->

## Scenarios

<!-- DELTA:NEW -->
### Scenario: Validate plan with a repeated anchor of the same kind reports an error

* *GIVEN* a plan named "prose-repeated-anchor" whose delta targets an existing feature
* *AND* the delta has two `<!-- DELTA:NEW -->` blocks anchored at the same `### Scenario:` heading
* *WHEN* the user runs `speq plan validate prose-repeated-anchor`
* *THEN* the system SHALL report an error naming the repeated anchor
* *AND* the system SHALL exit with non-zero code
<!-- /DELTA:NEW -->

<!-- DELTA:NEW -->
### Scenario: Validate plan with a repeated anchor of different kinds reports an error

* *GIVEN* a plan named "prose-mixed-kinds" whose delta targets an existing feature
* *AND* the delta has a `<!-- DELTA:NEW -->` block and a `<!-- DELTA:CHANGED -->` block anchored at the same `### Scenario:` heading
* *WHEN* the user runs `speq plan validate prose-mixed-kinds`
* *THEN* the system SHALL report an error naming the repeated anchor and both markers
* *AND* the system SHALL exit with non-zero code
<!-- /DELTA:NEW -->
