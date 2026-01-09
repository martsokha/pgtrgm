//! SQLx support for pg_trgm.
//!
//! SQLx uses raw SQL queries, so this module provides SQL constants and helper functions.
//!
//! # Example
//!
//! ```rust,ignore
//! use sqlx::postgres::PgPool;
//!
//! let results = sqlx::query_as!(
//!     User,
//!     r#"
//!     SELECT id, name, email
//!     FROM users
//!     WHERE name % $1
//!     ORDER BY name <-> $1
//!     LIMIT 10
//!     "#,
//!     search_term
//! )
//! .fetch_all(&pool)
//! .await?;
//! ```
//!
//! # Helper Functions
//!
//! ```rust
//! use pgtrgm::sqlx::{similarity_filter, distance_order, similarity_fn};
//!
//! let filter = similarity_filter("name", "$1");  // "name % $1"
//! let order = distance_order("name", "$1");      // "name <-> $1"
//! let score = similarity_fn("name", "$1");       // "similarity(name, $1)"
//! ```
//!
//! # SQL Constants
//!
//! ```rust
//! use pgtrgm::sqlx::{SIMILAR, DISTANCE, WORD_SIMILAR_LEFT};
//!
//! let query = format!("SELECT * FROM users WHERE name{SIMILAR}$1");
//! ```
//!
//! # Index Creation
//!
//! ```rust
//! use pgtrgm::sqlx::gin_index_sql;
//!
//! let sql = gin_index_sql("users_name_trgm_idx", "users", "name");
//! ```

mod constants;
mod helpers;
mod index;

pub use constants::*;
pub use helpers::*;
pub use index::*;
