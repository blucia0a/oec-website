use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

extern crate rand;

#[macro_use]
extern crate lazy_static;

use rand::Rng;
use std::sync::Mutex;

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

lazy_static! {
  static ref STARV: Mutex<Vec<Star>> = Mutex::new( vec![] );
}

#[derive(Debug, Copy, Clone)]
pub struct Coord {
  x: u32,
  y: u32
}

#[derive(Debug, Copy, Clone)]
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
pub fn get_num_stars() -> usize{
  STARV.lock().unwrap().len() 
}

#[wasm_bindgen]
pub fn get_star_x(ind: usize) -> u32{
  STARV.lock().unwrap()[ ind ].position.x
}

#[wasm_bindgen]
pub fn get_star_y(ind: usize) -> u32{
  STARV.lock().unwrap()[ ind ].position.y
}

#[wasm_bindgen]
pub fn get_star_r(ind: usize) -> u32{
  STARV.lock().unwrap()[ ind ].radius
}


#[wasm_bindgen]
pub fn setup_stars() {

  let mut rng = rand::thread_rng();
  for _ in 0..NUM_STARS{

    let x = rng.gen_range(1,XMAX);
    let y = rng.gen_range(1,YMAX);
    let vx = rng.gen_range(1,VMAX);
    let vy = rng.gen_range(1,VMAX);
    let r = rng.gen_range(1,RMAX);

    STARV.lock().unwrap().
      push( Star{position: Coord{ x:x,y:y }, 
                 velocity: Coord{ x:vx,y:vy }, radius: r});

  }
}

#[wasm_bindgen]
pub fn update_stars() {

  let mut rng = rand::thread_rng();
  let mut st = STARV.lock().unwrap();
  for s in &mut *st {
 
    let mut newx = s.position.x + s.velocity.x;
    let mut newy = s.position.y + s.velocity.y;
    let mut newvx = s.velocity.x;
    let mut newvy = s.velocity.y;

    if newx > XMAX { 
      newx = 0; 
      newvx = rng.gen_range(1,VMAX);
    }

    if newy > YMAX { 
      newy = 0; 
      newvy = rng.gen_range(1,VMAX);
    }

    s.position.x = newx;
    s.position.y = newy;
    s.velocity.x = newvx;
    s.velocity.y = newvy;

  } 

}

#[wasm_bindgen]
pub fn draw_stars(){

   let document = web_sys::window().unwrap().document().unwrap();
   let canvas = document.get_element_by_id("starCanvas").unwrap();
   let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

  
  context.set_fill_style(&JsValue::from_str("#000000"));
  context.fill_rect(0.0,0.0,1024.0,1024.0); 
  let mut st = STARV.lock().unwrap();
  for s in &mut *st {
    context.set_fill_style(&JsValue::from_str("#FFFFFF"));
    context.fill_rect(s.position.x as f64,s.position.y as f64,s.radius as f64,s.radius as f64); 
  }

}
