<script setup>
  import { ref, onMounted } from 'vue'
  import * as graph from "../lib/graph.js";
  
  
  const simulation = ref(null)
  
  
  
  onMounted(() => {
    async function main() {
      await graph.init(0.05,8);
      await graph.addMoleculeType(600,4,25,[60,0,0]);
      // await graph.addMoleculeType(100,4,25,[0,0,0]);
      await graph.watchParticle(1);
      await graph.render();
    }
    main();
  }) 
</script>

<template>
<div class="simulation">
  <canvas id="canvas_mkt_basic" width="800" height="600"></canvas>
  
</div>
</template>