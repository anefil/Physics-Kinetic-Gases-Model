<script setup>
import { RouterView } from 'vue-router'
import HeaderComponent from "@/components/HeaderComponent.vue";
</script>

<template>
<!--  Это лейаут, компонент RouterView это "окно" со всеми страницами-->
<!--  Тут есть хедер, он будет отображаться на всех страницах-->
  <HeaderComponent />

  <RouterView />
</template>

<style scoped>

</style>
