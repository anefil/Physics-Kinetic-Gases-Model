<script setup>
  import { ref, onMounted } from 'vue'
  import Multithread from '../assets/lib/multithread';
  

  const simulation = ref(null)


  let MT = new Multithread(4);

  onMounted(() => {
    const canvas = document.querySelector('canvas')
    const context = canvas.getContext('2d')

    const width = canvas.width
    const height = canvas.height

    // create particles
    let particles = []
    const numParticles = 150
    const timestep = 0.04;
    const velocity = 30;
    const radius = 3;
    let cells = [];

    for (let i = 0; i < height/radius/8; i++) {
      cells[i] = []
      for( let j = 0; j < width/radius/8; j++){
        cells[i][j] = [];
      }
    }

    for (let i = 0; i < numParticles; i++) {
      particles.push(
        newParticle(Math.random()*width*0.6+0.2*width, Math.random()*height*0.6+0.2*height, radius)
      )
    }

    // create particle
    function newParticle(x, y, radius) {
      let rand = Math.random()*Math.PI*2;  
      return {
        x,
        y,
        vx: Math.cos(rand)*velocity,
        vy: Math.sin(rand)*velocity,
        radius,
        prevCell: [Math.floor(x/radius/8),Math.floor(y/radius/8)]
      }
    }

    function draw(obj) {
      context.beginPath()
      context.arc(obj.x, obj.y, obj.radius, 0, Math.PI * 2, false)
      context.fillStyle = '#000'
      context.fill()
    }

    async function physicsUpdate(subiter) {
      { // 1. update position
        for(let i = 0; i < numParticles; i++) {
          particles[i].x += particles[i].vx*timestep
          particles[i].y += particles[i].vy*timestep

          if(particles[i].x >= width-radius) {
            particles[i].x = width-1-radius;
            particles[i].vx *= -1;
          }
          if(particles[i].x < 0) {
            particles[i].x = 0;
            particles[i].vx *= -1;
          }
          if(particles[i].y >= height-radius) {
            particles[i].y = height-1-radius;
            particles[i].vy *= -1;
          }
          if(particles[i].y < 0) {
            particles[i].y = 0;
            particles[i].vy *= -1;
          }

        }
      }

      { // 2. update cell
        for(let i = 0; i < numParticles; i++) {
          let [prx,pry] = particles[i].prevCell;
          cells[pry][prx].filter(v => v!==i);
          let [px,py] = [Math.floor(particles[i].x/radius/8), Math.floor(particles[i].y/radius/8)];
          // console.log(`${particles[i].x}, ${px}, ${py}, ${particles[i].y}`);
          if(!cells[py][px].includes(i))
            cells[py][px].push(i);
          particles[i].prevCell = [px,py];
        }
      }

      { // 3.
        for(let zz = 0; zz < subiter; zz++ )
        // MT.process((particles,cells,radius,velocity) => {
          for(let i = 0; i < cells.length; i++) {
            for(let j = 0; j < cells[i].length; j++) {
              for(let mainCellI = 0; mainCellI < cells[i][j].length; mainCellI++) {
                for(let m = 0; m < 3; m++) {
                  for(let n = 0; n < 3; n++) {
                    if((i+m-1)>=0&&(i+m-1)<cells.length&&(j+n-1)>=0&&(j+n-1)<cells[i].length)
                    for(let addCellI = 0; addCellI < cells[i-1+m][j-1+n].length; addCellI++) {
                      let mainCell = cells[i][j][mainCellI];
                      let addCell = cells[i+m-1][j+n-1][addCellI];
                      if(mainCell!==addCell) {
                        let dx = (particles[mainCell].x-particles[addCell].x);
                        let dy = (particles[mainCell].y-particles[addCell].y)
                        let dsquared = (dx**2+dy**2);
                        if(dsquared < radius**2) {
                          let ds = Math.sqrt(dsquared);
                          let dist = (radius-ds)/2;
                          particles[mainCell].x += (dx/ds)*dist;
                          particles[mainCell].y += (dy/ds)*dist;
                          particles[addCell].x -= (dx/ds)*dist;
                          particles[addCell].y -= (dy/ds)*dist;

                          particles[mainCell].vx = -velocity*(dx/ds);
                          particles[mainCell].vy = -velocity*(dy/ds);
                          particles[mainCell].vx = velocity*(dx/ds);
                          particles[mainCell].vy = velocity*(dy/ds);
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        //   return particles
        // }, (rt) => {
        //   particles = rt;
        // })(particles,cells,radius,velocity);
      }
    }

    // create particle
    function update() {
      context.clearRect(0, 0, width, height)

      physicsUpdate(2);

      for(let p of particles) draw(p);

      requestAnimationFrame(update)
    }

    update()
  })
</script>

<template>
<div class="simulation">
  <canvas width="400" height="300"></canvas>
</div>
</template>