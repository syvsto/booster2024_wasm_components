# 02 Component composition

One of the strengths of the component model is that you can compose multiple components together. As long as the output of one component matches the input from another component, the two are able to be composed together, creating a new component with the inputs of the first component and the outputs of the second.
In this task, we will build on the Javascript and Python `greeter` components to make a chain of components that in the end print out "Hello from WASM! and Python! and Javascript! and Python!", using both a Python and a Javascript component.

Within the `tasks/02` folder, you will find the following:

* `greeter-py/`: A directory containing a Python implementation of the greeter component that is composable.
* `greeter-js/`: A directory containing a Javascript implementation of the greeter component that is composable.
* `starter.wasm`: A simple component that exports a function returning the string "Hello from WASM!".
* `starter.wit`: The WIT file for the `starter.wasm` component.
* `command.wasm`: A simple component that imports a function returning a string and prints it to the terminal.

We will use these to compose a new component that prints out the greeting we want.

# 02.1 Rewriting the `greeter` component

In task 1, we developed a component named `greeter`, that displayed a greeting from the programming language it was developed in. This task, we want to make the greeting composable. We have provided a solution to task 1 in this directory, found in the subdirectories `greeter-py` and `greeter-js`.

### 02.1.1 Updating the WIT

In order to connect the inputs of one component to the outputs of another, we naturally need a way to specify the inputs of a component in our WIT files. You've already seen how to specify outputs, through the `export` keyword. The dual to `export` is `import`, which naturally specifies an input for the component.

The convention when composing WebAssembly components is that the inputs and outputs that match between components should be specified using an `interface`, a WIT construct for specifying interfaces within a world. Here's an example of an interface definition with two fields, one for a string and one for a function that returns a `u32` value:

```wit
interface interface-name {
    field: string;
    field2: func() -> u32;
}
```

We want to specify an interface for our `greeter` component so that we conform to the interface specified in the `starter` WIT file, which we will use as the starting point for our composition.

1. Change the `.wit` files of both the Python and the Javascript `greeter` projects so that you are able to compose arbitrary `greeter` components together through the `greeter` interface. This should be done in a manner that makes us conform to the interface specified in `starter.wit`.

Hint: We have separated the side effect of printing to the terminal into the pre-defined `command.wasm` component, meaning you don't need the `export wasi:cli/run@0.2.0;` line anymore. However, you will need to both import and export the same interface, so that the component can be composed with itself.

We have already implemented the required Python and Javascript code for the components, so if your WIT files are specified correctly you should now be ready to: 

2. Build both components.

We now (hopefully!) have two components that are composable! However, there is one more step that needs to be done before we can actually compose the components.


## 02.2 Virtualization

Both `componentize-py` and ComponentizeJS automatically add WASI imports so you are able to use the entire WASI interface when building the component. However, when composing components this automatic insertion of the WASI dependencies becomes a problem. When composing multiple components, you may get dependency conflicts for WASI dependencies. This happens for certain WASI subsystems when it is not possible to determine which of the components in a composition should receive the instance. In fact, if we were to compose the components we already have built using the composition tool we're going to use, we would get something along the lines of the following error. 

```
cannot import instance with name `wasi:io/error@0.2.0` for an instantiation argument of component `composite` because it conflicts with an imported instantiation argument of component `command-py`.
```

To alleviate this problem of conflicting imports, we can virtualize the WASI imports using [WASI-virt](https://github.com/bytecodealliance/WASI-Virt). Since ComponentizeJS doesn't automatically add WASI imports, we only need to perform the operation for the Python based greeter component.

1. Virtualize the WASI imports of the Python based component using `wasi-virt greeter-py.wasm -o greeter-py-virt.wasm --allow-env --allow-random`. This will create a new component that  The two arguments `--allow-env` and `--allow-random` are required by the Python runtime within the component to allow Python to fetch environment information and the random number generator from the WebAssembly runtime.
1. Do the same for the Javascript component.


## 02.3 Composition

When everything is built and virtualized, we are ready to compose! This can be done using the `wasm-tools` CLI application, but there is also a very handy online tool called the [WebAssembly Components Builder](https://wasmbuilder.app/) which lets you compose components using a node-based graphical editor. We have provided two helper components to start and end the composition. The `starter` component contains a function that returns "Hello from WASM!", and the `command` component takes in the entire string that we want to print and prints it to the screen.

1. Open the [WebAssembly Components Builder](https://wasmbuilder.app/) website and add the two `greeter` components, as well as the `starter.wasm` and `command.wasm` components available in `tasks/02/starter.wasm`.

Note: If you are using the Docker image provided, you need to move files to and from the image so you can upload them to your browser. First you need to find out the name of the running container. If you use VSCode, you can find the name under the Remote Explorer pane (the name is the `something_other` name listed after `booster2024wasm`). Alternatively, you can find the name in the terminal: 

```bash
docker container ls
```
Note: This command should be run outside your terminal that is connected to Docker. This command lists all containers. Unless you use docker a lot, you probably only have a single entry. Make a note of the container name from the `NAMES` column.

Once you have the name, you can copy files to and from the container using the `docker cp` command. To copy the task 2 directory from the container to your local machine, you can use the following command:

```bash
docker cp <container_name>:/booster2024_wasm_components/tasks/02 .
```

And vice versa, when you want to copy the composed component back to the container, you can use the following command:

```bash
docker cp composed.wasm <container_name>:/booster2024_wasm_components/tasks/02/composed.wasm
```

2. Add one instance of the `command` and `starter` components, two instances of the virtualized Python `greeter` component, and one instance of the virtualized Javascript `greeter` component into the editor.
3. Connect the exported interfaces in the order that makes sense in order to print "Hello from WASM! and Python! and Javascript! and Python!". 

This is all that's needed to compose components. However, if you want to export the interface from our composed component, you need to click the small checkbox in the top left corner of the checkbox. We want to use the exported greeting function in further compositions, so we need to select the final component in the chain as the exported outputs in our composed component.

4. Select the final component as our exported interface and download the component.
5. If using Docker, copy the composed component back to the container using the `docker cp` command.

## 02.4 Finishing touches

We now have a single binary built using both Python and Javascript that prints a nice greeting to the terminal. The only thing left to do is to try it out:

1. Run your composed component using `wasmtime`.

[I'm tired of greetings, give me a proper application to work on!](https://github.com/syvsto/booster2024_wasm_components/blob/master/task3.md)
