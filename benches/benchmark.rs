#[macro_use]
extern crate criterion;
extern crate graml;

use criterion::Criterion;
use criterion::Fun;

use graml::graph::EdgeList;
use graml::graph::AdjMatrix;

use graml::coloring::*;
use graml::graph::Graph;

fn edgelist_creation(n: usize) {
    let g = EdgeList::random(n, 0.5);

    let num_edges = g.edges().count();

    assert!(num_edges > 1);
}

fn adj_creation(n: usize) {
    let g = AdjMatrix::random(n, 0.5);

    let num_edges = g.edges().count();

    assert!(num_edges > 1);
}

// EdgeList
fn edgelist_rs(n: usize) {
    let g = EdgeList::random(n, 0.5);
    let _ = rs_coloring(&g);
}

fn edgelist_cs(n: usize) {
    let g = EdgeList::random(n, 0.5);
    let _ = cs_coloring(&g);
}

fn edgelist_lf(n: usize) {
    let g = EdgeList::random(n, 0.5);
    let _ = lf_coloring(&g);
}

fn edgelist_sl(n: usize) {
    let g = EdgeList::random(n, 0.5);
    let _ = sl_coloring(&g);
}

// AdjMatrix
fn adj_rs(n: usize) {
    let g = AdjMatrix::random(n, 0.5);
    let _ = rs_coloring(&g);
}

fn adj_cs(n: usize) {
    let g = AdjMatrix::random(n, 0.5);
    let _ = cs_coloring(&g);
}

fn adj_lf(n: usize) {
    let g = AdjMatrix::random(n, 0.5);
    let _ = lf_coloring(&g);
}

fn adj_sl(n: usize) {
    let g = AdjMatrix::random(n, 0.5);
    let _ = sl_coloring(&g);
}

fn graphs(c: &mut Criterion) {
    let edgelist = Fun::new("EdgeList", |b, i| b.iter(|| edgelist_creation(*i)));
    let adjmatrix = Fun::new("AdjMatrix", |b, i| b.iter(|| adj_creation(*i)));

    let functions = vec!(edgelist, adjmatrix);
    c.bench_functions("Graph Creation", functions, 100);

    let rs = Fun::new("RS", |b, i| b.iter(|| edgelist_rs(*i)));
    let cs = Fun::new("CS", |b, i| b.iter(|| edgelist_cs(*i)));
    let lf = Fun::new("LF", |b, i| b.iter(|| edgelist_lf(*i)));
    let sl = Fun::new("SL", |b, i| b.iter(|| edgelist_sl(*i)));

    let functions = vec!(rs, cs, lf, sl);
    c.bench_functions("Graph Coloring EdgeList", functions, 100);

    let rs = Fun::new("RS", |b, i| b.iter(|| adj_rs(*i)));
    let cs = Fun::new("CS", |b, i| b.iter(|| adj_cs(*i)));
    let lf = Fun::new("LF", |b, i| b.iter(|| adj_lf(*i)));
    let sl = Fun::new("SL", |b, i| b.iter(|| adj_sl(*i)));

    let functions = vec!(rs, cs, lf, sl);
    c.bench_functions("Graph Coloring AdjMatrix", functions, 100);

    let sl_el = Fun::new("EdgeList", |b, i| b.iter(|| edgelist_sl(*i)));
    let sl_am = Fun::new("AdjMatrix", |b, i| b.iter(|| adj_sl(*i)));

    let functions = vec![sl_el, sl_am];
    c.bench_functions("Graph Coloring SL", functions, 100);
}

criterion_group!(benches, graphs);
criterion_main!(benches);
