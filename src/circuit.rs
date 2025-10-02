use open_hypergraphs::lax::{OpenHypergraph, var};
use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////
// Define the theory of polynomial circuits

/// There is a single generating object in the category; thought of as a primitive type (like "Int"
/// or "Real".
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize, Copy)]
pub struct Obj {
    bus_width: usize,
}

impl Obj {
    pub fn new(n: usize) -> Self {
        Obj { bus_width: n }
    }
}

/// Generating arrows are basic arithmetic operations with copying and discarding.
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub enum Arr {
    Copy,
    Discard,
    Xor,
    F,
    And,
    T,
}

impl Arr {
    // Arity and coarity
    pub fn profile(&self) -> (usize, usize) {
        use Arr::*;
        match self {
            Copy => (1, 2),
            Discard => (1, 0),
            Xor | And => (2, 1),
            T | F => (0, 1),
        }
    }

    pub fn term(&self, obj: Obj) -> Term {
        let (m, n) = self.profile();
        let s = vec![obj; m];
        let t = vec![obj; n];
        OpenHypergraph::singleton(self.clone(), s, t)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Helper types and functions

/// Type alias for a polynomial circuit term
pub type Term = OpenHypergraph<Obj, Arr>;

pub type Var = var::Var<Obj, Arr>;
