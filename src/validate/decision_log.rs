use std::collections::HashSet;
use std::path::Path;

use thiserror::Error;

#[derive(Debug)]
pub struct DecisionLogValidationResult {
    pub errors: Vec<DecisionLogError>,
}

impl DecisionLogValidationResult {
    pub fn new() -> Self {
        Self { errors: Vec::new() }
    }

    pub fn is_success(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn add_error(&mut self, error: DecisionLogError) {
        self.errors.push(error);
    }
}

impl Default for DecisionLogValidationResult {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Error, PartialEq)]
pub enum DecisionLogError {
    // --- Permanent decision fragments (`specs/_decision/*.md`) ---
    #[error("Fragment '{fragment}' does not start with its H1 heading '# Decisions: <plan-name>'")]
    FragmentMissingH1 { fragment: String },

    #[error("Fragment file '{fragment}' is not named 'NNN-<plan-name>.md' with a numeric prefix")]
    FragmentFileName { fragment: String },

    #[error(
        "Fragment '{fragment}' ADR ID '{slug}' is not a kebab-case slug (lowercase letters and digits joined by single hyphens)"
    )]
    InvalidSlug { fragment: String, slug: String },

    #[error(
        "Fragment '{fragment}' ADR '{title}' has No identity: missing required '**ID:**' field"
    )]
    AdrMissingId { fragment: String, title: String },

    #[error("ADR '{adr}' is missing required field '{field}'")]
    AdrMissingField { adr: String, field: String },

    #[error("Duplicate ADR slug '{slug}' across fragments")]
    DuplicateSlug { slug: String },

    #[error("ADR '{adr}' '**Supersedes:**' target '{target}' does not resolve to a known ADR ID")]
    UnresolvedSupersedes { adr: String, target: String },

    #[error("ADR '{adr}' status 'Superseded by {target}' references unknown ADR ID '{target}'")]
    UnresolvedStatusSlug { adr: String, target: String },

    #[error(
        "ADR '{adr}' has invalid Status '{status}' (must be one of: Accepted, Deprecated, Superseded by <slug>)"
    )]
    InvalidStatus { adr: String, status: String },
}

const ADR_HEADING_PREFIX: &str = "## ADR:";
const FRAGMENT_H1_PREFIX: &str = "# Decisions:";
const SUPERSEDED_BY_PREFIX: &str = "Superseded by ";

/// Validate every decision fragment stored under `specs/_decision`.
///
/// Reads all `*.md` fragments from `dir` and validates them as a set. An
/// absent or empty `specs/_decision` directory is a success: plans are not
/// required to leave permanent decision records, so there is nothing to check.
pub fn validate_decisions_dir(dir: &Path) -> DecisionLogValidationResult {
    let fragments = read_fragments(dir);
    validate_fragments(&fragments)
}

/// Read every `*.md` fragment under `dir`, returning `(file_name, content)`
/// pairs sorted by file name. Returns an empty vector when the directory is
/// absent or cannot be read.
pub fn read_fragments(dir: &Path) -> Vec<(String, String)> {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return Vec::new(),
    };

    let mut fragments: Vec<(String, String)> = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) != Some("md") {
            continue;
        }
        let name = match path.file_name().and_then(|n| n.to_str()) {
            Some(name) => name.to_string(),
            None => continue,
        };
        if let Ok(content) = std::fs::read_to_string(&path) {
            fragments.push((name, content));
        }
    }

    fragments.sort_by(|a, b| a.0.cmp(&b.0));
    fragments
}

