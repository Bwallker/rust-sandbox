mod dijkstra;

use dijkstra::{dijkstra, Graph};

fn construct_graph() -> Graph {
    let mut graph = Graph::new();
    graph.add_edge("a", "b", 2);
    graph.add_edge("a", "e", 3);
    graph.add_edge("a", "f", 7);

    graph.add_edge("b", "c", 2);

    graph.add_edge("c", "h", 4);

    graph.add_edge("d", "a", 1);
    graph.add_edge("d", "j", 6);

    graph.add_edge("e", "d", 4);
    graph.add_edge("e", "f", 3);
    graph.add_edge("e", "j", 2);

    graph.add_edge("f", "c", 2);
    graph.add_edge("f", "g", 4);
    graph.add_edge("f", "m", 3);

    graph.add_edge("g", "c", 1);
    graph.add_edge("g", "h", 1);

    // H has no outgoing edges.
    graph.add_node("h");

    // Node I does not exist.

    graph.add_edge("j", "f", 4);
    graph.add_edge("j", "k", 6);

    graph.add_edge("k", "f", 2);
    graph.add_edge("k", "m", 6);

    // Node L does not exist.

    graph.add_edge("m", "g", 1);
    graph.add_edge("m", "h", 4);

    graph
}

fn main() {
    let graph = construct_graph();
    let map = dijkstra(&graph, "a");
    for (node, distance) in map {
        println!("{}: {}", node, distance);
    }
}
