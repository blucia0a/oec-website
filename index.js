import init from "./pkg/rustwasm_hello_world.js";

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
    const numStars = rustWasm.get_num_stars(); 

    /*Update the stars' positions and velocities*/
    rustWasm.update_stars();

    /*Clear the screen to black*/ 
    ctx.clearRect(0,0,c.height,c.width);
    ctx.beginPath();
    ctx.fillStyle = "#000000";
    ctx.fillRect(0,0,c.height,c.width);

    /*Draw all the stars*/
    for(var i = 0; i < numStars; i++){
   
      /*Grab the bytes from the star data structure*/
      var x = rustWasm.get_star_x(i);
      var y = rustWasm.get_star_y(i);
      var r = rustWasm.get_star_r(i);

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
