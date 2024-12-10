use std::collections::{HashMap, HashSet, VecDeque};
use rayon::prelude::*;

/// Calculates the degree centrality for each node in the graph.
pub fn calculate_degree_centrality(
    adjacency_list: &HashMap<u32, Vec<u32>>,
) -> HashMap<u32, f64> {
    let mut centrality = HashMap::new();
    for (&node, neighbors) in adjacency_list {
        centrality.insert(node, neighbors.len() as f64);
    }
    centrality
}

/// Calculates the closeness centrality for a subset of nodes in parallel.
pub fn calculate_closeness_centrality_parallel(
    adjacency_list: &HashMap<u32, Vec<u32>>,
    sample_size: usize,
) -> HashMap<u32, f64> {
    let total_nodes = adjacency_list.len() as f64;

    adjacency_list
        .keys()
        .cloned()
        .take(sample_size)
        .collect::<Vec<u32>>()
        .par_iter()
        .map(|&start_node| {
            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();
            let mut distances = HashMap::new();

            queue.push_back((start_node, 0));
            visited.insert(start_node);

            while let Some((current_node, distance)) = queue.pop_front() {
                distances.insert(current_node, distance);
                if let Some(neighbors) = adjacency_list.get(&current_node) {
                    for &neighbor in neighbors {
                        if visited.insert(neighbor) {
                            queue.push_back((neighbor, distance + 1));
                        }
                    }
                }
            }

            let total_distance: usize = distances.values().sum();
            let closeness = if total_distance > 0 {
                (total_nodes - 1.0) / total_distance as f64
            } else {
                0.0
            };

            (start_node, closeness)
        })
        .collect()
}

/// Calculates the betweenness centrality for a subset of nodes.
pub fn calculate_betweenness_centrality_sampled(
    adjacency_list: &HashMap<u32, Vec<u32>>,
    sample_size: usize,
) -> HashMap<u32, f64> {
    let mut centrality = HashMap::new();

    for &node in adjacency_list.keys() {
        centrality.insert(node, 0.0);
    }

    let sampled_nodes: Vec<u32> = adjacency_list.keys().cloned().take(sample_size).collect();

    for &start_node in sampled_nodes.iter() {
        let mut stack = Vec::new();
        let mut predecessors = HashMap::new();
        let mut sigma = HashMap::new();
        let mut distances = HashMap::new();
        let mut delta = HashMap::new();

        for &node in adjacency_list.keys() {
            predecessors.insert(node, Vec::new());
            sigma.insert(node, 0.0);
            distances.insert(node, -1);
            delta.insert(node, 0.0);
        }

        sigma.insert(start_node, 1.0);
        distances.insert(start_node, 0);

        let mut queue = VecDeque::new();
        queue.push_back(start_node);

        while let Some(v) = queue.pop_front() {
            stack.push(v);
            if let Some(neighbors) = adjacency_list.get(&v) {
                for &neighbor in neighbors {
                    if distances[&neighbor] < 0 {
                        queue.push_back(neighbor);
                        distances.insert(neighbor, distances[&v] + 1);
                    }
                    if distances[&neighbor] == distances[&v] + 1 {
                        predecessors.get_mut(&neighbor).unwrap().push(v);
                        *sigma.get_mut(&neighbor).unwrap() += sigma[&v];
                    }
                }
            }
        }

        while let Some(w) = stack.pop() {
            if let Some(preds) = predecessors.get(&w) {
                for &v in preds {
                    let delta_val = (sigma[&v] / sigma[&w]) * (1.0 + delta[&w]);
                    *delta.get_mut(&v).unwrap() += delta_val;
                }
            }
            if w != start_node {
                *centrality.get_mut(&w).unwrap() += delta[&w];
            }
        }
    }

    for value in centrality.values_mut() {
        *value /= sample_size as f64;
    }

    centrality
}
