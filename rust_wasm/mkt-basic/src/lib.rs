//
// TODO
// --- clean Empty
// include radius
//


mod utils;

use std::{sync::Mutex, f64::consts::PI};

use js_sys::Math::random;
use wasm_bindgen::prelude::*;
use web_sys::{HtmlCanvasElement, CanvasRenderingContext2d};

// use rand::prelude::*;
#[macro_use]
extern crate lazy_static;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(a: &str);
}

#[wasm_bindgen(raw_module = "../../../src/lib/graph.js")]
extern "C" {
    #[wasm_bindgen(js_name="jsMedianVelocitySq")]
    fn js_median_velocity_sq(velocity: Vec<f64>);
    #[wasm_bindgen(js_name="jsPathPointAndDraw")]
    fn js_path_point_and_draw(x: f64, y: f64, ctx: &CanvasRenderingContext2d);
}

macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

static mut PREV_POSITION: (f64,f64) = (0.,0.);
static mut WATCHING_INDEX: Option<usize> = None;



// static TIMESTEP: f64 = 0.05;

static CELL_SIZE: f64 = 4.;

lazy_static! {
    static ref PARTICLES: Mutex<Particles> = {
        let p = Particles {
            particles: Vec::new(),
            types: Vec::new()
        };
        Mutex::new(p)
    };
}

lazy_static! {
    static ref MAIN: Mutex<Main> = {
        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("canvas_mkt_basic");
        if let None = canvas {
            panic!("canvas is not found");
        }
        let canvas = canvas.unwrap();
        let canvas = canvas.dyn_into::<web_sys::HtmlCanvasElement>().map_err(|_| ()).unwrap();
        let (w,h) = (canvas.width(),canvas.height());

        let context = canvas.get_context("2d").unwrap().unwrap().dyn_into::<web_sys::CanvasRenderingContext2d>().unwrap();

        let s1 = (w as f64/CELL_SIZE).ceil() as usize;
        let s2 = (h as f64/CELL_SIZE).ceil() as usize;

        // console_log!("Initialized MAIN with such size ({},{})", s1, s2);
        
        Mutex::new(Main {
            canvas,
            context,
            cells: vec![vec![vec![];s1];s2],
            cell_size: (s1, s2),
            borders: (0.,0.,w as f64,h as f64)
        })
    };
}

static mut CLEAN: u8 = 0;



struct Particles {
    particles: Vec::<Particle>,
    types: Vec::<(usize,f64)> //num mass
}


#[derive(Default, Debug, Clone, Copy)]
struct Particle {
    position: (f64,f64),
    velocity: (f64,f64),
    radius: f64,
    color: (u8,u8,u8),
    mass: f64
}

struct Main {
    canvas: HtmlCanvasElement,
    context: CanvasRenderingContext2d,
    cells: Vec<Vec<Vec<CellParticle>>>,
    cell_size: (usize,usize),
    borders: (f64,f64,f64,f64)
}

unsafe impl Sync for Main {}
unsafe impl Send for Main {}


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub fn rs_watch_particle(idx: isize) {
    let mut p = PARTICLES.lock().unwrap();
    unsafe {
        if idx==-1 {
            WATCHING_INDEX = None;
        } else {
            WATCHING_INDEX = Some(idx as usize);
            PREV_POSITION = p.particles[idx as usize].position;
        }
    }
}

#[wasm_bindgen]
pub fn rs_add_particles(particle_num: usize, median_velocity: f64, mass: f64, color_1: u8, color_2: u8, color_3: u8) {
    
    let mut p = PARTICLES.lock().unwrap();
    let mut m = MAIN.lock().unwrap();
    
    let len = p.particles.len();
    // console_log!("Init. Old len: {:?}, current len: {:?}", len, particle_num);
    let _ = p.particles.resize_with(len+particle_num, || Particle::default());
    p.types.push((particle_num,mass));

    let radius = mass.powf(0.3333);

    for i in 0..particle_num {
        let seed1 = random();
        let seed2 = random();
        let seed3 = random()*2.*PI;
        let seed4 = random();


        p.particles[i+len] = Particle {
            position: (seed1*(0.8*m.canvas.width() as f64-2.*radius)+radius+(0.1*m.canvas.width() as f64),
                        seed2*(0.8*m.canvas.height() as f64-2.*radius)+radius+(0.1*m.canvas.height() as f64)),
            // position: (10.,10.),
            velocity: (median_velocity*seed3.cos(), median_velocity*seed3.sin()),
            // velocity: (0.,0.),
            radius,
            color: (color_1,color_2,color_3),
            mass
        };

        let cell_x = (p.particles[i].position.0/CELL_SIZE).floor() as usize;
        let cell_y = (p.particles[i].position.1/CELL_SIZE).floor() as usize;

        // console_log!("New particle at ({},{}) which is in ({},{})", p.particles[i].position.0, p.particles[i].position.1, cell_x, cell_y);


        m.cells[cell_y][cell_x].push(CellParticle::Full(i+len));
    }   

    // console_log!("{:?}", p.particles);
    
}

