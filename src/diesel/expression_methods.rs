//! Extension traits for pg_trgm operators.

use diesel::expression::{AsExpression, Expression};
use diesel::sql_types::{Array, Nullable, Text};

use super::dsl::array_to_string;
use super::operators::*;

/// Marker trait for types that support trigram operations.
///
/// Implemented for `Text` and `Nullable<Text>`.
/// Note: `Varchar` is an alias for `Text` in Diesel, so it's automatically supported.
pub trait TrgmTextType {}

impl TrgmTextType for Text {}
impl TrgmTextType for Nullable<Text> {}

/// Extension trait for text expressions to provide pg_trgm operators.
///
/// This trait is implemented for all Diesel expressions that return `Text`
/// or `Nullable<Text>`.
pub trait TrgmExpressionMethods: Expression + Sized
where
    Self::SqlType: TrgmTextType,
{
    /// Checks if this expression is similar to the given text using trigram matching.
    ///
    /// Uses the `%` operator. Returns true if similarity exceeds the threshold.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use pgtrgm::expression_methods::TrgmExpressionMethods;
    ///
    /// users.filter(name.trgm_similar_to("john"))
    /// ```
    fn trgm_similar_to<T>(self, other: T) -> Similar<Self, T::Expression>
    where
        T: AsExpression<Text>,
    {
        Similar::new(self, other.as_expression())
    }

    /// Checks if this expression contains a word similar to the given text.
    ///
    /// Uses the `%>` operator.
    fn trgm_word_similar_to<T>(self, other: T) -> WordSimilarRight<Self, T::Expression>
    where
        T: AsExpression<Text>,
    {
        WordSimilarRight::new(self, other.as_expression())
    }

    /// Checks if this expression contains a word strictly similar to the given text.
    ///
    /// Uses the `%>>` operator with strict word boundaries.
    fn trgm_strict_word_similar_to<T>(self, other: T) -> StrictWordSimilarRight<Self, T::Expression>
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
    /// use pgtrgm::expression_methods::TrgmExpressionMethods;
    ///
    /// users.order_by(name.trgm_distance("john"))
    /// ```
    fn trgm_distance<T>(self, other: T) -> Distance<Self, T::Expression>
    where
        T: AsExpression<Text>,
    {
        Distance::new(self, other.as_expression())
    }

    /// Returns the word similarity distance.
    ///
    /// Uses the `<->>` operator.
    fn trgm_word_distance<T>(self, other: T) -> WordDistanceRight<Self, T::Expression>
    where
        T: AsExpression<Text>,
    {
        WordDistanceRight::new(self, other.as_expression())
    }

    /// Returns the strict word similarity distance.
    ///
    /// Uses the `<->>>` operator.
    fn trgm_strict_word_distance<T>(self, other: T) -> StrictWordDistanceRight<Self, T::Expression>
    where
        T: AsExpression<Text>,
    {
        StrictWordDistanceRight::new(self, other.as_expression())
    }
}

impl<T> TrgmExpressionMethods for T
where
    T: Expression,
    T::SqlType: TrgmTextType,
{
}

/// Type alias for array_to_string expression.
type ArrayToStringExpr<T> =
    super::dsl::array_to_string<T, <&'static str as AsExpression<Text>>::Expression>;

/// Extension trait for text expressions with array support.
pub trait TrgmArrayExpressionMethods: Expression + Sized
where
    Self::SqlType: TrgmTextType,
{
    /// Checks if this expression is similar to the concatenated array elements.
    ///
    /// Joins the array elements with spaces before comparing.
    fn trgm_similar_to_array<T>(self, other: T) -> Similar<Self, ArrayToStringExpr<T::Expression>>
    where
        T: AsExpression<Array<Text>>,
    {
        Similar::new(self, array_to_string(other.as_expression(), " "))
    }

    /// Returns the trigram distance to the concatenated array elements.
    ///
    /// Joins the array elements with spaces before comparing.
    fn trgm_distance_to_array<T>(self, other: T) -> Distance<Self, ArrayToStringExpr<T::Expression>>
    where
        T: AsExpression<Array<Text>>,
    {
        Distance::new(self, array_to_string(other.as_expression(), " "))
    }
}

impl<T> TrgmArrayExpressionMethods for T
where
    T: Expression,
    T::SqlType: TrgmTextType,
{
}
