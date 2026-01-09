//! pg_trgm SQL functions for Diesel.

use diesel::sql_types::{Array, Float, Text};

diesel::define_sql_function! {
    /// Returns a number that indicates how similar the two arguments are.
    ///
    /// The range of the result is zero (indicating that the two strings are
    /// completely dissimilar) to one (indicating that the two strings are identical).
    fn similarity(a: Text, b: Text) -> Float;
}

diesel::define_sql_function! {
    /// Returns a number that indicates the greatest similarity between the set of
    /// trigrams in the first string and any continuous extent of an ordered set of
    /// trigrams in the second string.
    fn word_similarity(a: Text, b: Text) -> Float;
}

diesel::define_sql_function! {
    /// Same as `word_similarity`, but forces extent boundaries to match word boundaries.
    ///
    /// Since we don't have cross-word trigrams, this function actually returns greatest
    /// similarity between first string and any continuous extent of words of the second string.
    fn strict_word_similarity(a: Text, b: Text) -> Float;
}

diesel::define_sql_function! {
    /// Returns an array of all the trigrams in the given string.
    ///
    /// In practice this is seldom useful except for debugging.
    fn show_trgm(text: Text) -> Array<Text>;
}

diesel::define_sql_function! {
    /// Returns the current similarity threshold used by the `%` operator.
    ///
    /// This sets the minimum similarity between two words for them to be considered
    /// similar enough to be misspellings of each other, for example.
    ///
    /// **Deprecated:** Use `SHOW pg_trgm.similarity_threshold` instead.
    fn show_limit() -> Float;
}

diesel::define_sql_function! {
    /// Sets the current similarity threshold that is used by the `%` operator.
    ///
    /// The threshold must be between 0 and 1 (default is 0.3). Returns the same value passed in.
    ///
    /// **Deprecated:** Use `SET pg_trgm.similarity_threshold` instead.
    fn set_limit(threshold: Float) -> Float;
}

diesel::define_sql_function! {
    /// Concatenates array elements using the specified delimiter.
    ///
    /// This is a PostgreSQL built-in function used internally for array support.
    fn array_to_string(array: Array<Text>, delimiter: Text) -> Text;
}
