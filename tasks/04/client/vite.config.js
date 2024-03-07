import { defineConfig, searchForWorkspaceRoot } from "vite";
import wasm from "vite-plugin-wasm";

export default defineConfig({
  plugins: [wasm()],
  server: {
    fs: {
      allow: [
        searchForWorkspaceRoot(process.cwd()),
        "../03/clustering-rs/pkg/",
      ],
    },
    proxy: {
      "/cluster": {
        target: "<This should be the Fermyon endpoint>",
        changeOrigin: true,
      },
    },
  },
});
