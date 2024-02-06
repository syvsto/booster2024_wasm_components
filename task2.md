# Task 2: Component composition

## Virtualization

Both the `componentize-py` and `cargo component` tools automatically insert system dependencies as inputs in the WebAssembly Components they build. This is nice if the component should be executed directly by a runtime, but creates problems when trying to compose multiple 
