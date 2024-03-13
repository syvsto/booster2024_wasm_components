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
        world: "clustering:rs/cluster": Clustering
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

Before we can compile this project into a component, we need the WIT file! Within `tasks/04/clustering-rs/wit` we have provided the skeleton of the WIT file in `clustering.wit`.

1. Based on the input type of the `run` function and the `world` that is described in the `wit_bindgen::generate!()` clause, write the `clustering.wit` file. Hint: WIT supports composite types, including one named `list`, and various numeric types such as `f64` and `u32`.
2. Compile the project using the build command `cargo component build --release`. Cargo is Rust's build tool, and the `component` subcommand is used for working with WebAssembly components. Specifying `--release` builds in release mode, causing slower build times but a faster binary.
3. Since we are using the Rust component as a library, we need to virtualize it in order to avoid the overlapping WASI import instances that we saw in task 2. The compiled component is found within `target/wasm32-wasi/release/`. Virtualize the component using `wasi-virt`.

## 03.2 The server

The server, found in `tasks/03/handler`, is built in Python, using another WASI module: `wasi:http/proxy`. This module gives you low-level primitives for working with asynchronous HTTP requests and responses. The project is found in `app.py`. The folder also contains a source file containing utility functions for handling asynchronicity called `poller.py`, which you will not need to edit.

The purpose of the handler component is to read JSON specifying the points to cluster and the number of clusters from the body of a request, then call the clustering algorithm and return an array of values where each value is an ID of the cluster the point is in. The input JSON should be structured as the following:

```json
{
    "points": [[1,0], [1,1], [1.5,0.5], [0,0], [4,3.5], [5,4]],
    "n_clusters": 2
}
```

We have implemented most of the required functionality and have implemented the WIT file required (take a peek if you're struggling with task 03.1!), but we're missing the connection to the clustering algorithm.

1. Update `app.py` to support the clustering algorithm. We have already defined a function called `run_cluster` that takes the bytes of the request body and parses it as JSON. This is a good place to perform the clustering algorithm.

Hint: The Python syntax for getting an element from a dictionary is `dictionary['key']`. 

Hint 2: Remember to update the imports so you can use the imported clustering interface within the implementation!

2. Build the component.

## 03.3 Composition

Now that we have two components that should be compatible, we need to compose the components.

1. Using the [WebAssembly Components Builder](https://wasmbuilder.app/), compose the virtualized clustering algorithm component and the handler component, and download it. If you use the Docker container for  building, use the `docker cp` command to copy to and from the container as described in task 2.

In order to try the application, you can run it in wasmtime using:

```bash
wasmtime serve --wasi common composed.wasm ; Note the `serve` command, which starts `wasmtime` in server mode.
```

Where `composed.wasm` is the name of your composed component.

If the server runs successfully, you can try the endpoint using CURL or any other HTTP client. Here's an example CURL request:

```bash
curl -i -H 'content-type: text/plain' --data-binary @- http://127.0.0.1:8080/cluster <<EOF
{ "points": [[1,1],[2,1],[1,1],[4,0]], "n_clusters": 2 }
EOF
```

If your response looks correct: Congratulations, You've built a functioning two-language server!

[Now let's host it and run it on the client!](https://github.com/syvsto/booster2024_wasm_components/blob/master/task4.md)
