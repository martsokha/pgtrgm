//! SQLx support for pg_trgm.
//!
//! This module provides helper functions and SQL fragments for using
//! PostgreSQL's `pg_trgm` extension with SQLx.
//!
//! Since SQLx uses raw SQL queries, this module primarily provides:
//! - SQL fragment constants for operators
//! - Helper functions for building queries
//!
//! # Example
//!
//! ```rust,ignore
//! use sqlx::postgres::PgPool;
//!
//! let results = sqlx::query_as!(
//!     User,
//!     r#"
//!     SELECT * FROM users
//!     WHERE name % $1
//!     ORDER BY name <-> $1
//!     LIMIT 10
//!     "#,
//!     search_term
//! )
//! .fetch_all(&pool)
//! .await?;
//! ```

/// SQL operator for trigram similarity check.
///
/// Returns true if the similarity between two strings exceeds the threshold.
pub const SIMILAR: &str = " % ";

/// SQL operator for word similarity (left).
pub const WORD_SIMILAR_LEFT: &str = " <% ";

/// SQL operator for word similarity (right).
pub const WORD_SIMILAR_RIGHT: &str = " %> ";

/// SQL operator for strict word similarity (left).
pub const STRICT_WORD_SIMILAR_LEFT: &str = " <<% ";

/// SQL operator for strict word similarity (right).
pub const STRICT_WORD_SIMILAR_RIGHT: &str = " %>> ";

/// SQL operator for trigram distance.
///
/// Returns 1 - similarity. Lower values indicate more similar strings.
pub const DISTANCE: &str = " <-> ";

/// SQL operator for word similarity distance (left).
pub const WORD_DISTANCE_LEFT: &str = " <<-> ";

/// SQL operator for word similarity distance (right).
pub const WORD_DISTANCE_RIGHT: &str = " <->> ";

/// SQL operator for strict word similarity distance (left).
pub const STRICT_WORD_DISTANCE_LEFT: &str = " <<<-> ";

/// SQL operator for strict word similarity distance (right).
pub const STRICT_WORD_DISTANCE_RIGHT: &str = " <->>> ";

/// Builds a similarity filter clause.
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

/// Builds a distance ordering clause.
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

/// Builds a similarity function call.
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

/// Builds a word_similarity function call.
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

/// Builds a strict_word_similarity function call.
#[inline]
pub fn strict_word_similarity_fn(a: &str, b: &str) -> String {
    format!("strict_word_similarity({a}, {b})")
}

/// Builds a show_trgm function call.
///
/// Returns the SQL expression to get all trigrams from a string.
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
    fn test_distance_order() {
        assert_eq!(distance_order("name", "$1"), "name <-> $1");
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
}
