use crate::lib::read_data;
use std::collections::{HashMap, HashSet};
use std::hash::{Hash, Hasher};
#[derive(Eq, PartialEq, Debug)]
struct Node<'a> {
    connections: HashSet<&'a str>,
    name: &'a str,
    is_big: bool,
    is_start: bool,
    is_end: bool,
}

impl<'a> Hash for Node<'a> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

impl<'a> Node<'a> {
    fn new(name: &'a str) -> Node<'a> {
        Self {
            connections: HashSet::new(),
            name,
            is_big: name.chars().next().unwrap().is_ascii_uppercase(),
            is_start: name == "start",
            is_end: name == "end",
        }
    }
}

pub fn main() {
    let data = read_data();
    run_game(data);
}

fn run_game(data: String) -> usize {
    let mut nodes = init(&data);
    let count = visit_all_neighbours("start", &mut nodes, 2, HashSet::new());

    println!("{count}");
    count
}

fn init(data: &str) -> HashMap<&str, Node> {
    let mut nodes = find_all_nodes(data);
    find_all_neighbours(&mut nodes, data);
    nodes
}
fn visit_all_neighbours<'a>(
    name: &'a str,
    nodes: *mut HashMap<&'a str, Node<'a>>,
    mut part: usize,
    visited_caves: HashSet<&'a str>,
) -> usize {
    unsafe {
        let node = (*nodes).get_mut(name).unwrap() as *mut Node;
        if (*node).is_end {
            return 1;
        }
        if visited_caves.contains(name) {
            if (*node).is_start {
                return 0;
            }
            if !(*node).is_big {
                if part == 1 {
                    return 0;
                } else {
                    part = 1;
                }
            }
        }
        let mut count = 0;
        for neighbour in (*node).connections.iter() {
            let mut new_set = visited_caves.clone();
            new_set.insert(name);
            count += visit_all_neighbours(neighbour, nodes, part, new_set);
        }
        count
    }
}

fn find_all_nodes(data: &str) -> HashMap<&str, Node> {
    let mut res = HashMap::new();

    for line in data.lines() {
        let mut split = line.split("-");
        let first_node_name = split.next().unwrap().trim();
        let second_node_name = split.next().unwrap().trim();
        let first_node = Node::new(first_node_name);
        let second_node = Node::new(second_node_name);
        res.insert(first_node_name, first_node);
        res.insert(second_node_name, second_node);
    }

    res
}

fn find_all_neighbours<'a>(nodes: &mut HashMap<&'a str, Node<'a>>, data: &'a str) {
    for line in data.lines() {
        let mut split = line.split("-");
        let first_node_name = split.next().unwrap().trim();
        let second_node_name = split.next().unwrap().trim();
        let first_node = nodes.get_mut(first_node_name).unwrap();
        first_node.connections.insert(second_node_name);
        let second_node = nodes.get_mut(second_node_name).unwrap();
        second_node.connections.insert(first_node_name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day12::{init, run_game};
    use std::collections::{HashMap, HashSet};
    use std::fs::read_to_string;

    #[test]
    fn run_game_1() {
        let data = "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end".to_string();
        let res = run_game(data);
        assert_eq!(res, 10);
    }

    #[test]
    fn run_game_2() {
        let data =
            "dc-end\nHN-start\nstart-kj\ndc-start\ndc-HN\nLN-dc\nHN-end\nkj-sa\nkj-HN\nkj-dc"
                .to_string();
        let res = run_game(data);
        assert_eq!(res, 19);
    }

    #[test]
    fn test_init_1() {
        let data = read_to_string("day12-simple.txt").unwrap();
        let nodes = init(&data);
        let mut test_nodes = HashMap::new();
        test_nodes.insert(
            "start",
            Node {
                connections: HashSet::from(["A", "b"]),
                name: "start",
                is_big: false,
                is_start: true,
                is_end: false,
            },
        );
        test_nodes.insert(
            "A",
            Node {
                connections: HashSet::from(["start", "b", "c", "end"]),
                name: "A",
                is_big: true,
                is_start: false,
                is_end: false,
            },
        );
        test_nodes.insert(
            "b",
            Node {
                connections: HashSet::from(["start", "A", "d", "end"]),
                name: "b",
                is_big: false,
                is_start: false,
                is_end: false,
            },
        );
        test_nodes.insert(
            "c",
            Node {
                connections: HashSet::from(["A"]),
                name: "c",
                is_big: false,
                is_start: false,
                is_end: false,
            },
        );
        test_nodes.insert(
            "d",
            Node {
                connections: HashSet::from(["b"]),
                name: "d",
                is_big: false,
                is_start: false,
                is_end: false,
            },
        );
        test_nodes.insert(
            "end",
            Node {
                connections: HashSet::from(["A", "b"]),
                name: "end",
                is_big: false,
                is_start: false,
                is_end: true,
            },
        );
        assert_eq!(nodes, test_nodes);
    }
}
