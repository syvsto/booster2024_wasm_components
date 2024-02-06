# WASM Components: Bringing down the tower of Babel

Welcome to the code repository for the Booster workshop on WebAssembly Components. This repository contains the tasks to solve, solutions to the tasks (if you want to cheat), as well as setup instructions, links, and scripts.

--- 

## Getting started

To complete the tasks, you need various tooling. We have prepared installation scripts for the tooling so you don't have to download all the tooling manually.

* **MacOS**: Run `./install-tooling-macos.sh`. Requires [Homebrew](https://brew.sh/).
* **Linux**: Run `./install-tooling-linux.sh`.
* **Windows**: Run `./install-tooling-win.bat`.

### Rust

* **The Rust compiler**: 
* **The WebAssembly toolchain**:
* **cargo component**:

### Python

* **Python 3**:
* **componentize-py**:

### Runtime

* **Wasmtime**: 

### Tooling

* **WASI Virt**: 

### Deployment

* **Fermyon Spin**: 



## The tasks

* **[Task 1: Hello, WebAssembly Components!](https://github.com/syvsto/booster2024_wasm_components/blob/master/task1.md)**: Build your first WebAssembly component using Python!
* **[Task 2: Component composition](https://github.com/syvsto/booster2024_wasm_components/blob/master/task2.md)**: Build another component using Rust, and compose Python and Rust components.
* **[Task 3: Building something useful](https://github.com/syvsto/booster2024_wasm_components/blob/master/task3.md)**: In this task you will make a useful WebAssembly Component using both Rust and Python, which contains a clustering algorithm within an HTTP server.
* **[Task 4: Deployment](https://github.com/syvsto/booster2024_wasm_components/blob/master/task4.md)**: Use the Fermyon Spin environment to host your clustering application.
* **[Task 5: Going further](https://github.com/syvsto/booster2024_wasm_components/blob/master/task5.md)**: Make the clustering application support middleware components, and build a logging middleware.
