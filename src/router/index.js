import { createRouter, createWebHistory } from 'vue-router'
import HomeView from '../views/HomeView.vue'
import IdealGasGenSimView from '../views/IdealGasGenSimView.vue'
import TwoGasSimView from '../views/TwoGasSimView.vue'

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
    },
    {
      path: '/TwoGas',
      name: 'TwoGas',
      component: TwoGasSimView
    }
  ]
})

export default router
