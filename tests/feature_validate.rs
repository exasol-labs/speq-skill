use assert_cmd::Command;
use predicates::prelude::*;

const FIXTURE_DIR: &str = "tests/fixtures/feature_validate/mixed";

fn cmd() -> Command {
    Command::new(assert_cmd::cargo::cargo_bin!("speq"))
}

#[test]
fn unknown_domain_reports_not_found_and_exits_1() {
    cmd()
        .current_dir(FIXTURE_DIR)
        .args(["feature", "validate", "nonexistent"])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("Domain 'nonexistent' not found"));
}

#[test]
fn summary_counts_passed_failed_and_warnings() {
    cmd()
        .current_dir(FIXTURE_DIR)
        .args(["feature", "validate"])
        .assert()
        .code(1)
        .stdout(predicate::str::contains(
            "Summary: 2 passed, 1 failed, 1 warnings",
        ));
}

#[test]
fn summary_for_a_passing_domain() {
    cmd()
        .current_dir(FIXTURE_DIR)
        .args(["feature", "validate", "good"])
        .assert()
        .success()
        .stdout(predicate::str::contains(
            "Summary: 2 passed, 0 failed, 1 warnings",
        ));
}

#[test]
fn underscore_directory_is_not_a_domain() {
    cmd()
        .current_dir(FIXTURE_DIR)
        .args(["feature", "validate", "_plans"])
        .assert()
        .code(1)
        .stdout(predicate::str::contains("Domain '_plans' not found"));
}
