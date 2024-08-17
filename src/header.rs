//! This module contains the logic for parsing rust test headers
//! See [rustc dev guide](https://rustc-dev-guide.rust-lang.org/tests/headers.html#test-headers)

#[derive(Debug)]
pub struct HeaderLine<'ln> {
    pub line_number: usize,
    /// The main part of the header directive, after removing the comment prefix
    /// and the optional revision specifier.
    pub _directive: &'ln str,
    /// DejaGnu formatted header line
    pub dejagnu_header: String,
}

