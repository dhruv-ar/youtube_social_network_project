use csv::ReaderBuilder;
use std::collections::HashMap;

pub fn load_graph(file_path: &str) -> HashMap<u32, Vec<u32>> {
    let mut adjacency_list: HashMap<u32, Vec<u32>> = HashMap::new();
    let file_path = "data/com-youtube.ungraph.csv";
    let mut reader = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'\t')
        .from_path(file_path)
        .expect("Failed to open the file");

    for (i, result) in reader.records().enumerate() {
        match result {
            Ok(record) => {
                if record.len() != 2 {
                    println!("Skipping line {}: Unexpected number of fields", i + 1);
                    continue;
                }
                let node1 = record[0].trim().parse::<u32>();
                let node2 = record[1].trim().parse::<u32>();

                match (node1, node2) {
                    (Ok(n1), Ok(n2)) => {
                        adjacency_list.entry(n1).or_default().push(n2);
                        adjacency_list.entry(n2).or_default().push(n1);
                    }
                    _ => {
                        println!("Skipping invalid line {}: {:?}", i + 1, record);
                    }
                }
            }
            Err(err) => {
                println!("Error reading line {}: {}", i + 1, err);
            }
        }
    }

    adjacency_list
}

pub fn get_graph_info(adjacency_list: &HashMap<u32, Vec<u32>>) -> (usize, usize) {
    let num_nodes = adjacency_list.len();
    let num_edges = adjacency_list.values().map(|neighbors| neighbors.len()).sum::<usize>() / 2;
    (num_nodes, num_edges)
}
