# graml

graml implements heuristics and other methods for graph coloring.

This project contains a library as well as a binary application making use of the library.

### Running
To see a comparison between different coloring methods run
```
cargo run --release
```

### Documentation
For documentation of the library, execute
```
cargo doc
```
and navigate your browser to ```$repo/target/doc/graml/index.html```

### Testing 
A comprehensive test set is provided. If a new graph implementation is added, consider
adding it to the test suite in ```$repo/src/graph/mod.rs```. To run the tests, execute
```
cargo test --release
```

### Benchmarking 
To see a performance comparison between the different coloring methods as well as graph
implementations, execute
Execute
```
cargo bench
```
and navigate your browser to ```$repo/target/criterion/report/index.html```.
