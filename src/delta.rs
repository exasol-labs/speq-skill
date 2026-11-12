use std::fs;
use std::path::{Path, PathBuf};

use thiserror::Error;

/// Why a plan's delta specs could not be found or a delta spec could not be
/// parsed.
#[derive(Error, Debug)]
pub enum DeltaError {
    #[error("Failed to read file: {path}")]
    FileReadError { path: String },

    #[error("Malformed delta marker at line {line}: {content}")]
    MalformedDelta { line: usize, content: String },
}

#[derive(Debug, Clone, PartialEq)]
enum DeltaKind {
    New,
    Changed,
    Removed,
}

impl DeltaKind {
    fn marker(&self) -> &'static str {
        match self {
            DeltaKind::New => "DELTA:NEW",
            DeltaKind::Changed => "DELTA:CHANGED",
            DeltaKind::Removed => "DELTA:REMOVED",
        }
    }
}

const SCENARIO_HEADING_PREFIX: &str = "### Scenario:";
const BACKGROUND_HEADING: &str = "## Background";
const FEATURE_HEADING_PREFIX: &str = "# Feature";

/// The section of a feature spec that a delta block targets.
///
/// The anchor owns the heading text of each recognized section, so rule
/// messages name a section without a caller passing a heading string around.
/// A block whose first non-empty line names none of the recognized sections is
/// `Unrecognized` rather than absent, so the rule checks can report it.
#[derive(Debug, Clone, PartialEq, Eq)]
enum DeltaAnchor {
    Scenario(String),
    Background,
    Description,
    Unrecognized,
}

impl DeltaAnchor {
    fn label(&self) -> String {
        match self {
            DeltaAnchor::Scenario(title) => format!("{SCENARIO_HEADING_PREFIX} {title}"),
            DeltaAnchor::Background => BACKGROUND_HEADING.to_string(),
            DeltaAnchor::Description => format!("{FEATURE_HEADING_PREFIX}: <name>"),
            DeltaAnchor::Unrecognized => "no recognized anchor".to_string(),
        }
    }
}

/// One `<!-- DELTA:* -->` block parsed out of a delta spec.
///
/// `content` is the block body with the opening and closing markers stripped
/// and surrounding whitespace trimmed. `anchor` is classified from the first
/// non-empty line of `content` alone — no later line in the block can change
/// what the block targets.
#[derive(Debug, Clone)]
pub struct DeltaBlock {
    kind: DeltaKind,
    content: String,
    anchor: DeltaAnchor,
}

/// Every `spec.md` under `plan_dir`, in a stable order.
///
/// The order is sorted because callers report one message per delta spec, and
/// directory-read order would otherwise vary between runs on the same plan.
pub fn find_delta_specs(plan_dir: &Path) -> Result<Vec<PathBuf>, DeltaError> {
    let mut specs = Vec::new();
    find_delta_specs_recursive(plan_dir, &mut specs)?;
    specs.sort();
    Ok(specs)
}

fn find_delta_specs_recursive(dir: &Path, specs: &mut Vec<PathBuf>) -> Result<(), DeltaError> {
    let entries = fs::read_dir(dir).map_err(|_| DeltaError::FileReadError {
        path: dir.display().to_string(),
    })?;

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            find_delta_specs_recursive(&path, specs)?;
        } else if path.file_name().is_some_and(|n| n == "spec.md") {
            specs.push(path);
        }
    }

    Ok(())
}

