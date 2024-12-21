use std::collections::{BinaryHeap, HashMap};
use std::fmt::Debug;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub(crate) enum Step {
    Left = 0,
    Right = 1,
}

impl From<bool> for Step {
    fn from(value: bool) -> Self {
        match value {
            false => Step::Left,
            true => Step::Right,
        }
    }
}

impl From<Step> for bool {
    fn from(value: Step) -> Self {
        match value {
            Step::Left => false,
            Step::Right => true,
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub(crate) struct Steps {
    pub(crate) bits: u32,
    pub(crate) len: u8,
}

fn pop_impl(bits: &mut u32, len: &mut u8) -> Option<Step> {
    if *len == 0 {
        None
    } else {
        *len -= 1;
        let step = *bits & 1;
        *bits >>= 1;
        Some(Step::from(step != 0))
    }
}

impl Steps {
    pub(crate) fn new() -> Self {
        Self { bits: 0, len: 0 }
    }

    fn push(&mut self, step: Step) {
        self.bits <<= 1;
        self.bits |= step as u32;
        self.len += 1;
    }

    fn pop(&mut self) -> Option<Step> {
        pop_impl(&mut self.bits, &mut self.len)
    }
}

pub(crate) struct StepsIter {
    bits: u32,
    len: u8,
}

impl Iterator for StepsIter {
    type Item = Step;

    fn next(&mut self) -> Option<Self::Item> {
        pop_impl(&mut self.bits, &mut self.len)
    }
}

impl DoubleEndedIterator for StepsIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.len == 0 {
            return None;
        }
        self.len -= 1;
        let bit = (self.bits >> self.len) & 1;
        Some(Step::from(bit != 0))
    }
}

impl IntoIterator for Steps {
    type Item = Step;
    type IntoIter = StepsIter;

    fn into_iter(self) -> Self::IntoIter {
        StepsIter {
            bits: self.bits,
            len: self.len,
        }
    }
}

impl Debug for Steps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.into_iter().rev()).finish()
    }
}

#[derive(Debug)]
enum HuffmanNodeType {
    Leaf(char),
    Parent(Box<HuffmanNode>, Box<HuffmanNode>),
}

#[derive(Debug)]
struct HuffmanNode {
    frequency: u64,
    kind: HuffmanNodeType,
}

#[derive(Debug)]
pub(crate) struct HuffmanTree {
    root: HuffmanNode,
}

impl HuffmanNode {
    fn visit<F>(&self, steps: &mut Steps, f: &mut F)
    where
        F: FnMut(char, Steps),
    {
        match &self.kind {
            HuffmanNodeType::Leaf(value) => {
                f(*value, *steps);
            }
            HuffmanNodeType::Parent(left, right) => {
                steps.push(Step::Left);
                left.visit(steps, f);
                steps.pop();
                steps.push(Step::Right);
                right.visit(steps, f);
                steps.pop();
            }
        }
    }
}

impl HuffmanTree {
    fn walk<F>(&self, mut f: F)
    where
        F: FnMut(char, Steps),
    {
        let mut steps = Steps::new();
        self.root.visit(&mut steps, &mut f);
    }
}

#[repr(transparent)]
struct ComparableHuffmanNode(HuffmanNode);

impl PartialOrd for ComparableHuffmanNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ComparableHuffmanNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.frequency.cmp(&other.0.frequency).reverse()
    }
}

impl PartialEq for ComparableHuffmanNode {
    fn eq(&self, other: &Self) -> bool {
        self.0.frequency == other.0.frequency
    }
}

impl Eq for ComparableHuffmanNode {}

#[derive(Debug)]
pub(crate) struct HuffmanEncoding {
    char_to_steps: HashMap<char, Steps>,
    steps_to_char: HashMap<Steps, char>,
}

impl HuffmanEncoding {
    pub(crate) fn new(tree: &HuffmanTree) -> Self {
        let mut char_to_steps = HashMap::new();
        let mut steps_to_char = HashMap::new();
        tree.walk(|value, steps| {
            char_to_steps.insert(value, steps);
            steps_to_char.insert(steps, value);
        });
        Self {
            char_to_steps,
            steps_to_char,
        }
    }
}

#[derive(Debug)]
pub(crate) struct EncodedData {
    data: Vec<u8>,
    last_byte_len: u8,
}

pub(crate) fn create_huffman_tree(text: &str) -> Option<HuffmanTree> {
    let mut frequency_map = HashMap::new();
    for c in text.chars() {
        *frequency_map.entry(c).or_insert(0) += 1;
    }
    let mut queue = frequency_map
        .into_iter()
        .map(|(value, frequency)| {
            ComparableHuffmanNode(HuffmanNode {
                frequency,
                kind: HuffmanNodeType::Leaf(value),
            })
        })
        .collect::<BinaryHeap<_>>();
    while queue.len() > 1 {
        let ComparableHuffmanNode(left) = queue.pop().unwrap();
        let ComparableHuffmanNode(right) = queue.pop().unwrap();
        println!(
            "left frequency: {}, right frequency: {}",
            left.frequency, right.frequency
        );
        let new_node = ComparableHuffmanNode(HuffmanNode {
            frequency: left.frequency + right.frequency,
            kind: HuffmanNodeType::Parent(Box::new(left), Box::new(right)),
        });
        queue.push(new_node);
    }
    queue.pop().map(|v| HuffmanTree { root: v.0 })
}

pub(crate) fn create_huffman_encoding(text: &str) -> Option<HuffmanEncoding> {
    create_huffman_tree(text).as_ref().map(HuffmanEncoding::new)
}

pub(crate) fn encode(encoding: &HuffmanEncoding, text: &str) -> EncodedData {
    let mut result = Vec::new();
    let mut bits = 0;
    let mut len = 0;
    for c in text.chars() {
        let steps = encoding.char_to_steps.get(&c).unwrap();
        for step in steps.into_iter().rev() {
            bits <<= 1;
            bits |= step as u32;
            len += 1;
            if len == 8 {
                result.push(bits as u8);
                bits = 0;
                len = 0;
            }
        }
    }
    if len > 0 {
        result.push(bits as u8);
    }
    EncodedData {
        data: result,
        last_byte_len: len,
    }
}

pub(crate) fn decode(encoding: &HuffmanEncoding, encoded_data: &EncodedData) -> String {
    let mut result = String::new();
    let mut steps = Steps::new();
    let data_len = encoded_data.data.len();
    if data_len == 0 {
        return String::new();
    }
    let full_bytes_len = if encoded_data.last_byte_len == 0 {
        data_len
    } else {
        data_len - 1
    };
    macro_rules! process_step {
        ($index:ident, $byte:ident, $offset:expr) => {
            let step = Step::from($byte & (1 << ($offset - $index)) != 0);
            steps.push(step);
            if let Some(&c) = encoding.steps_to_char.get(&steps) {
                result.push(c);
                steps = Steps::new();
            }
        };
    }
    for byte in encoded_data.data[..full_bytes_len].iter().copied() {
        for i in 0..8u8 {
            process_step!(i, byte, 7);
        }
    }
    let byte = encoded_data.data[data_len - 1];
    for i in 0..encoded_data.last_byte_len {
        process_step!(i, byte, encoded_data.last_byte_len - 1);
    }
    assert_eq!(steps, Steps::new());
    result
}
