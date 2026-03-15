//! Unicode-aware slug generation for URL-safe strings.
//!
//! This crate provides both a quick `slugify` function and a configurable
//! `SlugBuilder` for generating URL-safe slugs from arbitrary strings.
//!
//! # Examples
//!
//! ```
//! use philiprehberger_slug::{slugify, SlugBuilder};
//!
//! // Quick slug generation
//! assert_eq!(slugify("Hello, World!"), "hello-world");
//!
//! // Unicode transliteration
//! assert_eq!(slugify("Café résumé"), "cafe-resume");
//!
//! // Custom configuration
//! let slug = SlugBuilder::new()
//!     .separator('_')
//!     .max_length(10)
//!     .slugify("Hello Beautiful World");
//! assert_eq!(slug, "hello");
//! ```

/// Transliterate a single character to its ASCII equivalent.
///
/// Returns `Some(&str)` for known Unicode-to-ASCII mappings, or `None`
/// if the character has no transliteration in the built-in map.
fn transliterate(c: char) -> Option<&'static str> {
    match c {
        '\u{00E0}' | '\u{00E1}' | '\u{00E2}' | '\u{00E3}' | '\u{00E4}' | '\u{00E5}' => {
            Some("a")
        }
        '\u{00C0}' | '\u{00C1}' | '\u{00C2}' | '\u{00C3}' | '\u{00C4}' | '\u{00C5}' => {
            Some("a")
        }
        '\u{00E8}' | '\u{00E9}' | '\u{00EA}' | '\u{00EB}' => Some("e"),
        '\u{00C8}' | '\u{00C9}' | '\u{00CA}' | '\u{00CB}' => Some("e"),
        '\u{00EC}' | '\u{00ED}' | '\u{00EE}' | '\u{00EF}' => Some("i"),
        '\u{00CC}' | '\u{00CD}' | '\u{00CE}' | '\u{00CF}' => Some("i"),
        '\u{00F2}' | '\u{00F3}' | '\u{00F4}' | '\u{00F5}' | '\u{00F6}' => Some("o"),
        '\u{00D2}' | '\u{00D3}' | '\u{00D4}' | '\u{00D5}' | '\u{00D6}' => Some("o"),
        '\u{00F9}' | '\u{00FA}' | '\u{00FB}' | '\u{00FC}' => Some("u"),
        '\u{00D9}' | '\u{00DA}' | '\u{00DB}' | '\u{00DC}' => Some("u"),
        '\u{00FD}' | '\u{00FF}' => Some("y"),
        '\u{00DD}' => Some("y"),
        '\u{00F1}' | '\u{00D1}' => Some("n"),
        '\u{00E7}' | '\u{00C7}' => Some("c"),
        '\u{00DF}' => Some("ss"),
        '\u{00F8}' | '\u{00D8}' => Some("o"),
        '\u{00E6}' | '\u{00C6}' => Some("ae"),
        '\u{0153}' | '\u{0152}' => Some("oe"),
        '\u{00F0}' | '\u{00D0}' => Some("d"),
        '\u{00FE}' | '\u{00DE}' => Some("th"),
        _ => None,
    }
}

/// Convert any string to a URL-safe slug.
///
/// This uses default settings: `-` as separator, no length limit, and
/// the built-in Unicode transliteration map.
///
/// # Examples
///
/// ```
/// use philiprehberger_slug::slugify;
///
/// assert_eq!(slugify("Hello, World!"), "hello-world");
/// assert_eq!(slugify("Café résumé"), "cafe-resume");
/// assert_eq!(slugify("  leading and trailing  "), "leading-and-trailing");
/// ```
pub fn slugify(input: &str) -> String {
    SlugBuilder::new().slugify(input)
}

/// A configurable slug builder.
///
/// Allows customizing the separator character, maximum length, and
/// character replacement rules.
///
/// # Examples
///
/// ```
/// use philiprehberger_slug::SlugBuilder;
///
/// let builder = SlugBuilder::new()
///     .separator('_')
///     .max_length(15)
///     .replacement('&', "and");
///
/// assert_eq!(builder.slugify("Salt & Pepper"), "salt_and_pepper");
/// ```
pub struct SlugBuilder {
    separator: char,
    max_length: Option<usize>,
    custom_replacements: Vec<(char, String)>,
}

