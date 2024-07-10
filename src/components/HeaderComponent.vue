<script setup>

</script>

<template>
  <div class="header">
    <router-link to="/" > Головна сторінка </router-link>
    <router-link to="/IdealGas" > Симуляція Ідеального Газу </router-link>
    <router-link to="/TwoGas" > Суміш газів </router-link>
  </div>
</template>

<style scoped>
.header {
  display: flex;
  flex-direction: row;
  justify-content: space-around;
  align-items: center;
  width: 100%;
  /* position: absolute; */
  /* right: 0; */
  /* top: 0; */
  padding: 20px 0;
  background-color: #f1f1f1;
  border-radius: 0 0 0 10px;
}

a {
  background-color: #eb93f3;
  color: rgb(0, 0, 0);
  border: none;
  border-radius: 0.5rem;
  padding: 0.5rem 1rem;
  font-size: 1.5rem;
  cursor: pointer;
  transition: background-color 0.2s ease-in-out;
  text-decoration: none;
}
</style>
