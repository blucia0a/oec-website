import init from "./pkg/rustwasm_hello_world.js";

/* Utility funtion to convert 4 consecutive bytes in an array to 
  a single 32-bit integer */
function i8Atoi32(i,arr){
  return arr[i + 3] << 24 | arr[i+2] << 16 | arr[i+1] << 8 | arr[i];
}

/*WASM main function*/
const runWasm = async () => {

  /*Initialize the WASM blob*/
  const rustWasm = await init("./pkg/rustwasm_hello_world_bg.wasm");

  /*Create the stars array and initialize randomly*/
  rustWasm.setup_stars();  

  /*Helper to draw the stars to the canvas called starCanvas*/
  const drawStarField = () => {
    var c = document.getElementById("starCanvas");
    var ctx = c.getContext("2d");

    /*Get Star parameters from WASM*/  
    const starsPtr = rustWasm.get_star_buf_ptr();
    const numStars = rustWasm.get_num_stars(); 
    const starSize = rustWasm.get_star_size(); 

    /*Update the stars' positions and velocities*/
    rustWasm.update_stars();

    /*Marshal the array of stars into JS world*/
    const wasmMem = new Uint8Array(rustWasm.memory.buffer);
    const starData = wasmMem.slice(starsPtr,starsPtr + (numStars * starSize * 4));
 
    /*Clear the screen to black*/ 
    ctx.clearRect(0,0,c.height,c.width);
    ctx.beginPath();
    ctx.fillStyle = "#000000";
    ctx.fillRect(0,0,c.height,c.width);

    /*Draw all the stars*/
    for(var i = 0; i < numStars * starSize; i += starSize){
   
      /*Grab the bytes from the star data structure*/
      var x = i8Atoi32(i,starData);
      var y = i8Atoi32(i+4,starData);
      var r = i8Atoi32(i+16,starData);

      /*Each star is a rectangle*/
      ctx.fillStyle = "#FFFFFF";
      ctx.fillRect(x, y, r, r);
      ctx.stroke();
    }
  }; /*End of drawStarField()*/

  /*Call drawStarField and schedule it to be called 30 times per second*/
  drawStarField();
  setInterval(() => {
    drawStarField();
  }, 33); 

};

runWasm();
