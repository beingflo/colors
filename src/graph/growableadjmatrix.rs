use std::collections::HashSet;
use std::iter::Iterator;

use graph::Graph;

/// Graph datastructure implemented as a growable adjacency matrix.
/// The graph is undirected and unweighted - only the connectivity pattern of
/// the vertices is captured. Multiple edges and self edges are also disallowed.
///
/// Vertices and edges may not be removed.
///
/// # Warning
/// Only use this if querying edges is the main operation and dynamically
/// growing the graph is necessary. Otherwise consider ```AdjMatrix``` as
/// ```neighbors``` calls are significantly faster.
/// If growing is only a necessity initially, consider constructing an ```EdgeList```
/// and converting to ```AdjMatrix``` via ```AdjMatrix::from_graph```.
#[derive(Debug, Clone)]
pub struct GrowableAdjMatrix {
    adj: Vec<bool>,
    vertices: HashSet<usize>,
    cap: usize,
}

impl GrowableAdjMatrix {
    /// Constructs a new empty graph
    pub fn new() -> Self {
        // Initialize to capacity for 256 vertices
        GrowableAdjMatrix::with_capacity(GrowableAdjMatrix::get_size(256))
    }

    /// Get index into adjacency array from edge.
    fn get_idx(mut u: usize, mut v: usize) -> usize {
        if u < v {
            let t = u;
            u = v;
            v = t;
        }

        // u is now bigger and cannot be 0 as self edges are not allowed
        assert!(u >= 1);

        u * (u-1) / 2 + v
    }

    /// Return the size neede to accommodate ```n``` vertices
    fn get_size(n: usize) -> usize {
        n * (n+1) / 2
    }

    /// Resize the capacity of the graph to accommodate ```n``` vertices
    fn resize(&mut self, n: usize) {
        let new_size = GrowableAdjMatrix::get_size(n);

        if n < self.cap {
            return;
        }

        self.adj.resize(new_size, false);
        self.cap = n;
    }

    /// Get capacity of graph
    pub fn capacity(&self) -> usize {
        self.cap
    }
}

impl Graph for GrowableAdjMatrix {
    /// Constructs a new graph with capacity for ```n``` vertices.
    fn with_capacity(n: usize) -> Self {
        let size = GrowableAdjMatrix::get_size(n);
        Self { adj: vec![false; size], vertices: HashSet::new(), cap: n }
    }

    /// Construct an instance of this type from another ```Graph``` implementor
    fn from_graph<G: Graph>(graph: &G) -> Self {
        let mut g = Self::with_capacity(graph.vertices().count());
        for (u,v) in graph.edges() {
            g.add_edge(u,v);
        }
        g
    }

    /// Queries whether an edge exists in the graph.
    fn has_edge(&self, u: usize, v: usize) -> bool {
        let idx = GrowableAdjMatrix::get_idx(u, v);

        if idx > self.adj.len() {
            return false;
        }

        self.adj[idx]
    }

    /// Adds an edge to the graph.
    /// ```add_edge(u,v)``` has the same effect as ```add_edge(v,u)```
    /// as the graph captures undirected edges.
    /// Adding an edge that already exists has no effect.
    fn add_edge(&mut self, u: usize, v: usize)  {
        // Self edges explicitly disallowed
        if u == v {
            return;
        }

        // Double capacity can vertices could fit
        while u > self.cap || v > self.cap {
            let size = 2*self.cap;
            self.resize(size);
        }

        let idx1 = GrowableAdjMatrix::get_idx(u, v);
        let idx2 = GrowableAdjMatrix::get_idx(v, u);
        self.adj[idx1] = true;
        self.adj[idx2] = true;

        self.vertices.insert(u);
        self.vertices.insert(v);
    }

