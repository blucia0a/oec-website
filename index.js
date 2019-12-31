import init from "./pkg/hello_world.js";

const runWasm = async () => {
  const hello = await init("./pkg/hello_world_bg.wasm");
  const addResult = hello.add(10,20);
  document.body.textContent = `Hello! ${addResult}`;
};

runWasm();
