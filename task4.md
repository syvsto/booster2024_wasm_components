# 04 Deployment

Now that we have a component that has some real-world use, we should deploy it. One of the nice things about WebAssembly components is that they are sandboxed, which makes them somewhat comparable to a tiny container. Several cloud providers have started allowing for WebAssembly as an alternative to Docker, and some have built their business around WebAssembly. We mentioned [Fermyon](https://www.fermyon.com/) in the introductory talk, who provide serverless functions that run WebAssembly components as their core business. We are going to use the free tier of Fermyon to host our component in this workshop.

## 04.1 Fermyon Spin

Fermyon has built a tool called [Spin](https://www.fermyon.com/spin) for working with and uploading WebAssembly components to their site. This tool handles the configuration of your component in the Fermyon environment through a configuration file, `spin.toml`. Let's start by adding one to our project.

1. Copy the composed component binary from Task 3 into `tasks/04`.
2. Open the `spin.toml` file in the directory.

We want to modify the configuration to load our component. There are two sections that need to be modified: The `[[trigger.http]]` field tells Fermyon that this component should trigger when an HTTP request is sent. The route field specifies which route that will trigger the component, and the `component` field specifies the name of the component to run. Following this, we need to specify our component. `[component.name]` specifies that a component called `name` exists. `name` here should match the name specified in the `component` field above. For this component, you need to specify a `description`, and a `source`, which is the path to your WebAssembly binary file.

3. Modify the `spin.toml` file to use the composed component from task 3.

You can now try out your application. Spin is available as a CLI application named `spin`.

4. Run `spin up` in the current directory to start your application.
5. Send a request to the endpoint you set up. 

If everything looks good, you can deploy it to Fermyon. For this, you need a starter Fermyon account, [which can be created here](https://cloud.fermyon.com/?signup=).

6. Run `spin login` to log in to your user locally.
7. Run `spin deploy` to upload your component to Fermyon.

If you visit [cloud.fermyon.com](https://cloud.fermyon.com) you should now see your newly uploaded component ready for real-world use!

*Note*: Fermyon and Spin has a lot of very interesting features such as support for key-value stores and databases, monitoring of specific components within a project, and support for a CMS for your applications. Take a look at [developer.fermyon.com](https://developer.fermyon.com) for more information!

## 04.2 In the browser

The clustering algorithm we built in task 3 seems like it could be useful in certain websites as well, for instance if you're building a dashboard. Since we're working with WebAssembly, that is not a problem! However, browsers need to support the interface between WebAssembly and the Javascript ABI. Hopefully, Browsers will begin to support the component specification in the future, so we can use WebAssembly components directly there as well, but for the time being we need to tweak our Rust clustering project slightly to support browsers.

First, we need to add [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) to our project, which similarly to `wit-bindgen` provides helpers for creating bindings to the generated WebAssembly. This time, we don't create bindings to a WIT file, but instead to Javascript for the browser. Additionally, to conform to the Javascript ABI, we cannot specify functions using nested vectors, and instead need one or more flat lists.

The simplest way to do so is to wrap our clustering method within a function that conforms to the Javascript ABI, and use this to build our WASM for the browser.

1. Write such a function. This requires a little bit of Rust knowledge, but should be quite possible in a few lines of code. Hint: You cannot use nested vectors or other complex composite types, but are allowed to use anything that has a native Javascript type, such as `Uint8Array`, `Float64Array` and similar. Single vectors are therefore fine.

`wasm-bindgen` is quite straight-forward to use. You decorate any functions with a procedural macro in the following manner:

```rust
#[wasm_bindgen]
pub fn function_name(arg: u8) -> u8 {
        ...
}
```

When the project is built, `wasm-bindgen` will now generate bindings to Javascript and Typescript types for this function.

2. Decorate your wrapper function with `#[wasm_bindgen]`.

Finally, there is a separate build tool for building WebAssembly projects for the web in Rust: [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/).

3. Run `wasm-pack build` to build the project for the browser. 

The results are found within the `pkg/` folder in the project directory.


## 04.3 What's next?

Congratulations! You have now built WebAssembly components in multiple languages, composed them together, deployed an application as a serverless function for real-world use and even pulled out useful parts of the application for use directly by the client in the browser!

If you want a bigger challenge, you can attempt [Task 5](https://github.com/syvsto/booster2024_wasm_components/blob/master/task5.md).
