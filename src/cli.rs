use std::{fs, path};

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "rust test to DejaGnu",
    long_about = "A tool to convert rust tests into DejaGnu tests format"
)]
pub struct Arguments {
    #[arg(
        short = 'f',
        long = "file",
        value_name = "FILE",
        help = "The rust source file to convert into DejaGnu format"
    )]
    pub source_file: path::PathBuf,

    #[arg(
        short = 'e',
        long = "stderr",
        value_name = "STDERR_FILE",
        help = "These file are used to extract rustc error codes, line/column numbers and convert them into DejaGnu format",
        required = false
    )]
    pub stderr_file: Option<path::PathBuf>,
}

pub fn parse_arguments_and_read_file(args: &Arguments) -> Result<(String, Option<String>)> {
    //TODO: maybe to use sanitization to prevent reading files outside the project directory
    let source_code = fs::read_to_string(&args.source_file)
        .with_context(|| format!("could not read sourcefile `{}`", args.source_file.display()))?;

    let err_file =
        match &args.stderr_file {
            Some(stderr_file) => Some(fs::read_to_string(stderr_file).with_context(|| {
                format!("could not read stderr file `{}`", stderr_file.display())
            })?),
            None => None,
        };

    Ok((source_code, err_file))
}

pub fn update_source_code(args: &Arguments, new_code: String) -> Result<()> {
    fs::remove_file(&args.source_file)
        .with_context(|| format!("could not remove file `{}`", &args.source_file.display()))?;

    fs::write(&args.source_file, new_code).with_context(|| {
        format!(
            "could not write updated code to file `{}`",
            &args.source_file.display()
        )
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_required_argument_file() {
        let args = Arguments::parse_from(["test", "-f", "test.rs"]);
        assert_eq!(args.source_file, path::PathBuf::from("test.rs"));
        assert_eq!(args.stderr_file, None);
    }

    #[test]
    fn test_optional_argument_file() {
        let args = Arguments::parse_from(["test", "-f", "test.rs", "-e", "test.stderr"]);
        assert_eq!(args.source_file, path::PathBuf::from("test.rs"));
        assert_eq!(args.stderr_file, Some(path::PathBuf::from("test.stderr")));
    }

    #[test]
    fn debug_args() {
        use clap::CommandFactory;
        let command = Arguments::command();
        command.debug_assert();
    }
}
