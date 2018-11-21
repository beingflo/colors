use std::iter::Iterator;

use graph::StaticGraph;
use graph::EdgeList;
use graph::AdjList;

/// Graph datastructure implemented as a set of edges as well as an adjacency list.
/// The graph is undirected and unweighted - only the connectivity pattern of
/// the vertices is captured. Multiple edges and self edges are also disallowed.
///
/// Vertices and edges may not be removed.
#[derive(Debug, Clone)]
pub struct Hybrid {
    el: EdgeList,
    al: AdjList,
}

impl Hybrid {
    /// Constructs a new empty graph
    pub fn new() -> Self {
        Self { el: EdgeList::new(), al: AdjList::new() }
    }
}

impl StaticGraph for Hybrid {
    /// Constructs a new graph with capacity for ```n``` vertices.
    fn with_capacity(n: usize) -> Self {
        Self { el: EdgeList::with_capacity(n), al: AdjList::with_capacity(n) }
    }

    /// Construct an instance of this type from another ```StaticGraph``` implementor
    fn from_graph<G: StaticGraph>(graph: &G) -> Self {
        Self { el: EdgeList::from_graph(graph), al: AdjList::from_graph(graph) }
    }

    /// Queries whether an edge exists in the graph.
    fn has_edge(&self, u: usize, v: usize) -> bool {
        // Faster to look up in the edge list
        self.el.has_edge(u,v)
    }

    /// Adds an edge to the graph.
    /// ```add_edge(u,v)``` has the same effect as ```add_edge(v,u)```
    /// as the graph captures undirected edges.
    /// Adding an edge that already exists has no effect.
    fn add_edge(&mut self, u: usize, v: usize)  {
        self.el.add_edge(u,v);
        self.al.add_edge(u,v);
    }

    /// Returns an iterator over all the edges in the graph.
    fn edges<'a>(&'a self) -> Box<Iterator<Item=(usize,usize)> + 'a> {
        // Faster in edge list
        self.el.edges()
    }

    /// Returns the number of vertices in the graph.
    fn num_vertices(&self) -> usize {
        assert_eq!(self.el.num_vertices(), self.al.num_vertices());

        self.el.num_vertices()
    }

    /// Returns an iterator over all the neighboring vertices in the graph.
    fn neighbors<'a>(&'a self, v: usize) -> Box<Iterator<Item=usize> + 'a> {
        // Faster in adjacency list
        self.al.neighbors(v)
    }
}
