import { defineConfig, searchForWorkspaceRoot } from "vite";
import wasm from "vite-plugin-wasm";

export default defineConfig({
  plugins: [wasm()],
  server: {
    fs: {
      allow: [
        searchForWorkspaceRoot(process.cwd()),
        "../../03/clustering-rs/pkg",
      ],
    },
    proxy: {
      "/cluster": {
        target: "https://booster-2024-clustering-iulo5ims.fermyon.app/",
        changeOrigin: true,
      },
    },
  },
});
