import * as wasm from '../../rust_wasm/mkt-basic/pkg/main';
import {updateChart} from "@/lib/GasChart";

let counter = 0;
export function jsMedianVelocitySq  (v) { 
  v[0] = v[0]-16; // почему? самому бы знать
  // console.log(v); 
  let pressure = 0;
  for (let i = 0; i < v.length; i++) {
    pressure += 1/3 * moleculeTypeArr[i][1] * moleculeTypeArr[i][0] / 400 * v[i]; // 400=Volume
  }

  counter++;
  if(counter%1===0) {
    console.log((counter/5)*3, pressure)
    updateChart([[(counter/100)*3], [pressure]]);
  }


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
  await wasm.rs_add_particles(num, v_avg, mass, color[0],color[1],color[2]);
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