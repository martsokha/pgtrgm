//! pg_trgm SQL functions for Diesel.
//!
//! This module provides bindings to PostgreSQL's pg_trgm functions.

#[allow(unused_imports)]
use diesel::sql_types::Array;
use diesel::sql_types::{Float, Text};

diesel::define_sql_function! {
    /// Returns the similarity between two strings as a float between 0 and 1.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use pgtrgm::similarity;
    ///
    /// users.select(similarity(name, "john"))
    /// ```
    fn similarity(a: Text, b: Text) -> Float;
}

diesel::define_sql_function! {
    /// Returns the word similarity between two strings.
    ///
    /// This function returns the greatest similarity between the first string
    /// and any continuous extent of words in the second string.
    fn word_similarity(a: Text, b: Text) -> Float;
}

diesel::define_sql_function! {
    /// Returns the strict word similarity between two strings.
    ///
    /// Similar to `word_similarity` but uses strict word boundaries.
    fn strict_word_similarity(a: Text, b: Text) -> Float;
}

diesel::define_sql_function! {
    /// Returns an array of all trigrams in the given string.
    ///
    /// Useful for debugging and understanding how pg_trgm tokenizes text.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use pgtrgm::show_trgm;
    ///
    /// // Returns something like ["  c", " ca", "cat", "at "]
    /// diesel::select(show_trgm("cat"))
    /// ```
    fn show_trgm(text: Text) -> Array<Text>;
}

diesel::define_sql_function! {
    /// Returns the current similarity threshold used by the `%` operator.
    ///
    /// The default threshold is 0.3.
    fn show_limit() -> Float;
}

diesel::define_sql_function! {
    /// Sets the similarity threshold used by the `%` operator.
    ///
    /// Returns the new threshold value. The threshold must be between 0 and 1.
    ///
    /// **Note:** This function is deprecated in PostgreSQL 9.6+.
    /// Use `SET pg_trgm.similarity_threshold = 0.5` instead.
    fn set_limit(threshold: Float) -> Float;
}
