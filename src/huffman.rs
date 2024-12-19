use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, PartialEq, Eq, Hash)]
enum HuffmanNodeType {
    Leaf(char),
    Parent(Box<HuffmanNode>, Box<HuffmanNode>),
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) struct HuffmanNode {
    frequency: u64,
    kind: HuffmanNodeType,
}

impl HuffmanNode {
    fn visit(&self, f: impl FnMut(char))
}

impl PartialOrd for HuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HuffmanNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.frequency.cmp(&other.frequency).reverse()
    }
}

pub(crate) fn create_huffman_encoding(text: &str) -> Option<HuffmanNode> {
    let mut frequency_map = HashMap::new();
    for c in text.chars() {
        *frequency_map.entry(c).or_insert(0) += 1;
    }
    let mut queue = frequency_map
        .into_iter()
        .map(|(value, frequency)| HuffmanNode {
            frequency,
            kind: HuffmanNodeType::Leaf(value),
        })
        .collect::<BinaryHeap<_>>();
    while queue.len() > 1 {
        let left = queue.pop().unwrap();
        let right = queue.pop().unwrap();
        let new_node = HuffmanNode {
            frequency: left.frequency + right.frequency,
            kind: HuffmanNodeType::Parent(Box::new(left), Box::new(right)),
        };
        queue.push(new_node);
    }
    queue.pop()
}
