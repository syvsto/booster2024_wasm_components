# 02 Component composition

One of the strengths of the component model is that you can compose multiple components together. As long as the output of one component matches the input from another component, the two are able to be composed together, creating a new component with the inputs of the first component and the outputs of the second.
In this task, we will build on our outputs from task 1 to make a chain of components that in the end print out "Hello from WASM! and Python! and Javascript! and Python!", using both a Python and a Javascript component.

# 02.1 Rewriting the `greeter` component

In task 1, we developed a component named `greeter`, that displayed a greeting from the programming language it was developed in. This task, we want to make the greeting composable, so we start by duplicating the solution from Task 1 to build upon.

1. Copy your Task 1 solutions into the folder named `/tasks/02`. 

### 02.1.1 Updating the WIT

In order to connect the inputs of one component to the outputs of another, we naturally need a way to specify the inputs of a component in our WIT files. You've already seen how to specify outputs, through the `export` keyword. The dual to `export` is `import`, which naturally specifies an input for the component.

The convention when composing WebAssembly components is that the inputs and outputs that match between components should be specified using an `interface`, a WIT construct for specifying interfaces within a world. This was briefly discussed in Task 1, where we exported from the `wasi:cli/run` interface.

1. Change the `.wit` files of both the Python and the Javascript `greeter` versions so that you are able to compose arbitrary `greeter` components together through the `greeter` interface. For now, don't worry about printing to the terminal (meaning you don't need the `export wasi:cli/run@0.2.0;` line anymore). The interface to use is already provided in the WIT file found in `tasks/02/start.wit`. 

### 02.1.2 Updating the implementations

The Python and Javascript source code also needs to be updated to match the changes to the WIT world.

1. Update the Python source so that the new exported interface from is implementend, and so that the function within it returns string "Hello from Python!". 
2. Update the function you just wrote so that it returns the import from the new interface to concatenate any previous input with the string " and Python!". 

To use imported WIT interfaces in Python, you use something akin to the following:

```python
from worldName import imports
from worldName.import import interfaceName
```

This allows you to call the functions declared 

3. Change the Javascript to do the same, but with the string " and Javascript!".

In Javascript, exported interfaces are implemented as an object in the following manner:

```javascript
export const interfaceName = {
        fieldName: ...
}
```

Imported interfaces are available through:

```javascript
import { interfaceName } from "packageName/worldName";
```

When everything is built, we are ready to compose! This can be done using the `wasm-tools` CLI application, but there is also a very handy online tool called the [WebAssembly Components Builder](https://wasmbuilder.app/) which lets you compose components using a node-based graphical editor:

1. Open the [WebAssembly Components Builder](https://wasmbuilder.app/) website and add the two `greeter` components, as well as the `starter.wasm` component available in `tasks/02/starter.wasm`.
2. Drag and drop one instance of each of the Javascript `greeter` and `starter` components, and two instances of the Python `greeter` components into the editor.
3. Connect the exported interfaces. 

This is all that's needed to compose components. However, if you want to export the interface from our composed component, you need to click the small checkbox in the top left corner of the checkbox. We want to use the exported greeting function in further compositions, so we need to select the final component in the chain as the exported outputs in our composed component.

4. Select the final component as our exported interface and download the component.

## 02.2 Virtualization

We still need a way to display the greeting. In task 1, we discussed command components:

1. Write a new command component that can read the output of your composed component and print it to the screen. You can copy the `deps/` folder from one of the existing `greeter` components to get access to the required WASI modules.
2. Add your composite component from before and the new command component to the WebAssembly Components Builder. If you want to clear the previous components, just refresh the page. 

You may have noticed that the editor in the WebAssembly Components Builder shows a bunch of WASI imports for both the composite component and the command component. Both `componentize-py` and other WebAssembly component compilers such as Rust's `cargo component` (which we'll use in the next task) automatically add WASI imports so you are able to use the entire WASI interface when building the component. However, when composing components this automatic insertion of the WASI dependencies becomes a problem. In fact, if you try to Download Component now, you will get an error akin to this:

```
cannot import instance with name `wasi:io/error@0.2.0` for an instantiation argument of component `composite` because it conflicts with an imported instantiation argument of component `command-py`.
```

To alleviate this problem of conflicting imports, we can virtualize the WASI imports using [WASI-virt](https://github.com/bytecodealliance/WASI-Virt). We can't virtualize the command component as we want to access the runtime in order to print to the terminal there, but the composite component just returns a string, so we should be fine virtualizing almost everything there.

2. Virtualize the WASI imports of the composite component using `wasi-virt composite-component.wasm -o composite-component-virt.wasm --allow-env --allow-random`. The two arguments `--allow-env` and `--allow-random` are required by the Python runtime within the component to allow Python to fetch environment information and the random number generator from the WebAssembly runtime.
3. Re-upload the virtualized component to [WebAssembly Components Builder](https://wasmbuilder.app/) and connect it to the command component like before (you can clear the previous uploads by refreshing). You should now be able to download the component.

## 02.3 Finishing touches

We now have a single binary built using both Python and Javascript that prints a nice greeting to the terminal. The only thing left to do is to try it out:

1. Run your composed component using `wasmtime`.
