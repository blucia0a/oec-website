use wasm_bindgen::prelude::*;

extern crate rand;
use rand::Rng;


#[wasm_bindgen]
extern "C" {
  // Use `js_namespace` here to bind `console.log(..)` instead of just
  // `log(..)`
#[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

const XMAX: u64 = 1024;
const YMAX: u64 = 1024;
const VMAX: u64 = 10;
const RMAX: u64 = 5;

#[derive(Debug)]
struct Coord {
  x: u64,
  y: u64
}

#[derive(Debug)]
struct Star {
  position: Coord,
  velocity: Coord,
  radius: u64
}

impl ToString for Star {
  fn to_string(&self) -> String{
    format!("{:?}",self)
  }
}

fn print_star(s: &Star){
  log(&s.to_string()[0..]); 
}

#[wasm_bindgen]
pub fn setup_stars(num_stars: usize) {

  let mut rng = rand::thread_rng();

  let mut stars = Vec::new();
  for _ in 0..num_stars{
    stars.push( Star{ 
                      position: 
                               Coord { x: rng.gen_range(0,XMAX), 
                                       y: rng.gen_range(0,YMAX) }, 
                      velocity: 
                               Coord { x: rng.gen_range(0,VMAX),
                                       y: rng.gen_range(0,VMAX)},
                      radius: rng.gen_range(0,RMAX)} );
  }
  log("GOT HERE!!\n");  
  for s in &stars{
    print_star(s);
  }

  //stars;

}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32{
  return a + b;
}

