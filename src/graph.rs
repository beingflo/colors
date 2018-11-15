use std::collections::HashSet;
use std::iter::Iterator;

/// Graph datastructure implemented as a set of edges.
/// The graph is undirected and unweighted - only the connectivity pattern of
/// the vertices is captured. Multiple edges and self edges are also disallowed.
///
/// Vertices and edges may not be removed.
pub struct Graph {
    edges: HashSet<(usize, usize)>,
    vertices: HashSet<usize>,
}

impl Graph {
    /// Constructs a new graph
    pub fn new() -> Self {
        Graph { edges: HashSet::new(), vertices: HashSet::new() }
    }

    /// Add edge to the graph
    /// add_edge(u,v) has the same effect as add_edge(v,u)
    /// as the graph captures undirected edges.
    /// Adding an edge that already exists has no effect.
    pub fn add_edge(&mut self, mut u: usize, mut v: usize)  {
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
    }

    /// Queries whether an edge exists in the graph.
    pub fn has_edge(&self, mut u: usize, mut v: usize) -> bool {
        if u > v {
            let t = u;
            u = v;
            v = t;
        }

        self.edges.contains(&(u,v))
    }

    /// Returns an itertator over all the edges in the graph
    pub fn edges(&self) -> impl Iterator<Item=&(usize,usize)> {
        self.edges.iter()
    }

    pub fn vertices(&self) -> impl Iterator<Item=&usize> {
        self.vertices.iter()
    }
}

#[cfg(test)]
mod tests {
    use graph::Graph;

    #[test]
    fn creation() {
        let g = Graph::new();

        assert!(!g.has_edge(0,1));
    }

    #[test]
    fn insertion() {
        let mut g = Graph::new();
        g.add_edge(1,2);

        assert!(g.has_edge(1,2));
        assert!(!g.has_edge(1,3));
    }

    #[test]
    fn insertion_reversed() {
        let mut g = Graph::new();
        g.add_edge(1,2);

        assert!(g.has_edge(2,1));
        assert!(!g.has_edge(1,3));
    }

    #[test]
    fn insertion_large() {
        let mut g = Graph::new();

        for u in 0..100 {
            for v in u..100 {
                g.add_edge(u,v);
            }
        }

        assert!(g.has_edge(40, 11));
    }

    #[test]
    fn edges() {
        let mut g = Graph::new();
        g.add_edge(1,2);
        g.add_edge(1,3);

        assert!(g.edges().any(|&x| x == (1,3)));
        assert!(g.edges().any(|&x| x == (1,2)));
        assert!(!g.edges().any(|&x| x == (2,3)));
    }
}
