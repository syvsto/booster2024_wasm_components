# 01 Hello, WebAssembly Components!

As is tradition, our first foray into WebAssembly components is building an application for greeting the world. We will build an application called `greeter`, which we will implement in two langauges. For each language, the end goal is a WebAssembly component that when run prints *Hello from `{Language name}`* where `{Language name}` is the name of the programming langauge.

In this task, we will build `greeter` using both Python and Javascript (or Typescript).

## 01.1 Greeter using Python

We start off by building the Python version of `greeter`. Navigate to the `tasks/01/` directory for the Python version of `greeter` (for instance called `greeter-py`), and navigate to it. The folder contains the following files:
* `app.py`: An empty Python source file that will contain the source code of the component.
* `greeter.wit`: Every WebAssembly component requires an interface specification. For the `greeter-py` app, this is contained within `greeter.wit`. 
* `deps/` Since we want to run the program locally, we need support for command line interfaces. This is made available through the WebAssembly System Interface (WASI). The `deps/` folder contains a minimal set of `.wit` files required to build this project with the required WASI files.

We are now ready to begin building!

### 01.1.1 WebAssembly Interface Types

The `greeter.wit` file needs to specify the required inputs and outputs for the function we want to run.

1. Specify the package identifier for the package we are building in the `.wit` file. 
2. We need a world that supports no inputs and no outputs to other components. Write such a world.



WASI has built-in support for CLI applications through what is known as "command" components, which are availble in the `wasi:cli/command` world.


## Greeter using Javascript (or Typescript)
