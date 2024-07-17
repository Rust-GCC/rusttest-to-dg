mod cli;
mod transform;

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

    let new_code = transform::transform_code(&code).with_context(|| {
        format!(
            "could not transform code from file `{}`",
            args.source_file.display()
        )
    })?;

    fs::remove_file(&args.source_file)?;
    fs::write(&args.source_file, new_code.join("\n"))?;

    Ok(())
}
