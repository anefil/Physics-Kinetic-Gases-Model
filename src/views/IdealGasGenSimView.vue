<script setup>
import { ref, onMounted } from 'vue'

const simulation = ref(null)

onMounted(() => {
  const canvas = document.querySelector('canvas')
  const context = canvas.getContext('2d')

  const width = canvas.width = window.innerWidth
  const height = canvas.height = window.innerHeight

  // create particles
  const particles = []
  const numParticles = 1000
  for (let i = 0; i < numParticles; i++) {
    particles.push(new Particle(width / 2, height / 2))
  }

  // create particle
  function Particle(x, y) {
    this.x = x
    this.y = y
    this.vx = Math.random() * 20 - 10
    this.vy = Math.random() * 20 - 10
    this.radius = Math.random() * 10 + 2
  }

  Particle.prototype.update = function () {
    this.x += this.vx
    this.y += this.vy

    if (this.x + this.radius > width) {
      this.x = width - this.radius
      this.vx *= -1
    }

    if (this.x - this.radius < 0) {
      this.x = this.radius
      this.vx *= -1
    }

    if (this.y + this.radius > height) {
      this.y = height - this.radius
      this.vy *= -1
    }

    if (this.y - this.radius < 0) {
      this.y = this.radius
      this.vy *= -1
    }
  }

  Particle.prototype.draw = function () {
    context.beginPath()
    context.arc(this.x, this.y, this.radius, 0, Math.PI * 2, false)
    context.fillStyle = '#000'
    context.fill()
  }

  // create particle
  function update() {
    context.clearRect(0, 0, width, height)

    for (let i = 0; i < numParticles; i++) {
      particles[i].update()
      particles[i].draw()
    }

    requestAnimationFrame(update)
  }

  update()
})
</script>

<template>
<div class="simulation">
  <canvas></canvas>
</div>
</template>