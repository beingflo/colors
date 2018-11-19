use std::collections::HashSet;
use std::iter::Iterator;
use itertools::Itertools;

use rand::random;

pub struct AdjMatrix {
    adj: Vec<bool>,
    vertices: HashSet<usize>,
    n: usize,
}

impl AdjMatrix {
    pub fn with_capacity(n: usize) -> Self {
        Self { adj: vec![false; n*n], vertices: HashSet::new(), n: n }
    }

    pub fn random(n: usize, p: f32) -> Self {
        let mut g = AdjMatrix::with_capacity(n);

        for u in 0..n {
            for v in u+1..n {
                if random::<f32>() < p {
                    g.add_edge(u,v);
                }
            }
        }

        g
    }

    pub fn add_edge(&mut self, u: usize, v: usize)  {
        // Self edges explicitly disallowed
        if u == v {
            return;
        }

        let idx1 = self.get_idx(u, v);
        let idx2 = self.get_idx(v, u);
        self.adj[idx1] = true;
        self.adj[idx2] = true;

        self.vertices.insert(u);
        self.vertices.insert(v);
    }

    pub fn has_edge(&self, u: usize, v: usize) -> bool {
        let idx = self.get_idx(u, v);
        self.adj[idx]
    }

    pub fn edges(&self) -> impl Iterator<Item=(usize,usize)> + '_ {
        let n = self.n;
        self.adj.iter().enumerate().filter(|(_, &b)| b).map(move |(i, _)| {
            let u = i / n;
            let v = i % n;

            if u > v { (v,u) } else { (u,v) }
        }).unique()
    }

    pub fn vertices(&self) -> impl Iterator<Item=usize> + '_ {
        self.vertices.iter().cloned()
    }

    pub fn neighbors(&self, v: usize) -> impl Iterator<Item=usize> + '_ {
        self.adj[(v * self.n)..(((v+1) * self.n) - 1)].iter().enumerate().filter(|(_, &b)| b).map(|(i, _)| i)
    }

    pub fn max_degree(&self) -> usize {
        let mut max = 0;
        for u in self.vertices() {
            max = max.max(self.neighbors(u).count());
        }

        max
    }

    /// Get index into adjacency array from edge.
    fn get_idx(&self, u: usize, v: usize) -> usize {
        v * self.n + u
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creation() {
        let g = AdjMatrix::with_capacity(100);

        assert!(!g.has_edge(0,1));
    }

    #[test]
    fn insertion() {
        let mut g = AdjMatrix::with_capacity(100);
        g.add_edge(1,2);

        assert!(g.has_edge(1,2));
        assert!(!g.has_edge(1,3));
    }

    #[test]
    fn insertion_reversed() {
        let mut g = AdjMatrix::with_capacity(100);
        g.add_edge(1,2);

        assert!(g.has_edge(2,1));
        assert!(!g.has_edge(1,3));
    }

    #[test]
    fn insertion_large() {
        let mut g = AdjMatrix::with_capacity(100);

        for u in 0..100 {
            for v in u..100 {
                g.add_edge(u,v);
            }
        }

        assert!(g.has_edge(40, 11));
    }

    #[test]
    fn edges() {
        let mut g = AdjMatrix::with_capacity(100);
        g.add_edge(1,2);
        g.add_edge(1,3);

        assert!(g.edges().any(|x| x == (1,3)));
        assert!(g.edges().any(|x| x == (1,2)));
        assert!(!g.edges().any(|x| x == (2,3)));
    }

    #[test]
    fn neighbors() {
        let mut g = AdjMatrix::with_capacity(100);

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
        let mut g = AdjMatrix::with_capacity(100);

        g.add_edge(1,2);
        g.add_edge(1,3);

        assert!(!g.neighbors(5).any(|x| x == 1));
    }

    #[test]
    fn max_degree() {
        let mut g = AdjMatrix::with_capacity(100);

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
        let g = AdjMatrix::random(100, 0.5);

        let num_edges = g.edges().count();

        // More than likely correct
        assert!(num_edges > 2000);
        assert!(num_edges < 3000);
    }

    #[test]
    fn random_full() {
        let g = AdjMatrix::random(100, 1.0);

        let num_edges = g.edges().count();

        assert_eq!(num_edges, (100*99)/2);
    }

    #[test]
    fn random_empty() {
        let g = AdjMatrix::random(100, 0.0);

        let num_edges = g.edges().count();

        assert_eq!(num_edges, 0);
    }
}
