//! This module contains the logic for parsing rust test headers
//! See [rustc dev guide](https://rustc-dev-guide.rust-lang.org/tests/headers.html#test-headers)

#[derive(Debug)]
pub struct HeaderLine<'ln> {
    pub line_number: usize,
    /// The main part of the header directive, after removing the comment prefix
    /// and the optional revision specifier.
    pub _directive: &'ln str,
    /// DejaGnu formatted header line
    pub dejagnu_header: String,
}

pub fn parse_additional_options(code: &str) -> Vec<HeaderLine> {
    let mut headers = Vec::new();

    for (line_number, line) in code.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("fn") || line.starts_with("mod") {
            continue;
        }
        if is_header_line(line) {
            if let Some(header_info) = add_additional_options(line, line_number) {
                headers.push(header_info);
            }
        }
    }
    headers
}

pub fn is_header_line(line: &str) -> bool {
    line.trim_start().starts_with("//@")
}

fn add_additional_options(code: &str, line_number: usize) -> Option<HeaderLine> {
    //TODO: If we know the file extension, then update this to
    // let comment = if testfile.extension().is_some_and(|e| e == "rs") { "//@" } else { "#" };
    let comment = "//@";

    if let Some((_header_revision, non_revisioned_directive_line)) = line_directive(comment, code) {
        // The non_revisioned_directive_line is the directive without the "//@" prefix
        let edition = parse_edition(non_revisioned_directive_line);
        edition.as_ref()?;
        Some(HeaderLine {
            line_number: line_number + 1, // 1 based-indexed instead of zero based
            _directive: "edition",
            dejagnu_header: to_dejagnu_edition(edition.unwrap().as_str()),
        })
    } else {
        None
    }
}

fn line_directive<'line>(
    comment: &str,
    original_line: &'line str,
) -> Option<(Option<&'line str>, &'line str)> {
    let after_comment = original_line
        .trim_start()
        .strip_prefix(comment)?
        .trim_start();

    if let Some(after_open_bracket) = after_comment.strip_prefix('[') {
        let Some((line_revision, directive)) = after_open_bracket.split_once(']') else {
            panic!(
                "malformed condition directive: expected `{comment}[foo]`, found `{original_line}`"
            )
        };

        Some((Some(line_revision), directive.trim_start()))
    } else {
        Some((None, after_comment))
    }
}

fn parse_edition(line: &str) -> Option<String> {
    parse_name_value_directive(line, "edition")
}

fn parse_name_value_directive(line: &str, directive: &str) -> Option<String> {
    let colon = directive.len();

    if line.starts_with(directive) && line.as_bytes().get(colon) == Some(&b':') {
        let value = line[(colon + 1)..].to_owned();
        Some(value)
    } else {
        None
    }
}

fn to_dejagnu_edition(edition: &str) -> String {
    format!(
        "// {{ dg-additional-options \"-frust-edition={}\" }}",
        edition
    )
}
