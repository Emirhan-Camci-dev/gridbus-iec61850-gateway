//! DealShield-Core (Community Edition)
//!
//! Basic redaction pipeline using standard regex pattern matching.
//! Suited for generic PII (emails, generic numbers).

use regex::Regex;

pub struct BasicRedactionEngine {
    email_regex: Regex,
}

impl Default for BasicRedactionEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl BasicRedactionEngine {
    pub fn new() -> Self {
        Self {
            email_regex: Regex::new(r"(?i)[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}").unwrap(),
        }
    }

    /// Redacts simple PII (like emails) with black-box replacement.
    pub fn sanitize_text(&self, input: &str) -> String {
        self.email_regex
            .replace_all(input, "[REDACTED_EMAIL]")
            .to_string()
    }
}
