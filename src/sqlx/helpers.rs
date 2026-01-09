//! Helper functions for building pg_trgm SQL fragments.

use super::constants::*;

/// Builds a similarity filter clause using the `%` operator.
///
/// # Example
///
/// ```rust
/// use pgtrgm::sqlx::similarity_filter;
///
/// let clause = similarity_filter("name", "$1");
/// assert_eq!(clause, "name % $1");
/// ```
#[inline]
pub fn similarity_filter(column: &str, param: &str) -> String {
    format!("{column}{SIMILAR}{param}")
}

/// Builds a word similarity filter clause using the `%>` operator.
///
/// # Example
///
/// ```rust
/// use pgtrgm::sqlx::word_similarity_filter;
///
/// let clause = word_similarity_filter("name", "$1");
/// assert_eq!(clause, "name %> $1");
/// ```
#[inline]
pub fn word_similarity_filter(column: &str, param: &str) -> String {
    format!("{column}{WORD_SIMILAR_RIGHT}{param}")
}

/// Builds a strict word similarity filter clause using the `%>>` operator.
///
/// # Example
///
/// ```rust
/// use pgtrgm::sqlx::strict_word_similarity_filter;
///
/// let clause = strict_word_similarity_filter("name", "$1");
/// assert_eq!(clause, "name %>> $1");
/// ```
#[inline]
pub fn strict_word_similarity_filter(column: &str, param: &str) -> String {
    format!("{column}{STRICT_WORD_SIMILAR_RIGHT}{param}")
}

/// Builds a distance ordering clause using the `<->` operator.
///
/// # Example
///
/// ```rust
/// use pgtrgm::sqlx::distance_order;
///
/// let clause = distance_order("name", "$1");
/// assert_eq!(clause, "name <-> $1");
/// ```
#[inline]
pub fn distance_order(column: &str, param: &str) -> String {
    format!("{column}{DISTANCE}{param}")
}

/// Builds a word distance ordering clause using the `<->>` operator.
///
/// # Example
///
/// ```rust
/// use pgtrgm::sqlx::word_distance_order;
///
/// let clause = word_distance_order("name", "$1");
/// assert_eq!(clause, "name <->> $1");
/// ```
#[inline]
pub fn word_distance_order(column: &str, param: &str) -> String {
    format!("{column}{WORD_DISTANCE_RIGHT}{param}")
}

/// Builds a strict word distance ordering clause using the `<->>>` operator.
///
/// # Example
///
/// ```rust
/// use pgtrgm::sqlx::strict_word_distance_order;
///
/// let clause = strict_word_distance_order("name", "$1");
/// assert_eq!(clause, "name <->>> $1");
/// ```
#[inline]
pub fn strict_word_distance_order(column: &str, param: &str) -> String {
    format!("{column}{STRICT_WORD_DISTANCE_RIGHT}{param}")
}

/// Builds a `similarity()` function call.
///
/// # Example
///
/// ```rust
/// use pgtrgm::sqlx::similarity_fn;
///
/// let expr = similarity_fn("name", "$1");
/// assert_eq!(expr, "similarity(name, $1)");
/// ```
#[inline]
pub fn similarity_fn(a: &str, b: &str) -> String {
    format!("similarity({a}, {b})")
}

/// Builds a `word_similarity()` function call.
///
/// # Example
///
/// ```rust
/// use pgtrgm::sqlx::word_similarity_fn;
///
/// let expr = word_similarity_fn("$1", "name");
/// assert_eq!(expr, "word_similarity($1, name)");
/// ```
#[inline]
pub fn word_similarity_fn(a: &str, b: &str) -> String {
    format!("word_similarity({a}, {b})")
}

/// Builds a `strict_word_similarity()` function call.
///
/// # Example
///
/// ```rust
/// use pgtrgm::sqlx::strict_word_similarity_fn;
///
/// let expr = strict_word_similarity_fn("$1", "name");
/// assert_eq!(expr, "strict_word_similarity($1, name)");
/// ```
#[inline]
pub fn strict_word_similarity_fn(a: &str, b: &str) -> String {
    format!("strict_word_similarity({a}, {b})")
}

/// Builds a `show_trgm()` function call.
///
/// # Example
///
/// ```rust
/// use pgtrgm::sqlx::show_trgm_fn;
///
/// let expr = show_trgm_fn("$1");
/// assert_eq!(expr, "show_trgm($1)");
/// ```
#[inline]
pub fn show_trgm_fn(text: &str) -> String {
    format!("show_trgm({text})")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_similarity_filter() {
        assert_eq!(similarity_filter("name", "$1"), "name % $1");
    }

    #[test]
    fn test_word_similarity_filter() {
        assert_eq!(word_similarity_filter("name", "$1"), "name %> $1");
    }

    #[test]
    fn test_strict_word_similarity_filter() {
        assert_eq!(strict_word_similarity_filter("name", "$1"), "name %>> $1");
    }

    #[test]
    fn test_distance_order() {
        assert_eq!(distance_order("name", "$1"), "name <-> $1");
    }

    #[test]
    fn test_word_distance_order() {
        assert_eq!(word_distance_order("name", "$1"), "name <->> $1");
    }

    #[test]
    fn test_strict_word_distance_order() {
        assert_eq!(strict_word_distance_order("name", "$1"), "name <->>> $1");
    }

    #[test]
    fn test_similarity_fn() {
        assert_eq!(similarity_fn("name", "$1"), "similarity(name, $1)");
    }

    #[test]
    fn test_word_similarity_fn() {
        assert_eq!(
            word_similarity_fn("$1", "name"),
            "word_similarity($1, name)"
        );
    }

    #[test]
    fn test_strict_word_similarity_fn() {
        assert_eq!(
            strict_word_similarity_fn("$1", "name"),
            "strict_word_similarity($1, name)"
        );
    }

    #[test]
    fn test_show_trgm_fn() {
        assert_eq!(show_trgm_fn("$1"), "show_trgm($1)");
    }
}
