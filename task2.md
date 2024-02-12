# 02 Component composition

One of the strengths of the component model is that you can compose multiple components together. As long as the output of one component matches the input from another component, the two are able to be composed together, creating a new component with the inputs of the first component and the outputs of the second.
In this task, we will build on our outputs from task 1 to make a chain of components that in the end print out "Hello from WASM! and Python! and Javascript! and Python!", using both a Python and a Javascript component.

# 02.1 Rewriting the `greeter` component

In task 1, we developed a component named `greeter`, that displayed a greeting from the programming language it was developed in. This task, we want to make the greeting composable, so we start by duplicating the solution from Task 1 to build upon.

1. Copy your Task 1 solutions into the folder named `/tasks/02`. 

In order to connect the inputs of one component to the outputs of another, we naturally need a way to specify the inputs of a component in our WIT files. You've already seen how to specify outputs, through the `export` keyword. The dual to `export` is `import`, which naturally specifies an input for the component.

2. Change the `.wit` files of both the Python and the Javascript `greeter` versions so that you are able to compose arbitrary `greeter` components together. For now, don't worry about printing to the terminal (meaning you don't need the `export wasi:cli/run@0.2.0;` line anymore). You will need to specify a function, which is typed using `func(argument: argtype) -> returntype`. `argtype` and `returntype` can be any supported type in WIT, for instance `u8`, `f64` or `string`. 

The convention when composing WebAssembly components is that the inputs and outputs that match between components should be specified using an `interface`, a WIT construct for specifying interfaces within a world. This was briefly discussed in Task 1, we exported from the `wasi:cli/run` interface.

3. Change the `.wit` files again so that you don't import and export functions directly, but import and export an interface instead. Hint: Take a look at how the minimal WASI bindings in the `deps` folder to get an idea of how interfaces are written.

# 02.1.1 Updating the implementations

The Python and Javascript source code also needs to be updated to match the changes to the WIT world.

1. Update the Python source so that the new exported interface from is implementend, and so that the function within it returns string "Hello from Python!". 
2. Update the function you just wrote so that it returns the import from the new interface to concatenate any previous input with the string " and Python!". The WIT should already be imported within the `app.py` file, so you only need to 
3. Change the Javascript to do the same, but with the string " and Javascript!".

We are now ready to compose! This can be done using the `wasm-tools` CLI application, but there is also a very handy online tool called the [WebAssembly Components Builder](https://wasmbuilder.app/) which lets you compose components using a node-based graphical editor:

1. Open the [WebAssembly Components Builder](https://wasmbuilder.app/) website and add the two `greeter` components.
2. Drag and drop one instance of the Javascript `greeter` and two instances of the Python `greeter` components into the editor, and connect the exported interfaces. 

This is (in theory) all that's needed to compose components. However, if you want to export the interface from any instance, you need to click the small checkbox in the top left corner of the checkbox. Since we only export an interface that returns a string and haven't yet added the `wasi:cli/run` interface back, we need to select the final component in the chain as the exported outputs in our composed component.

## 02.2 Virtualization

You may have noticed that the editor in the WebAssembly Components Builder shows a bunch of WASI imports. In fact, if you try to Download Component now, you will get an error about a mismatch between multiple instances of a `wasi` import. Both `componentize-py` and other WebAssembly component compilers such as Rust's `cargo component` (which we'll use in the next task) automatically add these imports so you are able to run the component within a runtime such as Wasmtime and use the WASI interface. However, it's not helpful for component composition. To alleviate this problem, we can virtualize these dependencies using [WASI-virt](https://github.com/bytecodealliance/WASI-Virt). 

1. Virtualize the `greeter` components using `wasi-virt greeter.wasm -o greeter-virtualized.wasm`.
2. Re-upload the virtualized components to [WebAssembly Components Builder](https://wasmbuilder.app/) and connect them like before (you can clear the previous uploads by refreshing). You should now be able to download the component.

## 02.3 Finishing touches

We now have a single binary built using both Python and Javascript that returns the string " and Python! and Javascript! and Python!". We are still missing two parts: The initial "Hello from WASM!" and a way to print to terminal. You have already used all the functionality needed to do the following:

1. Write a new component that takes no inputs and outputs the string "Hello from WASM!" in a way that's compatible with the previously composed component.
2. Write a new command component that can read the output of your composed component and print it to the screen. You can copy the `deps/` folder from one of the existing `greeter` components to get access to the required WASI modules.
3. Compose everything and run it using `wasmtime`.


