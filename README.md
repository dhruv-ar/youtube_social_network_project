Report on YouTube Social Network Dataset, Author: Dhruv Arora

Introduction and Dataset Selection
The project aims to explore the structure and properties of a large-scale social network using graph analysis techniques. I chose the com-youtube.ungraph.csv dataset because it represents a real-world social network, connecting YouTube users based on mutual relationships. This dataset intrigued me due to its relevance in understanding online interactions and the challenges of working with a large-scale dataset.
The dataset is particularly appealing due to its large size, containing over a million nodes and nearly three million edges. This offers the perfect foundation for studying complex graph properties such as degree distribution, community detection, and centrality measures. My primary objectives included uncovering meaningful insights into user connectivity, identifying influential individuals, and analyzing the formation of sub-communities.
Before diving into the analysis, I ensured the dataset’s structure was understood. The data consisted of two columns, each representing a connection between two users. I validated the dataset for any inconsistencies, such as missing or malformed data, ensuring the analysis proceeded smoothly.

Methodology
The analysis was divided into distinct steps, focusing on various aspects of the network. The overall thought process was to start with a fundamental understanding of the dataset and gradually build toward more sophisticated analyses.

Dataset Preparation
Graph Representation: The dataset was represented as an adjacency list to minimize memory usage and enable efficient graph traversal.
Error Handling: Rows with malformed data or unexpected fields were logged, ensuring transparency.
Validation: Small samples of the graph were examined to verify relationships and ensure data accuracy.
Basic Statistics: The total number of nodes and edges were calculated for validation.

Project Structure
The project is modularized for maintainability and scalability. Below is the structure of the project:
main.rs: Entry point of the application, orchestrating the overall workflow.
lib.rs: Provides the common interface for all modules, exposing public modules for use in the main.rs and tests.
graph.rs: Handles operations related to loading, parsing, and maintaining the graph structure. It also provides basic graph statistics like the number of nodes and edges.
analysis.rs: Contains functions for analyzing the graph, including shortest path calculations, degree distribution, and community detection.
centrality.rs: Implements centrality measures such as degree, closeness, and betweenness centrality.
utils.rs: Provides utility functions for file operations and plotting. It ensures that outputs like CSV files and visualizations are correctly saved.
tests/: Contains test cases in tests.rs to validate the functionality of each module and ensure correctness.
output/: Stores all generated outputs, such as CSV files and visualizations, ensuring a clean project structure.

Analytical Steps
Degree Distribution
Objective: Understand how connections are distributed among nodes.
Findings: The network exhibited a power-law degree distribution, highlighting the presence of "hubs" (nodes with exceptionally high connectivity).
Visualization: A degree distribution plot was generated, confirming the scale-free nature of the network.
Six Degrees of Separation
Objective: Test the small-world property of the network.
Method: Calculated the shortest paths between randomly sampled nodes using BFS.
Result: The average shortest path length was approximately 5.23, validating the small-world property and aligning with the "six degrees of separation" hypothesis.
Community Detection
Objective: Identify clusters or groups of closely connected users.
Method: Implemented a label propagation algorithm to detect communities.
Result: Distinct communities were identified, reflecting patterns of interaction and shared interests among users.
Centrality Measures
Centrality measures were used to identify influential nodes:
Degree Centrality: Identified the most connected users.
Closeness Centrality: Highlighted nodes with the shortest average distance to all others, indicating efficient communicators.
Betweenness Centrality: Identified critical nodes bridging different communities.
Optimization: Computational efficiency was improved using the rayon crate for parallel processing.

Testing and Validation
Unit Tests: A suite of tests was implemented in tests.rs to validate core functionalities, including:
Loading the graph.
Calculating degree distribution.
Computing centrality measures.
Verifying community detection accuracy.
Small-Scale Testing: Methods were first validated on smaller subgraphs before scaling to the full dataset.

Conclusion
The analysis of the YouTube social network dataset provided valuable insights into the structure and properties of large-scale social networks. By leveraging modular programming, efficient graph representations, and parallelization, I was able to perform computationally intensive tasks while maintaining accuracy. This project demonstrated the power of graph analysis in uncovering meaningful patterns and the potential for further exploration in real-world networks.


