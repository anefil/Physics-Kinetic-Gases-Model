<script setup>
import { RouterLink, RouterView } from 'vue-router'
</script>

<template>
<!--  Это лейаут, компонент RouterView это "окно" со всеми страницами-->
<!--  Тут есть хедер, он будет отображаться на всех страницах-->
  <header>
    хедер
    <router-link to="/">Main page</router-link>
    <br>
    <router-link to="/about">About</router-link>
  </header>

  <RouterView />
</template>

<style scoped>

</style>
