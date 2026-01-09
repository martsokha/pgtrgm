//! pg_trgm operators for Diesel.

use diesel::pg::Pg;
use diesel::sql_types::{Bool, Float};

// Returns true if its arguments have a similarity that is greater than the current
// similarity threshold set by `pg_trgm.similarity_threshold`.
diesel::infix_operator!(Similar, " % ", Bool, backend: Pg);

// Returns true if the similarity between the trigram set in the first argument and
// a continuous extent of an ordered trigram set in the second argument is greater
// than the current word similarity threshold set by `pg_trgm.word_similarity_threshold`.
diesel::infix_operator!(WordSimilarLeft, " <% ", Bool, backend: Pg);

// Commutator of the `<%` operator.
diesel::infix_operator!(WordSimilarRight, " %> ", Bool, backend: Pg);

// Returns true if its second argument has a continuous extent of an ordered trigram
// set that matches word boundaries, and its similarity to the trigram set of the first
// argument is greater than the current strict word similarity threshold set by
// `pg_trgm.strict_word_similarity_threshold`.
diesel::infix_operator!(StrictWordSimilarLeft, " <<% ", Bool, backend: Pg);

// Commutator of the `<<%` operator.
diesel::infix_operator!(StrictWordSimilarRight, " %>> ", Bool, backend: Pg);

// Returns the "distance" between the arguments, that is one minus the `similarity()` value.
diesel::infix_operator!(Distance, " <-> ", Float, backend: Pg);

// Returns the "distance" between the arguments, that is one minus the `word_similarity()` value.
diesel::infix_operator!(WordDistanceLeft, " <<-> ", Float, backend: Pg);

// Commutator of the `<<->` operator.
diesel::infix_operator!(WordDistanceRight, " <->> ", Float, backend: Pg);

// Returns the "distance" between the arguments, that is one minus the `strict_word_similarity()` value.
diesel::infix_operator!(StrictWordDistanceLeft, " <<<-> ", Float, backend: Pg);

// Commutator of the `<<<->` operator.
diesel::infix_operator!(StrictWordDistanceRight, " <->>> ", Float, backend: Pg);
