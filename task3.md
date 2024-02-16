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
    fn run((points: Vec<Vec<f64>>, n_clusters: u32) -> Vec<u32> { 
        ...
    }
}
```
This is the convention when writing WebAssembly components in Rust, similar to how you declare a class for your world or interface in Python.

Before we can compile this project into a component, we need the WIT file!

1. Create a new directory within `clustering-rs/` called `wit/`, and create a file named `clustering.wit`.
2. Based on the input type of the `categorize` function and the `world` that is described in the `wit_bindgen::generate!()` clause, write the `clustering.wit` file. Hint: WIT supports composite types, including one named `list`.
3. Compile the project using the build command `cargo component build --release`. Cargo is Rust's build tool, and the `component` subcommand is used for working with WebAssembly components. Specifying `--release` builds in release mode, causing slower build times but a faster binary.
4. Since we are using the Rust component as a library, we need to virtualize it in order to avoid the overlapping WASI import instances that we saw in task 2. The compiled component is found within `target/wasm32-wasi/release/`. Virtualize the component.

## 03.2 The server

The server, found in `tasks/03/handler`, is built in Python, using another WASI module: `wasi:http/proxy`. This module gives you low-level primitives for working with asynchronous HTTP requests and responses. The project is found in `app.py`, with supporting functions in `poller.py`. The purpose of the component is to read JSON specifying the points to cluster and the number of clusters from the body of a request, then call the clustering algorithm and return an array of values where each value is an ID of the cluster the point is in. The input JSON should be structured as the following:

```json
{
    "points": [[1,0], [1,1], [1.5,0.5], [0,0], [4,3.5], [5,4]],
    "n_clusters": 2
}
```

We have implemented most of the required functionality, but we're missing the connection to the clustering algorithm.

1. In the `wit/` folder, modify `cluster.wit` so you can call the clustering algorithm of the Rust component.
2. Update `app.py` to support the clustering algorithm as well. Hint: We have already defined a function called `run_cluster` that takes the bytes of the request body and parses it as JSON. This is a good place to perform the clustering algorithm.
3. Build the component.

## 03.3 Composition

Now that we have two components that should be compatible, we need to compose the components.

1. Using the [WebAssembly Components Builder](https://wasmbuilder.app/), compose the virtualized clustering algorithm component and the handler component, and download it.

In order to try the application, you can run it in wasmtime using:

```bash
wasmtime serve --wasi common composed-application.wasm
```

Where `composed-application.wasm` is the name of your composed component.

If the server runs successfully, you can try the endpoint using CURL or any other HTTP client. Here's an example CURL request:

```bash
curl -i -H 'content-type: text/plain' --data-binary @- http://127.0.0.1:8080/cluster <<EOF
{ "points": [[1,1],[2,1],[1,1],[4,0]], "n_clusters": 2 }
EOF
```

If your response looks correct: Congratulations, You've built a functioning two-language server!
