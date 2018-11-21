#[macro_use]
extern crate criterion;
extern crate graml;

use criterion::Criterion;
use criterion::Fun;

use graml::graph::EdgeList;
use graml::graph::AdjMatrix;
use graml::graph::GrowableAdjMatrix;
use graml::graph::AdjList;

use graml::coloring::*;
use graml::graph::StaticGraph;

fn edgelist_creation(n: usize, p: f32) {
    let g = EdgeList::random(n, p);

    let num_edges = g.edges().count();

    assert!(num_edges > 1);
}

fn adj_creation(n: usize, p: f32) {
    let g = AdjMatrix::random(n, p);

    let num_edges = g.edges().count();

    assert!(num_edges > 1);
}

fn gadj_creation(n: usize, p: f32) {
    let g = GrowableAdjMatrix::random(n, p);

    let num_edges = g.edges().count();

    assert!(num_edges > 1);
}

fn adjl_creation(n: usize, p: f32) {
    let g = AdjList::random(n, p);

    let num_edges = g.edges().count();

    assert!(num_edges > 1);
}

#[derive(Copy, Clone)]
enum C {
    RS,
    CS,
    LF,
    SL,
}

fn colorer<G: StaticGraph>(c: C, n: usize, p: f32) {
    let g = G::random(n, p);
    match c {
        C::RS => rs_coloring(&g),
        C::CS => cs_coloring(&g),
        C::LF => lf_coloring(&g),
        C::SL => sl_coloring(&g),
    };
}

fn graphs(c: &mut Criterion) {
    let n = 200;
    let p = 0.2;

    let edgelist = Fun::new("EdgeList", move |b, i| b.iter(|| edgelist_creation(*i, p)));
    let adjmatrix = Fun::new("AdjMatrix", move |b, i| b.iter(|| adj_creation(*i, p)));
    let gadjmatrix = Fun::new("GrowableAdjMatrix", move |b, i| b.iter(|| gadj_creation(*i, p)));
    let adjlmatrix = Fun::new("AdjList", move |b, i| b.iter(|| adjl_creation(*i, p)));

    let functions = vec!(edgelist, adjmatrix, gadjmatrix, adjlmatrix);
    c.bench_functions("Graph Creation", functions, n);

    let n = 50;
    let p = 0.2;

    let rs = Fun::new("RS", move |b, _| b.iter(|| colorer::<EdgeList>(C::RS, n, p)));
    let cs = Fun::new("CS", move |b, _| b.iter(|| colorer::<EdgeList>(C::CS, n, p)));
    let lf = Fun::new("LF", move |b, _| b.iter(|| colorer::<EdgeList>(C::LF, n, p)));
    let sl = Fun::new("SL", move |b, _| b.iter(|| colorer::<EdgeList>(C::SL, n, p)));

    let functions = vec!(rs, cs, lf, sl);
    c.bench_functions("Graph Coloring EdgeList", functions, 0);

    let rs = Fun::new("RS", move |b, _| b.iter(|| colorer::<AdjMatrix>(C::RS, n, p)));
    let cs = Fun::new("CS", move |b, _| b.iter(|| colorer::<AdjMatrix>(C::CS, n, p)));
    let lf = Fun::new("LF", move |b, _| b.iter(|| colorer::<AdjMatrix>(C::LF, n, p)));
    let sl = Fun::new("SL", move |b, _| b.iter(|| colorer::<AdjMatrix>(C::SL, n, p)));

    let functions = vec!(rs, cs, lf, sl);
    c.bench_functions("Graph Coloring AdjMatrix", functions, 0);

    let rs = Fun::new("RS", move |b, _| b.iter(|| colorer::<GrowableAdjMatrix>(C::RS, n, p)));
    let cs = Fun::new("CS", move |b, _| b.iter(|| colorer::<GrowableAdjMatrix>(C::CS, n, p)));
    let lf = Fun::new("LF", move |b, _| b.iter(|| colorer::<GrowableAdjMatrix>(C::LF, n, p)));
    let sl = Fun::new("SL", move |b, _| b.iter(|| colorer::<GrowableAdjMatrix>(C::SL, n, p)));

    let functions = vec!(rs, cs, lf, sl);
    c.bench_functions("Graph Coloring GrowableAdjMatrix", functions, 0);

    let rs = Fun::new("RS", move |b, _| b.iter(|| colorer::<AdjList>(C::RS, n, p)));
    let cs = Fun::new("CS", move |b, _| b.iter(|| colorer::<AdjList>(C::CS, n, p)));
    let lf = Fun::new("LF", move |b, _| b.iter(|| colorer::<AdjList>(C::LF, n, p)));
    let sl = Fun::new("SL", move |b, _| b.iter(|| colorer::<AdjList>(C::SL, n, p)));

    let functions = vec!(rs, cs, lf, sl);
    c.bench_functions("Graph Coloring AdjList", functions, 0);

    let el = Fun::new("EdgeList", move |b, _| b.iter(|| colorer::<EdgeList>(C::RS, n, p)));
    let am = Fun::new("AdjMatrix", move |b, _| b.iter(|| colorer::<AdjMatrix>(C::RS, n, p)));
    let gam = Fun::new("GrowableAdjMatrix", move |b, _| b.iter(|| colorer::<GrowableAdjMatrix>(C::RS, n, p)));
    let adl = Fun::new("AdjList", move |b, _| b.iter(|| colorer::<AdjList>(C::RS, n, p)));

    let functions = vec![el, am, gam, adl];
    c.bench_functions("Graph Coloring RS", functions, 0);

    let el = Fun::new("EdgeList", move |b, _| b.iter(|| colorer::<EdgeList>(C::CS, n, p)));
    let am = Fun::new("AdjMatrix", move |b, _| b.iter(|| colorer::<AdjMatrix>(C::CS, n, p)));
    let gam = Fun::new("GrowableAdjMatrix", move |b, _| b.iter(|| colorer::<GrowableAdjMatrix>(C::CS, n, p)));
    let adl = Fun::new("AdjList", move |b, _| b.iter(|| colorer::<AdjList>(C::CS, n, p)));

    let functions = vec![el, am, gam, adl];
    c.bench_functions("Graph Coloring CS", functions, 0);

    let el = Fun::new("EdgeList", move |b, _| b.iter(|| colorer::<EdgeList>(C::LF, n, p)));
    let am = Fun::new("AdjMatrix", move |b, _| b.iter(|| colorer::<AdjMatrix>(C::LF, n, p)));
    let gam = Fun::new("GrowableAdjMatrix", move |b, _| b.iter(|| colorer::<GrowableAdjMatrix>(C::LF, n, p)));
    let adl = Fun::new("AdjList", move |b, _| b.iter(|| colorer::<AdjList>(C::LF, n, p)));

    let functions = vec![el, am, gam, adl];
    c.bench_functions("Graph Coloring LF", functions, 0);

    let el = Fun::new("EdgeList", move |b, _| b.iter(|| colorer::<EdgeList>(C::SL, n, p)));
    let am = Fun::new("AdjMatrix", move |b, _| b.iter(|| colorer::<AdjMatrix>(C::SL, n, p)));
    let gam = Fun::new("GrowableAdjMatrix", move |b, _| b.iter(|| colorer::<GrowableAdjMatrix>(C::SL, n, p)));
    let adl = Fun::new("AdjList", move |b, _| b.iter(|| colorer::<AdjList>(C::SL, n, p)));

    let functions = vec![el, am, gam, adl];
    c.bench_functions("Graph Coloring SL", functions, 0);
}

criterion_group!(benches, graphs);
criterion_main!(benches);