impl SlugBuilder {
    /// Create a new `SlugBuilder` with default settings.
    ///
    /// Defaults: separator = `'-'`, no max length, no custom replacements.
    pub fn new() -> Self {
        Self {
            separator: '-',
            max_length: None,
            custom_replacements: Vec::new(),
        }
    }

    /// Set the separator character used between words.
    ///
    /// # Examples
    ///
    /// ```
    /// use philiprehberger_slug::SlugBuilder;
    ///
    /// let slug = SlugBuilder::new().separator('_').slugify("hello world");
    /// assert_eq!(slug, "hello_world");
    /// ```
    pub fn separator(mut self, sep: char) -> Self {
        self.separator = sep;
        self
    }

    /// Set the maximum length for the generated slug.
    ///
    /// Truncation is word-boundary-aware: the slug is cut at the last
    /// separator position before the limit, so words are not split.
    ///
    /// # Examples
    ///
    /// ```
    /// use philiprehberger_slug::SlugBuilder;
    ///
    /// let slug = SlugBuilder::new().max_length(10).slugify("hello beautiful world");
    /// assert_eq!(slug, "hello");
    /// ```
    pub fn max_length(mut self, len: usize) -> Self {
        self.max_length = Some(len);
        self
    }

    /// Add a custom character replacement.
    ///
    /// Custom replacements take priority over the built-in transliteration map.
    ///
    /// # Examples
    ///
    /// ```
    /// use philiprehberger_slug::SlugBuilder;
    ///
    /// let slug = SlugBuilder::new()
    ///     .replacement('@', "at")
    ///     .slugify("user@example");
    /// assert_eq!(slug, "user-at-example");
    /// ```
    pub fn replacement(mut self, from: char, to: &str) -> Self {
        self.custom_replacements.push((from, to.to_string()));
        self
    }

    /// Generate a slug from the given input string.
    ///
    /// Processing steps:
    /// 1. Apply custom replacements
    /// 2. Transliterate Unicode characters to ASCII
    /// 3. Lowercase the result
    /// 4. Replace non-alphanumeric characters with the separator
    /// 5. Collapse consecutive separators
    /// 6. Trim leading/trailing separators
    /// 7. Truncate to max length at word boundary (if set)
    pub fn slugify(&self, input: &str) -> String {
        let sep = self.separator;

        // Step 1 & 2: Apply custom replacements and transliterate
        let mut intermediate = String::with_capacity(input.len());
        for c in input.chars() {
            // Check custom replacements first
            if let Some((_, replacement)) = self.custom_replacements.iter().find(|(from, _)| *from == c) {
                if !replacement.is_empty() {
                    // For non-alphanumeric chars (like @ &), add word boundaries
                    // For alphanumeric/unicode letters (like ü), inline the replacement
                    if !c.is_alphanumeric() {
                        intermediate.push(' ');
                        intermediate.push_str(replacement);
                        intermediate.push(' ');
                    } else {
                        intermediate.push_str(replacement);
                    }
                }
                continue;
            }

            // Try transliteration
            if let Some(ascii) = transliterate(c) {
                intermediate.push_str(ascii);
                continue;
            }

            intermediate.push(c);
        }

        // Step 3 & 4: Lowercase and replace non-alphanumeric with separator
        let mut slug = String::with_capacity(intermediate.len());
        let mut prev_was_sep = true; // Start true to trim leading separators

        for c in intermediate.chars() {
            let lower = c.to_ascii_lowercase();
            if lower.is_ascii_alphanumeric() {
                slug.push(lower);
                prev_was_sep = false;
            } else if !prev_was_sep {
                // Step 5: Collapse consecutive separators by only adding
                // a separator if the previous char wasn't one
                slug.push(sep);
                prev_was_sep = true;
            }
        }

        // Step 6: Trim trailing separator
        if slug.ends_with(sep) {
            slug.pop();
        }

        // Step 7: Truncate at word boundary if max_length is set
        if let Some(max) = self.max_length {
            if slug.len() > max {
                let truncated = &slug[..max];
                // Find the last separator within the truncated portion
                if let Some(pos) = truncated.rfind(sep) {
                    slug.truncate(pos);
                } else {
                    // No separator found, truncate at max length directly
                    slug.truncate(max);
                }
            }
        }

        slug
    }
}

