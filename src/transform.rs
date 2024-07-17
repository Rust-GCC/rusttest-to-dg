use anyhow::Result;

// TODO: Add more directive relative to rustc and DejaGnu
pub const RUSTTEST_ERROR: &str = "//~^ ERROR ";
pub const DG_ERROR: &str = "// { dg-error \"";

/// This function takes the rust code and rust directive
/// and returns the code with DejaGnu directive
pub fn transform_code(code: &str) -> Result<String> {
    let new_code = code
        .lines()
        .map(|line| {
            if line.contains(RUSTTEST_ERROR) {
                transform_line(line, RUSTTEST_ERROR, DG_ERROR)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    Ok(new_code)
}

fn transform_line(line: &str, rust_directive: &str, dejagnu_directive: &str) -> String {
    let new_line = line.replace(rust_directive, dejagnu_directive);
    // format the line according to DejaGnu format
    format!("{}\" }}", new_line)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform() {
        let dg_msg = "// { dg-error \"expected one of `:`, `@`, or `|`, found `)`\" }";
        let rust_msg = "//~^ ERROR expected one of `:`, `@`, or `|`, found `)`";
        assert_eq!(transform_code(rust_msg).unwrap(), dg_msg);
    }

    #[test]
    fn test_transform_line() {
        let dg_msg = "// { dg-error \"expected one of `:`, `@`, or `|`, found `)`\" }";
        let rust_msg = "//~^ ERROR expected one of `:`, `@`, or `|`, found `)`";
        assert_eq!(transform_line(rust_msg, RUSTTEST_ERROR, DG_ERROR), dg_msg);
    }
}
