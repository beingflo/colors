mod adjlist;
mod adjmatrix;
mod edgelist;
mod growableadjmatrix;
mod hybrid;

use rand::random;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub use self::adjlist::AdjList;
pub use self::adjmatrix::AdjMatrix;
pub use self::edgelist::EdgeList;
pub use self::growableadjmatrix::GrowableAdjMatrix;
pub use self::hybrid::Hybrid;

pub type Graph = AdjList;

/// The trait to be implemented by any graph datastructure.
/// This requires that graphs can be instantiated with a given capacity
/// and adding edges must work properly if the vertices of the edge are within
/// the capacity of the graph.
/// Implementations may however dynamically grow the graph when an edge with large
/// vertices is added.
/// # Vertices
/// Vertices should be in consecutive order. That is if ```add_edge(1,2)``` is called,
/// the ```vertices``` iterator will yield ```[0, 1, 2]```. With ```0``` being an isolated
/// vertex.
pub trait StaticGraph: Sized {
    /// Constructs a new graph with capacity for ```n``` vertices.
    fn with_capacity(n: usize) -> Self;

    /// Construct an instance of this type from another ```StaticGraph``` implementor
    fn from_graph<G: StaticGraph>(graph: &G) -> Self;

    /// Queries whether an edge exists in the graph.
    fn has_edge(&self, u: usize, v: usize) -> bool;

    /// Adds an edge to the graph.
    /// ```add_edge(u,v)``` has the same effect as ```add_edge(v,u)```
    /// as the graph captures undirected edges.
    /// Adding an edge that already exists has no effect.
    fn add_edge(&mut self, u: usize, v: usize);

    /// Returns an iterator over all the edges in the graph.
    fn edges<'a>(&'a self) -> Box<Iterator<Item = (usize, usize)> + 'a>;

    /// Returns the number of vertices in the graph.
    fn num_vertices(&self) -> usize;

    /// Returns an iterator over all the neighboring vertices in the graph.
    fn neighbors<'a>(&'a self, v: usize) -> Box<Iterator<Item = usize> + 'a>;

    /// Returns an iterator over all the vertices in the graph.
    fn vertices<'a>(&'a self) -> Box<Iterator<Item = usize> + 'a> {
        let n = self.num_vertices();
        if n == 0 {
            Box::new(std::iter::empty())
        } else {
            Box::new(0..n)
        }
    }

    /// Constructs a random graph with ```n``` vertices where each undirected
    /// edge has probability ```p``` of occuring in the graph.
    fn random(n: usize, p: f32) -> Self {
        let mut g = Self::with_capacity(n);

        for u in 0..n {
            for v in u + 1..n {
                if random::<f32>() < p {
                    g.add_edge(u, v);
                }
            }
        }

        g
    }

    /// Constructs a complete graph of size ```n```.
    /// Every combination of vertices is connected by an edge.
    fn complete(n: usize) -> Self {
        let mut g = Self::with_capacity(n);
        for u in 0..n {
            for v in (u + 1)..n {
                g.add_edge(u, v);
            }
        }
        g
    }

    /// Returns the maximum degree of any node in the graph.
    /// That is the maximal number of neighbors any vertex has.
    fn max_degree(&self) -> usize {
        let mut max = 0;
        for u in self.vertices() {
            max = max.max(self.neighbors(u).count());
        }

        max
    }
}

/// Load a graph from file in DIMACS ```.col``` format. ([Specification](http://lcs.ios.ac.cn/~caisw/Resource/about_DIMACS_graph_format.txt))
///
/// A line may start with ```c```, ```p``` or ```e```.
/// ```c``` indicates a comment line and is ignored.
/// ```p``` lists the parameters of the graph as such:
/// ```p edge n m```
/// where 'edge' is simply written out, indicating that the following lines will each encode an edge,
/// n is the number of vertices in the graph and m is the number of edges.
/// This line must appear before any ```e``` lines.
/// Lastly, ```e``` lines appear as:
/// ```e u v```
/// where u and v are vertex ids in [1,n] (n inclusive).
pub fn load_graph(name: impl AsRef<Path>) -> std::io::Result<Graph> {
    let file = File::open(name)?;
    let mut graph = None;

    for line in BufReader::new(file).lines() {
        // Should always be valid UTF-8
        let line = line.unwrap();

        if line.starts_with('c') {
            continue;
        }

        if line.starts_with('p') {
            let splits = line.split(' ').collect::<Vec<_>>();
            //assert_eq!(splits[1], "edge".to_string());

            let n = splits[2].parse::<usize>().unwrap();
            let _m = splits[3].parse::<usize>().unwrap();

            graph = Some(Graph::with_capacity(n));
            continue;
        }

        if line.starts_with('e') {
            let splits = line.split(' ').collect::<Vec<_>>();
            let u = splits[1].parse::<usize>().unwrap();
            let v = splits[2].parse::<usize>().unwrap();

            // Shift everything down as vertices are in [1,n]
            if let Some(ref mut graph) = graph {
                graph.add_edge(u - 1, v - 1);
            } else {
                panic!("'e' line before 'p' line");
            }

            continue;
        }

        panic!("Unexpected line '{}'", line);
    }

    Ok(graph.unwrap())
}

