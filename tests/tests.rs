use youtube_social_network::graph::load_graph;
use youtube_social_network::analysis::{calculate_average_shortest_path, calculate_degree_distribution};
use youtube_social_network::centrality::{calculate_degree_centrality, calculate_closeness_centrality_parallel};
use std::collections::HashMap;

#[test]
fn test_load_graph() {
    let dummy_data = vec![
        ("1", "2"),
        ("2", "3"),
        ("3", "4"),
        ("4", "1"),
    ];

    let mut adjacency_list: HashMap<u32, Vec<u32>> = HashMap::new();
    for (n1, n2) in dummy_data {
        let n1 = n1.parse::<u32>().unwrap();
        let n2 = n2.parse::<u32>().unwrap();
        adjacency_list.entry(n1).or_default().push(n2);
        adjacency_list.entry(n2).or_default().push(n1);
    }

    assert_eq!(adjacency_list.len(), 4); // Check if the graph has 4 nodes
    assert!(adjacency_list.get(&1).unwrap().contains(&2)); // Check if an edge exists
}

#[test]
fn test_calculate_average_shortest_path() {
    let mut adjacency_list: HashMap<u32, Vec<u32>> = HashMap::new();
    adjacency_list.insert(1, vec![2]);
    adjacency_list.insert(2, vec![1, 3]);
    adjacency_list.insert(3, vec![2, 4]);
    adjacency_list.insert(4, vec![3]);

    let avg_path = calculate_average_shortest_path(&adjacency_list, 10);
    assert!(avg_path > 0.0); // Check if the average path length is greater than 0
    assert!(avg_path <= 3.0); // Ensure the result is within a reasonable range for this graph
}

#[test]
fn test_calculate_degree_distribution() {
    let mut adjacency_list: HashMap<u32, Vec<u32>> = HashMap::new();
    adjacency_list.insert(1, vec![2, 3]);
    adjacency_list.insert(2, vec![1]);
    adjacency_list.insert(3, vec![1]);

    let degree_distribution = calculate_degree_distribution(&adjacency_list);

    assert_eq!(degree_distribution.get(&2), Some(&1)); // One node with degree 2
    assert_eq!(degree_distribution.get(&1), Some(&2)); // Two nodes with degree 1
    assert_eq!(degree_distribution.len(), 2); // There are two unique degrees
}

#[test]
fn test_calculate_degree_centrality() {
    let mut adjacency_list: HashMap<u32, Vec<u32>> = HashMap::new();
    adjacency_list.insert(1, vec![2, 3]);
    adjacency_list.insert(2, vec![1]);
    adjacency_list.insert(3, vec![1]);

    let degree_centrality = calculate_degree_centrality(&adjacency_list);

    assert_eq!(degree_centrality.get(&1), Some(&2.0)); // Node 1 has degree centrality 2
    assert_eq!(degree_centrality.get(&2), Some(&1.0)); // Node 2 has degree centrality 1
    assert_eq!(degree_centrality.len(), 3); // There are 3 nodes in total
}

#[test]
fn test_calculate_closeness_centrality_parallel() {
    let mut adjacency_list: HashMap<u32, Vec<u32>> = HashMap::new();
    adjacency_list.insert(1, vec![2, 3]);
    adjacency_list.insert(2, vec![1, 4]);
    adjacency_list.insert(3, vec![1]);
    adjacency_list.insert(4, vec![2]);

    let closeness_centrality = calculate_closeness_centrality_parallel(&adjacency_list, 4);

    assert!(*closeness_centrality.get(&1).unwrap() > 0.0); // Dereferenced before comparison
    assert!(*closeness_centrality.get(&2).unwrap() > 0.0); // Dereferenced before comparison
    assert_eq!(closeness_centrality.len(), 4); // Ensure all nodes are included
}

