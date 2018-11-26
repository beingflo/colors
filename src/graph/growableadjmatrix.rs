use std::iter::Iterator;

use graph::StaticGraph;

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
/// and converting to ```GrowableAdjMatrix``` via ```GrowableAdjMatrix::from_graph```.
#[derive(Debug, Clone)]
pub struct GrowableAdjMatrix {
    adj: Vec<bool>,
    n: usize,
    cap: usize,
}

impl GrowableAdjMatrix {
    /// Constructs a new empty graph
    pub fn new() -> Self {
        // Initialize to capacity for 256 vertices
        Self::with_capacity(256)
    }

    /// Get index into adjacency array from edge.
    fn get_idx(mut u: usize, mut v: usize) -> usize {
        if u < v {
            std::mem::swap(&mut u, &mut v);
        }

        // u is now bigger and cannot be 0 as self edges are not allowed
        assert!(u >= 1);

        u * (u - 1) / 2 + v
    }

    /// Return the size neede to accommodate ```n``` vertices
    fn get_size(n: usize) -> usize {
        n * (n + 1) / 2
    }

    /// Resize the capacity of the graph to accommodate ```n``` vertices
    fn resize(&mut self, n: usize) {
        let new_size = Self::get_size(n);

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

impl StaticGraph for GrowableAdjMatrix {
    /// Constructs a new graph with capacity for ```n``` vertices.
    fn with_capacity(n: usize) -> Self {
        let cap = n.max(1);
        let size = Self::get_size(cap);
        Self {
            adj: vec![false; size],
            n,
            cap,
        }
    }

    /// Construct an instance of this type from another ```StaticGraph``` implementor
    fn from_graph<G: StaticGraph>(graph: &G) -> Self {
        let mut g = Self::with_capacity(graph.vertices().count());
        for (u, v) in graph.edges() {
            g.add_edge(u, v);
        }
        g
    }

    /// Queries whether an edge exists in the graph.
    fn has_edge(&self, u: usize, v: usize) -> bool {
        if u == v || u >= self.cap || v >= self.cap {
            return false;
        }

        let idx = Self::get_idx(u, v);

        if idx >= self.adj.len() {
            return false;
        }

        self.adj[idx]
    }

    /// Adds an edge to the graph.
    /// ```add_edge(u,v)``` has the same effect as ```add_edge(v,u)```
    /// as the graph captures undirected edges.
    /// Adding an edge that already exists has no effect.
    fn add_edge(&mut self, u: usize, v: usize) {
        // Self edges explicitly disallowed
        if u == v {
            return;
        }

        // Double capacity can vertices could fit
        while u >= self.cap || v >= self.cap {
            let size = 2 * self.cap;
            self.resize(size);
        }

        self.n = self.n.max(u + 1);
        self.n = self.n.max(v + 1);

        let idx1 = Self::get_idx(u, v);
        let idx2 = Self::get_idx(v, u);
        self.adj[idx1] = true;
        self.adj[idx2] = true;
    }

    /// Returns an iterator over all the edges in the graph.
    fn edges<'a>(&'a self) -> Box<Iterator<Item = (usize, usize)> + 'a> {
        Box::new(
            self.adj
                .iter()
                .enumerate()
                .filter(|(_, &b)| b)
                .map(move |(i, _)| {
                    let u = ((1.0 + (1.0 + 8.0 * i as f32).sqrt()) / 2.0).floor() as usize;
                    let v = i - (u * (u - 1) / 2);

                    (v, u)
                }),
        )
    }

    /// Returns the number of vertices in the graph.
    fn num_vertices(&self) -> usize {
        self.n
    }

    /// Returns an iterator over all the neighboring vertices in the graph.
    fn neighbors<'a>(&'a self, v: usize) -> Box<Iterator<Item = usize> + 'a> {
        Box::new(
            self.edges().filter(move |(a, b)| *a == v || *b == v).map(
                move |(a, b)| {
                    if a == v {
                        b
                    } else {
                        a
                    }
                },
            ),
        )
    }
}
