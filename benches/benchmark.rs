#[macro_use]
extern crate criterion;
extern crate graml;

use criterion::Criterion;
use criterion::Fun;

use graml::coloring::*;
use graml::graph::*;

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

fn hybrid_creation(n: usize, p: f32) {
    let g = Hybrid::random(n, p);

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

fn colorer<G: StaticGraph>(c: C, g: &G) {
    match c {
        C::RS => rs_coloring(g),
        C::CS => cs_coloring(g),
        C::LF => lf_coloring(g),
        C::SL => sl_coloring(g),
    };
}

fn graphs(c: &mut Criterion) {
    let n = 200;
    let p = 0.2;

    let edgelist = Fun::new("EdgeList", move |b, i| b.iter(|| edgelist_creation(*i, p)));
    let adjmatrix = Fun::new("AdjMatrix", move |b, i| b.iter(|| adj_creation(*i, p)));
    let gadjmatrix = Fun::new("GrowableAdjMatrix", move |b, i| b.iter(|| gadj_creation(*i, p)));
    let adjlmatrix = Fun::new("AdjList", move |b, i| b.iter(|| adjl_creation(*i, p)));
    let hybrid = Fun::new("Hybrid", move |b, i| b.iter(|| hybrid_creation(*i, p)));

    let functions = vec!(edgelist, adjmatrix, gadjmatrix, adjlmatrix, hybrid);
    c.bench_functions("Graph Creation", functions, n);

    let n = 50;
    let p = 0.2;

    let rs = Fun::new("RS", move |b, g| b.iter(|| colorer::<EdgeList>(C::RS, g)));
    let cs = Fun::new("CS", move |b, g| b.iter(|| colorer::<EdgeList>(C::CS, g)));
    let lf = Fun::new("LF", move |b, g| b.iter(|| colorer::<EdgeList>(C::LF, g)));
    let sl = Fun::new("SL", move |b, g| b.iter(|| colorer::<EdgeList>(C::SL, g)));

    let functions = vec!(rs, cs, lf, sl);
    c.bench_functions("Graph Coloring EdgeList", functions, EdgeList::random(n,p));

    let rs = Fun::new("RS", move |b, g| b.iter(|| colorer::<AdjMatrix>(C::RS, g)));
    let cs = Fun::new("CS", move |b, g| b.iter(|| colorer::<AdjMatrix>(C::CS, g)));
    let lf = Fun::new("LF", move |b, g| b.iter(|| colorer::<AdjMatrix>(C::LF, g)));
    let sl = Fun::new("SL", move |b, g| b.iter(|| colorer::<AdjMatrix>(C::SL, g)));

    let functions = vec!(rs, cs, lf, sl);
    c.bench_functions("Graph Coloring AdjMatrix", functions, AdjMatrix::random(n,p));

    let rs = Fun::new("RS", move |b, g| b.iter(|| colorer::<GrowableAdjMatrix>(C::RS, g)));
    let cs = Fun::new("CS", move |b, g| b.iter(|| colorer::<GrowableAdjMatrix>(C::CS, g)));
    let lf = Fun::new("LF", move |b, g| b.iter(|| colorer::<GrowableAdjMatrix>(C::LF, g)));
    let sl = Fun::new("SL", move |b, g| b.iter(|| colorer::<GrowableAdjMatrix>(C::SL, g)));

    let functions = vec!(rs, cs, lf, sl);
    c.bench_functions("Graph Coloring GrowableAdjMatrix", functions, GrowableAdjMatrix::random(n,p));

    let rs = Fun::new("RS", move |b, g| b.iter(|| colorer::<AdjList>(C::RS, g)));
    let cs = Fun::new("CS", move |b, g| b.iter(|| colorer::<AdjList>(C::CS, g)));
    let lf = Fun::new("LF", move |b, g| b.iter(|| colorer::<AdjList>(C::LF, g)));
    let sl = Fun::new("SL", move |b, g| b.iter(|| colorer::<AdjList>(C::SL, g)));

    let functions = vec!(rs, cs, lf, sl);
    c.bench_functions("Graph Coloring AdjList", functions, AdjList::random(n,p));

    let rs = Fun::new("RS", move |b, g| b.iter(|| colorer::<Hybrid>(C::RS, g)));
    let cs = Fun::new("CS", move |b, g| b.iter(|| colorer::<Hybrid>(C::CS, g)));
    let lf = Fun::new("LF", move |b, g| b.iter(|| colorer::<Hybrid>(C::LF, g)));
    let sl = Fun::new("SL", move |b, g| b.iter(|| colorer::<Hybrid>(C::SL, g)));

    let functions = vec!(rs, cs, lf, sl);
    c.bench_functions("Graph Coloring Hybrid", functions, Hybrid::random(n,p));
}

criterion_group!(benches, graphs);
criterion_main!(benches);
