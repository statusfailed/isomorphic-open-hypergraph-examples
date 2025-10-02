use crate::circuit::*;
use open_hypergraphs::category::*;

pub fn and() -> Term {
    Arr::And.term()
}

pub fn copy_and() -> Term {
    use Arr::*;
    Copy.term().compose(&And.term()).unwrap()
}