/// Pure validation of a set of `(file_name, content)` fragments.
///
/// Two passes: (1) per fragment, verify the `# Decisions:` H1 and each
/// `## ADR:` block's required fields and status vocabulary while collecting
/// each ADR's slug and references; (2) build the global slug set across every
/// fragment, report duplicate slugs once, and resolve all `**Supersedes:**`
/// and `Superseded by <slug>` references against that set.
pub fn validate_fragments(fragments: &[(String, String)]) -> DecisionLogValidationResult {
    let mut result = DecisionLogValidationResult::new();
    let mut adrs: Vec<AdrInfo> = Vec::new();

    // Pass 1: per-fragment structural validation.
    for (name, content) in fragments {
        if !is_valid_fragment_name(name) {
            result.add_error(DecisionLogError::FragmentFileName {
                fragment: name.clone(),
            });
        }
        if !fragment_has_valid_h1(content) {
            result.add_error(DecisionLogError::FragmentMissingH1 {
                fragment: name.clone(),
            });
        }

        for block in split_adr_blocks(content) {
            adrs.push(validate_adr_block(name, block, &mut result));
        }
    }

    // Pass 2: cross-fragment identity and reference resolution.
    let mut known: HashSet<String> = HashSet::new();
    let mut seen: HashSet<String> = HashSet::new();
    let mut reported: HashSet<String> = HashSet::new();
    for adr in &adrs {
        if let Some(id) = &adr.id {
            if !seen.insert(id.clone()) && reported.insert(id.clone()) {
                result.add_error(DecisionLogError::DuplicateSlug { slug: id.clone() });
            }
            // A duplicated slug still counts as known, so it never cascades
            // into spurious unresolved-reference errors.
            known.insert(id.clone());
        }
    }

    for adr in &adrs {
        if let Some(target) = &adr.supersedes
            && !known.contains(target)
        {
            result.add_error(DecisionLogError::UnresolvedSupersedes {
                adr: adr.identity.clone(),
                target: target.clone(),
            });
        }
        if let Some(target) = &adr.status_target
            && !known.contains(target)
        {
            result.add_error(DecisionLogError::UnresolvedStatusSlug {
                adr: adr.identity.clone(),
                target: target.clone(),
            });
        }
    }

    result
}

/// Assemble the permanent Architecture Decision Records from `fragments`.
///
/// Fragments are ordered by their numeric `NNN-` prefix ascending, then by
/// file name ascending for ties. Each fragment's own `# Decisions:` H1 is
/// dropped and its ADR blocks are emitted in document order beneath a single
/// `# Architecture Decision Records` H1.
pub fn render_permanent_log(fragments: &[(String, String)]) -> String {
    let mut sorted: Vec<&(String, String)> = fragments.iter().collect();
    sorted.sort_by(|a, b| {
        numeric_prefix(&a.0)
            .cmp(&numeric_prefix(&b.0))
            .then_with(|| a.0.cmp(&b.0))
    });

    let mut out = String::from("# Architecture Decision Records\n");
    for (_, content) in sorted {
        let body: Vec<&str> = content
            .lines()
            .filter(|line| !line.trim_start().starts_with(FRAGMENT_H1_PREFIX))
            .collect();
        let trimmed = body.join("\n");
        let trimmed = trimmed.trim();
        if !trimmed.is_empty() {
            out.push('\n');
            out.push_str(trimmed);
            out.push('\n');
        }
    }
    out
}

/// Everything collected about a single `## ADR:` block during pass 1.
struct AdrInfo {
    /// Slug from `**ID:**`, when present.
    id: Option<String>,
    /// The name used to identify this ADR in errors: its slug, or its title
    /// when the slug is absent.
    identity: String,
    /// Target slug from `**Supersedes:**`, when present.
    supersedes: Option<String>,
    /// Target slug parsed from a `Superseded by <slug>` status, when present.
    status_target: Option<String>,
}

/// A single ADR block: its title and the lines beneath the heading.
struct AdrBlock<'a> {
    title: String,
    lines: Vec<&'a str>,
}

fn split_adr_blocks(content: &str) -> Vec<AdrBlock<'_>> {
    let mut blocks: Vec<AdrBlock<'_>> = Vec::new();
    let mut current: Option<AdrBlock<'_>> = None;

    for line in content.lines() {
        let trimmed = line.trim_start();
        if let Some(rest) = trimmed.strip_prefix(ADR_HEADING_PREFIX) {
            if let Some(block) = current.take() {
                blocks.push(block);
            }
            current = Some(AdrBlock {
                title: rest.trim().to_string(),
                lines: Vec::new(),
            });
        } else if let Some(block) = current.as_mut() {
            block.lines.push(line);
        }
    }

    if let Some(block) = current.take() {
        blocks.push(block);
    }

    blocks
}

