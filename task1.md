# 01 Hello, WebAssembly Components!

As is tradition, our first foray into WebAssembly components is building an application for greeting the world. We will
build an application called `greeter`, which we will implement in two langauges. For each language, the end goal is a
WebAssembly component that when run prints *Hello from `{Language name}`* where `{Language name}` is the name of the
programming langauge.

In this task, we will build `greeter` using both Python and Javascript (or Typescript).

## 01.1 Greeter using Python

We start off by building the Python version of `greeter`. Navigate to the `tasks/01/` directory for the Python version
of `greeter` (for instance called `greeter-py`), and navigate to it. The folder contains the following files:

- `app.py`: An empty Python source file that will contain the source code of the component.
- `greeter.wit`: Every WebAssembly component requires a contract with the outside world in the form of an interface. For
  the `greeter-py` app, this is contained within `greeter.wit`.
- `deps/` Since we want to run the program locally, we need support for command line interfaces. This is made available
  through the WebAssembly System Interface (WASI). The `deps/` folder contains a minimal set of `.wit` files required to
  build this project with the required WASI files.

We are now ready to begin building!

### 01.1.1 WebAssembly Interface Types

The `greeter.wit` file contains the imports and exports that allow this WebAssembly component to be used externally.
Every `.wit` contract needs two things, a `package` declaration (at the top of the file), and the declaration of a
`world`. 

1. We have provided an basic world for our Python application in `tasks/01/greeter-py/greeter.wit`. Take a look at the file to familiarize yourself with WIT syntax.

Our end goal with `greeter` is to print a line of text to the screen, which means we need to interface with the world
outside the WebAssembly runtime. For this, we can use the WebAssembly System Interface (WASI). Did you notice the `deps/` folder within the `greeter-py` folder? It contains interface definitions for the parts of WASI we need. There is nothing special about the WASI `.wit` files, but WebAssembly runtimes that support WASI know to perform specific actions if a component implements things from the WASI interfaces.
For example, exporting the `wasi:cli/run` interface lets a component act as a CLI application (it becomes what is called a "command" component) that can read and write to the terminal. 

2. Take a look at `deps/cli/run.wit` within `greeter-py`, and make note of the interface name and singular function, named `run`.

The WIT file tells us that in order to implement the `wasi:cli/run` interface, we need to implement a `run` function in the application code. For the time being, let's make sure we specify that we use the `run` interface in our component so that the WebAssembly runtime can run it as a CLI application that prints a message to the screen:

3. Export `wasi:cli/run@0.2.0` in our world in `greeter.wit`.

You should now have a completed WIT file for a component that can print to the terminal!

### 01.1.2 The Python code

When writing code for a component, there are certain languague-specific conventions you have to follow, that often correspond to how you would call foreign functions. For Python, you
need an import that corresponds to the world you want to use from your WIT file. If you named your world `greeter` in the previous excercise,
you would start your file off like this:

```python
import greeter
```

Then, you must implement the interface specified within the `greeter` world. In Python, you implement worlds and
interfaces as classes, and any exported functions as methods on that class. The convention used is that interface names
and method names must match their counterparts in the `.wit` files, but any `kebab-case` identifier in the `.wit` file
becomes either `camelCase` if it's a function name or `PascalCase` if it's an interface or world.

For example, if you have an interface named `foo` that contains a function named `bar`, the python code would look like this:

```python
class Foo:
    def bar(self):
       ...
```

Let us now implement the WASI `run` interface we need to turn our component into a CLI application.
Remember the structure of the `run.wit` file from task 01.1.1? This is the interface we need to implement. Reminder: The `deps/` folder
contains the interface for the run function within the `run.wit` function. 

If you've implemented the interface correctly, the WebAssembly runtime will use the `run` function similarly to a `main` function in langauges like Java or C++, and you are able to use the Python `print()`
function to print to the terminal.

1. Implement the `run()` function from the `run` interface so that it prints "Hello from Python!".

### 01.1.3 Running the component

You can now build the component using:

```sh
componentize-py -d . -w greeter componentize app -o greeter.wasm
```

Let's break this down.
 * `-d` sets the root folder we use for our `.wit` files. We need to set it to the current folder, since we want to use both the `greeter.wit` file that specifies our current component, and the dependencies from the WASI dependencies. 
 * `-w` sets the world within the WIT files we want to use. The world for our component is named `greeter`.
 * `componentize app` specifies that we want to build `app.py` into a component.
 * `-o` sets the output file name of the component we want to build.

If you don't get any compilation errors, you can run the `greeter.wasm` component using:

```sh
wasmtime -S common greeter.wasm 
```

`-S` specifies that Wasmtime should run using WASI, with the `common` set of WASI modules. This includes `wasi:cli/run`, which we need to support to print to the screen.
 
If you've done everything right, your terminal should say "Hello from Python!".

## 01.2 Greeter using Javascript

Next, we build the same component using Javascript.

### 01.2.1 Setup

1. Navigate to `tasks/01/greeter-js`. This is the project directory of the Javascript project.

To compile Javascript as a WebAssembly component, we use the `componentize-js` package. It differs from `componentize-py` by being a
library instead of a CLI application, and therefore needs to be installed into the project where it is used.

2. JavaScript projects commonly use `npm` to manage dependencies. If you have navigated to the `greeter-js` project, install the `componentize-js` library into our project using `npm install @bytecodealliance/componentize-js`.

Using `componentize-js` requires that you provide a build script that specifies which files we want to build, where our WIT files are located, etc. We've provided a build script in `build.mjs`.

### 01.2.2 The contract

Next up, we need an interface for our component. As with the Python component, the Javascript component is also going to print to the screen. As for the Python project, we have provided the needed WASI interfaces in `deps/`. 

1. Write the `greeter.wit` file required for the Javascript component. Hint: We want to do exactly the same thing as the Python component. 

## 01.2.3 The implementation

We can now write the required implementation. The convention in Javascript is that any WIT exports are implemented as a function in an object field, with the name of the function to export as the key and the function as the value. Interfaces are specified as a variable that contains an object. This variable must be `export`ed.

For example:

```javascript
export const foo = {
  bar: function() { ... }
};
```

2. Write the necessary Javascript within the `app.js` file.

## 01.2.3 Building and running

We are now ready to build and run the Javascript application!

1. Run `node build.mjs`. This uses `node` to run the Javascript build script locally. 

If everything builds correctly, you should have a `greeter.wasm` file.

2. Run the `greeter.wasm` file using Wasmtime. Remember to enable the WASI capabilities of Wasmtime using the `-S common` flag!

[Onward!](https://github.com/syvsto/booster2024_wasm_components/blob/master/task2.md)
