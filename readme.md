# WASM Components: Bringing down the tower of Babel

Welcome to the code repository for the Booster workshop on WebAssembly Components. This repository contains the tasks to solve as well as setup instructions, links, and scripts.

--- 

## Getting started

To complete the tasks, you need various tooling. The easiest way to get started is using a Docker image we have prepared:

 * If you don't already have it installed, install [Docker Desktop](https://www.docker.com/products/docker-desktop/)
 * Use `docker pull syvsto/booster2024wasm` to pull the image with all the required tooling for the workshop.
 * Run the image using `docker run -it syvsto/booster2024wasm:latest`.

Now that you have the image up and running, you have some options for how to develop in it:

 * If you use Visual Studio Code, the Dev Containers extension is great for working with the Docker image. Visit the `Remote Explorer` pane in the sidebar, and your running image should show up in the list.
 * Otherwise, feel free to install Vim or any other terminal editor you're comfortable with inside the image. The image is based on Ubuntu, so `apt install vim` inside the image should get you going.

The image contains the task repository, which is found in `/booster2024_wasm_components`.

### Other install options

If you want to run everything locally, we have prepared some installation scripts for the tooling so you don't have to download everything manually.

* **MacOS**: Run `./scripts/install-tooling-macos.sh`. Requires [Homebrew](https://brew.sh/).
* **Linux**: Run `./scripts/install-tooling-linux.sh`. Assumes that Node 18 (or later) and Python 3 is already installed and is the default Python implementation.
* **Windows**: Run `.\scripts\install-tooling-win.bat` Assumes the linux dependencies in addition to [Rustup](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe).

#### Rust

* **[The Rust compiler](https://rustup.rs/)**: The Rust compiler is needed to compile Rust. Duh!
* **[The WebAssembly toolchain](https://rustup.rs/)**: To compile Rust to WebAssembly, you need to install compile targets for WebAssembly. The following targets need to be installed: `wasm32-unknown-unknown` and `wasm32-wasi`.
* **[cargo component](https://github.com/bytecodealliance/cargo-component)**: This tool allows you to compile Rust to WebAssembly components.
* **[wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)**: This tool allows you to compile Rust to WebAssembly modules that can be used in the browser, and generates bindings to Javascript.

#### Typescript / Javascript

* **[Node/Npm](https://nodejs.org)**: The most common toolchain, package manager and runtime for running Javascript outside the browser
* **[ComponentizeJS](https://github.com/bytecodealliance/ComponentizeJS)**: Tooling to convert Javascript into WebAssembly components.

#### Python

* **[Python 3](https://www.python.org/)**: If you want to build WebAssembly components using Python, you'll need the Python interpreter and `pip`, the Python package manager.
* **[componentize-py](https://github.com/bytecodealliance/componentize-py)**: This tool converts Python code into WebAssembly components by embedding the Python runtime within a WebAssembly module.

#### Runtime

* **[Wasmtime](https://wasmtime.dev/)**: This is the most established WebAssembly runtime for use outside the browser. 

#### Tooling

* **[WASI Virt](https://github.com/bytecodealliance/WASI-Virt)**: This tool lets you virtualize the system interface for a WebAssembly component.

#### Deployment

* **[Fermyon Spin](https://www.fermyon.com/spin)**: Fermyon is one of multiple vendors that handle hosting and running WebAssembly components. The `spin` tool allows building and deploying your components as serverless functions in the Fermyon platform.

## The tasks

* **[Task 1: Hello, WebAssembly Components!](https://github.com/syvsto/booster2024_wasm_components/blob/master/task1.md)**: Build your first WebAssembly component using Python, and your second using Javascript!
* **[Task 2: Component composition](https://github.com/syvsto/booster2024_wasm_components/blob/master/task2.md)**: Build another component using Rust, and compose the Javascript, Python and Rust components.
* **[Task 3: Building something useful](https://github.com/syvsto/booster2024_wasm_components/blob/master/task3.md)**: In this task you will make a server application that performs clustering on a dataset built as a WebAssembly component using both Rust and Python.
* **[Task 4: Deployment](https://github.com/syvsto/booster2024_wasm_components/blob/master/task4.md)**: Use the Fermyon Spin environment to host your clustering application, and build the clustering library for use in the browser.
* **[Task 5: Going further](https://github.com/syvsto/booster2024_wasm_components/blob/master/task5.md)**: Make the clustering application support middleware components, and build a logging middleware.


**To get started, head over to [task 1](https://github.com/syvsto/booster2024_wasm_components/blob/master/task1.md)!**
