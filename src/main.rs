use fenotype::graph::Graph;
use fenotype::graph::GraphBuilder;
use std::path::Path;


fn main() {

    let path = Path::new("./data/karate.tsv");
    let graph = Graph::from_file(path, false);

    println!("Node {} has {:?} neighbors", 0, graph.neighbors(0));

}
