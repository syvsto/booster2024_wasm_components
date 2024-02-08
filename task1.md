# 01 Hello, WebAssembly Components!

As is tradition, our first foray into WebAssembly components is building an application for greeting the world. We will build an application called `greeter`, which we will implement in two langauges. For each language, the end goal is a WebAssembly component that when run prints *Hello from `{Language name}`* where `{Language name}` is the name of the programming langauge.

In this task, we will build `greeter` using both Python and Javascript (or Typescript).

## 01.1 Greeter using Python

We start off by building the Python version of `greeter`. Navigate to the `tasks/01/` directory for the Python version of `greeter` (for instance called `greeter-py`), and navigate to it. The folder contains the following files:
* `app.py`: An empty Python source file that will contain the source code of the component.
* `greeter.wit`: Every WebAssembly component requires a contract with the outside world in the form of an interface. For the `greeter-py` app, this is contained within `greeter.wit`. 
* `deps/` Since we want to run the program locally, we need support for command line interfaces. This is made available through the WebAssembly System Interface (WASI). The `deps/` folder contains a minimal set of `.wit` files required to build this project with the required WASI files.

We are now ready to begin building!

### 01.1.1 WebAssembly Interface Types

The `greeter.wit` file contains the imports and exports that allow this WebAssembly component to be used externally. Every `.wit` contract needs two things, a `package` declaration (at the top of the file), and the declaration of a `world`.

1. Specify the package identifier for the package we are building in the `.wit` file. 
2. We need a world that supports no inputs and no outputs to other components, i.e. it should be empty. Write such a world.

Our end goal with `greeter` is to print a line of text to the screen, which means we need to interface with the world outside the WebAssembly runtime. For this, we can use the WebAssembly System Interface (WASI). WASI has built-in support for CLI applications through what is known as "command" components. A command component is a component which only exports the `wasi:cli/run` interface.

3. Export the `wasi:cli/run` interface in `greeter.wit`.

You should now have a completed contract for a component that can print to the terminal!

### 01.1.2 The Python code

When writing code for a component, there are certain languague-specific conventions you have to follow. For Python, you type in an import that corresponds to the world you use. If you named your world `greeter` in the previous excercise, you would start your file off like this: 

```python
import greeter
```

Then, you must implement  the interface specified within the `greeter` world. In Python, you implement worlds and interfaces as classes, and any exported functions as methods on that class. The convention used is that interface names and method names must match their counterparts in the `.wit` files, but any `kebab-case` identifier in the `.wit` file becomes either `camelCase` if it's a function name or `PascalCase` if it's an interface or world. 

 Since we've specified that we're using the `wasi:cli/run` interface, we need to implement it. The `deps/` folder contains the interface for the run function.  If you've implemented the interface correctly, you can use the `print()` function in Python to print to the terminal.

 1. Find the correct interface in `deps/`, and implement a function that prints "Hello from Python!".


### 01.1.3 Running the component

You can now build the component using:

```sh
componentize-py -d . -w greeter componentize app -o greeter.wasm
```

`-d` sets the root folder we use for our `.wit` files, `-w` sets the world we use, and `-o` sets the output file name.

Now run it using:

```sh
wasmtime -S common greeter.wasm 
```

`-S` specifies that Wasmtime should run using WASI, with the `common` set of WASI modules.

If you've done everything right, your terminal should say "Hello from Python!".


## 01.2 Greeter using Javascript

Next, we build the same component using Javascript.

### 01.2.1 Setup

To compile Javascript to WebAssembly, we use the `componentize-js` package. It differs from `componentize-py` by being a library instead of a CLI application, and therefore needs to be installed into the project where it is used.

1. Install `componentize-js` using `npm install @bytecodealliance/componentize-js`.

Using `componentize-js` requires making a build script. We've provided one in `build.mjs`.

### 01.2.2 The contract

Next up, we need an interface for our component. The Javascript component is going to do the same thing as the Python component.

1. Write the `greeter.wit` file required for the Javascript component.


TODO: Wait for proper WASI support in componentize-js
