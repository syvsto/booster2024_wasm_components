# 02 Component composition

One of the strengths of the component model is that you can compose multiple components together, connecting the inputs of one to the outputs of another. In this task, we will
build on our outputs from task 1 to make a chain of components that in the end print out "Hello from WASM! and Python! and Javascript! and Python!", using both a Python and a Javascript component.

## Virtualization

Both the `componentize-py` and `cargo component` tools automatically insert system dependencies as inputs in the WebAssembly Components they build. This is nice if the component should be executed directly by a runtime, but creates problems when trying to compose multiple 
