# pgtrgm

[![Crates.io](https://img.shields.io/crates/v/pgtrgm.svg)](https://crates.io/crates/pgtrgm)
[![Documentation](https://docs.rs/pgtrgm/badge.svg)](https://docs.rs/pgtrgm)
[![License](https://img.shields.io/crates/l/pgtrgm.svg)](LICENSE)
[![CI](https://github.com/martsokha/pgtrgm/actions/workflows/ci.yml/badge.svg)](https://github.com/martsokha/pgtrgm/actions/workflows/ci.yml)

PostgreSQL [pg_trgm](https://www.postgresql.org/docs/current/pgtrgm.html) extension support for [Diesel](https://diesel.rs/) and [SQLx](https://github.com/launchbadge/sqlx).

This crate provides bindings for PostgreSQL's `pg_trgm` extension, enabling trigram-based text similarity matching and fuzzy search.

## Features

- **Diesel support** - Full query builder integration with operators and functions
- **SQLx support** - Helper functions and SQL constants for raw queries
- Trigram similarity operators (`%`, `<%`, `%>`, `<<%`, `%>>`)
- Distance operators (`<->`, `<<->`, `<->>`, `<<<->`, `<->>>`)
- SQL functions (`similarity`, `word_similarity`, `strict_word_similarity`, `show_trgm`)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
# For Diesel users
pgtrgm = { version = "0.1", features = ["diesel"] }

# For SQLx users
pgtrgm = { version = "0.1", features = ["sqlx"] }

# Or both
pgtrgm = { version = "0.1", features = ["diesel", "sqlx"] }
```

## PostgreSQL Setup

Ensure the `pg_trgm` extension is enabled in your database:

```sql
CREATE EXTENSION IF NOT EXISTS pg_trgm;
```

For optimal performance, create a GIN index on columns you'll search:

```sql
CREATE INDEX users_name_trgm_idx ON users USING gin (name gin_trgm_ops);
```

## Diesel Usage

### Similarity Search

Use the `%` operator to find similar strings:

```rust
use diesel::prelude::*;
use pgtrgm::diesel::TrgmExpressionMethods;

// Find users with names similar to "john"
let results = users::table
    .filter(users::name.similar_to("john"))
    .load::<User>(&mut conn)?;
```

### Ordering by Similarity

Use the distance operator to order results by relevance:

```rust
use diesel::prelude::*;
use pgtrgm::diesel::TrgmExpressionMethods;

// Find and order by similarity (lower distance = more similar)
let results = users::table
    .filter(users::name.similar_to("john"))
    .order_by(users::name.distance("john"))
    .load::<User>(&mut conn)?;
```

### Similarity Score

Get the actual similarity score between strings:

```rust
use diesel::prelude::*;
use pgtrgm::diesel::similarity;

// Select similarity score alongside results
let results = users::table
    .select((users::name, similarity(users::name, "john")))
    .filter(users::name.similar_to("john"))
    .load::<(String, f32)>(&mut conn)?;
```

### Word Similarity

For matching words within longer text:

```rust
use diesel::prelude::*;
use pgtrgm::diesel::TrgmExpressionMethods;

// Find where a word in the text is similar to the search term
let results = articles::table
    .filter(articles::content.word_similar_to("database"))
    .load::<Article>(&mut conn)?;
```

## SQLx Usage

SQLx uses raw SQL queries, so this module provides SQL constants and helper functions:

```rust
use sqlx::postgres::PgPool;

// Direct SQL with pg_trgm operators
let results = sqlx::query_as!(
    User,
    r#"
    SELECT id, name, email
    FROM users
    WHERE name % $1
    ORDER BY name <-> $1
    LIMIT 10
    "#,
    search_term
)
.fetch_all(&pool)
.await?;
```

### Helper Functions

```rust
use pgtrgm::sqlx::{similarity_filter, distance_order, similarity_fn};

// Build query fragments
let filter = similarity_filter("name", "$1");  // "name % $1"
let order = distance_order("name", "$1");      // "name <-> $1"
let score = similarity_fn("name", "$1");       // "similarity(name, $1)"
```

### SQL Constants

```rust
use pgtrgm::sqlx::{SIMILAR, DISTANCE, WORD_SIMILAR_LEFT};

// Use constants in dynamic queries
let query = format!("SELECT * FROM users WHERE name{SIMILAR}$1");
```

## API Reference

### Diesel Operators

| Method | SQL Operator | Description |
|--------|--------------|-------------|
| `similar_to()` | `%` | Returns true if similarity > threshold |
| `word_similar_to()` | `%>` | Word similarity check |
| `strict_word_similar_to()` | `%>>` | Strict word similarity check |
| `distance()` | `<->` | Returns 1 - similarity (for ordering) |
| `word_distance()` | `<->>` | Word similarity distance |
| `strict_word_distance()` | `<->>>` | Strict word similarity distance |

### Diesel Functions

| Function | Description |
|----------|-------------|
| `similarity(a, b)` | Returns similarity as float (0.0 to 1.0) |
| `word_similarity(a, b)` | Returns word similarity score |
| `strict_word_similarity(a, b)` | Returns strict word similarity score |
| `show_trgm(text)` | Returns array of trigrams for debugging |
| `show_limit()` | Returns current similarity threshold |

### SQLx Constants

| Constant | SQL |
|----------|-----|
| `SIMILAR` | ` % ` |
| `WORD_SIMILAR_LEFT` | ` <% ` |
| `WORD_SIMILAR_RIGHT` | ` %> ` |
| `DISTANCE` | ` <-> ` |

## Configuration

The default similarity threshold is 0.3. You can change it per-session:

```sql
SET pg_trgm.similarity_threshold = 0.5;
SET pg_trgm.word_similarity_threshold = 0.6;
SET pg_trgm.strict_word_similarity_threshold = 0.5;
```

## License

Licensed under the MIT License. See [LICENSE](LICENSE) for details.
