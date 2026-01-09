//! SQL operator constants for pg_trgm.

/// SQL operator for trigram similarity check.
pub const SIMILAR: &str = " % ";

/// SQL operator for word similarity (left).
pub const WORD_SIMILAR_LEFT: &str = " <% ";

/// SQL operator for word similarity (right).
pub const WORD_SIMILAR_RIGHT: &str = " %> ";

/// SQL operator for strict word similarity (left).
pub const STRICT_WORD_SIMILAR_LEFT: &str = " <<% ";

/// SQL operator for strict word similarity (right).
pub const STRICT_WORD_SIMILAR_RIGHT: &str = " %>> ";

/// SQL operator for trigram distance.
pub const DISTANCE: &str = " <-> ";

/// SQL operator for word similarity distance (left).
pub const WORD_DISTANCE_LEFT: &str = " <<-> ";

/// SQL operator for word similarity distance (right).
pub const WORD_DISTANCE_RIGHT: &str = " <->> ";

/// SQL operator for strict word similarity distance (left).
pub const STRICT_WORD_DISTANCE_LEFT: &str = " <<<-> ";

/// SQL operator for strict word similarity distance (right).
pub const STRICT_WORD_DISTANCE_RIGHT: &str = " <->>> ";
