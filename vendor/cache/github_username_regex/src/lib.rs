#![forbid(unsafe_code, future_incompatible, rust_2018_idioms)]
#![deny(missing_debug_implementations, nonstandard_style)]
#![warn(missing_docs, rustdoc::all, unreachable_pub)]

//! A lightweight Rust crate to check if a GitHub username / handle is valid

use regex::RegexBuilder;

/// check if a GitHub username is valid
/// # Examples
/// ```
/// use github_username_regex::valid;
/// assert_eq!(valid("monalisa"), true);
/// ```
pub fn valid(handle: &str) -> bool {
    let handle_regex: regex::Regex = RegexBuilder::new(r"^[a-z\d](-?[a-z\d]){0,38}$")
        .case_insensitive(true)
        .build()
        .unwrap();
    handle_regex.is_match(handle)
}
