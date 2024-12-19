mod network_flow;
mod huffman;

use network_flow::{max_flow, NetworkGraph};

fn construct_network_graph() -> NetworkGraph {
    let mut graph = NetworkGraph::new();
    graph.add_edge("s", "a", 7);
    graph.add_edge("s", "c", 4);

    graph.add_edge("a", "b", 4);
    graph.add_edge("a", "d", 2);

    graph.add_edge("b", "e", 3);
    graph.add_edge("b", "c", 2);

    graph.add_edge("c", "e", 2);
    graph.add_edge("c", "g", 3);

    graph.add_edge("d", "f", 4);

    graph.add_edge("e", "f", 5);
    graph.add_edge("e", "g", 3);
    graph.add_edge("e", "t", 4);

    graph.add_edge("f", "t", 7);

    graph.add_edge("g", "t", 3);

    graph.add_node("t");

    graph
}

fn main() {
    let graph = construct_network_graph();

    let result = max_flow(&graph, "s", "t");
    println!("Max flow: {}", result.flow);
    println!("Flow map: {:?}", result.flow_map);
    println!("Residual map: {:?}", result.residual_map);
}
