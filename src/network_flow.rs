use std::{
    collections::{HashMap, HashSet},
    num::NonZeroU64,
};

pub(crate) struct NetworkGraph {
    edges: HashMap<String, HashMap<String, u64>>,
}

impl NetworkGraph {
    pub(crate) fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    pub(crate) fn add_edge(&mut self, from: &str, to: &str, capacity: u64) {
        self.edges
            .entry(from.to_owned())
            .or_insert_with(HashMap::new)
            .insert(to.to_owned(), capacity);
    }

    pub(crate) fn add_node(&mut self, name: &str) {
        self.edges
            .entry(name.to_owned())
            .or_insert_with(HashMap::new);
    }
}

fn find_path<'graph>(
    source: &'graph str,
    target: &'graph str,
    residual: &'_ HashMap<&'graph str, HashMap<&'graph str, u64>>,
    path: &'_ mut Vec<&'graph str>,
    visited: &'_ mut HashSet<&'graph str>,
) -> Option<NonZeroU64> {
    assert_ne!(source, target);
    visited.clear();
    path.clear();
    path.push(source);
    visited.insert(source);

    let mut current = source;
    let mut supported_capacity = u64::MAX;
    let mut old_capacity = u64::MAX;
    loop {
        let mut found = false;
        for (neighbor, capacity) in residual.get(current).unwrap().iter().map(|(k, v)| (*k, *v)) {
            if visited.contains(neighbor) {
                continue;
            }
            old_capacity = supported_capacity;
            supported_capacity = supported_capacity.min(capacity);
            path.push(neighbor);
            visited.insert(neighbor);
            current = neighbor;
            found = true;
            break;
        }

        if !found {
            path.pop();
            supported_capacity = old_capacity;
            if path.is_empty() || supported_capacity == 0 {
                return None;
            }
            current = path.last().unwrap();
        } else {
            if current == target {
                return Some(NonZeroU64::new(supported_capacity)?);
            }
        }
    }
}

pub(crate) struct FlowResult<'graph> {
    pub(crate) flow: u64,
    pub(crate) flow_map: HashMap<&'graph str, HashMap<&'graph str, u64>>,
    pub(crate) residual_map: HashMap<&'graph str, HashMap<&'graph str, u64>>,
}

pub(crate) fn max_flow<'graph>(
    network_graph: &'graph NetworkGraph,
    source: &'_ str,
    sink: &'_ str,
) -> FlowResult<'graph> {
    let source = network_graph
        .edges
        .get_key_value(source)
        .unwrap()
        .0
        .as_str();
    let sink = network_graph.edges.get_key_value(sink).unwrap().0.as_str();

    let mut flow_map: HashMap<&'graph str, HashMap<&'graph str, u64>> = network_graph
        .edges
        .iter()
        .map(|(k, v)| (k.as_str(), v.iter().map(|(k, _)| (k.as_str(), 0)).collect()))
        .collect();
    let mut residual_map: HashMap<&'graph str, HashMap<&'graph str, u64>> = network_graph
        .edges
        .iter()
        .map(|(k, v)| {
            (
                k.as_str(),
                v.iter().map(|(k, v)| (k.as_str(), *v)).collect(),
            )
        })
        .collect();

    let mut flow = 0;

    let mut path = Vec::new();
    let mut visited = HashSet::new();
    loop {
        let Some(supported_capacity) =
            find_path(source, sink, &residual_map, &mut path, &mut visited)
        else {
            break;
        };
        let supported_capacity = supported_capacity.get();
        println!("Found path: {path:?} with capacity {supported_capacity}");
        flow += supported_capacity;
        for i in 0..path.len() - 1 {
            let from = path[i];
            let to = path[i + 1];
            println!("Adding flow from {from} to {to}");
            *flow_map.entry(from).or_default().entry(to).or_default() += supported_capacity;
            let ref_ = residual_map.get_mut(from).unwrap().get_mut(to).unwrap();
            *ref_ -= supported_capacity;
            if *ref_ == 0 {
                residual_map.get_mut(from).unwrap().remove(to);
            }
            *residual_map.entry(to).or_default().entry(from).or_default() += supported_capacity;
        }
    }

    FlowResult {
        flow,
        flow_map,
        residual_map,
    }
}
