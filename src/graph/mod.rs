mod edgelist;
mod adjmatrix;

pub use self::edgelist::EdgeList;
pub use self::adjmatrix::AdjMatrix;

pub trait Graph {
    /// Constructs a new graph with capacity for ```n``` vertices.
    fn with_capacity(n: usize) -> Self;

    /// Construct an instance of this type from another ```Graph``` implementor
    fn from_graph<G: Graph>(&G) -> Self;

    /// Queries whether an edge exists in the graph.
    fn has_edge(&self, usize, usize) -> bool;

    /// Returns an itertator over all the edges in the graph.
    fn edges<'a>(&'a self) -> Box<Iterator<Item=(usize,usize)> + 'a>;

    /// Returns an iterator over all the vertices in the graph.
    fn vertices<'a>(&'a self) -> Box<Iterator<Item=usize> + 'a>;

    /// Returns an iterator over all the neighboring vertices in the graph.
    fn neighbors<'a>(&'a self, usize) -> Box<Iterator<Item=usize> + 'a>;

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
