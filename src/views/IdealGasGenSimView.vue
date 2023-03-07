<script setup>
  import { ref, onMounted } from 'vue'
  import * as graph from "@/lib/graph.js";
  import {initChart} from "@/lib/GasChart";
  import SimulationPack from "@/components/SimulationPack.vue";
  
  
  const simulation = ref(null)



  onMounted(() => {
    async function main() {
      await graph.init(0.05,8);
      await graph.addMoleculeType(1200,4,5,[120,30,30]);
      // await graph.addMoleculeType(100,4,25,-2);
      await graph.watchParticle(1);
      await graph.render();
    }
    main();
    initChart();
  })
</script>

<template>
<SimulationPack></SimulationPack>
  <div>
    <svg id="root"></svg>
  </div>
</template>