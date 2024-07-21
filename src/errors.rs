use std::{fmt, str::FromStr};

// https://rustc-dev-guide.rust-lang.org/tests/ui.html#error-levels
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RustcErrorKind {
    Help,
    Error,
    Note,
    Suggestion,
    Warning,
}

impl FromStr for RustcErrorKind {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_uppercase();
        // Some RustcErrorKinds has this colon, so we need to split it
        // See this for example:
        // https://github.com/rust-lang/rust/blob/master/tests/ui/async-await/in-trait/fn-not-async-err.rs#L9
        let part0: &str = s
            .split(':')
            .next()
            .expect("split always returns at least one element");
        match part0 {
            "HELP" => Ok(RustcErrorKind::Help),
            "ERROR" => Ok(RustcErrorKind::Error),
            "NOTE" => Ok(RustcErrorKind::Note),
            "SUGGESTION" => Ok(RustcErrorKind::Suggestion),
            "WARN" | "WARNING" => Ok(RustcErrorKind::Warning),
            _ => Err(()),
        }
    }
}

impl fmt::Display for RustcErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            RustcErrorKind::Help => write!(f, "help message"),
            RustcErrorKind::Error => write!(f, "error"),
            RustcErrorKind::Note => write!(f, "note"),
            RustcErrorKind::Suggestion => write!(f, "suggestion"),
            RustcErrorKind::Warning => write!(f, "warning"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_str_help_returns_help() {
        assert_eq!(
            RustcErrorKind::from_str("help").unwrap(),
            RustcErrorKind::Help
        );
        assert_eq!(
            RustcErrorKind::from_str("help:").unwrap(),
            RustcErrorKind::Help
        );
    }

    #[test]
    fn from_str_error_returns_error() {
        assert_eq!(
            RustcErrorKind::from_str("error").unwrap(),
            RustcErrorKind::Error
        );
    }

    #[test]
    fn from_str_note_returns_note() {
        assert_eq!(
            RustcErrorKind::from_str("note").unwrap(),
            RustcErrorKind::Note
        );
    }

    #[test]
    fn from_str_suggestion_returns_suggestion() {
        assert_eq!(
            RustcErrorKind::from_str("suggestion").unwrap(),
            RustcErrorKind::Suggestion
        );
    }

    #[test]
    fn from_str_warning_returns_warning() {
        assert_eq!(
            RustcErrorKind::from_str("warning").unwrap(),
            RustcErrorKind::Warning
        );
    }

    #[test]
    fn from_str_warn_returns_warning() {
        assert_eq!(
            RustcErrorKind::from_str("warn").unwrap(),
            RustcErrorKind::Warning
        );
    }

    #[test]
    fn from_str_unrecognized_returns_err() {
        assert!(RustcErrorKind::from_str("unrecognized").is_err());
    }

    #[test]
    fn from_str_empty_string_returns_err() {
        // split always returns at least one element
        assert!(RustcErrorKind::from_str("").is_err());
    }

    #[test]
    fn display_help_outputs_correct_string() {
        assert_eq!(format!("{}", RustcErrorKind::Help), "help message");
    }

    #[test]
    fn display_error_outputs_correct_string() {
        assert_eq!(format!("{}", RustcErrorKind::Error), "error");
    }

    #[test]
    fn display_note_outputs_correct_string() {
        assert_eq!(format!("{}", RustcErrorKind::Note), "note");
    }

    #[test]
    fn display_suggestion_outputs_correct_string() {
        assert_eq!(format!("{}", RustcErrorKind::Suggestion), "suggestion");
    }

    #[test]
    fn display_warning_outputs_correct_string() {
        assert_eq!(format!("{}", RustcErrorKind::Warning), "warning");
    }
}
