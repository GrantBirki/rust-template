#[cfg(test)]

mod tests {
    // Import the check function from the crate root (src/lib.rs)
    use github_username_regex::valid;

    const VALID_MSG: &str = "the provided handle should be valid";
    const INVALID_MSG: &str = "the provided handle should be invalid";

    #[test]
    fn handle_is_valid() {
        assert!(valid("GrantBirki"), "{VALID_MSG}");
        assert!(valid("MonaLisa"), "{VALID_MSG}");
        assert!(valid("hubot"), "{VALID_MSG}");
        assert!(valid("sup3r-cool-user123"), "{VALID_MSG}");
    }

    #[test]
    fn handle_is_valid_with_a_dash() {
        assert!(valid("test-user"), "{VALID_MSG}");
    }

    #[test]
    fn handle_is_invalid_with_trailing_dash() {
        assert!(!valid("test-"), "{INVALID_MSG}");
    }

    #[test]
    fn handle_is_invalid_with_leading_dash() {
        assert!(!valid("-test"), "{INVALID_MSG}");
    }

    #[test]
    fn handle_is_invalid_due_to_length() {
        assert!(
            !valid(
                "testtesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttesttes"
            ),
            "{INVALID_MSG}"
        );
    }

    #[test]
    fn handle_is_invalid_due_to_special_characters() {
        assert!(!valid("mona!lisa"), "{INVALID_MSG}");
    }

    #[test]
    fn handle_is_invalid_due_to_double_dashes() {
        assert!(!valid("mona--lisa"), "{INVALID_MSG}");
    }

    #[test]
    fn handle_is_invalid_due_to_underscore() {
        assert!(!valid("mona_lisa"), "{INVALID_MSG}");
    }
}
