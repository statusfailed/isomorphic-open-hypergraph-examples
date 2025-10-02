use open_hypergraphs::lax::{OpenHypergraph, var};
use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////
// Define the theory of polynomial circuits

/// There is a single generating object in the category; thought of as a primitive type (like "Int"
/// or "Real".
#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Obj;

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

    pub fn term(&self) -> Term {
        self.into()
    }
}

impl From<&Arr> for OpenHypergraph<Obj, Arr> {
    fn from(value: &Arr) -> Self {
        let (m, n) = value.profile();
        let s = vec![Obj; m];
        let t = vec![Obj; n];
        OpenHypergraph::singleton(value.clone(), s, t)
    }
}

////////////////////////////////////////////////////////////////////////////////
// Helper types and functions

/// Type alias for a polynomial circuit term
pub type Term = OpenHypergraph<Obj, Arr>;

pub type Var = var::Var<Obj, Arr>;
