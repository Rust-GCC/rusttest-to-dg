//! This module contains the code transformation logic.

use {
    crate::{
        errors,
        header::{is_header_line, parse_additional_options},
        regex,
    },
    anyhow::Result,
};

/// Transform code to `DejaGnu` format
pub fn transform_code(code: &str, stderr_file: Option<&str>) -> Result<String> {
    // Load the rustc error messages, codes, lines and relative line numbers
    let errors = errors::load_error(code, stderr_file);
    // For storing the transformed code
    let mut new_code = String::new();
    let additional_options = parse_additional_options(code);

    let mut line_num = 1;
    // finding the respective line number and adding the error code
    for line in code.lines() {
        let mut new_line = line.to_string();

        if is_header_line(line) {
            for header in additional_options.iter() {
                if header.line_number != line_num {
                    continue;
                }
                new_line = header.dejagnu_header.to_string();
                break;
            }
        } else {
            // TODO: This is not the efficient way to find respective line number
            for error in errors.iter() {
                // Checking the original line number
                if (error.line_num as i32 - error.relative_line_num) != line_num as i32 {
                    continue;
                }
                // In rustc test suites, the error directive is
                // on the same line or on the next line, but not on the previous line
                // See this: https://rustc-dev-guide.rust-lang.org/tests/ui.html#error-annotations
                // For the error on the next line
                if error.relative_line_num != 0 {
                    // We simply add the error message, not to worry about the code
                    // The error was printed by our overloaded `Display` trait
                    new_line = format!("{}", error);
                } else {
                    // For the error on the same line, we need to add error message at the end of the line
                    let captures = regex!(r"//(?:\[(?P<revs>[\w\-,]+)])?~(?P<adjust>\||\^*)")
                        .captures(line)
                        .expect("Could not find the error directive");

                    // Get the part of comment before the sigil (e.g. `~^` or ~|)
                    let whole_match = captures.get(0).unwrap();
                    // Get the existing source code before the error directive //~ ERROR or similar to this
                    let before_match = &line[..whole_match.start()];

                    // The error was printed by our overloaded `Display` trait
                    new_line = format!("{}{}", before_match, error);
                }
                break;
            }
        }
        new_code.push_str(&new_line);
        new_code.push('\n');
        line_num += 1;
    }

    Ok(new_code)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform() {
        // as suggested by @CohenArthur, we only need to add error code in msg
        let dg_msg = "// { dg-error \"\" \"\" { target *-*-* } .-1 }\n";
        let rust_msg = "//~^ ERROR expected one of `:`, `@`, or `|`, found `)`";
        assert_eq!(transform_code(rust_msg, None).unwrap(), dg_msg);
    }
}