    /// Returns an iterator over all the edges in the graph.
    fn edges<'a>(&'a self) -> Box<Iterator<Item=(usize,usize)> + 'a> {
        Box::new(self.adj.iter().enumerate().filter(|(_, &b)| b).map(move |(i, _)| {
            let u = ((1.0 + (1.0 + 8.0 * i as f32).sqrt()) / 2.0).floor() as usize;
            let v = i - (u * (u-1) / 2);

            (v,u)
        }))
    }

    /// Returns an iterator over all the vertices in the graph.
    fn vertices<'a>(&'a self) -> Box<Iterator<Item=usize> + 'a> {
        Box::new(self.vertices.iter().cloned())
    }

    /// Returns an iterator over all the neighboring vertices in the graph.
    fn neighbors<'a>(&'a self, v: usize) -> Box<Iterator<Item=usize> + 'a> {
        Box::new(self.edges().filter(move |(a,b)| *a == v || *b == v).map(move |(a,b)| {
            if a == v {
                b
            } else {
                a
            }
        }))
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use graph::*;

    #[test]
    fn creation() {
        let g = GrowableAdjMatrix::with_capacity(100);

        assert!(!g.has_edge(0,1));
    }

    #[test]
    fn insertion() {
        let mut g = GrowableAdjMatrix::with_capacity(100);
        g.add_edge(1,2);

        assert!(g.has_edge(1,2));
        assert!(!g.has_edge(1,3));
    }

    #[test]
    fn insertion_reversed() {
        let mut g = GrowableAdjMatrix::with_capacity(100);
        g.add_edge(1,2);

        assert!(g.has_edge(2,1));
        assert!(!g.has_edge(1,3));
    }

    #[test]
    fn insertion_large() {
        let mut g = GrowableAdjMatrix::with_capacity(100);

        for u in 0..100 {
            for v in u..100 {
                g.add_edge(u,v);
            }
        }

        assert!(g.has_edge(40, 11));
    }

    #[test]
    fn edges() {
        let mut g = GrowableAdjMatrix::with_capacity(100);
        g.add_edge(1,2);
        g.add_edge(1,3);

        assert!(g.edges().any(|x| x == (1,3)));
        assert!(g.edges().any(|x| x == (1,2)));
        assert!(!g.edges().any(|x| x == (2,3)));
    }

    #[test]
    fn neighbors() {
        let mut g = GrowableAdjMatrix::with_capacity(100);

        g.add_edge(1,2);
        g.add_edge(1,3);

        assert!(g.neighbors(1).any(|x| x == 2));
        assert!(g.neighbors(1).any(|x| x == 3));
        assert!(g.neighbors(2).any(|x| x == 1));
        assert!(g.neighbors(3).any(|x| x == 1));

        assert!(!g.neighbors(1).any(|x| x == 1));
        assert!(!g.neighbors(2).any(|x| x == 3));
    }

    #[test]
    fn neighbors_empty() {
        let mut g = GrowableAdjMatrix::with_capacity(100);

        g.add_edge(1,2);
        g.add_edge(1,3);

        assert!(!g.neighbors(5).any(|x| x == 1));
    }

    #[test]
    fn max_degree() {
        let mut g = GrowableAdjMatrix::with_capacity(100);

        g.add_edge(1,2);
        g.add_edge(1,3);
        g.add_edge(1,1);
        g.add_edge(1,2);

        assert_eq!(g.max_degree(), 2);

        g.add_edge(2,3);
        g.add_edge(2,4);

        assert_eq!(g.max_degree(), 3);
    }

    #[test]
    fn random() {
        // Expected (100*99)/2 * 0.5 = 2475 edges
        let g = GrowableAdjMatrix::random(100, 0.5);

        let num_edges = g.edges().count();

        // More than likely correct
        assert!(num_edges > 2000);
        assert!(num_edges < 3000);
    }

    #[test]
    fn random_full() {
        let g = GrowableAdjMatrix::random(100, 1.0);

        let num_edges = g.edges().count();

        assert_eq!(num_edges, (100*99)/2);
    }

    #[test]
    fn random_empty() {
        let g = GrowableAdjMatrix::random(100, 0.0);

        let num_edges = g.edges().count();

        assert_eq!(num_edges, 0);
    }

    #[test]
    fn from_graph() {
        let g1 = EdgeList::random(100, 0.5);
        let g2 = GrowableAdjMatrix::from_graph(&g1);

        let edges1 = g1.edges().collect::<HashSet<(usize,usize)>>();
        let edges2 = g2.edges().collect::<HashSet<(usize,usize)>>();

        assert_eq!(edges1, edges2);
    }

    #[test]
    fn complete() {
        let n = 50;
        let g = GrowableAdjMatrix::complete(n);

        assert_eq!(g.edges().count(), n * (n-1) / 2);
    }

    #[test]
    fn complete_large() {
        let n = 500;
        let g = GrowableAdjMatrix::complete(n);

        assert_eq!(g.edges().count(), n * (n-1) / 2);
    }
}
