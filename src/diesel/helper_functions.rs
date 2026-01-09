//! Helper functions for pg_trgm operators.

use diesel::expression::AsExpression;
use diesel::sql_types::Text;

use super::operators::*;

/// Checks if the given text has a word similar to the expression.
///
/// Uses the `<%` operator.
///
/// # Example
///
/// ```rust,ignore
/// use pgtrgm::diesel::word_similar;
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
