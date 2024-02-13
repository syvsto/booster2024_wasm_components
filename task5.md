# 05 Going further

Now that you've built a functioning application, it's time to use the things we've learned. In this final task, we will work on building middlewares for our application. 

## 05.1 Logging

Our clustering application is missing logging, let's build some: 

1. Build a component for logging to the terminal, and modify the clustering algorithm and HTTP server WIT so that you can slot in the logging component between the other two. 
2. Make sure your WIT interfaces allows composing multiple middlewares together.

Building middleware in this manner shows how useful WebAssembly components can be! Considering how you can use components other people have written in any language as middlewares in your own application, combined with the per-component sandboxing and how you are able to control which features each component should be able to perform through for instance `WASI-virt`, you can see how WebAssembly components offer a very high level of cross-language reuse and security.

This concludes the workshop! I hope it was interesting and got you thinking about the possibilities of building and composing WebAssembly components.

## 05.2 BONUS: Other middleware

If you have some extra time, feel free to think of other middlewares, and implement them.
