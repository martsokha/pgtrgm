# pgtrgm

[![Build](https://github.com/martsokha/pgtrgm/actions/workflows/build.yml/badge.svg)](https://github.com/martsokha/pgtrgm/actions/workflows/build.yml)
[![Crates.io](https://img.shields.io/crates/v/pgtrgm.svg)](https://crates.io/crates/pgtrgm)
[![Documentation](https://docs.rs/pgtrgm/badge.svg)](https://docs.rs/pgtrgm)

PostgreSQL [pg_trgm](https://www.postgresql.org/docs/current/pgtrgm.html) extension support for [Diesel](https://diesel.rs/) and [SQLx](https://github.com/launchbadge/sqlx).

This crate provides bindings for PostgreSQL's `pg_trgm` extension, enabling trigram-based text similarity matching and fuzzy search.

## Features

- **Diesel support** - Full query builder integration with operators and functions
- **SQLx support** - Helper functions and SQL constants for raw queries
- Trigram similarity operators (`%`, `<%`, `%>`, `<<%`, `%>>`)
- Distance operators (`<->`, `<<->`, `<->>`, `<<<->`, `<->>>`)

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

## Usage

See the [API documentation](https://docs.rs/pgtrgm) for detailed usage examples.

## Acknowledgments

This crate is inspired by [triforce_rs](https://github.com/callym/triforce_rs).

## Contributing

Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md)
for details on how to submit pull requests, report issues, and contribute to the
project.

## License

This project is licensed under the MIT License - see the
[LICENSE.txt](LICENSE.txt) file for details.
