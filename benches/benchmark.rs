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

fn colorer<G: StaticGraph>(c: ColoringAlgo, g: &G) {
    match c {
        ColoringAlgo::RS => rs_coloring(g),
        ColoringAlgo::CS => cs_coloring(g),
        ColoringAlgo::LF => lf_coloring(g),
        ColoringAlgo::SL => sl_coloring(g),
        ColoringAlgo::SDO => sdo_coloring(g),
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

    let rs = Fun::new("RS", move |b, g| b.iter(|| colorer::<EdgeList>(ColoringAlgo::RS, g)));
    let cs = Fun::new("CS", move |b, g| b.iter(|| colorer::<EdgeList>(ColoringAlgo::CS, g)));
    let lf = Fun::new("LF", move |b, g| b.iter(|| colorer::<EdgeList>(ColoringAlgo::LF, g)));
    let sl = Fun::new("SL", move |b, g| b.iter(|| colorer::<EdgeList>(ColoringAlgo::SL, g)));
    let sdo = Fun::new("SDO", move |b, g| b.iter(|| colorer::<EdgeList>(ColoringAlgo::SDO, g)));

    let functions = vec!(rs, cs, lf, sl, sdo);
    c.bench_functions("Graph Coloring EdgeList", functions, EdgeList::random(n,p));

    let rs = Fun::new("RS", move |b, g| b.iter(|| colorer::<AdjMatrix>(ColoringAlgo::RS, g)));
    let cs = Fun::new("CS", move |b, g| b.iter(|| colorer::<AdjMatrix>(ColoringAlgo::CS, g)));
    let lf = Fun::new("LF", move |b, g| b.iter(|| colorer::<AdjMatrix>(ColoringAlgo::LF, g)));
    let sl = Fun::new("SL", move |b, g| b.iter(|| colorer::<AdjMatrix>(ColoringAlgo::SL, g)));
    let sdo = Fun::new("SDO", move |b, g| b.iter(|| colorer::<AdjMatrix>(ColoringAlgo::SDO, g)));

    let functions = vec!(rs, cs, lf, sl, sdo);
    c.bench_functions("Graph Coloring AdjMatrix", functions, AdjMatrix::random(n,p));

    let rs = Fun::new("RS", move |b, g| b.iter(|| colorer::<GrowableAdjMatrix>(ColoringAlgo::RS, g)));
    let cs = Fun::new("CS", move |b, g| b.iter(|| colorer::<GrowableAdjMatrix>(ColoringAlgo::CS, g)));
    let lf = Fun::new("LF", move |b, g| b.iter(|| colorer::<GrowableAdjMatrix>(ColoringAlgo::LF, g)));
    let sl = Fun::new("SL", move |b, g| b.iter(|| colorer::<GrowableAdjMatrix>(ColoringAlgo::SL, g)));
    let sdo = Fun::new("SDO", move |b, g| b.iter(|| colorer::<GrowableAdjMatrix>(ColoringAlgo::SDO, g)));

    let functions = vec!(rs, cs, lf, sl, sdo);
    c.bench_functions("Graph Coloring GrowableAdjMatrix", functions, GrowableAdjMatrix::random(n,p));

    let rs = Fun::new("RS", move |b, g| b.iter(|| colorer::<AdjList>(ColoringAlgo::RS, g)));
    let cs = Fun::new("CS", move |b, g| b.iter(|| colorer::<AdjList>(ColoringAlgo::CS, g)));
    let lf = Fun::new("LF", move |b, g| b.iter(|| colorer::<AdjList>(ColoringAlgo::LF, g)));
    let sl = Fun::new("SL", move |b, g| b.iter(|| colorer::<AdjList>(ColoringAlgo::SL, g)));
    let sdo = Fun::new("SDO", move |b, g| b.iter(|| colorer::<AdjList>(ColoringAlgo::SDO, g)));

    let functions = vec!(rs, cs, lf, sl, sdo);
    c.bench_functions("Graph Coloring AdjList", functions, AdjList::random(n,p));

    let rs = Fun::new("RS", move |b, g| b.iter(|| colorer::<Hybrid>(ColoringAlgo::RS, g)));
    let cs = Fun::new("CS", move |b, g| b.iter(|| colorer::<Hybrid>(ColoringAlgo::CS, g)));
    let lf = Fun::new("LF", move |b, g| b.iter(|| colorer::<Hybrid>(ColoringAlgo::LF, g)));
    let sl = Fun::new("SL", move |b, g| b.iter(|| colorer::<Hybrid>(ColoringAlgo::SL, g)));
    let sdo = Fun::new("SDO", move |b, g| b.iter(|| colorer::<Hybrid>(ColoringAlgo::SDO, g)));

    let functions = vec!(rs, cs, lf, sl, sdo);
    c.bench_functions("Graph Coloring Hybrid", functions, Hybrid::random(n,p));
}

criterion_group!(benches, graphs);
criterion_main!(benches);
