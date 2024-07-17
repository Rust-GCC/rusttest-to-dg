mod cli;

use anyhow::{Context, Result};
use clap::Parser;
use std::fs;

const RUSTTEST_ERROR: &str = "//~^ ERROR ";
const DG_ERROR: &str = "// { dg-error \"";

fn main() -> Result<()> {
    try_parse()
}

fn try_parse() -> Result<()> {
    let args = cli::Arguments::parse();

    let code = fs::read_to_string(&args.source_file)
        .with_context(|| format!("could not read file `{}`", args.source_file.display()))?;

    let new_code = transform_code(&code, RUSTTEST_ERROR);

    fs::remove_file(&args.source_file)?;
    fs::write(&args.source_file, new_code.join("\n"))?;

    Ok(())
}

/// This function takes the rust code and rust directive
/// and returns the code with dejagnu directive
fn transform_code(code: &str, rust_directive: &str) -> Vec<String> {
    let mut new_code = Vec::new();

    for line in code.lines() {
        if line.contains(rust_directive) {
            // replace the rust directive to dejagnu directive
            // TODO: Add more directive relative to rustc
            let new_line = line.replace(rust_directive, DG_ERROR);

            // format the line according to dejagnu format
            let new_line = format!("{}\" }}", new_line);
            new_code.push(new_line);
        } else {
            new_code.push(line.to_string());
        }
    }
    new_code
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_code() {
        let dg_msg = "// { dg-error \"expected one of `:`, `@`, or `|`, found `)`\" }";
        let rust_msg = "//~^ ERROR expected one of `:`, `@`, or `|`, found `)`";
        assert_eq!(transform_code(rust_msg, RUSTTEST_ERROR), vec![dg_msg]);
    }
}
