<script setup>
import * as graph from "@/lib/graph";

function add_particles_small(){
  graph.addMoleculeType(100,4,5,[125,80,80]);
}
function add_particles_big(){
  graph.addMoleculeType(100,4,12,[80,80,125]);
}
</script>

<template>
  <div class="simulation">
    <canvas id="canvas_mkt_basic" width="800" height="600"></canvas>
  </div>
  <button @click="add_particles_small" id="add_particles_small">Додати маленьких частинок</button>
  <button @click="add_particles_big" id="add_particles_big">Додати великих частинок</button>
</template>