pub fn rs_clear_particles() {
    let mut p = PARTICLES.lock().unwrap();
    p.particles.resize(0, Particle::default());
    p.types.resize(0, (0,0.));
}

#[derive(Clone, Copy)]
enum CellParticle {
    Empty,
    Full(usize)
}

#[wasm_bindgen]
pub fn rs_next_step(subiter: u8, timestep: f64,) {
    for _ in 0..subiter {

        update(timestep);
    }
    draw();
    clean();
}

fn update(timestep: f64) {
    
        let mut m = MAIN.lock().unwrap();
        let mut p = PARTICLES.lock().unwrap();

        
        for i in 0..m.cell_size.1 {
            for j in 0..m.cell_size.0 {
                for index in 0..m.cells[i][j].len() {
                    let cp_main_particle_id = m.cells[i][j][index];
                    if let CellParticle::Full(main_particle_id) =  cp_main_particle_id {
                            let dx = p.particles[main_particle_id].velocity.0*timestep;
                            let dy = p.particles[main_particle_id].velocity.1*timestep;

                            let position_x = p.particles[main_particle_id].position.0.clone() + dx;
                            let position_y = p.particles[main_particle_id].position.1.clone() + dy;

                            let left_border = p.particles[main_particle_id].radius.clone()+m.borders.0;
                            let right_border = m.borders.2 - p.particles[main_particle_id].radius.clone() - 1.;

                            let top_border = p.particles[main_particle_id].radius.clone()+m.borders.1;
                            let bottom_border = m.borders.3 - p.particles[main_particle_id].radius.clone() - 1.;

                            if position_x < left_border || position_x > right_border {
                                let nx = 
                                    if position_x < left_border  { left_border  } else { right_border };
                                let ny = p.particles[main_particle_id].position.1 + dy * ((nx-p.particles[main_particle_id].position.0)/dx).abs();
                                p.particles[main_particle_id].position.0 = nx;
                                p.particles[main_particle_id].position.1 = ny;
                                p.particles[main_particle_id].velocity.0 *= -1.;
                            } else if position_y < top_border || position_y > bottom_border {
                                // console_log!("xxxxx {}+{}, {}", particle.position.1, dy, m.canvas.height()-1);
                                let ny = 
                                    if position_y < top_border { top_border } else { bottom_border };
                                let nx = p.particles[main_particle_id].position.0 + dx * ((ny-p.particles[main_particle_id].position.1)/dy).abs();
                                p.particles[main_particle_id].position.0 = nx;
                                p.particles[main_particle_id].position.1 = ny;
                                p.particles[main_particle_id].velocity.1 *= -1.;
                            } else {
                                p.particles[main_particle_id].position.1 += dy;
                                p.particles[main_particle_id].position.0 += dx;
                            }   

                            unsafe { if let Some(id) = WATCHING_INDEX {
                                PREV_POSITION = p.particles[id].position;
                            } };
                            

                            let cell_x = (p.particles[main_particle_id].position.0/CELL_SIZE).floor() as usize;
                            let cell_y = (p.particles[main_particle_id].position.1/CELL_SIZE).floor() as usize;

                            if cell_x != j || cell_y != i {
                                // console_log!("Moved from {}, {} to {}, {}", j,i,cell_x,cell_y);
                                m.cells[i][j][index] = CellParticle::Empty;
                                m.cells[cell_y][cell_x].push(CellParticle::Full(main_particle_id));
                            }
                        

                            for di in 0..=2 {
                                for dj in 0..=2 {
                                    if !((di!=0 && dj!=0) && i+(di as usize)>1 && i+di<m.cell_size.1+1 && j+dj>1 && j+dj < m.cell_size.0+1) {
                                        continue;
                                    }
                                    for second_index in 0..m.cells[i+di-1][j+dj-1].len()  {
                                        let cp_second_particle_id = m.cells[i+di-1][j+dj-1][second_index];
                                        if let CellParticle::Full(second_particle_id) = cp_second_particle_id {

                                            
                                            // console_log!("{:?}, {:?}", p.particles[main_particle_id],p.particles[second_particle_id]);
                                            // console_log!("{:?}", particles);
                                            let dx = p.particles[second_particle_id].position.0-p.particles[main_particle_id].position.0;
                                            let dy = p.particles[second_particle_id].position.1-p.particles[main_particle_id].position.1;
                                            let distance_squared = (dx).powi(2)+(dy).powi(2);
                                            let double_radius_squared = (p.particles[second_particle_id].radius+p.particles[main_particle_id].radius).powi(2);
                                            if distance_squared < double_radius_squared {
                                                let m1 = p.particles[main_particle_id].mass;
                                                let m2 = p.particles[second_particle_id].mass;
                                                let (v1x,v1y) = p.particles[main_particle_id].velocity;
                                                let (v2x,v2y) = p.particles[second_particle_id].velocity;
                                                
                                                let nv1x =  (m1-m2)/(m1+m2)*v1x + 2.*m2/(m1+m2)*v2x;
                                                let nv1y =  (m1-m2)/(m1+m2)*v1y + 2.*m2/(m1+m2)*v2y;

                                                let nv2x =  (m2-m1)/(m1+m2)*v2x + 2.*m1/(m1+m2)*v1x;
                                                let nv2y =  (m2-m1)/(m1+m2)*v2y + 2.*m1/(m1+m2)*v1y;

                                                p.particles[main_particle_id].velocity = (nv1x,nv1y);
                                                p.particles[second_particle_id].velocity = (nv2x,nv2y);

                                                let diff = (double_radius_squared.sqrt() - distance_squared.sqrt())/2.;

                                                
                                                if distance_squared!=0. {
                                                    p.particles[main_particle_id].position.0 -= diff*dx/distance_squared.sqrt();
                                                    p.particles[main_particle_id].position.1 -= diff*dy/distance_squared.sqrt();

                                                    p.particles[second_particle_id].position.0 += diff*dx/distance_squared.sqrt();
                                                    p.particles[second_particle_id].position.1 += diff*dy/distance_squared.sqrt();
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                    }
                }
            }
        }
}

fn draw() {
    let m = MAIN.lock().unwrap();
    let p = PARTICLES.lock().unwrap();
    let ctx = &m.context;
    let canvas = &m.canvas;
    ctx.clear_rect(0., 0., canvas.width() as f64, canvas.height() as f64);

    
    let mut v_squared = Vec::new();
    v_squared.resize(p.types.len(), 0.);
    let mut v_index = 0;
    let mut next_index = p.types[0].0;
    
    for index in 0..p.particles.len() {
        // ctx.fill_rect(30., 30., 20., 20.);

        let particle = &p.particles[index];

        ctx.begin_path();
        ctx.ellipse(particle.position.0, particle.position.1, particle.radius, particle.radius, 0., 0., 2.*std::f64::consts::PI);
        // ctx.ellipse(10., 10., 5., 5., 0., 0., 2.*std::f64::consts::PI);
        ctx.stroke();

        if index+1 == next_index {
            v_squared[v_index] = v_squared[v_index] / ( p.types[v_index].0  as f64); //v_squared[v_index]/(p.types[v_index].0 as f64);
        }
        if index == next_index {
            v_index += 1;
            if v_index<p.types.len() {
                next_index += p.types[v_index].0;
            }
        }
        v_squared[v_index] += particle.velocity.0.powi(2)+particle.velocity.1.powi(2);
        
    }
    // console_log!("{:#?}", v_squared);
    
    js_median_velocity_sq(v_squared);

    unsafe { if let Some(idx) = WATCHING_INDEX {
        js_path_point_and_draw(p.particles[idx].position.0,p.particles[idx].position.1, ctx);
    } };
}

fn clean() {
    let mut m = MAIN.lock().unwrap();
    unsafe {
        if CLEAN==10 {
            CLEAN = 0;
            for i in m.cells.iter_mut() {
                for c in i.iter_mut() {
                    c.retain(|x| {
                        match x {
                            CellParticle::Empty => false,
                            CellParticle::Full(_) => true
                        }
                    });
                    
                }
            }
            CLEAN = 0;
        }
        CLEAN+=1; 
    } 
}

pub fn set_borders(left: f64, right: f64, top: f64, bottom: f64) {
    let mut m = MAIN.lock().unwrap();
    m.borders = (left,top,right-left,bottom-top);
}