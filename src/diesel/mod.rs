//! Diesel ORM support for pg_trgm.
//!
//! # Similarity Search
//!
//! Use the `%` operator to find similar strings:
//!
//! ```rust,ignore
//! use diesel::prelude::*;
//! use pgtrgm::diesel::TrgmExpressionMethods;
//!
//! let results = users::table
//!     .filter(users::name.similar_to("john"))
//!     .load::<User>(&mut conn)?;
//! ```
//!
//! # Ordering by Similarity
//!
//! Use the distance operator to order results by relevance:
//!
//! ```rust,ignore
//! use diesel::prelude::*;
//! use pgtrgm::diesel::TrgmExpressionMethods;
//!
//! let results = users::table
//!     .filter(users::name.similar_to("john"))
//!     .order_by(users::name.distance("john"))
//!     .load::<User>(&mut conn)?;
//! ```
//!
//! # Similarity Score
//!
//! Get the actual similarity score between strings:
//!
//! ```rust,ignore
//! use diesel::prelude::*;
//! use pgtrgm::diesel::similarity;
//!
//! let results = users::table
//!     .select((users::name, similarity(users::name, "john")))
//!     .filter(users::name.similar_to("john"))
//!     .load::<(String, f32)>(&mut conn)?;
//! ```
//!
//! # Word Similarity
//!
//! For matching words within longer text:
//!
//! ```rust,ignore
//! use diesel::prelude::*;
//! use pgtrgm::diesel::TrgmExpressionMethods;
//!
//! let results = articles::table
//!     .filter(articles::content.word_similar_to("database"))
//!     .load::<Article>(&mut conn)?;
//! ```

mod array_expression_methods;
mod expression_methods;
mod functions;
mod helper_functions;
mod operators;

pub use array_expression_methods::TrgmArrayExpressionMethods;
pub use expression_methods::{TrgmExpressionMethods, TrgmTextType};
pub use functions::*;
pub use helper_functions::*;
