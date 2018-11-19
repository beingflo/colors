mod edgelist;
mod adjmatrix;

pub use self::edgelist::EdgeList;
pub use self::adjmatrix::AdjMatrix;

pub trait Graph {
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
}