impl Default for SlugBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_ascii() {
        assert_eq!(slugify("Hello, World!"), "hello-world");
        assert_eq!(slugify("foo bar baz"), "foo-bar-baz");
        assert_eq!(slugify("already-slugged"), "already-slugged");
    }

    #[test]
    fn test_unicode_transliteration() {
        assert_eq!(slugify("Café résumé"), "cafe-resume");
        assert_eq!(slugify("über cool"), "uber-cool");
        assert_eq!(slugify("El Niño"), "el-nino");
        assert_eq!(slugify("Ørsted"), "orsted");
        assert_eq!(slugify("naïve"), "naive");
        assert_eq!(slugify("Ægir"), "aegir");
        assert_eq!(slugify("œuvre"), "oeuvre");
        assert_eq!(slugify("Straße"), "strasse");
        assert_eq!(slugify("Ísland with ð and þ"), "island-with-d-and-th");
    }

    #[test]
    fn test_consecutive_separators() {
        assert_eq!(slugify("hello   world"), "hello-world");
        assert_eq!(slugify("a - b - c"), "a-b-c");
        assert_eq!(slugify("too!!!many???marks"), "too-many-marks");
    }

    #[test]
    fn test_max_length_truncation() {
        let slug = SlugBuilder::new()
            .max_length(10)
            .slugify("hello beautiful world");
        assert_eq!(slug, "hello");

        let slug = SlugBuilder::new()
            .max_length(20)
            .slugify("hello beautiful world");
        assert_eq!(slug, "hello-beautiful");

        // Exact fit
        let slug = SlugBuilder::new()
            .max_length(5)
            .slugify("hello");
        assert_eq!(slug, "hello");
    }

    #[test]
    fn test_max_length_no_separator_in_range() {
        // Single long word with no separator in truncation range
        let slug = SlugBuilder::new()
            .max_length(5)
            .slugify("abcdefghij");
        assert_eq!(slug, "abcde");
    }

    #[test]
    fn test_custom_separator() {
        let slug = SlugBuilder::new()
            .separator('_')
            .slugify("hello world");
        assert_eq!(slug, "hello_world");

        let slug = SlugBuilder::new()
            .separator('.')
            .slugify("foo bar baz");
        assert_eq!(slug, "foo.bar.baz");
    }

    #[test]
    fn test_custom_replacements() {
        let slug = SlugBuilder::new()
            .replacement('&', "and")
            .slugify("Salt & Pepper");
        assert_eq!(slug, "salt-and-pepper");

        let slug = SlugBuilder::new()
            .replacement('@', "at")
            .slugify("user@example");
        assert_eq!(slug, "user-at-example");

        // Custom replacement overrides built-in transliteration
        let slug = SlugBuilder::new()
            .replacement('\u{00FC}', "ue")
            .slugify("über");
        assert_eq!(slug, "ueber");
    }

    #[test]
    fn test_empty_input() {
        assert_eq!(slugify(""), "");
    }

    #[test]
    fn test_all_special_chars() {
        assert_eq!(slugify("!!!@@@###$$$"), "");
        assert_eq!(slugify("---"), "");
    }

    #[test]
    fn test_leading_trailing_whitespace() {
        assert_eq!(slugify("  hello world  "), "hello-world");
        assert_eq!(slugify("\thello\t"), "hello");
        assert_eq!(slugify("\nhello\n"), "hello");
    }

    #[test]
    fn test_numbers() {
        assert_eq!(slugify("version 2.0"), "version-2-0");
        assert_eq!(slugify("100 ways"), "100-ways");
    }

    #[test]
    fn test_mixed_case() {
        assert_eq!(slugify("CamelCaseString"), "camelcasestring");
        assert_eq!(slugify("ALLCAPS"), "allcaps");
    }

    #[test]
    fn test_builder_default() {
        let builder = SlugBuilder::default();
        assert_eq!(builder.slugify("hello world"), "hello-world");
    }
}
