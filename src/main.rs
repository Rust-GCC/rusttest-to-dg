use anyhow::{Context, Result};
use clap::Parser;

mod cli;
mod transform;

fn main() -> Result<()> {
    try_parse()
}

fn try_parse() -> Result<()> {
    let args = cli::Arguments::parse();

    let (code, _stderr_code) = cli::parse_arguments_and_read_file(&args)?;

    let new_code = transform::transform_code(&code).with_context(|| {
        format!(
            "could not transform code from file `{}`",
            args.source_file.display()
        )
    })?;

    cli::update_source_code(&args, new_code)?;

    Ok(())
}
