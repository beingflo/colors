mod edgelist;
mod adjmatrix;

pub use self::edgelist::EdgeList;
pub use self::adjmatrix::AdjMatrix;

pub type Graph = EdgeList;
