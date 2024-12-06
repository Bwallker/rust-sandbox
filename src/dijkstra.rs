use std::collections::{BinaryHeap, HashMap, HashSet};

pub(crate) struct Graph {
    nodes: HashMap<String, HashMap<String, u64>>,
}

impl Graph {
    pub(crate) fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub(crate) fn add_edge(&mut self, from: &str, to: &str, weight: u64) {
        self.nodes
            .entry(from.to_owned())
            .or_insert_with(HashMap::new)
            .insert(to.to_owned(), weight);
    }

    pub(crate) fn add_node(&mut self, name: &str) {
        self.nodes
            .entry(name.to_owned())
            .or_insert_with(HashMap::new);
    }
}

struct QueueItem<'a>(u64, &'a str);

impl PartialEq for QueueItem<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl PartialOrd for QueueItem<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for QueueItem<'_> {}

impl Ord for QueueItem<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Considers the smallest distance to be the highest priority.
        self.0.cmp(&other.0).reverse()
    }
}

pub(crate) fn dijkstra<'a>(graph: &'a Graph, start_node: &'_ str) -> HashMap<&'a str, u64> {
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();
    let start_node = graph.nodes.get_key_value(start_node).unwrap().0.as_str();
    queue.push(QueueItem(0, start_node));
    let mut ret = HashMap::new();
    ret.insert(start_node, 0);
    while let Some(QueueItem(distance, node)) = queue.pop() {
        if visited.contains(node) {
            continue;
        }
        println!("Visiting node: {}", node);
        visited.insert(node);
        for (neighbor, weight) in graph.nodes[node].iter() {
            if visited.contains(neighbor.as_str()) {
                continue;
            }
            let new_distance = distance + weight;
            let ref_ = ret.entry(neighbor).or_insert(u64::MAX);
            *ref_ = new_distance.min(*ref_);
            queue.push(QueueItem(new_distance, neighbor));
        }
    }
    ret
}
