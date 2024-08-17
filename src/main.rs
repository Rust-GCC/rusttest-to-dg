//! The main entry point of the program.

use {
    anyhow::{Context, Result},
    clap::Parser,
};

mod cli;
mod errors;
mod header;
mod transform;

fn main() -> Result<()> {
    try_parse()
}

fn try_parse() -> Result<()> {
    let args = cli::Arguments::parse();

    let (code, stderr_code) = cli::parse_arguments_and_read_file(&args)?;

    let new_code = transform::transform_code(&code, stderr_code.as_deref()).with_context(|| {
        format!(
            "could not transform code from file `{}`",
            args.source_file.display()
        )
    })?;

    cli::print_source_code(&new_code);

    Ok(())
}
