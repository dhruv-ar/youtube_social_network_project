use csv::ReaderBuilder;
use std::collections::{HashMap, HashSet, VecDeque};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs::File;
use std::io::Write;

fn calculate_closeness_centrality_sampled(
    adjacency_list: &HashMap<u32, Vec<u32>>,
    sample_size: usize,
) -> HashMap<u32, f64> {
    let mut centrality = HashMap::new();
    let total_nodes = adjacency_list.len() as f64;

    // Sample a subset of nodes
    let sampled_nodes: Vec<u32> = adjacency_list.keys().cloned().take(sample_size).collect();

    for &start_node in sampled_nodes.iter() {
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
        if total_distance > 0 {
            let closeness = (total_nodes - 1.0) / total_distance as f64;
            centrality.insert(start_node, closeness);
        }
    }

    centrality
}

fn main() {
    println!("Loading the graph...");

    // Initialize an adjacency list
    let mut adjacency_list: HashMap<u32, Vec<u32>> = HashMap::new();

    // File path
    let file_path = "data/com-youtube.ungraph.csv";

    // Open the CSV file
    let mut reader = ReaderBuilder::new()
        .has_headers(false) // Assume no headers in the dataset
        .delimiter(b'\t') // Use tab as the delimiter; change to b' ' if multiple spaces
        .from_path(file_path)
        .expect("Failed to open the file");

    let mut processed_count = 0;
    let mut skipped_count = 0;

    // Parse the CSV file
    for (i, result) in reader.records().enumerate() {
        match result {
            Ok(record) => {
                // Ensure the record has exactly two fields
                if record.len() != 2 {
                    println!("Skipping line {}: Unexpected number of fields", i + 1);
                    skipped_count += 1;
                    continue;
                }

                // Trim and parse fields
                let node1 = record[0].trim().parse::<u32>();
                let node2 = record[1].trim().parse::<u32>();

                match (node1, node2) {
                    (Ok(n1), Ok(n2)) => {
                        adjacency_list.entry(n1).or_default().push(n2);
                        adjacency_list.entry(n2).or_default().push(n1);
                        processed_count += 1;
                    }
                    _ => {
                        println!("Skipping invalid line {}: {:?}", i + 1, record);
                        skipped_count += 1;
                    }
                }
            }
            Err(err) => {
                println!("Error reading line {}: {}", i + 1, err);
                skipped_count += 1;
            }
        }
    }

    println!("Graph loaded successfully.");
    println!("Number of processed edges: {}", processed_count);
    println!("Number of skipped lines: {}", skipped_count);
    println!("Number of nodes: {}", adjacency_list.len());

    // Step 1: Calculate Six Degrees of Separation
    println!("Calculating average shortest path length...");
    let sample_size = 100; // Adjust the sample size as needed
    let avg_shortest_path = calculate_average_shortest_path(&adjacency_list, sample_size);

    println!(
        "Average Shortest Path Length (based on {} samples): {:.4}",
        sample_size, avg_shortest_path
    );

    // Step 2: Calculate Degree Distribution
    println!("Calculating degree distribution...");
    let degree_distribution = calculate_degree_distribution(&adjacency_list);

    println!("Sample of degree distribution:");
    for (degree, count) in degree_distribution.iter().take(10) {
        println!("Degree: {}, Count: {}", degree, count);
    }

    // Save degree distribution to a CSV file
    let output_file = "degree_distribution.csv";
    save_degree_distribution(output_file, &degree_distribution);
    println!("Degree distribution saved to {}", output_file);

    // Plot and save degree distribution visualization
    let plot_output_file = "degree_distribution.png";
    plot_degree_distribution(plot_output_file, &degree_distribution);
    println!("Degree distribution plot saved to {}", plot_output_file);

    // Save degree distribution to a CSV file
    let output_file = "degree_distribution.csv";
    save_degree_distribution(output_file, &degree_distribution);
    println!("Degree distribution saved to {}", output_file);
    
    // Plot and save degree distribution visualization
    let plot_output_file = "degree_distribution.png";
    plot_degree_distribution(plot_output_file, &degree_distribution);
    println!("Degree distribution plot saved to {}", plot_output_file);

    // Community Detection
    println!("Detecting communities...");
    let communities = detect_communities(&adjacency_list);
    save_communities(&communities, "community_results_rust.txt");
    println!("Community detection completed. Results saved to 'community_results_rust.txt'.");
    // Centrality Analysis
    println!("Calculating centrality measures...");
    // Degree Centrality
    println!("Calculating degree centrality...");
    let degree_centrality = calculate_degree_centrality(&adjacency_list);
    save_centrality("degree_centrality.csv", &degree_centrality);
    println!("Degree centrality saved to 'degree_centrality.csv'.");
    // Closeness Centrality
    println!("Calculating closeness centrality (sampled)...");
    let sampled_nodes = 1000; // Adjust sample size as needed
    let closeness_centrality_sampled = calculate_closeness_centrality_sampled(&adjacency_list, sampled_nodes);
    save_centrality("closeness_centrality_sampled.csv", &closeness_centrality_sampled);
    println!("Closeness centrality (sampled) saved to 'closeness_centrality_sampled.csv'.");
    // Betweenness Centrality
    println!("Calculating betweenness centrality (sampled)...");
    let sample_size = 1000; // Adjust sample size as needed
    let betweenness_centrality_sampled = calculate_betweenness_centrality_sampled(&adjacency_list, sample_size);
    save_centrality("betweenness_centrality_sampled.csv", &betweenness_centrality_sampled);
    println!("Betweenness centrality (sampled) saved to 'betweenness_centrality_sampled.csv'.");
    
}

