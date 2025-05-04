// Implements closeness and degree centrality calculations for ranking node importance.

use std::collections::{HashMap, VecDeque};
use crate::graph::Graph;

/// Computes closeness centrality for each node.
/// Returns: HashMap of node name to centrality score.
pub fn closeness_centrality(graph: &Graph) -> HashMap<String, f64> {
    let mut centrality = HashMap::new();

    for (name, &idx) in graph.nodes() {
        let mut visited = vec![false; graph.node_count()];
        let mut queue = VecDeque::new();
        let mut total_distance = 0;
        let mut visited_count = 0;

        visited[idx] = true;
        queue.push_back((idx, 0));

        while let Some((node, dist)) = queue.pop_front() {
            total_distance += dist;
            visited_count += 1;
            for &neighbor in graph.neighbors(node) {
                if !visited[neighbor] {
                    visited[neighbor] = true;
                    queue.push_back((neighbor, dist + 1));
                }
            }
        }

        centrality.insert(name.clone(), if visited_count > 1 {
            (visited_count - 1) as f64 / total_distance as f64
        } else { 0.0 });
    }

    centrality
}

/// Computes degree centrality.
/// Returns: HashMap of node name to degree count.
pub fn degree_centrality(graph: &Graph) -> HashMap<String, usize> {
    graph.nodes()
        .map(|(name, &idx)| (name.clone(), graph.neighbors(idx).len()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_closeness_on_small_graph() {
        let mut g = Graph::new();
        g.add_edge("A", "B");
        g.add_edge("B", "C");
        let result = closeness_centrality(&g);
        assert!(result.contains_key("A"));
    }
}
