use std::collections::{HashMap, HashSet, VecDeque};
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn calculate_average_shortest_path(
    adjacency_list: &HashMap<u32, Vec<u32>>,
    sample_size: usize,
) -> f64 {
    let nodes: Vec<&u32> = adjacency_list.keys().collect();
    let mut total_distance = 0;
    let mut total_pairs = 0;

    let sampled_nodes = nodes.iter().take(sample_size);

    for &&start_node in sampled_nodes {
        let mut visited: HashSet<u32> = HashSet::new();
        let mut queue: VecDeque<(u32, usize)> = VecDeque::new();

        visited.insert(start_node);
        queue.push_back((start_node, 0));

        while let Some((current_node, distance)) = queue.pop_front() {
            if let Some(neighbors) = adjacency_list.get(&current_node) {
                for &neighbor in neighbors {
                    if visited.insert(neighbor) {
                        queue.push_back((neighbor, distance + 1));
                        total_distance += distance + 1;
                        total_pairs += 1;
                    }
                }
            }
        }
    }

    if total_pairs == 0 {
        0.0
    } else {
        total_distance as f64 / total_pairs as f64
    }
}

pub fn calculate_degree_distribution(
    adjacency_list: &HashMap<u32, Vec<u32>>,
) -> HashMap<usize, usize> {
    let mut degree_counts: HashMap<usize, usize> = HashMap::new();

    for (_, neighbors) in adjacency_list {
        let degree = neighbors.len();
        *degree_counts.entry(degree).or_insert(0) += 1;
    }

    degree_counts
}

pub fn detect_communities(adjacency_list: &HashMap<u32, Vec<u32>>) -> HashMap<u32, u32> {
    let mut labels: HashMap<u32, u32> = adjacency_list.keys().map(|&node| (node, node)).collect();
    let mut rng = thread_rng();
    let nodes: Vec<u32> = adjacency_list.keys().cloned().collect();

    for _ in 0..10 {
        let mut shuffled_nodes = nodes.clone();
        shuffled_nodes.shuffle(&mut rng);

        for node in shuffled_nodes {
            if let Some(neighbors) = adjacency_list.get(&node) {
                let mut label_counts: HashMap<u32, usize> = HashMap::new();
                for &neighbor in neighbors {
                    if let Some(&label) = labels.get(&neighbor) {
                        *label_counts.entry(label).or_insert(0) += 1;
                    }
                }
                if let Some((&most_common_label, _)) = label_counts.iter().max_by_key(|&(_, count)| count) {
                    labels.insert(node, most_common_label);
                }
            }
        }
    }

    labels
}
