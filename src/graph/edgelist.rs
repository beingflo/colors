use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::Iterator;

use rand::random;
use graph::Graph;

/// Graph datastructure implemented as a set of edges.
/// The graph is undirected and unweighted - only the connectivity pattern of
/// the vertices is captured. Multiple edges and self edges are also disallowed.
///
/// Vertices and edges may not be removed.
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

    /// Constructs a random graph with ```n``` vertices where each undirected
    /// edge has probability ```p``` of occuring in the graph.
    pub fn random(n: usize, p: f32) -> Self {
        let mut g = EdgeList::new();

        for u in 0..n {
            for v in u+1..n {
                if random::<f32>() < p {
                    g.add_edge(u,v);
                }
            }
        }

        g
    }

    /// Adds an edge to the graph.
    /// ```add_edge(u,v)``` has the same effect as ```add_edge(v,u)```
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

        if !self.neighbors.contains_key(&u) {
            self.neighbors.insert(u, HashSet::new());
        }

        self.neighbors.get_mut(&u).unwrap().insert(v);

        if !self.neighbors.contains_key(&v) {
            self.neighbors.insert(v, HashSet::new());
        }

        self.neighbors.get_mut(&v).unwrap().insert(u);
    }

    /// Returns the maximum degree of any node in the graph.
    /// That is the maximal number of neighbors any vertex has.
    pub fn max_degree(&self) -> usize {
        let mut max = 0;
        for u in self.vertices() {
            max = max.max(self.neighbors(u).count());
        }

        max
    }
}

impl Graph for EdgeList {
    /// Queries whether an edge exists in the graph.
    fn has_edge(&self, mut u: usize, mut v: usize) -> bool {
        if u > v {
            let t = u;
            u = v;
            v = t;
        }

        self.edges.contains(&(u,v))
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



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation() {
        let g = EdgeList::new();

        assert!(!g.has_edge(0,1));
    }

    #[test]
    fn insertion() {
        let mut g = EdgeList::new();
        g.add_edge(1,2);

        assert!(g.has_edge(1,2));
        assert!(!g.has_edge(1,3));
    }

    #[test]
    fn insertion_reversed() {
        let mut g = EdgeList::new();
        g.add_edge(1,2);

        assert!(g.has_edge(2,1));
        assert!(!g.has_edge(1,3));
    }

    #[test]
    fn insertion_large() {
        let mut g = EdgeList::new();

        for u in 0..100 {
            for v in u..100 {
                g.add_edge(u,v);
            }
        }

        assert!(g.has_edge(40, 11));
    }

    #[test]
    fn edges() {
        let mut g = EdgeList::new();
        g.add_edge(1,2);
        g.add_edge(1,3);

        assert!(g.edges().any(|x| x == (1,3)));
        assert!(g.edges().any(|x| x == (1,2)));
        assert!(!g.edges().any(|x| x == (2,3)));
    }

    #[test]
    fn neighbors() {
        let mut g = EdgeList::new();

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
        let mut g = EdgeList::new();

        g.add_edge(1,2);
        g.add_edge(1,3);

        assert!(!g.neighbors(5).any(|x| x == 1));
    }

    #[test]
    fn max_degree() {
        let mut g = EdgeList::new();

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
        let g = EdgeList::random(100, 0.5);

        let num_edges = g.edges().count();

        // More than likely correct
        assert!(num_edges > 2000);
        assert!(num_edges < 3000);
    }

    #[test]
    fn random_full() {
        let g = EdgeList::random(100, 1.0);

        let num_edges = g.edges().count();

        assert_eq!(num_edges, (100*99)/2);
    }

    #[test]
    fn random_empty() {
        let g = EdgeList::random(100, 0.0);

        let num_edges = g.edges().count();

        assert_eq!(num_edges, 0);
    }
}
