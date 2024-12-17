# rust-de Bruijn graphs
 - construction of de Bruijn graphs using BTreeMaps.
 - gives you a binary search implementation by in the end encoding the MapStore and the string as a BtreeMap, which can be easily used into the Binary Heap as a priority queue. 
 - uses graphviz for writing the dot files: ![graphviz](https://www.graphviz.org/)
 - writes colour coded dot files for visualization with graphviz crate ![graphviz crate](https://www.cs.brandeis.edu/~cs146a/rust/doc-02-21-2015/graphviz/index.html) and [graphviz_rust](https://docs.rs/graphviz-rust/latest/graphviz_rust/)
 - general note: Incase of Golang and RUST, please see the last commit message and if it says compiled binary then it is completed or else still in development version.
 

 ```
 cargo build 

 ```

 - to build he binary and build the hashes
 ```
  ➜ gauravsablok  rust-de-Bruijn-graphs git:(main) ✗ ./target/debug/rust-de-Bruijn-graphs -h
Usage: rust-de-Bruijn-graphs <DEBRUIJN_ARG> <KMER_ARG>

Arguments:
  <DEBRUIJN_ARG>  please provide the path to the fastq file
  <KMER_ARG>      please provide the kmer for the graph construction

Options:
  -h, --help     Print help
  -V, --version  Print version
```
```
./target/debug/rust-de-Bruijn-graphs ./sample-files/sample.fasta 3
```
```
"ATATATGATAGATAGATGATGATGATAGATGATAGATGATGATGATAGTAGATGAT"      
MapStore { node_edge_relation: [NodeEdgeHold { node: "AT", edge: "TA" }, 
NodeEdgeHold { node: "TA", edge: "AT" }, NodeEdgeHold { node: "AT", edge: "TA" }, NodeEdgeHold { node: "TA", edge: "AT" }, NodeEdgeHold { node: "AT", edge: "TG" }, NodeEdgeHold { node: "TG", edge: "GA" }, NodeEdgeHold { node: "GA", edge: "AT" }, NodeEdgeHold { node: "AT", edge: "TA" }, NodeEdgeHold { node: "TA", edge: "AG" }, NodeEdgeHold { node: "AG", edge: "GA" }, 
NodeEdgeHold { node: "GA", edge: "AT" },
```

 Gaurav Sablok 
