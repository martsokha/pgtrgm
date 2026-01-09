//! pg_trgm DSL functions for Diesel.
//!
//! This module contains SQL functions and helper functions for pg_trgm operations.

use diesel::expression::AsExpression;
use diesel::sql_types::{Array, Float, Text};

use super::operators::*;

// SQL functions

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

// Helper functions

/// Checks if the given text has a word similar to the expression.
///
/// Uses the `<%` operator.
///
/// # Example
///
/// ```rust,ignore
/// use pgtrgm::dsl::word_similar;
///
/// users.filter(word_similar("john", name))
/// ```
pub fn word_similar<T, U>(left: T, right: U) -> WordSimilarLeft<T::Expression, U::Expression>
where
    T: AsExpression<Text>,
    U: AsExpression<Text>,
{
    WordSimilarLeft::new(left.as_expression(), right.as_expression())
}

/// Checks if the given text has a word strictly similar to the expression.
///
/// Uses the `<<%` operator.
pub fn strict_word_similar<T, U>(
    left: T,
    right: U,
) -> StrictWordSimilarLeft<T::Expression, U::Expression>
where
    T: AsExpression<Text>,
    U: AsExpression<Text>,
{
    StrictWordSimilarLeft::new(left.as_expression(), right.as_expression())
}

/// Returns the word similarity distance with arguments in the left-hand order.
///
/// Uses the `<<->` operator.
pub fn word_distance_left<T, U>(left: T, right: U) -> WordDistanceLeft<T::Expression, U::Expression>
where
    T: AsExpression<Text>,
    U: AsExpression<Text>,
{
    WordDistanceLeft::new(left.as_expression(), right.as_expression())
}

/// Returns the strict word similarity distance with arguments in the left-hand order.
///
/// Uses the `<<<->` operator.
pub fn strict_word_distance_left<T, U>(
    left: T,
    right: U,
) -> StrictWordDistanceLeft<T::Expression, U::Expression>
where
    T: AsExpression<Text>,
    U: AsExpression<Text>,
{
    StrictWordDistanceLeft::new(left.as_expression(), right.as_expression())
}
