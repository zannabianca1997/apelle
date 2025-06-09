use itertools::Itertools as _;

/// Normalize a query to help with caching
///
/// This will:
/// - Remove non-alphanumeric characters (by unicode definition, so kanji & co. are not removed)
/// - Replace multiple spaces with a single space
/// - Remove leading and trailing spaces
/// - Convert to lowercase
pub fn normalize_query(query: &str) -> String {
    query
        // Remove non-alphanumeric characters
        .replace(|ch: char| !ch.is_alphanumeric(), " ")
        // Replace multiple spaces with a single space
        .split_whitespace()
        .join(" ")
        // Remove leading and trailing spaces
        .trim()
        // Convert to lowercase
        .to_lowercase()
}

#[cfg(test)]
mod tests {
    mod normalize_query {
        use super::super::*;

        #[test]
        fn should_make_query_lowercase() {
            assert_eq!(normalize_query("Hello World"), "hello world");
        }

        #[test]
        fn should_remove_non_alphanumeric_characters() {
            assert_eq!(normalize_query("hello\nworld ! 42"), "hello world 42");
        }

        #[test]
        fn should_preserve_unicode() {
            assert_eq!(normalize_query("漢字 漢字"), "漢字 漢字");
            assert_eq!(normalize_query("۱ ۲ ۳ ۴ ۵ ۶ ۷ ۸ ۹"), "۱ ۲ ۳ ۴ ۵ ۶ ۷ ۸ ۹");
        }

        #[test]
        fn should_replace_multiple_spaces_with_a_single_space() {
            assert_eq!(normalize_query("  hello    world  "), "hello world");
        }
    }
}
