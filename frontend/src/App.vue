<template>
  <div id="app">
    <Navbar />
    <main class="container-fluid mt-3">
      <router-view />
    </main>
  </div>
</template>

<script>
import { onMounted } from 'vue'
import { useAuthStore } from './stores/auth'
import Navbar from './components/Navbar.vue'

export default {
  name: 'App',
  components: {
    Navbar
  },
  setup() {
    const authStore = useAuthStore()

    onMounted(async () => {
      await authStore.initializeAuth()
    })

    return {}
  }
}
</script>

<style>
#app {
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  min-height: 100vh;
}

.fade-enter-active, .fade-leave-active {
  transition: opacity 0.5s;
}

.fade-enter-from, .fade-leave-to {
  opacity: 0;
}

.btn-primary {
  --bs-btn-bg: #0d6efd;
  --bs-btn-border-color: #0d6efd;
}

.btn-primary:hover {
  --bs-btn-bg: #0b5ed7;
  --bs-btn-border-color: #0a58ca;
}
</style>
