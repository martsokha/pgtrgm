//! Array extension traits for pg_trgm operators.

use diesel::expression::{AsExpression, Expression};
use diesel::sql_types::{Array, Text};

use super::expression_methods::TrgmTextType;
use super::functions::array_to_string;
use super::operators::{Distance, Similar};

/// Type alias for array_to_string expression.
type ArrayToStringExpr<T> =
    super::functions::array_to_string<T, <&'static str as AsExpression<Text>>::Expression>;

/// Extension trait for text expressions with array support.
pub trait TrgmArrayExpressionMethods: Expression + Sized
where
    Self::SqlType: TrgmTextType,
{
    /// Checks if this expression is similar to the concatenated array elements.
    ///
    /// Joins the array elements with spaces before comparing.
    fn similar_to_array<T>(self, other: T) -> Similar<Self, ArrayToStringExpr<T::Expression>>
    where
        T: AsExpression<Array<Text>>,
    {
        Similar::new(self, array_to_string(other.as_expression(), " "))
    }

    /// Returns the trigram distance to the concatenated array elements.
    ///
    /// Joins the array elements with spaces before comparing.
    fn distance_to_array<T>(self, other: T) -> Distance<Self, ArrayToStringExpr<T::Expression>>
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
