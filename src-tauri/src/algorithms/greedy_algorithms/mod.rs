pub mod activity_selection;
pub mod fractional_knapsack;
pub mod huffman_coding;
pub mod kruskals_algorithm;
pub mod prims_algorithm;

pub use activity_selection::activity_selection;
pub use fractional_knapsack::fractional_knapsack;
pub use huffman_coding::build_tree;
pub use huffman_coding::generate_codes;
pub use kruskals_algorithm::kruskal;
pub use prims_algorithm::prim;
