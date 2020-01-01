import init from "./pkg/rustwasm_hello_world.js";

const runWasm = async () => {

  const rustWasm = await init("./pkg/rustwasm_hello_world_bg.wasm");
  rustWasm.setup_stars();  

  const drawStarField = () => {
    var c = document.getElementById("starCanvas");
    var ctx = c.getContext("2d");
  
    const starsPtr = rustWasm.get_star_buf_ptr();
    const numStars = rustWasm.get_num_stars(); 
    const starSize = rustWasm.get_star_size(); 

    rustWasm.update_stars();

    const wasmMem = new Uint8Array(rustWasm.memory.buffer);
    const starData = wasmMem.slice(starsPtr,starsPtr + (numStars * starSize * 4));
  
    ctx.clearRect(0,0,c.height,c.width);
    for(var i = 0; i < numStars * starSize * 4; i += starSize*4){
      var x = starData[i + 3] << 24 | starData[i+2] << 16 | starData[i+1] << 8 | starData[i];
      var y = starData[i + 7] << 24 | starData[i+6] << 16 | starData[i+5] << 8 | starData[i+4];
      var r = starData[i + 19] << 24 | starData[i + 18] << 16 | starData[i + 17] << 8 | starData[i + 16];
        //console.log("[star "+x+" "+y+" "+r+"]");
      ctx.beginPath();
      ctx.rect(x, y, r, r);
      ctx.stroke();
    }
  };

  drawStarField();
  setInterval(() => {
    drawStarField();
  }, 30); 

};

runWasm();
