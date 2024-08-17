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
