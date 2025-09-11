use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, PartialEq, Eq)]
pub enum HuffmanNode {
    Leaf {
        ch: char,
        freq: usize,
    },
    Internal {
        freq: usize,
        left: Box<HuffmanNode>,
        right: Box<HuffmanNode>,
    },
}

#[derive(Eq, PartialEq)]
struct HeapNode {
    freq: usize,
    node: Box<HuffmanNode>,
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.freq.cmp(&self.freq) // reverse to make min-heap
    }
}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Build Huffman tree given character frequencies
pub fn build_tree(freq_map: &HashMap<char, usize>) -> Option<Box<HuffmanNode>> {
    let mut heap = BinaryHeap::new();

    for (&ch, &freq) in freq_map.iter() {
        heap.push(HeapNode {
            freq,
            node: Box::new(HuffmanNode::Leaf { ch, freq }),
        });
    }

    if heap.is_empty() {
        return None;
    }

    while heap.len() > 1 {
        let left = heap.pop().unwrap();
        let right = heap.pop().unwrap();

        let merged_freq = left.freq + right.freq;
        let merged_node = HuffmanNode::Internal {
            freq: merged_freq,
            left: left.node,
            right: right.node,
        };

        heap.push(HeapNode {
            freq: merged_freq,
            node: Box::new(merged_node),
        });
    }

    Some(heap.pop().unwrap().node)
}

pub fn generate_codes(node: &HuffmanNode, prefix: String, codes: &mut HashMap<char, String>) {
    match node {
        HuffmanNode::Leaf { ch, .. } => {
            codes.insert(*ch, prefix);
        }
        HuffmanNode::Internal { left, right, .. } => {
            generate_codes(left, format!("{}0", prefix), codes);
            generate_codes(right, format!("{}1", prefix), codes);
        }
    }
}
