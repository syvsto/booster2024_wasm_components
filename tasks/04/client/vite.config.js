import { defineConfig, searchForWorkspaceRoot } from "vite";
import wasm from "vite-plugin-wasm";

export default defineConfig({
  plugins: [wasm()],
  server: {
    fs: {
      allow: [searchForWorkspaceRoot(process.cwd())], // This should be expanded with the path to the linked NPM module for task 04.2.1
    },
    proxy: {
      "/cluster": {
        target: "<This should be the Fermyon endpoint>",
        changeOrigin: true,
      },
    },
  },
});
