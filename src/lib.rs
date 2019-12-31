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
const NUM_STARS: usize = 256;

const STAR_SZ: usize = 5;
const STAR_BUF_SZ: usize = NUM_STARS * STAR_SZ; //pos x,y  vel x,y  rad --> 5 values
static mut STAR_BUF: [u64;  STAR_BUF_SZ] = [0; STAR_BUF_SZ];

#[wasm_bindgen]
pub fn get_star_buf_ptr() -> *const u64 {
  let pointer: *const u64;
  unsafe {
    pointer = STAR_BUF.as_ptr();
  }

  return pointer;
}

pub fn get_star(ind: usize) -> Option<Star>{
  if ind > NUM_STARS{ return None; }
  let mut s: Star = Star { position: Coord{x:0,y:0}, velocity: Coord{x:0,y:0}, radius: 0};
  unsafe {
    let px = STAR_BUF[ind * STAR_SZ];
    let py = STAR_BUF[ind * STAR_SZ + 1];
    let vx = STAR_BUF[ind * STAR_SZ + 2];
    let vy = STAR_BUF[ind * STAR_SZ + 3];
    let ra = STAR_BUF[ind * STAR_SZ + 4];
    s.position.x = px;
    s.position.y = py;
    s.velocity.x = vx;
    s.velocity.x = vy;
    s.radius = ra;
  }
  return Some(s); 
  
}

pub fn put_star(ind: usize, s: Star) {
  if ind > NUM_STARS{ return; }
  unsafe {
    STAR_BUF[ind * STAR_SZ] = s.position.x;
    STAR_BUF[ind * STAR_SZ + 1] = s.position.y;
    STAR_BUF[ind * STAR_SZ + 2] = s.velocity.x;
    STAR_BUF[ind * STAR_SZ + 3] = s.velocity.y;
    STAR_BUF[ind * STAR_SZ + 4] = s.radius;
  }
  return;
}

#[derive(Debug)]
pub struct Coord {
  x: u64,
  y: u64
}

#[derive(Debug)]
pub struct Star {
  position: Coord,
  velocity: Coord,
  radius: u64
}

impl ToString for Star {
  fn to_string(&self) -> String{
    format!("{:?}",self)
  }
}

fn print_star(s: Star){
  log(&s.to_string()[0..]); 
}

#[wasm_bindgen]
pub fn setup_stars() {

  let mut rng = rand::thread_rng();

  for i in 0..NUM_STARS{
      put_star(i, Star{ position: 
                                 Coord { x: rng.gen_range(0,XMAX), 
                                         y: rng.gen_range(0,YMAX) }, 
                        velocity: 
                                 Coord { x: rng.gen_range(0,VMAX),
                                         y: rng.gen_range(0,VMAX)},
                        radius: rng.gen_range(0,RMAX)} );
  }
  log("GOT HERE!!\n");  
  for i in 0..NUM_STARS {
    let os = get_star(i);
    match os{
      Some(s) => print_star(s),
      None => (),
    } 
  }

}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32{
  return a + b;
}

