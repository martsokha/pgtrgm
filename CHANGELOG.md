# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to
[Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.0] - 2026-08-07

### Security

- Updated `diesel` to 2.3 to address six RUSTSEC advisories affecting 2.2.x:
  - [RUSTSEC-2026-0136]: command injection in `COPY FROM`/`COPY TO`
  - [RUSTSEC-2026-0137]: unaligned data access in `SqliteAggregate`
    implementations
  - [RUSTSEC-2026-0135]: unsound transmute when debug printing batch inserts
  - [RUSTSEC-2026-0111]: UTF-8 corruption in the SQLite backend
  - [RUSTSEC-2026-0134]: unsound padding-byte access in MySQL date/time
    serialization
  - [RUSTSEC-2026-0172]: use after free in
    `SqliteConnection::deserialize_readonly_database`

### Changed

- **Breaking:** minimum supported `diesel` version is now 2.3 (was 2.2). No
  fix exists in the 2.2 series; all six advisories are patched only in 2.3.
- **Breaking:** MSRV raised to 1.86 (was 1.85), required by `diesel` 2.3.

## [0.4.0] - 2026-01-10

### Changed

- Disabled diesel default features for minimal dependency footprint
- Added Related Crates section to README (diesel_full_text_search, pgvector)

## [0.3.0] - 2026-01-09

### Added

- `prelude` module for convenient glob imports

### Changed

- Renamed all expression methods with `trgm_` prefix to avoid conflicts with
  Diesel's `PgTextExpressionMethods::similar_to`:
  - `similar_to` → `trgm_similar_to`
  - `word_similar_to` → `trgm_word_similar_to`
  - `strict_word_similar_to` → `trgm_strict_word_similar_to`
  - `distance` → `trgm_distance`
  - `word_distance` → `trgm_word_distance`
  - `strict_word_distance` → `trgm_strict_word_distance`
  - `similar_to_array` → `trgm_similar_to_array`
  - `distance_to_array` → `trgm_distance_to_array`

## [0.2.0] - 2026-01-09

### Changed

- Removed SQLx support (wasn't useful for raw SQL queries)
- Reorganized module structure: `dsl` and `expression_methods` modules
- `TrgmExpressionMethods` and `TrgmArrayExpressionMethods` now in
  `expression_methods` module

### Removed

- `sqlx` feature flag and all SQLx-related code

## [0.1.0] - 2026-01-09

### Added

- Initial release
- Feature flags: `diesel` and `sqlx`
- **Diesel support:**
  - Trigram similarity operators (`%`, `<%`, `%>`, `<<%`, `%>>`)
  - Distance operators (`<->`, `<<->`, `<->>`, `<<<->`, `<->>>`)
  - SQL functions: `similarity`, `word_similarity`, `strict_word_similarity`
  - Utility functions: `show_trgm`, `show_limit`, `set_limit`
  - `TrgmExpressionMethods` trait for ergonomic operator usage
  - Helper functions: `word_similar`, `strict_word_similar`,
    `word_distance_left`, `strict_word_distance_left`
- **SQLx support:**
  - SQL operator constants (`SIMILAR`, `DISTANCE`, etc.)
  - Helper functions for building query fragments

[Unreleased]: https://github.com/martsokha/pgtrgm/compare/v0.5.0...HEAD
[0.5.0]: https://github.com/martsokha/pgtrgm/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/martsokha/pgtrgm/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/martsokha/pgtrgm/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/martsokha/pgtrgm/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/martsokha/pgtrgm/releases/tag/v0.1.0

[RUSTSEC-2026-0111]: https://rustsec.org/advisories/RUSTSEC-2026-0111
[RUSTSEC-2026-0134]: https://rustsec.org/advisories/RUSTSEC-2026-0134
[RUSTSEC-2026-0135]: https://rustsec.org/advisories/RUSTSEC-2026-0135
[RUSTSEC-2026-0136]: https://rustsec.org/advisories/RUSTSEC-2026-0136
[RUSTSEC-2026-0137]: https://rustsec.org/advisories/RUSTSEC-2026-0137
[RUSTSEC-2026-0172]: https://rustsec.org/advisories/RUSTSEC-2026-0172
