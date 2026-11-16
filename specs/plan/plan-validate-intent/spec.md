# Feature: Plan Intent Validation

Validates the optional plan-level `intent.md` during plan validation. Catches a malformed intent file before the plan is recorded.

## Background

* Plans are stored in `specs/_plans/<plan-name>/`
* A plan MAY contain an `intent.md` file with the user's interview answers and the plan's assumptions
* An intent file uses H1 `# Intent: <plan-name>` and at least one of `## Interview`, `## Assumptions`
* An assumption line MAY carry an `ADR:` value of `yes` or `no`

## Scenarios

### Scenario: Validate plan with valid intent.md passes

* *GIVEN* a plan named "with-intent" exists with `plan.md` and a well-formed `intent.md`
* *WHEN* the user runs `speq plan validate with-intent`
* *THEN* the system SHALL report validation passed
* *AND* the system SHALL exit with code 0

### Scenario: Validate plan without intent.md still passes

* *GIVEN* a plan named "valid-plan" exists with `plan.md` and no `intent.md`
* *WHEN* the user runs `speq plan validate valid-plan`
* *THEN* the system SHALL report validation passed
* *AND* the system MUST NOT report any error related to intent.md

### Scenario: Validate plan with intent.md having no sections

* *GIVEN* a plan named "intent-no-sections" exists with an `intent.md` containing H1 but neither an `## Interview` nor an `## Assumptions` section
* *WHEN* the user runs `speq plan validate intent-no-sections`
* *THEN* the system SHALL report an error that the intent file MUST contain an "Interview" or "Assumptions" section
* *AND* the system SHALL exit with non-zero code

### Scenario: Validate plan with an invalid ADR value on an assumption

* *GIVEN* a plan named "intent-bad-adr" exists with an assumption line under `## Assumptions` whose `ADR:` value is neither `yes` nor `no`
* *WHEN* the user runs `speq plan validate intent-bad-adr`
* *THEN* the system SHOULD report a warning that the `ADR` value MUST be `yes` or `no`
* *AND* the system SHALL exit with code 0

### Scenario: Validate plan with intent.md having wrong H1 heading

* *GIVEN* a plan named "intent-bad-h1" exists with an `intent.md` whose first H1 is not `# Intent: <plan-name>`
* *WHEN* the user runs `speq plan validate intent-bad-h1`
* *THEN* the system SHALL report an error that the intent H1 MUST match `# Intent: <plan-name>`
* *AND* the system SHALL exit with non-zero code
