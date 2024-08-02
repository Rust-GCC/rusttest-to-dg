use std::cell::OnceCell;

use anyhow::Result;
use regex::Regex;

use crate::errors;

/// This function takes the rust code as input
/// and returns the code with DejaGnu directive
pub fn transform_code(code: &str, stderr_file: Option<&str>) -> Result<String> {
    let errors = errors::load_error(code, stderr_file);
    let mut new_code = String::new();

    let mut line_num = 1;
    for line in code.lines() {
        let mut new_line = line.to_string();
        // TODO: This is not the efficient way to find respective line number
        for error in errors.iter() {
            if (error.line_num as i32 - error.relative_line_num) != line_num {
                continue;
            }
            // In rustc test suites, the error directive is
            // on the same line or the next line not on the previous line
            // For the error on the next line
            if error.relative_line_num != 0 {
                new_line = format!("{}", error);
            } else {
                // For the error on the same line
                let re: OnceCell<Regex> = OnceCell::new();

                let captures = re
                    .get_or_init(|| {
                        Regex::new(r"//(?:\[(?P<revs>[\w\-,]+)])?~(?P<adjust>\||\^*)").unwrap()
                    })
                    .captures(line)
                    .expect("Could not find the error directive");

                // Get the part of comment before the sigil (e.g. `~^` or ~|)
                let whole_match = captures.get(0).unwrap();
                let before_match = &line[..whole_match.start()];
                new_line = format!("{}{}", before_match, error);
            }
            break;
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