/// Every `<!-- DELTA:* -->` block of a delta spec, in file order.
///
/// A marker counts only when it is a whole line after trimming, and text
/// outside any block is ignored. Fails with `MalformedDelta` naming the line on
/// a nested open marker, a close marker with no open block, a close marker of
/// another kind than the open one, or a block still open at the end of the
/// file.
pub fn parse_deltas(content: &str) -> Result<Vec<DeltaBlock>, DeltaError> {
    let mut deltas = Vec::new();
    let mut current_kind: Option<DeltaKind> = None;
    let mut current_content = String::new();
    let mut in_delta = false;

    for (line_num, line) in content.lines().enumerate() {
        let trimmed = line.trim();

        if let Some(kind) = parse_delta_open(trimmed) {
            if in_delta {
                return Err(DeltaError::MalformedDelta {
                    line: line_num + 1,
                    content: line.to_string(),
                });
            }
            in_delta = true;
            current_kind = Some(kind);
            current_content.clear();
        } else if let Some(close_kind) = parse_delta_close(trimmed) {
            if !in_delta {
                return Err(DeltaError::MalformedDelta {
                    line: line_num + 1,
                    content: line.to_string(),
                });
            }

            if current_kind.as_ref() != Some(&close_kind) {
                return Err(DeltaError::MalformedDelta {
                    line: line_num + 1,
                    content: line.to_string(),
                });
            }

            let content_trimmed = current_content.trim().to_string();
            let anchor = extract_anchor(&content_trimmed);

            deltas.push(DeltaBlock {
                kind: current_kind.take().unwrap(),
                content: content_trimmed,
                anchor,
            });

            in_delta = false;
            current_content.clear();
        } else if in_delta {
            if !current_content.is_empty() {
                current_content.push('\n');
            }
            current_content.push_str(line);
        }
    }

    if in_delta {
        return Err(DeltaError::MalformedDelta {
            line: content.lines().count(),
            content: current_content,
        });
    }

    Ok(deltas)
}

fn parse_delta_open(line: &str) -> Option<DeltaKind> {
    if line == "<!-- DELTA:NEW -->" {
        Some(DeltaKind::New)
    } else if line == "<!-- DELTA:CHANGED -->" {
        Some(DeltaKind::Changed)
    } else if line == "<!-- DELTA:REMOVED -->" {
        Some(DeltaKind::Removed)
    } else {
        None
    }
}

fn parse_delta_close(line: &str) -> Option<DeltaKind> {
    if line == "<!-- /DELTA:NEW -->" {
        Some(DeltaKind::New)
    } else if line == "<!-- /DELTA:CHANGED -->" {
        Some(DeltaKind::Changed)
    } else if line == "<!-- /DELTA:REMOVED -->" {
        Some(DeltaKind::Removed)
    } else {
        None
    }
}

fn extract_anchor(content: &str) -> DeltaAnchor {
    let Some(line) = first_non_empty_line(content) else {
        return DeltaAnchor::Unrecognized;
    };

    if let Some(title) = line.strip_prefix(SCENARIO_HEADING_PREFIX) {
        return DeltaAnchor::Scenario(title.trim().to_string());
    }
    if line == BACKGROUND_HEADING {
        return DeltaAnchor::Background;
    }
    if line.starts_with(FEATURE_HEADING_PREFIX) {
        return DeltaAnchor::Description;
    }
    DeltaAnchor::Unrecognized
}

fn first_non_empty_line(content: &str) -> Option<&str> {
    content.lines().map(str::trim).find(|line| !line.is_empty())
}

/// One message per rule violation, empty when every block is legal.
///
/// This function holds the `(DeltaKind, DeltaAnchor)` legality matrix and the
/// repeated-anchor rule that `speq plan validate` enforces. The recorder agent
/// applies the same rules by hand from the anti-pattern table in
/// `/speq-spec-merge`, so a rule change here needs the same change there. The
/// rules hold against the delta file alone and read no target spec. Whether an
/// anchor exists in the target spec is settled at merge time instead.
///
/// A message names no file, because the caller knows which delta file the blocks
/// came from and prefixes each message with that path.
pub fn check_anchor_rules(blocks: &[DeltaBlock]) -> Vec<String> {
    let mut messages: Vec<String> = blocks.iter().filter_map(illegal_pair_message).collect();
    messages.extend(repeated_anchor_messages(blocks));
    messages
}

fn illegal_pair_message(block: &DeltaBlock) -> Option<String> {
    match (&block.kind, &block.anchor) {
        (_, DeltaAnchor::Unrecognized) => Some(unrecognized_anchor_message(&block.content)),
        (DeltaKind::New, anchor @ (DeltaAnchor::Background | DeltaAnchor::Description)) => {
            Some(new_on_prose_anchor_message(anchor))
        }
        (DeltaKind::Removed, anchor @ (DeltaAnchor::Background | DeltaAnchor::Description)) => {
            Some(removed_on_prose_anchor_message(anchor))
        }
        (DeltaKind::New | DeltaKind::Changed | DeltaKind::Removed, DeltaAnchor::Scenario(_))
        | (DeltaKind::Changed, DeltaAnchor::Background | DeltaAnchor::Description) => None,
    }
}

