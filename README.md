# pgtrgm

[![Build](https://github.com/martsokha/pgtrgm/actions/workflows/build.yml/badge.svg)](https://github.com/martsokha/pgtrgm/actions/workflows/build.yml)
[![Crates.io](https://img.shields.io/crates/v/pgtrgm.svg)](https://crates.io/crates/pgtrgm)
[![Documentation](https://docs.rs/pgtrgm/badge.svg)](https://docs.rs/pgtrgm)

PostgreSQL [pg_trgm](https://www.postgresql.org/docs/current/pgtrgm.html)
extension support for [Diesel](https://diesel.rs/).

A fork of [triforce_rs](https://github.com/callym/triforce_rs), this crate
provides bindings for PostgreSQL's `pg_trgm` extension, enabling trigram-based
text similarity matching and fuzzy search.

## Features

- Full query builder integration with operators and functions
- Trigram similarity operators (`%`, `<%`, `%>`, `<<%`, `%>>`)
- Distance operators (`<->`, `<<->`, `<->>`, `<<<->`, `<->>>`)

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
pgtrgm = { version = "0.4", features = ["diesel"] }
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

## Usage

```rust,ignore
use diesel::prelude::*;
use pgtrgm::prelude::*;

// Find similar names
let results = users::table
    .filter(users::name.trgm_similar_to("john"))
    .order_by(users::name.trgm_distance("john"))
    .load::<User>(&mut conn)?;

// Get similarity score
let results = users::table
    .select((users::name, similarity(users::name, "john")))
    .filter(users::name.trgm_similar_to("john"))
    .load::<(String, f32)>(&mut conn)?;

// Word similarity for matching within longer text
let results = articles::table
    .filter(articles::content.trgm_word_similar_to("database"))
    .load::<Article>(&mut conn)?;
```

## Related Crates

- [diesel_full_text_search] — PostgreSQL [Full Text Search] support for Diesel
- [pgvector-rust] — PostgreSQL [Vector Similarity Search] support for Diesel

[diesel_full_text_search]: https://github.com/diesel-rs/diesel_full_text_search
[Full Text Search]: https://www.postgresql.org/docs/current/textsearch.html
[pgvector-rust]: https://github.com/pgvector/pgvector-rust
[Vector Similarity Search]: https://github.com/pgvector/pgvector

## Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md)
for details on how to submit pull requests, report issues, and contribute to the
project.

## License

This project is licensed under the MIT License - see the
[LICENSE.txt](LICENSE.txt) file for details.
