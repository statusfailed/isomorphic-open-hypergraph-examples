mod circuit;
mod examples;
mod svg;

use circuit::*;
use examples::*;
use svg::save_svg;

use serde_json;
use std::fs;
use std::path::Path;

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
    println!("generated permutation {permutation:?}");
    FiniteFunction::new(VecArray(permutation), n).unwrap()
}

fn permute_hypergraph(
    h: Hypergraph<Obj, Arr>,
    pw: Permutation,
    px: Permutation,
) -> Hypergraph<Obj, Arr> {
    Hypergraph {
        s: h.s.map_indexes(&px).unwrap().map_values(&pw).unwrap(),
        t: h.t.map_indexes(&px).unwrap().map_values(&pw).unwrap(),
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
    let px = random_permutation(term.h.x.len(), seed.wrapping_add(1));

    permute_open_hypergraph(term.clone(), pw, px)
}

fn main() {
    let examples = [
        ("and", and()),
        ("copy_and", copy_and()),
        ("half_adder", half_adder()),
        //
    ];

    // seed zero tends to give identity permutations
    let seed_offset = 2;

    let examples = examples.into_iter().enumerate().map(|(i, (k, v))| {
        let v = v.to_strict();
        let u = random_isomorphic(&v, i as u64 + seed_offset);
        (k, (v, u))
    });

    for (name, (a, b)) in examples {
        let a = lax::OpenHypergraph::from_strict(a);
        let b = lax::OpenHypergraph::from_strict(b);

        println!("{:?}", name);

        // make output dir
        let output_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("example_isomorphisms")
            .join(name);
        std::fs::create_dir_all(&output_dir).unwrap();

        // Save a.json
        let a_json = serde_json::to_string_pretty(&a).unwrap();
        fs::write(&Path::new(&output_dir).join("a.json"), a_json).unwrap();
        // save a.svg
        render_term(a, &Path::new(&output_dir).join("a.svg"));

        // Save a.json
        let b_json = serde_json::to_string_pretty(&b).unwrap();
        fs::write(&Path::new(&output_dir).join("b.json"), b_json).unwrap();
        // save a.svg
        render_term(b, &Path::new(&output_dir).join("b.svg"));
    }
}

fn render_term(term: lax::OpenHypergraph<Obj, Arr>, path: &Path) {
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