fn unrecognized_anchor_message(content: &str) -> String {
    let opening = match first_non_empty_line(content) {
        Some(line) => format!("Delta block starts with `{line}`"),
        None => "Delta block is empty".to_string(),
    };
    format!(
        "{opening}, so it names no section to merge into. \
         The first non-empty line of a delta block MUST be \
         `{SCENARIO_HEADING_PREFIX} <name>`, `{BACKGROUND_HEADING}`, or \
         `{FEATURE_HEADING_PREFIX}: <name>`. Move that heading inside the delta marker."
    )
}

fn new_on_prose_anchor_message(anchor: &DeltaAnchor) -> String {
    format!(
        "{} MUST NOT target `{}`. That section is required, so it already exists in the \
         target spec. Use {} instead.",
        DeltaKind::New.marker(),
        anchor.label(),
        DeltaKind::Changed.marker()
    )
}

fn removed_on_prose_anchor_message(anchor: &DeltaAnchor) -> String {
    format!(
        "{} MUST NOT target `{}`. That section is required, so removing it produces an \
         invalid spec. Use {} instead.",
        DeltaKind::Removed.marker(),
        anchor.label(),
        DeltaKind::Changed.marker()
    )
}

fn repeated_anchor_messages(blocks: &[DeltaBlock]) -> Vec<String> {
    let mut groups: Vec<(&DeltaAnchor, Vec<&'static str>)> = Vec::new();

    for block in blocks {
        if block.anchor == DeltaAnchor::Unrecognized {
            continue;
        }
        match groups
            .iter()
            .position(|(anchor, _)| *anchor == &block.anchor)
        {
            Some(index) => groups[index].1.push(block.kind.marker()),
            None => groups.push((&block.anchor, vec![block.kind.marker()])),
        }
    }

    groups
        .iter()
        .filter(|(_, markers)| markers.len() > 1)
        .map(|(anchor, markers)| {
            format!(
                "Anchor `{}` appears in {} delta blocks ({}). A recognized anchor MAY appear \
                 in at most one delta block per delta file, whatever the markers, because the \
                 merged result would otherwise depend on the order of the blocks. Write one \
                 block carrying the final text.",
                anchor.label(),
                markers.len(),
                markers.join(", ")
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn parses_new_delta() {
        let content = r#"## Scenarios

<!-- DELTA:NEW -->
### Scenario: User logs in

* *GIVEN* a user exists
* *WHEN* they log in
* *THEN* they SHALL be authenticated
<!-- /DELTA:NEW -->
"#;

        let deltas = parse_deltas(content).unwrap();
        assert_eq!(deltas.len(), 1);
        assert_eq!(deltas[0].kind, DeltaKind::New);
        assert_eq!(
            deltas[0].anchor,
            DeltaAnchor::Scenario("User logs in".to_string())
        );
    }

    #[test]
    fn parses_changed_delta() {
        let content = r#"<!-- DELTA:CHANGED -->
### Scenario: Old feature

Updated content
<!-- /DELTA:CHANGED -->"#;

        let deltas = parse_deltas(content).unwrap();
        assert_eq!(deltas.len(), 1);
        assert_eq!(deltas[0].kind, DeltaKind::Changed);
    }

    #[test]
    fn parses_removed_delta() {
        let content = r#"<!-- DELTA:REMOVED -->
### Scenario: Deprecated feature
<!-- /DELTA:REMOVED -->"#;

        let deltas = parse_deltas(content).unwrap();
        assert_eq!(deltas.len(), 1);
        assert_eq!(deltas[0].kind, DeltaKind::Removed);
        assert_eq!(
            deltas[0].anchor,
            DeltaAnchor::Scenario("Deprecated feature".to_string())
        );
    }

    #[test]
    fn parses_multiple_deltas() {
        let content = r#"## Scenarios

<!-- DELTA:NEW -->
### Scenario: First
* content
<!-- /DELTA:NEW -->

<!-- DELTA:CHANGED -->
### Scenario: Second
* updated
<!-- /DELTA:CHANGED -->

<!-- DELTA:REMOVED -->
### Scenario: Third
<!-- /DELTA:REMOVED -->
"#;

        let deltas = parse_deltas(content).unwrap();
        assert_eq!(deltas.len(), 3);
        assert_eq!(deltas[0].kind, DeltaKind::New);
        assert_eq!(deltas[1].kind, DeltaKind::Changed);
        assert_eq!(deltas[2].kind, DeltaKind::Removed);
    }

    #[test]
    fn error_on_nested_delta_markers() {
        let content = r#"<!-- DELTA:NEW -->
### Scenario: Test
<!-- DELTA:CHANGED -->
nested
<!-- /DELTA:CHANGED -->
<!-- /DELTA:NEW -->
"#;

        let result = parse_deltas(content);
        assert!(matches!(result, Err(DeltaError::MalformedDelta { .. })));
    }

    #[test]
    fn error_on_close_without_open() {
        let content = r#"### Scenario: Test
<!-- /DELTA:NEW -->
"#;

        let result = parse_deltas(content);
        assert!(matches!(result, Err(DeltaError::MalformedDelta { .. })));
    }

    #[test]
    fn error_on_mismatched_close_marker() {
        let content = r#"<!-- DELTA:NEW -->
### Scenario: Test
<!-- /DELTA:CHANGED -->
"#;

        let result = parse_deltas(content);
        assert!(matches!(result, Err(DeltaError::MalformedDelta { .. })));
    }

    #[test]
    fn error_on_unclosed_delta_at_end_of_file() {
        let content = r#"<!-- DELTA:REMOVED -->
### Scenario: Test
"#;

        let result = parse_deltas(content);
        assert!(matches!(result, Err(DeltaError::MalformedDelta { .. })));
    }

    fn block(kind: DeltaKind, content: &str) -> DeltaBlock {
        DeltaBlock {
            kind,
            content: content.to_string(),
            anchor: extract_anchor(content),
        }
    }

    #[test]
    fn check_anchor_rules_accepts_every_legal_marker_and_anchor_pair() {
        let blocks = vec![
            block(DeltaKind::New, "### Scenario: Added"),
            block(DeltaKind::Changed, "### Scenario: Updated"),
            block(DeltaKind::Removed, "### Scenario: Dropped"),
            block(DeltaKind::Changed, "## Background\n\n* Context."),
            block(DeltaKind::Changed, "# Feature: Renamed\n\nDescription."),
        ];

        assert_eq!(check_anchor_rules(&blocks), Vec::<String>::new());
    }

    #[test]
    fn check_anchor_rules_accepts_no_blocks() {
        assert_eq!(check_anchor_rules(&[]), Vec::<String>::new());
    }

    #[test]
    fn check_anchor_rules_rejects_unrecognized_anchor() {
        let blocks = vec![block(DeltaKind::Changed, "* A bullet, not a heading")];

        let messages = check_anchor_rules(&blocks);

        assert_eq!(messages.len(), 1);
        assert!(messages[0].contains("### Scenario: <name>"));
        assert!(messages[0].contains("## Background"));
        assert!(messages[0].contains("# Feature: <name>"));
        assert!(messages[0].contains("* A bullet, not a heading"));
        assert!(messages[0].contains("inside the delta marker"));
    }

    #[test]
    fn check_anchor_rules_rejects_new_on_background() {
        let blocks = vec![block(DeltaKind::New, "## Background\n\n* Context.")];

        let messages = check_anchor_rules(&blocks);

        assert_eq!(messages.len(), 1);
        assert!(messages[0].contains("DELTA:NEW"));
        assert!(messages[0].contains("## Background"));
        assert!(messages[0].contains("DELTA:CHANGED"));
    }

    #[test]
    fn check_anchor_rules_rejects_removed_on_description() {
        let blocks = vec![block(DeltaKind::Removed, "# Feature: Gone\n\nDescription.")];

        let messages = check_anchor_rules(&blocks);

        assert_eq!(messages.len(), 1);
        assert!(messages[0].contains("DELTA:REMOVED"));
        assert!(messages[0].contains("# Feature"));
        assert!(messages[0].contains("DELTA:CHANGED"));
    }

    #[test]
    fn check_anchor_rules_rejects_one_anchor_carried_by_two_blocks_of_one_kind() {
        let blocks = vec![
            block(DeltaKind::New, "### Scenario: Login\n\n* first"),
            block(DeltaKind::New, "### Scenario: Login\n\n* second"),
        ];

        let messages = check_anchor_rules(&blocks);

        assert_eq!(messages.len(), 1);
        assert!(messages[0].contains("### Scenario: Login"));
        assert!(messages[0].contains("DELTA:NEW"));
    }

    #[test]
    fn check_anchor_rules_rejects_one_anchor_carried_by_two_kinds() {
        let blocks = vec![
            block(DeltaKind::New, "### Scenario: Login\n\n* added"),
            block(DeltaKind::Changed, "### Scenario: Login\n\n* updated"),
        ];

        let messages = check_anchor_rules(&blocks);

        assert_eq!(messages.len(), 1);
        assert!(messages[0].contains("### Scenario: Login"));
        assert!(messages[0].contains("DELTA:NEW"));
        assert!(messages[0].contains("DELTA:CHANGED"));
    }

    #[test]
    fn check_anchor_rules_accepts_two_blocks_on_different_anchors() {
        let blocks = vec![
            block(DeltaKind::Changed, "### Scenario: Login"),
            block(DeltaKind::Changed, "### Scenario: Login fails"),
        ];

        assert_eq!(check_anchor_rules(&blocks), Vec::<String>::new());
    }

    #[test]
    fn check_anchor_rules_exempts_unrecognized_anchors_from_the_repeat_rule() {
        let blocks = vec![
            block(DeltaKind::Changed, "* one bullet"),
            block(DeltaKind::Changed, "* another bullet"),
        ];

        assert_eq!(check_anchor_rules(&blocks).len(), 2);
    }

    #[test]
    fn check_anchor_rules_reports_every_violation_of_the_file() {
        let blocks = vec![
            block(DeltaKind::New, "## Background\n\n* first"),
            block(DeltaKind::Changed, "## Background\n\n* second"),
            block(DeltaKind::Changed, "* not a heading"),
        ];

        let messages = check_anchor_rules(&blocks);

        assert_eq!(messages.len(), 3);
    }

    #[test]
    fn extract_anchor_returns_unrecognized_for_prose_without_a_heading() {
        let content = "Just some text\nwithout scenario heading";
        assert_eq!(extract_anchor(content), DeltaAnchor::Unrecognized);
    }

    #[test]
    fn extract_anchor_reads_scenario_title() {
        let content = "### Scenario: User logs in\n\n* *GIVEN* a user exists";
        assert_eq!(
            extract_anchor(content),
            DeltaAnchor::Scenario("User logs in".to_string())
        );
    }

    #[test]
    fn extract_anchor_reads_background_heading() {
        let content = "## Background\n\n* Command syntax: `speq plan validate`";
        assert_eq!(extract_anchor(content), DeltaAnchor::Background);
    }

    #[test]
    fn extract_anchor_reads_feature_heading_by_prefix() {
        let content = "# Feature: Any Name At All\n\nDescription.";
        assert_eq!(extract_anchor(content), DeltaAnchor::Description);
    }

    #[test]
    fn extract_anchor_skips_leading_empty_lines() {
        let content = "\n\n   \n## Background\n\n* Context.";
        assert_eq!(extract_anchor(content), DeltaAnchor::Background);
    }

    #[test]
    fn extract_anchor_reads_the_first_non_empty_line_only() {
        let content = "* A bullet comes first\n\n### Scenario: Not the anchor";
        assert_eq!(extract_anchor(content), DeltaAnchor::Unrecognized);
    }

    #[test]
    fn extract_anchor_rejects_background_heading_with_extra_words() {
        let content = "## Background and Context\n\n* Context.";
        assert_eq!(extract_anchor(content), DeltaAnchor::Unrecognized);
    }

    #[test]
    fn extract_anchor_returns_unrecognized_for_empty_content() {
        assert_eq!(extract_anchor(""), DeltaAnchor::Unrecognized);
    }

    #[test]
    fn find_delta_specs_returns_paths_in_a_stable_order() {
        let tmp = TempDir::new().unwrap();
        let plan = tmp.path();
        for feature in ["c/three", "a/one", "b/two"] {
            let spec_dir = plan.join(feature);
            fs::create_dir_all(&spec_dir).unwrap();
            fs::write(spec_dir.join("spec.md"), "# Feature: Test\n").unwrap();
        }

        let found = find_delta_specs(plan).unwrap();

        let mut sorted = found.clone();
        sorted.sort();
        assert_eq!(found, sorted);
        assert_eq!(found.len(), 3);
    }

    #[test]
    fn find_delta_specs_fails_naming_the_directory_when_it_is_missing() {
        let tmp = TempDir::new().unwrap();
        let missing = tmp.path().join("missing");

        let result = find_delta_specs(&missing);

        assert!(matches!(
            result,
            Err(DeltaError::FileReadError { path }) if path == missing.display().to_string()
        ));
    }
}
