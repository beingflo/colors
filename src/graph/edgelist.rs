use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::Iterator;

use graph::StaticGraph;

/// Graph datastructure implemented as a set of edges.
/// The graph is undirected and unweighted - only the connectivity pattern of
/// the vertices is captured. Multiple edges and self edges are also disallowed.
///
/// Vertices and edges may not be removed.
#[derive(Debug, Clone)]
pub struct EdgeList {
    edges: HashSet<(usize, usize)>,
    vertices: HashSet<usize>,

    neighbors: HashMap<usize, HashSet<usize>>,
}

impl EdgeList {
    /// Constructs a new empty graph
    pub fn new() -> Self {
        Self { edges: HashSet::new(), vertices: HashSet::new(), neighbors: HashMap::new() }
    }
}

impl StaticGraph for EdgeList {
    /// Constructs a new graph with capacity for ```n``` vertices.
    fn with_capacity(n: usize) -> Self {
        // Only implemented for compatibility, not very useful here
        let mut g = Self::new();
        g.vertices.reserve(n);
        g
    }

    /// Construct an instance of this type from another ```StaticGraph``` implementor
    fn from_graph<G: StaticGraph>(graph: &G) -> Self {
        let mut g = Self::new();
        for (u,v) in graph.edges() {
            g.add_edge(u,v);
        }
        g
    }

    /// Queries whether an edge exists in the graph.
    fn has_edge(&self, mut u: usize, mut v: usize) -> bool {
        if u > v {
            let t = u;
            u = v;
            v = t;
        }

        self.edges.contains(&(u,v))
    }

    /// Adds an edge to the graph.
    /// ```add_edge(u,v)``` has the same effect as ```add_edge(v,u)```
    /// as the graph captures undirected edges.
    /// Adding an edge that already exists has no effect.
    fn add_edge(&mut self, mut u: usize, mut v: usize)  {
        if u == v {
            return;
        }

        if u > v {
            let t = u;
            u = v;
            v = t;
        }

        self.edges.insert((u,v));

        self.vertices.insert(u);
        self.vertices.insert(v);

        if !self.neighbors.contains_key(&u) {
            self.neighbors.insert(u, HashSet::new());
        }

        self.neighbors.get_mut(&u).unwrap().insert(v);

        if !self.neighbors.contains_key(&v) {
            self.neighbors.insert(v, HashSet::new());
        }

        self.neighbors.get_mut(&v).unwrap().insert(u);
    }

    /// Returns an iterator over all the edges in the graph.
    fn edges<'a>(&'a self) -> Box<Iterator<Item=(usize,usize)> + 'a> {
        Box::new(self.edges.iter().cloned())
    }

    /// Returns an iterator over all the vertices in the graph.
    fn vertices<'a>(&'a self) -> Box<Iterator<Item=usize> + 'a> {
        Box::new(self.vertices.iter().cloned())
    }

    /// Returns an iterator over all the neighboring vertices in the graph.
    fn neighbors<'a>(&'a self, v: usize) -> Box<Iterator<Item=usize> + 'a> {
        if let Some(set) = self.neighbors.get(&v) {
            Box::new(set.iter().cloned())
        } else {
            Box::new(std::iter::empty())
        }
    }
}
