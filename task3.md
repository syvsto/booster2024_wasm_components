# 03 Building something useful

Now that you know how to write, compile, and compose WebAssembly components, we can start building something useful. For this task, we are switching out Javascript for Rust, which has the most mature tooling of any language in WebAssembly, perhaps aside from C or C++.

We are going to build a WebAssembly component for clustering data. The clustering algorithm will be built in Rust, and will be composed with another component that will expose the algorithm as an endpoint in an HTTP server written in Python.

## 03.1 The clustering algorithm

Navigate to the `tasks/03/clustering-rs` folder. This is a Rust project using the [linfa](https://github.com/rust-ml/linfa) machine learning framework, which implements an algorithm for clustering. We have already implemented a function that performs the clustering in `src/lib.rs`. 

In Rust, the [wit-bindgen](https://github.com/bytecodealliance/wit-bindgen) library is used to specify how to interface with the WIT file for this component, and is used with the `generate!` macro:

```rust
wit_bindgen::generate!({
    world: "clustering",

    exports: {
        world: Clustering
    }
})
```

This creates Rust bindings for a WIT file with a world named `clustering`, in a similar manner to how `componentize-py` creates bindings between the WIT and Python when building a component.

For those familiar with Rust, you might balk a bit at at this section:

```rust
struct Clustering;
impl Guest for Clustering {
    fn categorize((points: Vec<Vec<f64>>, n_clusters: u8) -> Vec<u8> { 
        ...
    }
}
```
This is the convention when writing WebAssembly components in Rust, similar to how you declare a class for your world or interface in Python.

Before we can compile this project into a component, we need the WIT file!

1. Create a new directory within `clustering-rs/` called `wit/`, and create a file named `clustering.wit`.
2. Based on the input type of the `categorize` function and the `world` that is described in the `wit_bindgen::generate!()` clause, write the `clustering.wit` file. Hint: WIT supports composite types, including one named `list`.
3. Compile the project using the build command `cargo component build --release`. Cargo is Rust's build tool, and the `component` subcommand is used for working with WebAssembly components. Specifying `--release` builds in release mode, causing slower build times but a faster binary.

## 03.2 The server

The server is built in Python, 

TODO: Need some Python source code to build on.