// Function to calculate average shortest path length using BFS
fn calculate_average_shortest_path(
    adjacency_list: &HashMap<u32, Vec<u32>>,
    sample_size: usize,
) -> f64 {
    let nodes: Vec<&u32> = adjacency_list.keys().collect();
    let mut total_distance = 0;
    let mut total_pairs = 0;

    // Sample a subset of nodes
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

// Function to calculate degree distribution
fn calculate_degree_distribution(adjacency_list: &HashMap<u32, Vec<u32>>) -> HashMap<usize, usize> {
    let mut degree_counts: HashMap<usize, usize> = HashMap::new();

    for (_, neighbors) in adjacency_list {
        let degree = neighbors.len();
        *degree_counts.entry(degree).or_insert(0) += 1;
    }

    degree_counts
}

// Function to save degree distribution to a CSV file
fn save_degree_distribution(file_path: &str, degree_distribution: &HashMap<usize, usize>) {
    let mut writer = csv::Writer::from_path(file_path).expect("Failed to create output file");

    // Write headers
    writer
        .write_record(&["Degree", "Count"])
        .expect("Failed to write header");

    for (degree, count) in degree_distribution {
        writer
            .write_record(&[degree.to_string(), count.to_string()])
            .expect("Failed to write record");
    }
}

// Community detection using Label Propagation
fn detect_communities(adjacency_list: &HashMap<u32, Vec<u32>>) -> HashMap<u32, u32> {
    let mut labels: HashMap<u32, u32> = adjacency_list.keys().map(|&node| (node, node)).collect();
    let mut rng = thread_rng();
    let nodes: Vec<u32> = adjacency_list.keys().cloned().collect();

    for _ in 0..10 { // Number of iterations
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

fn save_communities(labels: &HashMap<u32, u32>, output_file: &str) {
    let mut community_map: HashMap<u32, Vec<u32>> = HashMap::new();
    for (&node, &label) in labels {
        community_map.entry(label).or_default().push(node);
    }

    let mut file = File::create(output_file).expect("Unable to create file");
    for (community, members) in community_map {
        writeln!(&mut file, "Community {}: {:?}", community, members).expect("Unable to write to file");
    }
}
// Centrality Analysis
fn calculate_degree_centrality(adjacency_list: &HashMap<u32, Vec<u32>>) -> HashMap<u32, f64> {
    let mut centrality = HashMap::new();

    for (&node, neighbors) in adjacency_list {
        centrality.insert(node, neighbors.len() as f64);
    }

    centrality
}

fn calculate_closeness_centrality(adjacency_list: &HashMap<u32, Vec<u32>>) -> HashMap<u32, f64> {
    let mut centrality = HashMap::new();
    let total_nodes = adjacency_list.len() as f64;

    for &start_node in adjacency_list.keys() {
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
        if total_distance > 0 {
            let closeness = (total_nodes - 1.0) / total_distance as f64;
            centrality.insert(start_node, closeness);
        }
    }

    centrality
}

fn calculate_betweenness_centrality_sampled(
    adjacency_list: &HashMap<u32, Vec<u32>>,
    sample_size: usize,
) -> HashMap<u32, f64> {
    let mut centrality = HashMap::new();

    // Initialize centrality values for all nodes
    for &node in adjacency_list.keys() {
        centrality.insert(node, 0.0);
    }

    // Sample a subset of nodes
    let sampled_nodes: Vec<u32> = adjacency_list.keys().cloned().take(sample_size).collect();

    // Perform betweenness centrality calculation on sampled nodes
    for &start_node in sampled_nodes.iter() {
        let mut stack = Vec::new();
        let mut predecessors: HashMap<u32, Vec<u32>> = HashMap::new();
        let mut sigma = HashMap::new();
        let mut distance = HashMap::new();
        let mut delta = HashMap::new();

        for &node in adjacency_list.keys() {
            predecessors.insert(node, Vec::new());
            sigma.insert(node, 0.0);
            distance.insert(node, -1);
            delta.insert(node, 0.0);
        }

        sigma.insert(start_node, 1.0);
        distance.insert(start_node, 0);

        let mut queue = VecDeque::new();
        queue.push_back(start_node);

        while let Some(v) = queue.pop_front() {
            stack.push(v);
            if let Some(neighbors) = adjacency_list.get(&v) {
                for &neighbor in neighbors {
                    if distance[&neighbor] < 0 {
                        queue.push_back(neighbor);
                        *distance.get_mut(&neighbor).unwrap() = distance[&v] + 1;
                    }
                    if distance[&neighbor] == distance[&v] + 1 {
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

    // Normalize centrality scores by dividing by the sample size
    for value in centrality.values_mut() {
        *value /= sample_size as f64;
    }

    centrality
}


fn save_centrality(file_path: &str, centrality: &HashMap<u32, f64>) {
    let mut writer = csv::Writer::from_path(file_path).expect("Failed to create output file");

    writer
        .write_record(&["Node", "Centrality"])
        .expect("Failed to write header");

    for (node, value) in centrality {
        writer
            .write_record(&[node.to_string(), value.to_string()])
            .expect("Failed to write record");
    }
}
use plotters::prelude::*;

fn plot_degree_distribution(file_path: &str, degree_distribution: &HashMap<usize, usize>) {
    let root = BitMapBackend::new(file_path, (1024, 768))
        .into_drawing_area();
    root.fill(&WHITE).expect("Failed to fill background");

    let mut chart = ChartBuilder::on(&root)
        .caption("Degree Distribution", ("sans-serif", 50).into_font())
        .margin(10)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_cartesian_2d(
            0..degree_distribution.keys().max().unwrap_or(&0) + 10,
            0..*degree_distribution.values().max().unwrap_or(&0) + 10,
        )
        .expect("Failed to build chart");

    chart.configure_mesh()
        .x_desc("Degree")
        .y_desc("Count")
        .draw()
        .expect("Failed to draw mesh");

    let data: Vec<(usize, usize)> = degree_distribution.iter().map(|(&k, &v)| (k, v)).collect();

    chart.draw_series(
        data.iter()
            .map(|&(x, y)| Circle::new((x, y), 3, BLUE.filled())),
    ).expect("Failed to draw series");
}

