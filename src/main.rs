mod analysis;
mod centrality;
mod utils;
mod graph;

use crate::analysis::{calculate_average_shortest_path, calculate_degree_distribution, detect_communities};
use crate::centrality::{calculate_degree_centrality, calculate_closeness_centrality_parallel, calculate_betweenness_centrality_sampled};
use crate::utils::{save_centrality, save_degree_distribution, plot_degree_distribution, save_communities};
use crate::graph::{load_graph, get_graph_info};

fn main() {
    println!("Loading the graph...");

    
    let file_path = "data/com-youtube.ungraph.csv";
    let adjacency_list = load_graph(file_path);

    // Retrieve graph info
    let (num_nodes, num_edges) = get_graph_info(&adjacency_list);
    println!("Graph loaded successfully.");
    println!("Number of nodes: {}", num_nodes);
    println!("Number of edges: {}", num_edges);

    // Step 1: Calculate Six Degrees of Separation
    println!("Calculating average shortest path length...");
    let sample_size = 100;
    let avg_shortest_path = calculate_average_shortest_path(&adjacency_list, sample_size);
    println!(
        "Average Shortest Path Length (based on {} samples): {:.4}",
        sample_size, avg_shortest_path
    );

    // Step 2: Calculate Degree Distribution
    println!("Calculating degree distribution...");
    let degree_distribution = calculate_degree_distribution(&adjacency_list);
    save_degree_distribution("degree_distribution.csv", &degree_distribution);
    plot_degree_distribution("degree_distribution.png", &degree_distribution);

    // Step 3: Community Detection
    println!("Detecting communities...");
    let communities = detect_communities(&adjacency_list);
    save_communities(&communities, "community_results_rust.txt");

    // Step 4: Centrality Measures
    println!("Calculating centrality measures...");
    let degree_centrality = calculate_degree_centrality(&adjacency_list);
    save_centrality("degree_centrality.csv", &degree_centrality);

    let closeness_centrality = calculate_closeness_centrality_parallel(&adjacency_list, 500);
    save_centrality("closeness_centrality.csv", &closeness_centrality);

    let betweenness_centrality = calculate_betweenness_centrality_sampled(&adjacency_list, 500);
    save_centrality("betweenness_centrality.csv", &betweenness_centrality);
}
