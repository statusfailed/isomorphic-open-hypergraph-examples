use crate::circuit::*;
use open_hypergraphs::category::*;
use open_hypergraphs::lax::OpenHypergraph;

pub fn and() -> Term {
    Arr::And.term()
}

pub fn copy_and() -> Term {
    use Arr::*;
    Copy.term().compose(&And.term()).unwrap()
}

// not-quite a full adder! Need a twist
pub fn half_adder() -> Term {
    use Arr::*;
    let copies = &Copy.term() | &Copy.term();
    let id = OpenHypergraph::identity(vec![Obj]);
    let twist = OpenHypergraph::twist(vec![Obj], vec![Obj]);
    let middle = &(&id | &twist) | &id;

    let and_xor = &And.term() | &Xor.term();

    copies.compose(&middle).unwrap().compose(&and_xor).unwrap()
}
