mod circuit;
mod examples;
mod svg;

use circuit::*;
use examples::*;
use svg::save_svg;

use open_hypergraphs::lax;
use open_hypergraphs::strict::vec::{FiniteFunction, Hypergraph, OpenHypergraph, VecArray};
use rand::SeedableRng;
use rand::rngs::StdRng;
use rand::seq::SliceRandom;

pub type Permutation = FiniteFunction;

fn random_permutation(n: usize, seed: u64) -> Permutation {
    let mut rng = StdRng::seed_from_u64(seed);
    let mut permutation: Vec<usize> = (0..n).collect();
    permutation.shuffle(&mut rng);
    FiniteFunction::new(VecArray(permutation), n).unwrap()
}

fn permute_hypergraph(
    h: Hypergraph<Obj, Arr>,
    pw: Permutation,
    px: Permutation,
) -> Hypergraph<Obj, Arr> {
    Hypergraph {
        s: h.s.map_values(&pw).unwrap(),
        t: h.t.map_values(&pw).unwrap(),
        w: (&pw >> &h.w).unwrap(),
        x: (&px >> &h.x).unwrap(),
    }
}

fn permute_open_hypergraph(
    term: OpenHypergraph<Obj, Arr>,
    pw: Permutation,
    px: Permutation,
) -> OpenHypergraph<Obj, Arr> {
    let OpenHypergraph { s, t, h } = term;
    let s = (&s >> &pw).unwrap();
    let t = (&t >> &pw).unwrap();
    OpenHypergraph::new(s, t, permute_hypergraph(h, pw, px)).unwrap()
}

// Construct a random isomorphism of hypergraphs from a fixed source
fn random_isomorphic(term: &OpenHypergraph<Obj, Arr>, seed: u64) -> OpenHypergraph<Obj, Arr> {
    let pw = random_permutation(term.h.w.len(), seed);
    let px = random_permutation(term.h.x.len(), seed);

    permute_open_hypergraph(term.clone(), pw, px)
}

fn main() {
    let examples = [
        ("and", and()),
        ("copy_and", copy_and()),
        //
    ];

    let examples = examples.into_iter().enumerate().map(|(i, (k, v))| {
        let v = v.to_strict();
        let u = random_isomorphic(&v, i as u64);
        (k, (v, u))
    });

    for (name, (a, b)) in examples {
        println!("{:?}", name);
        // make dir examples/{name}, and save into it...

        // TODO: a.json (use serde)
        // a.svg
        save_image(a, name);

        // TODO: b.json (use serde)
        // b.svg
        save_image(b, name);
    }
}

fn save_image(term: OpenHypergraph<Obj, Arr>, path: &str) {
    let term = lax::OpenHypergraph::from_strict(term);
    let term = term
        .with_nodes(|nodes| {
            nodes
                .into_iter()
                .enumerate()
                .map(|(i, _)| i.to_string())
                .collect()
        })
        .unwrap()
        .with_edges(|edges| {
            edges
                .into_iter()
                .enumerate()
                .map(|(i, a)| format!("{a:?} ({i})"))
                .collect()
        })
        .unwrap();

    save_svg(&term, &path).unwrap()
}
