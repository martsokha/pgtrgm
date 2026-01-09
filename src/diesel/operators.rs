//! pg_trgm operators for Diesel.

use diesel::pg::Pg;
use diesel::sql_types::{Bool, Float};

// The `%` operator checks if two strings are similar based on trigram matching.
diesel::infix_operator!(Similar, " % ", Bool, backend: Pg);

// The `<%` operator checks if the first argument has a word similar to the second.
diesel::infix_operator!(WordSimilarLeft, " <% ", Bool, backend: Pg);

// The `%>` operator - commutator of `<%`.
diesel::infix_operator!(WordSimilarRight, " %> ", Bool, backend: Pg);

// The `<<%` operator checks for strict word similarity.
diesel::infix_operator!(StrictWordSimilarLeft, " <<% ", Bool, backend: Pg);

// The `%>>` operator - commutator of `<<%`.
diesel::infix_operator!(StrictWordSimilarRight, " %>> ", Bool, backend: Pg);

// The `<->` operator returns the distance between two strings.
diesel::infix_operator!(Distance, " <-> ", Float, backend: Pg);

// The `<<->` operator returns the word similarity distance.
diesel::infix_operator!(WordDistanceLeft, " <<-> ", Float, backend: Pg);

// The `<->>` operator - commutator of `<<->`.
diesel::infix_operator!(WordDistanceRight, " <->> ", Float, backend: Pg);

// The `<<<->` operator returns the strict word similarity distance.
diesel::infix_operator!(StrictWordDistanceLeft, " <<<-> ", Float, backend: Pg);

// The `<->>>` operator - commutator of `<<<->`.
diesel::infix_operator!(StrictWordDistanceRight, " <->>> ", Float, backend: Pg);
