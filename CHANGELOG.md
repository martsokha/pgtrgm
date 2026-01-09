# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
  - Helper functions: `word_similar`, `strict_word_similar`, `word_distance_left`, `strict_word_distance_left`
- **SQLx support:**
  - SQL operator constants (`SIMILAR`, `DISTANCE`, etc.)
  - Helper functions for building query fragments

[Unreleased]: https://github.com/martsokha/pgtrgm/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/martsokha/pgtrgm/releases/tag/v0.1.0
