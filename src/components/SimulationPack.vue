<script setup>
    import { ref, onMounted } from 'vue'
    import * as graph from "@/lib/graph";
    onMounted(() => {
        document.getElementById("add_particles_small").onclick = _ => {
            graph.addMoleculeType(100,4,5,[125,80,80]);
        };
        document.getElementById("add_particles_big").onclick = _ => {
            graph.addMoleculeType(100,4,12,[80,80,125]);
        };
    });
</script>

<template>
    <div class="simulation">
        <canvas id="canvas_mkt_basic" width="800" height="600"></canvas>
    </div>
    <button id="add_particles_small">Додати маленьких частинок</button>
    <button id="add_particles_big">Додати великих частинок</button>
</template>