fn validate_adr_block(
    fragment: &str,
    block: AdrBlock<'_>,
    result: &mut DecisionLogValidationResult,
) -> AdrInfo {
    let mut id: Option<String> = None;
    let mut has_plan = false;
    let mut status: Option<String> = None;
    let mut supersedes: Option<String> = None;
    let mut has_context = false;
    let mut has_decision = false;

    for line in &block.lines {
        if let Some(value) = extract_bold_field(line, "ID") {
            id = Some(value.to_string());
        }
        if extract_bold_field(line, "Plan").is_some() {
            has_plan = true;
        }
        if let Some(value) = extract_bold_field(line, "Status") {
            status = Some(value.to_string());
        }
        if let Some(value) = extract_bold_field(line, "Supersedes") {
            supersedes = Some(value.to_string());
        }
        let trimmed = line.trim();
        if trimmed == "### Context" {
            has_context = true;
        }
        if trimmed == "### Decision" {
            has_decision = true;
        }
    }

    let identity = id.clone().unwrap_or_else(|| block.title.clone());

    if let Some(slug) = &id
        && !is_kebab_case(slug)
    {
        result.add_error(DecisionLogError::InvalidSlug {
            fragment: fragment.to_string(),
            slug: slug.clone(),
        });
    }
    if id.is_none() {
        result.add_error(DecisionLogError::AdrMissingId {
            fragment: fragment.to_string(),
            title: block.title.clone(),
        });
    }
    if !has_plan {
        result.add_error(DecisionLogError::AdrMissingField {
            adr: identity.clone(),
            field: "**Plan:**".to_string(),
        });
    }
    if status.is_none() {
        result.add_error(DecisionLogError::AdrMissingField {
            adr: identity.clone(),
            field: "**Status:**".to_string(),
        });
    }
    if !has_context {
        result.add_error(DecisionLogError::AdrMissingField {
            adr: identity.clone(),
            field: "### Context".to_string(),
        });
    }
    if !has_decision {
        result.add_error(DecisionLogError::AdrMissingField {
            adr: identity.clone(),
            field: "### Decision".to_string(),
        });
    }

    let mut status_target = None;
    if let Some(status) = &status {
        let trimmed = status.trim();
        if !is_valid_status(trimmed) {
            result.add_error(DecisionLogError::InvalidStatus {
                adr: identity.clone(),
                status: trimmed.to_string(),
            });
        } else if let Some(rest) = trimmed.strip_prefix(SUPERSEDED_BY_PREFIX) {
            status_target = Some(rest.trim().to_string());
        }
    }

    AdrInfo {
        id,
        identity,
        supersedes,
        status_target,
    }
}

/// A fragment's first non-empty line must be `# Decisions: <plan-name>`.
fn fragment_has_valid_h1(content: &str) -> bool {
    content
        .lines()
        .map(str::trim_start)
        .find(|line| !line.is_empty())
        .is_some_and(|line| line.starts_with(FRAGMENT_H1_PREFIX))
}

/// A fragment file is named `NNN-<plan-name>.md`: a numeric prefix, a hyphen,
/// and a non-empty plan name.
fn is_valid_fragment_name(name: &str) -> bool {
    let Some(stem) = name.strip_suffix(".md") else {
        return false;
    };
    match stem.split_once('-') {
        Some((prefix, plan)) => {
            !prefix.is_empty() && prefix.chars().all(|c| c.is_ascii_digit()) && !plan.is_empty()
        }
        None => false,
    }
}

/// Lowercase ASCII letters and digits in hyphen-separated words.
fn is_kebab_case(slug: &str) -> bool {
    !slug.is_empty()
        && slug.split('-').all(|word| {
            !word.is_empty()
                && word
                    .chars()
                    .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit())
        })
}

/// Extract a `**Name:**` field value, anchoring to the START of the trimmed
/// line. Using `strip_prefix` (not `find`) means prose that mentions a
/// backtick-quoted `**ID:**` or `**Status:**` mid-line never false-matches.
fn extract_bold_field<'a>(line: &'a str, name: &str) -> Option<&'a str> {
    let trimmed = line.trim();
    let prefix = format!("**{name}:**");
    let rest = trimmed.strip_prefix(&prefix)?;
    Some(rest.trim())
}

