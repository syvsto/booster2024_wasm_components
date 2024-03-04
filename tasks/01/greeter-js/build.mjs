import { componentize } from "@bytecodealliance/componentize-js";
import { readFile, writeFile } from "node:fs/promises";

const jsSource = await readFile("app.js", "utf8");
const opts = {
    "witPath" : ".",
    "enableStdout" : true
}

const { component } = await componentize(jsSource, opts);

await writeFile("greeter.wasm", component);
