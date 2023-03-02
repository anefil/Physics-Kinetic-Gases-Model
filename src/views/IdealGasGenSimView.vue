<script setup>
  import { ref, onMounted } from 'vue'
  import * as graph from "../lib/graph.js";
  import {initChart} from "@/lib/GasChart";
  
  
  const simulation = ref(null)



  onMounted(() => {
    async function main() {
      await graph.init(0.05,8);
      await graph.addMoleculeType(600,4,5,-2);
      await graph.addMoleculeType(100,4,25,-2);
      await graph.watchParticle(1);
      await graph.render();
    }
    main();
    initChart();
  })
</script>

<template>
<div class="simulation">
  <canvas id="canvas_mkt_basic" width="800" height="600"></canvas>
</div>
  <div>
    <svg id="root"></svg>
  </div>
</template>