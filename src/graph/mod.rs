mod edgelist;
mod adjmatrix;

use rand::random;

pub use self::edgelist::EdgeList;
pub use self::adjmatrix::AdjMatrix;

pub trait Graph: Sized {
    /// Constructs a new graph with capacity for ```n``` vertices.
    fn with_capacity(n: usize) -> Self;

    /// Construct an instance of this type from another ```Graph``` implementor
    fn from_graph<G: Graph>(&G) -> Self;

    /// Queries whether an edge exists in the graph.
    fn has_edge(&self, usize, usize) -> bool;

    /// Adds an edge to the graph.
    /// ```add_edge(u,v)``` has the same effect as ```add_edge(v,u)```
    /// as the graph captures undirected edges.
    /// Adding an edge that already exists has no effect.
    fn add_edge(&mut self, usize, usize);

    /// Returns an iterator over all the edges in the graph.
    fn edges<'a>(&'a self) -> Box<Iterator<Item=(usize,usize)> + 'a>;

    /// Returns an iterator over all the vertices in the graph.
    fn vertices<'a>(&'a self) -> Box<Iterator<Item=usize> + 'a>;

    /// Returns an iterator over all the neighboring vertices in the graph.
    fn neighbors<'a>(&'a self, usize) -> Box<Iterator<Item=usize> + 'a>;

    /// Constructs a random graph with ```n``` vertices where each undirected
    /// edge has probability ```p``` of occuring in the graph.
    fn random(n: usize, p: f32) -> Self {
        let mut g = Self::with_capacity(n);

        for u in 0..n {
            for v in u+1..n {
                if random::<f32>() < p {
                    g.add_edge(u,v);
                }
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
