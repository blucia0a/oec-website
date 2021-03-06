import init from "./pkg/oec_website.js";

/*WASM main function*/
const runWasm = async () => {

  /*Initialize the WASM blob*/
  const rustWasm = await init("./pkg/oec_website_bg.wasm");

  /*Create the stars array and initialize randomly*/
  rustWasm.setup_stars();  

  /*Call drawStarField and schedule it to be called 30 times per second*/
  //drawStarField();
  rustWasm.draw_stars();
  setInterval(() => {
    rustWasm.update_stars();
    rustWasm.draw_stars();
  }, 33); 

};

runWasm();
