# mdbook_simulted_annealing_in_rust

To compile this book you need the Rust compiler and install the mdbook-compile-output carate in this repo as well as the mdbook and mdbook-katex crates:

```
git pull stela2502/mdbook_simulted_annealing_in_rust
cargo install mdbook
cargo install mdbook-katex
cargo install --path mdbook_simulted_annealing_in_rust/mdbook-complie-output
```

Afterwards you can compile and serve the book locally using

```
cd mdbook_simulted_annealing_in_rust
mdbook build
mdbook serve
```

You can see the compiled version of the tutorial [here](https://stela2502.github.io/mdbook_simulted_annealing_in_rust/).
