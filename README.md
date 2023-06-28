# **Fe**n**O**type

The name is an unholy amalgam of:
* FeO - Iron Oxide - Rust  :crab:  
* Phenotype - the set of observable characteristics of an individual resulting from the interaction of its genotype with the environment.

Various graph algorithms.  

*NOTE:* I'm using this as an opportunity to learn Rust.

---

### Usage

```rs
let path = Path::new("./data/karate.tsv");
let graph = Graph::from_file(path, false);

let node: i64 = 0;
println!("Node {} has {:?} neighbors", node, graph.neighbors(node));
```