# graml

graml implements heuristics and other methods for graph coloring.

This project contains a library as well as a binary application making use of the library.

### Building
Install Rust from the [Rust website](https://www.rust-lang.org/)

Clone the repository and execute

```
cargo build --release
```


### Running
To see a comparison between different coloring methods on random graphs, run
```
cargo run --release
```

For evaluating the methods on specific graphs, simply provide them to the binary
like this
```
cargo run --release -- /path/to/graph.col
```

or this
```
cargo run --release -- /path/to/dir/of/graphs
```

to run an evaluation on all the graphs in the directory.

The graphs have to be provided in the DIMACS .col format. See the documentation
for ```load_graph``` for more details. Some compatible graphs can be found
[here](http://www.info.univ-angers.fr/~porumbel/graphs/).


### Documentation
To view documentation of the library, execute
```
cargo doc --open
```

### Testing 
A comprehensive test set is provided. If a new graph implementation is added, consider
adding it to the test suite in ```src/graph/mod.rs```. To run the tests, execute
```
cargo test --release
```

### Benchmarking 
To see a performance comparison between the different coloring methods as well as graph
implementations, execute
```
cargo bench
```
and navigate your browser to ```target/criterion/report/index.html```.
