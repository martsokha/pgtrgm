//! PostgreSQL pg_trgm extension support for Diesel and SQLx.
//!
//! This crate provides bindings for PostgreSQL's `pg_trgm` extension,
//! which enables trigram-based text similarity matching.
//!
//! # Features
//!
//! - `diesel` - Enable Diesel ORM support
//! - `sqlx` - Enable SQLx support
//!
//! # Diesel Usage
//!
//! ```rust,ignore
//! use pgtrgm::diesel::TrgmExpressionMethods;
//!
//! // Similarity search using the % operator
//! users.filter(name.similar_to("john"))
//!
//! // Word similarity using the <% operator
//! users.filter("john".word_similar_to(name))
//!
//! // Distance operator for ordering
//! users.order_by(name.distance("john"))
//! ```
//!
//! # SQLx Usage
//!
//! ```rust,ignore
//! use pgtrgm::sqlx::{similarity, word_similarity};
//!
//! // Use in raw SQL queries
//! sqlx::query_scalar!(
//!     "SELECT * FROM users WHERE name % $1 ORDER BY name <-> $1",
//!     search_term
//! )
//! ```
//!
//! # PostgreSQL Setup
//!
//! Ensure the pg_trgm extension is enabled in your database:
//!
//! ```sql
//! CREATE EXTENSION IF NOT EXISTS pg_trgm;
//! ```

#[cfg(feature = "diesel")]
pub mod diesel;

#[cfg(feature = "sqlx")]
pub mod sqlx;
