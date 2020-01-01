use wasm_bindgen::prelude::*;

extern crate rand;
use rand::Rng;


#[wasm_bindgen]
extern "C" {

#[wasm_bindgen(js_namespace = console)]
  fn log(s: &str);
}

const XMAX: u32 = 1024;
const YMAX: u32 = 1024;
const VMAX: u32 = 5;
const RMAX: u32 = 6;
const NUM_STARS: usize = 256;

//pos x,y  vel x,y  rad --> 5 values
const STAR_SZ: usize = 5;

const STAR_BUF_SZ: usize = NUM_STARS * STAR_SZ; 

/*WASM linear memory buffer*/
static mut STAR_BUF: [u32;  STAR_BUF_SZ] = [0; STAR_BUF_SZ];

#[wasm_bindgen]
pub fn get_num_stars() -> usize{
  NUM_STARS 
}

#[wasm_bindgen]
pub fn get_star_size() -> usize {
  STAR_SZ
}

#[wasm_bindgen]
pub fn get_star_buf_ptr() -> *const u32 {
  let pointer: *const u32;
  unsafe {
    pointer = STAR_BUF.as_ptr();
  }

  return pointer;
}

pub fn get_star(ind: usize) -> Option<Star>{
  if ind > NUM_STARS{ return None; }
  let mut s: Star = 
    Star { position: Coord{x:0,y:0}, velocity: Coord{x:0,y:0}, radius: 0};
  unsafe {
    let px = STAR_BUF[ind * STAR_SZ];
    let py = STAR_BUF[ind * STAR_SZ + 1];
    let vx = STAR_BUF[ind * STAR_SZ + 2];
    let vy = STAR_BUF[ind * STAR_SZ + 3];
    let ra = STAR_BUF[ind * STAR_SZ + 4];
    s.position.x = px;
    s.position.y = py;
    s.velocity.x = vx;
    s.velocity.y = vy;
    s.radius = ra;
  }
  return Some(s); 
  
}

pub fn put_star(ind: usize, s: &Star) {
  if ind > NUM_STARS{ return; }
  //log(&format!("{}",s.position.x)[0..]);
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
  x: u32,
  y: u32
}

#[derive(Debug)]
pub struct Star {
  position: Coord,
  velocity: Coord,
  radius: u32
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
pub fn setup_stars() {

  let mut rng = rand::thread_rng();

  for i in 0..NUM_STARS{
      let x = rng.gen_range(1,XMAX);
      let y = rng.gen_range(1,YMAX);
      let vx = rng.gen_range(1,VMAX);
      let vy = rng.gen_range(1,VMAX);
      let r = rng.gen_range(1,RMAX);
      put_star(i, &Star{ position: 
                         Coord { x: x, 
                                 y: y}, 
                        velocity: 
                         Coord { x: vx,
                                 y: vy},
                        radius: r} );
  }

  for i in 0..NUM_STARS {

    let os = get_star(i);
    match os{
      Some(s) => print_star(&s),
      None => (),
    } 

  }

}

#[wasm_bindgen]
pub fn update_stars() {

  for i in 0..NUM_STARS {
    let os = get_star(i);
    match os{
      Some(s) => { 
                       put_star(i,&Star{ 
                                    position: 
                                     Coord{ x: s.position.x + s.velocity.x, 
                                            y: s.position.y + s.velocity.y}, 
                                    velocity: s.velocity, 
                                    radius: s.radius}); 
                 },
      None => (),
    }          
  } 

}

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32{
  return a + b;
}

