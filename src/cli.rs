use clap::Parser;
use std::path;

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
