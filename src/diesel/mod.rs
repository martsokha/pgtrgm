//! Diesel ORM support for pg_trgm.

mod expression_methods;
mod functions;
mod helper_functions;
mod operators;

#[doc(hidden)]
pub mod prelude;

pub use expression_methods::TrgmExpressionMethods;
pub use functions::*;
pub use helper_functions::*;
