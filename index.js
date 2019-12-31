import init from "./pkg/rustwasm_hello_world.js";

const runWasm = async () => {
  const hello = await init("./pkg/rustwasm_hello_world_bg.wasm");
  const addResult = hello.add(10,20);
  document.body.textContent = `Hello!!!! ${addResult}`;

  hello.setup_stars(10);  
};

runWasm();
