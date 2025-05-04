// Defines the Graph structure and methods for adding edges and computing neighbors, nodes, and edges.

use std::collections::HashMap;

/// Struct: Graph
/// Represents an undirected graph with node indices and adjacency lists.
pub struct Graph {
    nodes: HashMap<String, usize>,
    adjacency: Vec<Vec<usize>>,
}

impl Graph {
    /// Creates a new empty graph.
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            adjacency: Vec::new(),
        }
    }

    /// Adds an undirected edge between two nodes.
    pub fn add_edge(&mut self, node1: &str, node2: &str) {
        let idx1 = self.get_or_insert_node(node1);
        let idx2 = self.get_or_insert_node(node2);
        self.adjacency[idx1].push(idx2);
        self.adjacency[idx2].push(idx1);
    }

    /// Gets or inserts a node index.
    fn get_or_insert_node(&mut self, node: &str) -> usize {
        if let Some(&idx) = self.nodes.get(node) {
            idx
        } else {
            let idx = self.adjacency.len();
            self.nodes.insert(node.to_string(), idx);
            self.adjacency.push(Vec::new());
            idx
        }
    }

    /// Returns the number of nodes.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// Returns the number of edges.
    pub fn edge_count(&self) -> usize {
        self.adjacency.iter().map(|e| e.len()).sum::<usize>() / 2
    }

    /// Returns a slice of neighbor indices.
    pub fn neighbors(&self, node: usize) -> &[usize] {
        &self.adjacency[node]
    }

    /// Returns an iterator over node names and indices.
    pub fn nodes(&self) -> impl Iterator<Item = (&String, &usize)> {
        self.nodes.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_edge_and_count() {
        let mut g = Graph::new();
        g.add_edge("A", "B");
        g.add_edge("B", "C");
        assert_eq!(g.node_count(), 3);
        assert_eq!(g.edge_count(), 2);
    }
}
