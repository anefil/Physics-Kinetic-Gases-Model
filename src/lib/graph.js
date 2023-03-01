import * as wasm from '../../rust_wasm/mkt-basic/pkg/main';

export function jsMedianVelocitySq  (v) { 
  v = v.map(a=>a-16); // почему? самому бы знать
  // console.log(v); 
}

export function jsPathPointAndDraw (x,y, ctx) {
  points.push([x,y]);
  ctx.beginPath();
    ctx.moveTo(points[0][0],points[0][1]);
  for(let p of points) {
    ctx.lineTo(p[0],p[1]);
  }
  ctx.stroke();
}

export async function addMoleculeType(num, v_avg, mass, color) {
  await wasm.rs_add_particles(num, v_avg, mass, 0,0,0);
  moleculeTypeArr.push([num,mass]);
}


let moleculeTypeArr = [];
let points = [];
// let ctx;

let start;
let i = 0;
let TIMESTEP, SUBITERATIONS;

export async function render() {    
  await wasm.rs_next_step(SUBITERATIONS, TIMESTEP);
  i++;
  if(i===600)
    console.log("Dif time: "+(performance.now()-start)/1000);
  requestAnimationFrame(render);
}

export async function init(timestep, subiterations) {
  await wasm.init();
  TIMESTEP = timestep; SUBITERATIONS = subiterations;
  start = performance.now();
}

export async function watchParticle(idx) {
  await wasm.rs_watch_particle(1);
}