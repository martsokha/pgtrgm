//! Diesel ORM support for PostgreSQL pg_trgm extension.
//!
//! Provides operators, functions, and expression methods for trigram-based
//! similarity matching in Diesel queries.

mod operators;

pub mod dsl;
pub mod expression_methods;
