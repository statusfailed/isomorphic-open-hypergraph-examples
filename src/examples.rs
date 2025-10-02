use crate::circuit::*;
use open_hypergraphs::category::*;
use open_hypergraphs::lax::OpenHypergraph;

pub fn and() -> Term {
    Arr::And.term(Obj::new(1))
}

pub fn copy_and() -> Term {
    use Arr::*;
    Copy.term(Obj::new(2))
        .compose(&And.term(Obj::new(2)))
        .unwrap()
}

// not-quite a full adder! Need a twist
pub fn half_adder() -> Term {
    use Arr::*;
    let bus = Obj::new(2);

    let copies = &Copy.term(bus) | &Copy.term(bus);
    let id = OpenHypergraph::identity(vec![bus]);
    let twist = OpenHypergraph::twist(vec![bus], vec![bus]);
    let middle = &(&id | &twist) | &id;

    let and_xor = &And.term(bus) | &Xor.term(bus);

    copies.compose(&middle).unwrap().compose(&and_xor).unwrap()
}
