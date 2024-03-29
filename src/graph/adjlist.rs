use itertools::Itertools;

use crate::graph::StaticGraph;

/// Graph datastructure implemented as an adjacency list.
/// The graph is undirected and unweighted - only the connectivity pattern of
/// the vertices is captured. Multiple edges and self edges are also disallowed.
///
/// Vertices and edges may not be removed.
#[derive(Debug, Clone)]
pub struct AdjList {
    adj: Vec<Vec<usize>>,
    n: usize,
}

impl AdjList {
    pub fn new() -> Self {
        Self { adj: vec![], n: 0 }
    }
}

impl StaticGraph for AdjList {
    /// Constructs a new graph with capacity for ```n``` vertices.
    fn with_capacity(n: usize) -> Self {
        let mut adj = vec![];
        for _ in 0..n {
            adj.push(vec![]);
        }

        Self { adj, n }
    }

    /// Construct an instance of this type from another ```StaticGraph``` implementor
    fn from_graph<G: StaticGraph>(graph: &G) -> Self {
        let mut g = Self::new();
        for (u, v) in graph.edges() {
            g.add_edge(u, v);
        }
        g
    }

    /// Queries whether an edge exists in the graph.
    fn has_edge(&self, u: usize, v: usize) -> bool {
        if u >= self.adj.len() {
            return false;
        }

        for &to in &self.adj[u] {
            if v == to {
                return true;
            }
        }

        false
    }

    /// Adds an edge to the graph.
    /// ```add_edge(u,v)``` has the same effect as ```add_edge(v,u)```
    /// as the graph captures undirected edges.
    /// Adding an edge that already exists has no effect.
    fn add_edge(&mut self, u: usize, v: usize) {
        if u == v {
            return;
        }

        self.n = self.n.max(u + 1);
        self.n = self.n.max(v + 1);

        while self.adj.len() <= u {
            self.adj.push(vec![]);
        }

        while self.adj.len() <= v {
            self.adj.push(vec![]);
        }

        if !self.has_edge(u, v) {
            self.adj[u].push(v);
            self.adj[v].push(u);
        }
    }

    /// Returns an iterator over all the edges in the graph.
    fn edges<'a>(&'a self) -> Box<Iterator<Item = (usize, usize)> + 'a> {
        Box::new(
            self.adj
                .iter()
                .enumerate()
                .flat_map(|(u, vec)| {
                    vec.iter()
                        .map(move |&v| if u > v { (v, u) } else { (u, v) })
                })
                .unique(),
        )
    }

    /// Returns the number of vertices in the graph.
    fn num_vertices(&self) -> usize {
        self.n
    }

    /// Returns an iterator over all the neighboring vertices in the graph.
    fn neighbors<'a>(&'a self, v: usize) -> Box<Iterator<Item = usize> + 'a> {
        if v >= self.adj.len() {
            Box::new(std::iter::empty())
        } else {
            Box::new(self.adj[v].iter().cloned())
        }
    }
}
