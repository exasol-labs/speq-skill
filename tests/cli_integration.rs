use assert_cmd::Command;
use predicates::prelude::*;
use std::fs;
use tempfile::TempDir;

fn setup_test_specs() -> TempDir {
    let tmp = TempDir::new().unwrap();
    let specs = tmp.path().join("specs");

    // Create valid hierarchy
    let cli_validate = specs.join("cli/validate");
    fs::create_dir_all(&cli_validate).unwrap();
    fs::write(
        cli_validate.join("spec.md"),
        r#"# Feature: CLI Validate

The system SHALL provide validation.

## Background

* Test context.

## Scenarios

### Scenario: Basic test

* *GIVEN* a setup
* *WHEN* an action occurs
* *THEN* the system SHALL respond
"#,
    )
    .unwrap();

    let validation_doc = specs.join("validation/document-structure");
    fs::create_dir_all(&validation_doc).unwrap();
    fs::write(
        validation_doc.join("spec.md"),
        r#"# Feature: Document Validation

The system SHALL validate documents.

## Background

* Documents have structure.

## Scenarios

### Scenario: Valid docs pass

* *GIVEN* a valid document
* *WHEN* validation runs
* *THEN* the system SHALL pass
"#,
    )
    .unwrap();

    tmp
}

fn cmd() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin!("speq"))
}

mod feature_list {
    use super::*;

    #[test]
    fn lists_all_features() {
        let tmp = setup_test_specs();

        cmd()
            .current_dir(tmp.path())
            .args(["feature", "list"])
            .assert()
            .success()
            .stdout(predicate::str::contains("cli/"))
            .stdout(predicate::str::contains("validate"))
            .stdout(predicate::str::contains("validation/"))
            .stdout(predicate::str::contains("document-structure"));
    }

    #[test]
    fn lists_domain_features() {
        let tmp = setup_test_specs();

        cmd()
            .current_dir(tmp.path())
            .args(["feature", "list", "cli"])
            .assert()
            .success()
            .stdout(predicate::str::contains("cli/"))
            .stdout(predicate::str::contains("validate"));
    }

    #[test]
    fn empty_domain_shows_message() {
        let tmp = setup_test_specs();

        cmd()
            .current_dir(tmp.path())
            .args(["feature", "list", "nonexistent"])
            .assert()
            .success()
            .stdout(predicate::str::contains("No features found"));
    }
}

mod feature_validate {
    use super::*;

    #[test]
    fn validates_all_specs() {
        let tmp = setup_test_specs();

        cmd()
            .current_dir(tmp.path())
            .args(["feature", "validate"])
            .assert()
            .success()
            .stdout(predicate::str::contains("cli/validate"))
            .stdout(predicate::str::contains("validation/document-structure"));
    }

    #[test]
    fn validates_single_domain() {
        let tmp = setup_test_specs();

        cmd()
            .current_dir(tmp.path())
            .args(["feature", "validate", "cli"])
            .assert()
            .success()
            .stdout(predicate::str::contains("cli/validate"));
    }

    #[test]
    fn validates_single_feature() {
        let tmp = setup_test_specs();

        cmd()
            .current_dir(tmp.path())
            .args(["feature", "validate", "cli/validate"])
            .assert()
            .success()
            .stdout(predicate::str::contains("cli/validate"))
            .stdout(predicate::str::contains("0 errors"));
    }

    #[test]
    fn reports_invalid_spec() {
        let tmp = TempDir::new().unwrap();
        let specs = tmp.path().join("specs/broken/test");
        fs::create_dir_all(&specs).unwrap();
        fs::write(specs.join("spec.md"), "# Feature: Broken\n\nNo sections.\n").unwrap();

        cmd()
            .current_dir(tmp.path())
            .args(["feature", "validate", "broken/test"])
            .assert()
            .code(1)
            .stdout(predicate::str::contains("✗"))
            .stdout(predicate::str::contains("error"));
    }

    #[test]
    fn no_features_found() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("specs")).unwrap();

        cmd()
            .current_dir(tmp.path())
            .args(["feature", "validate"])
            .assert()
            .success()
            .stdout(predicate::str::contains("No features found"));
    }
}

mod domain_list {
    use super::*;

    #[test]
    fn lists_all_domains() {
        let tmp = setup_test_specs();

        cmd()
            .current_dir(tmp.path())
            .args(["domain", "list"])
            .assert()
            .success()
            .stdout(predicate::str::contains("cli/"))
            .stdout(predicate::str::contains("validation/"));
    }

    #[test]
    fn empty_specs_shows_message() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("specs")).unwrap();

        cmd()
            .current_dir(tmp.path())
            .args(["domain", "list"])
            .assert()
            .success()
            .stdout(predicate::str::contains("No domains found"));
    }
}

mod plan_list {
    use super::*;

    #[test]
    fn lists_active_plans() {
        cmd()
            .current_dir("tests/fixtures/plan_list")
            .args(["plan", "list"])
            .assert()
            .success()
            .stdout("add-auth\nfix-validation\n");
    }

    #[test]
    fn no_plans_shows_message() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("specs/_plans")).unwrap();

        cmd()
            .current_dir(tmp.path())
            .args(["plan", "list"])
            .assert()
            .success()
            .stdout(predicate::str::contains("No active plans"));
    }

    #[test]
    fn missing_plans_dir_shows_message() {
        let tmp = TempDir::new().unwrap();
        fs::create_dir_all(tmp.path().join("specs")).unwrap();

        cmd()
            .current_dir(tmp.path())
            .args(["plan", "list"])
            .assert()
            .success()
            .stdout(predicate::str::contains("No active plans"));
    }
}

mod feature_get {
    use super::*;

    #[test]
    fn gets_full_feature_spec() {
        let tmp = setup_test_specs();

        cmd()
            .current_dir(tmp.path())
            .args(["feature", "get", "cli/validate"])
            .assert()
            .success()
            .stdout(predicate::str::contains("CLI Validate"))
            .stdout(predicate::str::contains("Basic test"));
    }

    #[test]
    fn gets_single_scenario() {
        let tmp = setup_test_specs();

        cmd()
            .current_dir(tmp.path())
            .args(["feature", "get", "cli/validate/Basic test"])
            .assert()
            .success()
            .stdout(predicate::str::contains("cli/validate/Basic test"))
            .stdout(predicate::str::contains("Given"));
    }

    #[test]
    fn feature_not_found() {
        let tmp = setup_test_specs();

        cmd()
            .current_dir(tmp.path())
            .args(["feature", "get", "cli/nonexistent"])
            .assert()
            .code(1)
            .stdout(predicate::str::contains("Feature not found"));
    }

    #[test]
    fn scenario_not_found() {
        let tmp = setup_test_specs();

        cmd()
            .current_dir(tmp.path())
            .args(["feature", "get", "cli/validate/Missing"])
            .assert()
            .code(1)
            .stdout(predicate::str::contains("Scenario 'Missing' not found"));
    }
}
