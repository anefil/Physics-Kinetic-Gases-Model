import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import IdealGasGenSimView from '../views/IdealGasGenSimView.vue'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      name: 'home',
      component: HomeView
    },
    {
      path: '/IdealGas',
      name: 'IdealGas',
      component: IdealGasGenSimView
    }
  ]
})

export default router
