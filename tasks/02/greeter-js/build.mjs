import { componentize } from "@bytecodealliance/componentize-js";
import { readFile, writeFile } from "node:fs/promises";

const jsSource = await readFile("app.js", "utf8");
const witSource = await readFile("greeter.wit", "utf8");

const { component } = await componentize(jsSource, witSource);

await writeFile("greeter.wasm", component);
