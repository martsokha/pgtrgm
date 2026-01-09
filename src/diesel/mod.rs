//! Diesel ORM support for pg_trgm.
//!
//! This module provides Diesel bindings for PostgreSQL's `pg_trgm` extension.

mod functions;
mod operators;

pub use functions::*;
pub use operators::{
    strict_word_distance_left, strict_word_similar, word_distance_left, word_similar,
    TrgmExpressionMethods,
};
