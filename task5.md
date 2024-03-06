# 05 Going further

Congratulations on completing the workshop! This final task is a bonus round where you will take advantage of the features of the component model architecture to build middlewares for our clustering server.
As this is a bonus round, you're left a lot more to your own devices, but we'll provide a couple of hints to get you going.

## 05.1 Tracing

Real-world web servers usually provide a mechanism for tracing an individual request by making the request identifiable. We will implement a simple form of tracing as a middleware that modifies our request body and amends a field to the JSON there that specifies an identifier for the request.  

We will build the middleware as a component that reads the HTTP request from the clustering server and amends the response body. To add the middleware to our application, we will compose the middleware with the rest of our clustering server.
You can choose any language you like to implement the middleware.
 
1. Build a component that reads an incoming HTTP request and modifies it to add a field called `requestId`. For simplicity, use a random number as the `requestId`. 

Hint: Fermyon Spin can run any component that implements the `incoming-handler` interface from `wasi:http`. The Python handler component implements this interface, and since we want to build middleware, we should both import and export this interface so we can chain middlewares.
Take a look at the WASI WIT interface for `incoming-handler` in the handler project from task 3 to see what the interface you need to implement looks like. 

2. Change the handler component so that it extends the request body instead of creating a new, empty response body. This may require virtualization of some of the components.
3. Compose the clustering component, the handler component and the middleware component together and upload it to Fermyon.

Building middleware in this manner shows how useful WebAssembly components can be! Considering how you can use components other people have written in any language as middlewares in your own application, combined with the per-component sandboxing and how you are able to control which features each component should be able to perform through for instance `WASI-virt`, you can see how WebAssembly components offer a very high level of cross-language reuse and security.

## 05.2 Bonus round for the bonus round: Other components

Fermyon lets you use key-value stores, databases, etc. through component composition, with components they have created. If you want even more to do, you could take a look at these and add caching to the application, so you don't have to recompute the clustered values every time the application runs. 