#[cfg(test)]
mod tests {
    use crate::graph::*;
    use std::collections::HashSet;

    #[test]
    fn test_edgelist() {
        let tester = GraphTester::<EdgeList>::new();
        tester.run();
    }

    #[test]
    fn test_adjmatrix() {
        let tester = GraphTester::<AdjMatrix>::new();
        tester.run();
    }

    #[test]
    fn test_growableadjmatrix() {
        let tester = GraphTester::<GrowableAdjMatrix>::new();
        tester.run();
    }

    #[test]
    fn test_adjlist() {
        let tester = GraphTester::<AdjList>::new();
        tester.run();
    }

    #[test]
    fn test_hybrid() {
        let tester = GraphTester::<Hybrid>::new();
        tester.run();
    }

    #[test]
    fn test_el_adj() {
        let tester = GraphInteropTester::<EdgeList, AdjMatrix>::new();
        tester.run();
    }

    #[test]
    fn test_el_gadj() {
        let tester = GraphInteropTester::<EdgeList, GrowableAdjMatrix>::new();
        tester.run();
    }

    #[test]
    fn test_adj_gadj() {
        let tester = GraphInteropTester::<AdjMatrix, GrowableAdjMatrix>::new();
        tester.run();
    }

    #[test]
    fn test_hybrid_el() {
        let tester = GraphInteropTester::<Hybrid, EdgeList>::new();
        tester.run();
    }

    #[test]
    fn test_hybrid_al() {
        let tester = GraphInteropTester::<Hybrid, AdjList>::new();
        tester.run();
    }

    #[test]
    fn test_hybrid_am() {
        let tester = GraphInteropTester::<Hybrid, AdjMatrix>::new();
        tester.run();
    }

    // Tester

    struct GraphTester<G: StaticGraph> {
        _dummy: G,
    }

    impl<G: StaticGraph> GraphTester<G> {
        fn new() -> Self {
            Self {
                _dummy: G::with_capacity(0),
            }
        }

        fn run(&self) {
            self.creation_empty();
            self.creation_empty_selfedge();
            self.creation_empty_with_capacity();
            self.insertion_1();
            self.insertion_out_of_bounds();
            self.insertion_1_duplicate();
            self.insertion_selfedge();
            self.insertion_1_test_nonexistent();
            self.insertion_large();
            self.insertion_large_with_duplicates();
            self.edges();
            self.edges_empty();
            self.neighbors();
            self.neighbors_empty();
            self.max_degree();
            self.random();
            self.random_full();
            self.random_empty();
            self.complete();
        }

        fn creation_empty(&self) {
            let g = G::with_capacity(0);
            assert!(!g.has_edge(0, 1));
        }

        fn creation_empty_selfedge(&self) {
            let g = G::with_capacity(0);
            assert!(!g.has_edge(0, 0));
        }

        fn creation_empty_with_capacity(&self) {
            let g = G::with_capacity(200);
            assert!(!g.has_edge(0, 0));
        }

        fn insertion_1(&self) {
            let mut g = G::with_capacity(2);
            g.add_edge(0, 1);
            assert!(!g.has_edge(0, 0));
            assert!(g.has_edge(0, 1));
            assert!(g.has_edge(1, 0));
        }

        fn insertion_out_of_bounds(&self) {
            // Shouldn't panic
            // Some implementations will grow the graph, others will not
            let mut g = G::with_capacity(0);
            g.add_edge(0, 1);
        }

        fn insertion_1_duplicate(&self) {
            let mut g = G::with_capacity(2);
            g.add_edge(0, 1);
            g.add_edge(0, 1);
            g.add_edge(1, 0);
            assert!(!g.has_edge(0, 0));
            assert!(g.has_edge(0, 1));
            assert!(g.has_edge(1, 0));
        }

        fn insertion_selfedge(&self) {
            let mut g = G::with_capacity(2);
            g.add_edge(0, 0);
            g.add_edge(1, 1);
            assert!(!g.has_edge(0, 0));
            assert!(!g.has_edge(0, 1));
            assert!(!g.has_edge(1, 0));
            assert!(!g.has_edge(1, 1));
        }

        fn insertion_1_test_nonexistent(&self) {
            let mut g = G::with_capacity(2);
            g.add_edge(0, 1);
            assert!(!g.has_edge(0, 0));
            assert!(!g.has_edge(1, 1));
            assert!(!g.has_edge(1, 2));
        }

