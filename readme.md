# WASM Components: Bringing down the tower of Babel

Welcome to the code repository for the Booster workshop on WebAssembly Components. This repository contains the tasks to solve, solutions to the tasks (if you want to cheat), as well as setup instructions, links, and scripts.

--- 

## Getting started

To complete the tasks, you need various tooling. We have prepared installation scripts for the tooling so you don't have to download all the tooling manually.

* **MacOS**: Run `./scripts/install-tooling-macos.sh`. Requires [Homebrew](https://brew.sh/).
* **Linux**: Run `./scripts/install-tooling-linux.sh`. Assumes that Node 18 (or later) and Python 3 is already installed and is the default Python implementation.
* **Windows**: Run `.\scripts\install-tooling-win.bat` twice (restarting cmd to update path necessary). Assumes that Node 18 (or later) and Python 3 is already installed and is the default Python implementation.

### Rust

* **[The Rust compiler](https://rustup.rs/)**: The Rust compiler is needed to compile Rust. Duh!
* **[The WebAssembly toolchain](https://rustup.rs/)**: To compile Rust to WebAssembly, you need to install compile targets for WebAssembly. The following targets need to be installed: `wasm32-unknown-unknown` and `wasm32-wasi`.
* **[cargo component](https://github.com/bytecodealliance/cargo-component)**: This tool allows you to convert Rust WebAssembly modules to components.

### Typescript / Javascript

* **[Node/Npm](https://nodejs.org)**: The most common toolchain, package manager and runtime for running Javascript outside the browser
* **[ComponentizeJS](https://github.com/bytecodealliance/ComponentizeJS)**: Tooling to convert Javascript into WebAssembly components.

### Python

* **[Python 3](https://www.python.org/)**: If you want to build WebAssembly components using Python, you'll need the Python interpreter and `pip`, the Python package manager.
* **[componentize-py](https://github.com/bytecodealliance/componentize-py)**: This tool converts Python code into WebAssembly components by embedding the Python runtime within a WebAssembly module.

### Runtime

* **[Wasmtime](https://wasmtime.dev/)**: This is the most established WebAssembly runtime for use outside the browser. 

### Tooling

* **[WASI Virt](https://github.com/bytecodealliance/WASI-Virt)**: This tool lets you virtualize the system interface for a WebAssembly component.

### Deployment

* **[Fermyon Spin](https://www.fermyon.com/spin)**: Fermyon is one of multiple vendors that handle hosting and running WebAssembly components. The `spin` tool allows building and deploying your components as serverless functions in the Fermyon platform.

## The tasks

* **[Task 1: Hello, WebAssembly Components!](https://github.com/syvsto/booster2024_wasm_components/blob/master/task1.md)**: Build your first WebAssembly component using Python, and your second using Javascript!
* **[Task 2: Component composition](https://github.com/syvsto/booster2024_wasm_components/blob/master/task2.md)**: Build another component using Rust, and compose the Javascript, Python and Rust components.
* **[Task 3: Building something useful](https://github.com/syvsto/booster2024_wasm_components/blob/master/task3.md)**: In this task you will make a server application that performs clustering on a dataset built as a WebAssembly component using both Rust and Python.
* **[Task 4: Deployment](https://github.com/syvsto/booster2024_wasm_components/blob/master/task4.md)**: Use the Fermyon Spin environment to host your clustering application, and build the clustering library for use in the browser.
* **[Task 5: Going further](https://github.com/syvsto/booster2024_wasm_components/blob/master/task5.md)**: Make the clustering application support middleware components, and build a logging middleware.
