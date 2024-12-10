use csv::Writer;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::Write;
use plotters::prelude::*;

/// Ensure the `output/` directory exists
pub fn ensure_output_dir_exists() {
    let output_dir = "output";
    if !fs::metadata(output_dir).is_ok() {
        fs::create_dir(output_dir).expect("Failed to create output directory");
    }
}

pub fn save_centrality(file_name: &str, centrality: &HashMap<u32, f64>) {
    ensure_output_dir_exists(); // Ensure the directory exists
    let file_path = format!("output/{}", file_name);
    let mut writer = Writer::from_path(&file_path).expect("Failed to create output file");

    writer
        .write_record(&["Node", "Centrality"])
        .expect("Failed to write header");

    for (node, value) in centrality {
        writer
            .write_record(&[node.to_string(), value.to_string()])
            .expect("Failed to write record");
    }
    println!("Centrality data saved to '{}'.", file_path);
}

pub fn save_degree_distribution(file_name: &str, degree_distribution: &HashMap<usize, usize>) {
    ensure_output_dir_exists(); // Ensure the directory exists
    let file_path = format!("output/{}", file_name);
    let mut writer = Writer::from_path(&file_path).expect("Failed to create output file");

    writer
        .write_record(&["Degree", "Count"])
        .expect("Failed to write header");

    for (degree, count) in degree_distribution {
        writer
            .write_record(&[degree.to_string(), count.to_string()])
            .expect("Failed to write record");
    }
    println!("Degree distribution data saved to '{}'.", file_path);
}

pub fn save_communities(labels: &HashMap<u32, u32>, file_name: &str) {
    ensure_output_dir_exists(); // Ensure the directory exists
    let file_path = format!("output/{}", file_name);

    let mut community_map: HashMap<u32, Vec<u32>> = HashMap::new();
    for (&node, &label) in labels {
        community_map.entry(label).or_default().push(node);
    }

    let mut file = File::create(&file_path).expect("Unable to create file");
    for (community, members) in community_map {
        writeln!(&mut file, "Community {}: {:?}", community, members).expect("Unable to write to file");
    }
    println!("Community detection results saved to '{}'.", file_path);
}

pub fn plot_degree_distribution(file_name: &str, degree_distribution: &HashMap<usize, usize>) {
    ensure_output_dir_exists(); // Ensure the directory exists
    let file_path = format!("output/{}", file_name);

    let root = BitMapBackend::new(&file_path, (1024, 768))
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

    chart
        .draw_series(data.iter().map(|&(x, y)| Circle::new((x, y), 3, BLUE.filled())))
        .expect("Failed to draw series");
    println!("Degree distribution plot saved to '{}'.", file_path);
}
