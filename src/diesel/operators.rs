//! pg_trgm operators for Diesel.
//!
//! This module provides the trigram similarity and distance operators
//! from PostgreSQL's pg_trgm extension.

use diesel::expression::{AsExpression, Expression};
use diesel::pg::Pg;
use diesel::sql_types::{Bool, Float, Text};

// The `%` operator checks if two strings are similar based on trigram matching.
// Returns true if the similarity is above the current threshold (default 0.3).
diesel::infix_operator!(Similar, " % ", Bool, backend: Pg);

// The `<%` operator checks if the first argument has a word similar to the second.
// Returns true if there exists a continuous extent in the second operand whose
// similarity to the first operand is greater than the current word similarity threshold.
diesel::infix_operator!(WordSimilarLeft, " <% ", Bool, backend: Pg);

// The `%>` operator - commutator of `<%`.
diesel::infix_operator!(WordSimilarRight, " %> ", Bool, backend: Pg);

// The `<<%` operator checks for strict word similarity.
// Similar to `<%` but uses strict word boundaries.
diesel::infix_operator!(StrictWordSimilarLeft, " <<% ", Bool, backend: Pg);

// The `%>>` operator - commutator of `<<%`.
diesel::infix_operator!(StrictWordSimilarRight, " %>> ", Bool, backend: Pg);

// The `<->` operator returns the distance between two strings.
// Distance is calculated as 1 - similarity.
diesel::infix_operator!(Distance, " <-> ", Float, backend: Pg);

// The `<<->` operator returns the word similarity distance.
diesel::infix_operator!(WordDistanceLeft, " <<-> ", Float, backend: Pg);

// The `<->>` operator - commutator of `<<->`.
diesel::infix_operator!(WordDistanceRight, " <->> ", Float, backend: Pg);

// The `<<<->` operator returns the strict word similarity distance.
diesel::infix_operator!(StrictWordDistanceLeft, " <<<-> ", Float, backend: Pg);

// The `<->>>` operator - commutator of `<<<->`.
diesel::infix_operator!(StrictWordDistanceRight, " <->>> ", Float, backend: Pg);

/// Extension trait for text expressions to provide pg_trgm operators.
///
/// This trait is implemented for all Diesel expressions that return `Text`
/// or `Nullable<Text>`.
pub trait TrgmExpressionMethods: Expression + Sized {
    /// Checks if this expression is similar to the given text using trigram matching.
    ///
    /// Uses the `%` operator. Returns true if similarity exceeds the threshold.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use pgtrgm::TrgmExpressionMethods;
    ///
    /// users.filter(name.similar_to("john"))
    /// ```
    fn similar_to<T>(self, other: T) -> Similar<Self, T::Expression>
    where
        T: AsExpression<Text>,
    {
        Similar::new(self, other.as_expression())
    }

    /// Checks if this expression contains a word similar to the given text.
    ///
    /// Uses the `%>` operator.
    fn word_similar_to<T>(self, other: T) -> WordSimilarRight<Self, T::Expression>
    where
        T: AsExpression<Text>,
    {
        WordSimilarRight::new(self, other.as_expression())
    }

    /// Checks if this expression contains a word strictly similar to the given text.
    ///
    /// Uses the `%>>` operator with strict word boundaries.
    fn strict_word_similar_to<T>(self, other: T) -> StrictWordSimilarRight<Self, T::Expression>
    where
        T: AsExpression<Text>,
    {
        StrictWordSimilarRight::new(self, other.as_expression())
    }

    /// Returns the trigram distance between this expression and the given text.
    ///
    /// Uses the `<->` operator. Distance is 1 - similarity, so lower is better.
    /// Useful for ordering results by similarity.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use pgtrgm::TrgmExpressionMethods;
    ///
    /// users.order_by(name.distance("john"))
    /// ```
    fn distance<T>(self, other: T) -> Distance<Self, T::Expression>
    where
        T: AsExpression<Text>,
    {
        Distance::new(self, other.as_expression())
    }

    /// Returns the word similarity distance.
    ///
    /// Uses the `<->>` operator.
    fn word_distance<T>(self, other: T) -> WordDistanceRight<Self, T::Expression>
    where
        T: AsExpression<Text>,
    {
        WordDistanceRight::new(self, other.as_expression())
    }

    /// Returns the strict word similarity distance.
    ///
    /// Uses the `<->>>` operator.
    fn strict_word_distance<T>(self, other: T) -> StrictWordDistanceRight<Self, T::Expression>
    where
        T: AsExpression<Text>,
    {
        StrictWordDistanceRight::new(self, other.as_expression())
    }
}

impl<T> TrgmExpressionMethods for T where T: Expression {}

/// Checks if the given text has a word similar to the expression.
///
/// Uses the `<%` operator.
///
/// # Example
///
/// ```rust,ignore
/// use pgtrgm::word_similar;
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