        fn insertion_large(&self) {
            let n = 100;
            let mut g = G::with_capacity(n);

            for u in 0..n {
                for v in (u + 1)..n {
                    g.add_edge(u, v);
                }
            }

            assert!(g.has_edge(40, 11));
            assert_eq!(g.edges().count(), n * (n - 1) / 2);
        }

        fn insertion_large_with_duplicates(&self) {
            let n = 100;
            let mut g = G::with_capacity(n);

            for u in 0..n {
                for v in 0..n {
                    g.add_edge(u, v);
                }
            }

            assert!(g.has_edge(40, 11));
            assert_eq!(g.edges().count(), n * (n - 1) / 2);
        }

        fn edges(&self) {
            let mut g = G::with_capacity(3);
            g.add_edge(0, 1);
            g.add_edge(0, 2);

            assert!(g.edges().any(|x| x == (0, 1)));
            assert!(g.edges().any(|x| x == (0, 2)));
            assert!(!g.edges().any(|x| x == (1, 2)));
            assert_eq!(g.edges().count(), 2);
        }

        fn edges_empty(&self) {
            let g = G::with_capacity(100);

            assert_eq!(g.edges().count(), 0);
        }

        fn neighbors(&self) {
            let mut g = G::with_capacity(5);

            g.add_edge(1, 2);
            g.add_edge(1, 3);

            assert!(g.neighbors(1).any(|x| x == 2));
            assert!(g.neighbors(1).any(|x| x == 3));
            assert!(g.neighbors(2).any(|x| x == 1));
            assert!(g.neighbors(3).any(|x| x == 1));
            assert_eq!(g.neighbors(1).count(), 2);
            assert_eq!(g.neighbors(2).count(), 1);
            assert_eq!(g.neighbors(3).count(), 1);

            assert!(!g.neighbors(1).any(|x| x == 1));
            assert!(!g.neighbors(2).any(|x| x == 2));
            assert!(!g.neighbors(2).any(|x| x == 3));
        }

        fn neighbors_empty(&self) {
            let mut g = G::with_capacity(5);

            g.add_edge(1, 2);
            g.add_edge(1, 3);

            assert!(!g.neighbors(5).any(|x| x == 1));
            assert_eq!(g.neighbors(5).count(), 0);
        }

        fn max_degree(&self) {
            let mut g = G::with_capacity(5);

            g.add_edge(1, 2);
            g.add_edge(1, 3);
            g.add_edge(1, 1);
            g.add_edge(1, 2);

            assert_eq!(g.max_degree(), 2);

            g.add_edge(2, 3);
            g.add_edge(2, 4);

            assert_eq!(g.max_degree(), 3);
        }

        fn random(&self) {
            // Expected (100*99)/2 * 0.5 = 2475 edges
            let g = G::random(100, 0.5);

            let num_edges = g.edges().count();

            // More than likely correct
            assert!(num_edges > 2000);
            assert!(num_edges < 3000);
        }

        fn random_full(&self) {
            let g = G::random(100, 1.0);

            let num_edges = g.edges().count();

            assert_eq!(num_edges, (100 * 99) / 2);
        }

        fn random_empty(&self) {
            let g = G::random(100, 0.0);

            let num_edges = g.edges().count();

            assert_eq!(num_edges, 0);
        }

        fn complete(&self) {
            let n = 50;
            let g = G::complete(n);

            assert_eq!(g.edges().count(), n * (n - 1) / 2);
        }
    }

    // Graph Interoperability tester

    struct GraphInteropTester<G1: StaticGraph, G2: StaticGraph> {
        _dummy1: G1,
        _dummy2: G2,
    }

    impl<G1: StaticGraph, G2: StaticGraph> GraphInteropTester<G1, G2> {
        fn new() -> Self {
            Self {
                _dummy1: G1::with_capacity(0),
                _dummy2: G2::with_capacity(0),
            }
        }

        fn run(&self) {
            self.from_graph();
            self.from_graph_rev();
        }

        fn from_graph(&self) {
            let g1 = G1::random(100, 0.5);
            let g2 = G2::from_graph(&g1);

            let edges1 = g1.edges().collect::<HashSet<(usize, usize)>>();
            let edges2 = g2.edges().collect::<HashSet<(usize, usize)>>();

            assert_eq!(edges1, edges2);
        }

        fn from_graph_rev(&self) {
            let g1 = G2::random(100, 0.5);
            let g2 = G1::from_graph(&g1);

            let edges1 = g1.edges().collect::<HashSet<(usize, usize)>>();
            let edges2 = g2.edges().collect::<HashSet<(usize, usize)>>();

            assert_eq!(edges1, edges2);
        }
    }
}
