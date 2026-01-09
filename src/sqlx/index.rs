//! SQL templates for creating pg_trgm indexes.
//!
//! These constants provide SQL templates for creating GIN and GiST indexes
//! optimized for trigram similarity searches.

/// SQL template for creating a GIN index with trigram ops.
///
/// GIN indexes are generally preferred for similarity searches as they
/// are faster to search but slower to build and update.
///
/// # Example
///
/// ```rust
/// use pgtrgm::sqlx::GIN_INDEX_TEMPLATE;
///
/// let sql = GIN_INDEX_TEMPLATE
///     .replace("{index_name}", "users_name_trgm_idx")
///     .replace("{table}", "users")
///     .replace("{column}", "name");
///
/// assert_eq!(
///     sql,
///     "CREATE INDEX users_name_trgm_idx ON users USING gin (name gin_trgm_ops)"
/// );
/// ```
pub const GIN_INDEX_TEMPLATE: &str =
    "CREATE INDEX {index_name} ON {table} USING gin ({column} gin_trgm_ops)";

/// SQL template for creating a GiST index with trigram ops.
///
/// GiST indexes are faster to build and update but slower to search.
/// They also support the `<->` distance operator for KNN searches.
///
/// # Example
///
/// ```rust
/// use pgtrgm::sqlx::GIST_INDEX_TEMPLATE;
///
/// let sql = GIST_INDEX_TEMPLATE
///     .replace("{index_name}", "users_name_trgm_idx")
///     .replace("{table}", "users")
///     .replace("{column}", "name");
///
/// assert_eq!(
///     sql,
///     "CREATE INDEX users_name_trgm_idx ON users USING gist (name gist_trgm_ops)"
/// );
/// ```
pub const GIST_INDEX_TEMPLATE: &str =
    "CREATE INDEX {index_name} ON {table} USING gist ({column} gist_trgm_ops)";

/// Generates SQL for creating a GIN index.
///
/// # Example
///
/// ```rust
/// use pgtrgm::sqlx::gin_index_sql;
///
/// let sql = gin_index_sql("users_name_trgm_idx", "users", "name");
/// assert_eq!(
///     sql,
///     "CREATE INDEX users_name_trgm_idx ON users USING gin (name gin_trgm_ops)"
/// );
/// ```
#[inline]
pub fn gin_index_sql(index_name: &str, table: &str, column: &str) -> String {
    format!("CREATE INDEX {index_name} ON {table} USING gin ({column} gin_trgm_ops)")
}

/// Generates SQL for creating a GiST index.
///
/// # Example
///
/// ```rust
/// use pgtrgm::sqlx::gist_index_sql;
///
/// let sql = gist_index_sql("users_name_trgm_idx", "users", "name");
/// assert_eq!(
///     sql,
///     "CREATE INDEX users_name_trgm_idx ON users USING gist (name gist_trgm_ops)"
/// );
/// ```
#[inline]
pub fn gist_index_sql(index_name: &str, table: &str, column: &str) -> String {
    format!("CREATE INDEX {index_name} ON {table} USING gist ({column} gist_trgm_ops)")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gin_index_sql() {
        assert_eq!(
            gin_index_sql("idx", "tbl", "col"),
            "CREATE INDEX idx ON tbl USING gin (col gin_trgm_ops)"
        );
    }

    #[test]
    fn test_gist_index_sql() {
        assert_eq!(
            gist_index_sql("idx", "tbl", "col"),
            "CREATE INDEX idx ON tbl USING gist (col gist_trgm_ops)"
        );
    }
}
