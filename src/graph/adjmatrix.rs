use std::iter::Iterator;
use itertools::Itertools;

use graph::StaticGraph;

/// Graph datastructure implemented as an adjacency matrix.
/// The graph is undirected and unweighted - only the connectivity pattern of
/// the vertices is captured. Multiple edges and self edges are also disallowed.
///
/// Vertices and edges may not be removed.
#[derive(Debug, Clone)]
pub struct AdjMatrix {
    adj: Vec<bool>,
    n: usize,
}

impl AdjMatrix {
    /// Get index into adjacency array from edge.
    fn get_idx(&self, u: usize, v: usize) -> usize {
        v * self.n + u
    }
}

impl StaticGraph for AdjMatrix {
    /// Constructs a new graph with capacity for ```n``` vertices.
    fn with_capacity(n: usize) -> Self {
        Self { adj: vec![false; n*n], n: n }
    }

    /// Construct an instance of this type from another ```StaticGraph``` implementor
    fn from_graph<G: StaticGraph>(graph: &G) -> Self {
        let mut g = Self::with_capacity(graph.vertices().count());
        for (u,v) in graph.edges() {
            g.add_edge(u,v);
        }
        g
    }

    /// Queries whether an edge exists in the graph.
    fn has_edge(&self, u: usize, v: usize) -> bool {
        if u >= self.n || v >= self.n {
            return false;
        }

        let idx = self.get_idx(u, v);
        self.adj[idx]
    }

    /// Adds an edge to the graph.
    /// ```add_edge(u,v)``` has the same effect as ```add_edge(v,u)```
    /// as the graph captures undirected edges.
    /// Adding an edge that already exists has no effect.
    fn add_edge(&mut self, u: usize, v: usize)  {
        // Self edges explicitly disallowed
        // If no capacity, just return
        if u == v || u >= self.n || v >= self.n {
            return;
        }

        let idx1 = self.get_idx(u, v);
        let idx2 = self.get_idx(v, u);

        self.adj[idx1] = true;
        self.adj[idx2] = true;
    }

    /// Returns an iterator over all the edges in the graph.
    fn edges<'a>(&'a self) -> Box<Iterator<Item=(usize,usize)> + 'a> {
        let n = self.n;
        Box::new(self.adj.iter().enumerate().filter(|(_, &b)| b).map(move |(i, _)| {
            let u = i / n;
            let v = i % n;

            if u > v { (v,u) } else { (u,v) }
        }).unique())
    }

    /// Returns an iterator over all the vertices in the graph.
    fn vertices<'a>(&'a self) -> Box<Iterator<Item=usize> + 'a> {
        if self.n == 0 {
            Box::new(std::iter::empty())
        } else {
            Box::new(0..self.n+1)
        }
    }

    /// Returns an iterator over all the neighboring vertices in the graph.
    fn neighbors<'a>(&'a self, v: usize) -> Box<Iterator<Item=usize> + 'a> {
        if v < self.n {
            Box::new(self.adj[(v * self.n)..((v+1) * self.n)].iter().enumerate().filter(|(_, &b)| b).map(|(i, _)| i))
        } else {
            Box::new(std::iter::empty())
        }
    }
}
