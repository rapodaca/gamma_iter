pub use super::error::Error;

/// An unweighted, undirected graph.
pub trait Graph {
    /// Returns true if there are no nodes, or false otherwise.
    fn is_empty(&self) -> bool;

    /// Returns the number of nodes in this graph.
    fn order(&self) -> usize;

    /// Returns the number of edges in this graph.
    fn size(&self) -> usize;

    /// Returns the nodes of this graph.
    // fn nodes(&self) -> &[usize];
    fn nodes(&self) -> Box<dyn Iterator<Item=usize> + '_>;

    /// Iterates the neighbors of the node.
    /// Returns an error if id not found.
    // fn neighbors(&self, id: usize) -> Result<&[usize], Error>;
    fn neighbors(&self, id: usize) -> Result<Box<dyn Iterator<Item=usize> + '_>, Error>;
    
    /// Returns true if node is a member, or false otherwise.
    fn has_node(&self, id: usize) -> bool;

    /// Returns the count of neighbors at node. REturns an error if id not
    /// found.
    fn degree(&self, id: usize) -> Result<usize, Error>;

    /// Returns the edges of this graph.
    // fn edges(&self) -> &[(usize, usize)];
    fn edges(&self) -> Box<dyn Iterator<Item=(usize, usize)> + '_>;

    /// Returns true if the edge (sid, tid) exists, or false otherwise.
    /// Returns MissingNode if either sid or tid are not members.
    fn has_edge(&self, sid: usize, tid: usize) -> Result<bool, Error>;
}