fn is_valid_status(status: &str) -> bool {
    let trimmed = status.trim();
    if trimmed == "Accepted" || trimmed == "Deprecated" {
        return true;
    }
    matches!(
        trimmed.strip_prefix(SUPERSEDED_BY_PREFIX),
        Some(rest) if !rest.trim().is_empty()
    )
}

/// Leading numeric `NNN-` prefix of a fragment file name; fragments without a
/// numeric prefix sort last.
fn numeric_prefix(name: &str) -> u32 {
    let digits: String = name.chars().take_while(|c| c.is_ascii_digit()).collect();
    digits.parse::<u32>().unwrap_or(u32::MAX)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fragment_name_rule() {
        for name in ["001-plan-a.md", "7-x.md"] {
            assert!(is_valid_fragment_name(name), "{name}");
        }
        for name in [
            "plan-a.md",
            "001-.md",
            "-plan.md",
            "a01-plan.md",
            "001-plan-a.txt",
        ] {
            assert!(!is_valid_fragment_name(name), "{name}");
        }
    }

    #[test]
    fn kebab_case_rule() {
        for slug in ["use-line-scanner", "v2-scanner"] {
            assert!(is_kebab_case(slug), "{slug}");
        }
        for slug in [
            "",
            "Use-scanner",
            "use_scanner",
            "use--scanner",
            "-use",
            "use-",
            "use scanner",
        ] {
            assert!(!is_kebab_case(slug), "{slug}");
        }
    }

    fn fragment(name: &str, content: &str) -> (String, String) {
        (name.to_string(), content.to_string())
    }

    const VALID_A: &str = "# Decisions: plan-a\n\n## ADR: Use line scanner\n\n**ID:** use-line-scanner\n**Plan:** plan-a\n**Status:** Superseded by faster-scanner\n\n### Context\n\nx\n\n### Decision\n\ny\n";
    const VALID_B: &str = "# Decisions: plan-b\n\n## ADR: Faster scanner\n\n**ID:** faster-scanner\n**Plan:** plan-b\n**Status:** Accepted\n**Supersedes:** use-line-scanner\n\n### Context\n\nx\n\n### Decision\n\ny\n";

    // --- fragment validation ---

    #[test]
    fn valid_fragments_pass_with_cross_references() {
        let frags = vec![
            fragment("001-plan-a.md", VALID_A),
            fragment("002-plan-b.md", VALID_B),
        ];
        let result = validate_fragments(&frags);
        assert!(result.is_success(), "errors: {:?}", result.errors);
    }

    #[test]
    fn empty_fragment_set_passes() {
        let result = validate_fragments(&[]);
        assert!(result.is_success());
    }

    #[test]
    fn absent_dir_passes() {
        let result = validate_decisions_dir(Path::new("/nonexistent/decision/dir"));
        assert!(result.is_success());
    }

    #[test]
    fn slug_identity_collected() {
        let frags = vec![
            fragment("001-plan-a.md", VALID_A),
            fragment("002-plan-b.md", VALID_B),
        ];
        let result = validate_fragments(&frags);
        // Both references resolve, so no unresolved errors.
        assert!(!result.errors.iter().any(|e| matches!(
            e,
            DecisionLogError::UnresolvedSupersedes { .. }
                | DecisionLogError::UnresolvedStatusSlug { .. }
        )));
    }

    #[test]
    fn duplicate_slug_reported_once() {
        let a = "# Decisions: plan-a\n\n## ADR: A\n\n**ID:** dup\n**Plan:** plan-a\n**Status:** Accepted\n\n### Context\n\nx\n\n### Decision\n\ny\n";
        let b = "# Decisions: plan-c\n\n## ADR: B\n\n**ID:** dup\n**Plan:** plan-c\n**Status:** Accepted\n\n### Context\n\nx\n\n### Decision\n\ny\n";
        let frags = vec![fragment("001-a.md", a), fragment("002-c.md", b)];
        let result = validate_fragments(&frags);
        let dupes: Vec<_> = result
            .errors
            .iter()
            .filter(|e| matches!(e, DecisionLogError::DuplicateSlug { slug } if slug == "dup"))
            .collect();
        assert_eq!(dupes.len(), 1, "errors: {:?}", result.errors);
    }

    #[test]
    fn duplicate_slug_does_not_cascade_into_unresolved() {
        // Two ADRs share slug `dup`; a third supersedes `dup`. The duplicate
        // slug must still be known so the reference resolves.
        let a = "# Decisions: plan-a\n\n## ADR: A\n\n**ID:** dup\n**Plan:** plan-a\n**Status:** Accepted\n\n### Context\n\nx\n\n### Decision\n\ny\n";
        let b = "# Decisions: plan-b\n\n## ADR: B\n\n**ID:** dup\n**Plan:** plan-b\n**Status:** Accepted\n\n### Context\n\nx\n\n### Decision\n\ny\n\n## ADR: C\n\n**ID:** c\n**Plan:** plan-b\n**Status:** Accepted\n**Supersedes:** dup\n\n### Context\n\nx\n\n### Decision\n\ny\n";
        let result = validate_fragments(&[fragment("001-a.md", a), fragment("002-b.md", b)]);
        assert!(
            !result
                .errors
                .iter()
                .any(|e| matches!(e, DecisionLogError::UnresolvedSupersedes { .. }))
        );
    }

    #[test]
    fn unresolved_supersedes_fails() {
        let a = "# Decisions: plan-a\n\n## ADR: Dangling supersede\n\n**ID:** dangling-supersede\n**Plan:** plan-a\n**Status:** Accepted\n**Supersedes:** ghost-slug\n\n### Context\n\nx\n\n### Decision\n\ny\n";
        let result = validate_fragments(&[fragment("001-plan-a.md", a)]);
        assert!(result.errors.iter().any(|e| matches!(
            e,
            DecisionLogError::UnresolvedSupersedes { adr, target }
                if adr == "dangling-supersede" && target == "ghost-slug"
        )));
    }

    #[test]
    fn unresolved_status_slug_fails() {
        let a = "# Decisions: plan-a\n\n## ADR: Dangling status\n\n**ID:** dangling-status\n**Plan:** plan-a\n**Status:** Superseded by ghost-slug\n\n### Context\n\nx\n\n### Decision\n\ny\n";
        let result = validate_fragments(&[fragment("001-plan-a.md", a)]);
        assert!(result.errors.iter().any(|e| matches!(
            e,
            DecisionLogError::UnresolvedStatusSlug { adr, target }
                if adr == "dangling-status" && target == "ghost-slug"
        )));
    }

    #[test]
    fn missing_id_fails_naming_fragment_and_title() {
        let a = "# Decisions: plan-a\n\n## ADR: No identity\n\n**Plan:** plan-a\n**Status:** Accepted\n\n### Context\n\nx\n\n### Decision\n\ny\n";
        let result = validate_fragments(&[fragment("001-plan-a.md", a)]);
        assert!(result.errors.iter().any(|e| matches!(
            e,
            DecisionLogError::AdrMissingId { fragment, title }
                if fragment == "001-plan-a.md" && title == "No identity"
        )));
        // Error text must surface both the fragment and the title.
        let rendered: String = result.errors.iter().map(|e| e.to_string()).collect();
        assert!(rendered.contains("001-plan-a.md"));
        assert!(rendered.contains("No identity"));
    }

    #[test]
    fn missing_h1_fails_naming_fragment() {
        let a = "## ADR: Orphan\n\n**ID:** orphan\n**Plan:** plan-a\n**Status:** Accepted\n\n### Context\n\nx\n\n### Decision\n\ny\n";
        let result = validate_fragments(&[fragment("001-plan-a.md", a)]);
        assert!(result.errors.iter().any(|e| matches!(
            e,
            DecisionLogError::FragmentMissingH1 { fragment } if fragment == "001-plan-a.md"
        )));
    }

    #[test]
    fn missing_field_names_slug_and_field() {
        let a = "# Decisions: plan-a\n\n## ADR: Use scanner\n\n**ID:** use-line-scanner\n**Plan:** plan-a\n\n### Context\n\nx\n\n### Decision\n\ny\n";
        let result = validate_fragments(&[fragment("001-plan-a.md", a)]);
        assert!(result.errors.iter().any(|e| matches!(
            e,
            DecisionLogError::AdrMissingField { adr, field }
                if adr == "use-line-scanner" && field == "**Status:**"
        )));
    }

    #[test]
    fn invalid_status_fails() {
        let a = "# Decisions: plan-a\n\n## ADR: Use scanner\n\n**ID:** use-line-scanner\n**Plan:** plan-a\n**Status:** Pending\n\n### Context\n\nx\n\n### Decision\n\ny\n";
        let result = validate_fragments(&[fragment("001-plan-a.md", a)]);
        assert!(result.errors.iter().any(|e| matches!(
            e,
            DecisionLogError::InvalidStatus { status, .. } if status == "Pending"
        )));
    }

    #[test]
    fn status_vocabulary_accepted_deprecated_superseded() {
        assert!(is_valid_status("Accepted"));
        assert!(is_valid_status("Deprecated"));
        assert!(is_valid_status("Superseded by some-slug"));
        assert!(!is_valid_status("Superseded by "));
        assert!(!is_valid_status("Pending"));
    }

    #[test]
    fn optional_sections_absent_passes() {
        let a = "# Decisions: plan-a\n\n## ADR: Use scanner\n\n**ID:** use-line-scanner\n**Plan:** plan-a\n**Status:** Accepted\n\n### Context\n\nx\n\n### Decision\n\ny\n";
        let result = validate_fragments(&[fragment("001-plan-a.md", a)]);
        assert!(result.is_success(), "errors: {:?}", result.errors);
    }

    #[test]
    fn prose_mentioning_field_names_passes() {
        let a = "# Decisions: plan-a\n\n## ADR: Use scanner\n\n**ID:** use-line-scanner\n**Plan:** plan-a\n**Status:** Accepted\n\n### Context\n\nA prose mention of `**ID:**` or `**Status:**` in backticks MUST NOT be parsed as a real field.\n\n### Decision\n\nMatch `**Status:**` and `**ID:**` only when the trimmed line begins with the marker.\n";
        let result = validate_fragments(&[fragment("001-plan-a.md", a)]);
        assert!(result.is_success(), "errors: {:?}", result.errors);
    }

    // --- render ordering ---

    #[test]
    fn render_orders_by_numeric_prefix() {
        let out = render_permanent_log(&[
            fragment("002-plan-b.md", VALID_B),
            fragment("001-plan-a.md", VALID_A),
        ]);
        let a = out.find("## ADR: Use line scanner").unwrap();
        let b = out.find("## ADR: Faster scanner").unwrap();
        assert!(a < b, "out: {out}");
    }

    #[test]
    fn render_breaks_ties_by_filename() {
        let out = render_permanent_log(&[
            fragment("001-plan-b.md", VALID_B),
            fragment("001-plan-a.md", VALID_A),
        ]);
        let a = out.find("## ADR: Use line scanner").unwrap();
        let b = out.find("## ADR: Faster scanner").unwrap();
        assert!(a < b, "out: {out}");
    }

    #[test]
    fn render_preserves_intra_fragment_order() {
        let a = "# Decisions: plan-a\n\n## ADR: Zeta first\n\n**ID:** zeta-first\n**Plan:** plan-a\n**Status:** Accepted\n\n### Context\n\nx\n\n### Decision\n\ny\n\n## ADR: Alpha second\n\n**ID:** alpha-second\n**Plan:** plan-a\n**Status:** Accepted\n\n### Context\n\nx\n\n### Decision\n\ny\n";
        let out = render_permanent_log(&[fragment("001-plan-a.md", a)]);
        let zeta = out.find("## ADR: Zeta first").unwrap();
        let alpha = out.find("## ADR: Alpha second").unwrap();
        assert!(zeta < alpha, "out: {out}");
    }

    #[test]
    fn render_header_only_when_empty() {
        let out = render_permanent_log(&[]);
        assert_eq!(out, "# Architecture Decision Records\n");
    }

    #[test]
    fn render_drops_fragment_h1() {
        let out = render_permanent_log(&[fragment("001-plan-a.md", VALID_A)]);
        assert!(!out.contains("# Decisions:"), "out: {out}");
        assert!(out.contains("**ID:** use-line-scanner"));
        assert!(out.contains("### Decision"));
    }
}
