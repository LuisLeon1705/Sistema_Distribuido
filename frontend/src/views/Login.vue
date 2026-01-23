<template>
  <div class="auth-page">
    <div class="auth-card">
      <div class="text-center mb-4">
        <h4 class="fw-bold text-dark">Bienvenido</h4>
        <p class="text-muted small">Ingresa a tu cuenta WeCommerce</p>
      </div>

      <form @submit.prevent="handleLogin">
        <div class="mb-3">
          <label class="form-label small text-muted fw-bold">Email</label>
          <input 
            type="email" 
            class="form-control" 
            v-model="credentials.email" 
            required
          >
        </div>

        <div class="mb-4">
          <label class="form-label small text-muted fw-bold">Contraseña</label>
          <input 
            type="password" 
            class="form-control" 
            v-model="credentials.password" 
            required
          >
        </div>

        <div v-if="authStore.error" class="alert alert-danger py-2 small text-center mb-3 border-0 bg-danger bg-opacity-10 text-danger">
          {{ authStore.error }}
        </div>

        <button type="submit" class="btn btn-dark w-100 fw-bold py-2 mb-4" :disabled="authStore.isLoading">
          {{ authStore.isLoading ? '...' : 'Iniciar Sesión' }}
        </button>

        <div class="text-center pt-3 border-top">
          <router-link to="/register" class="text-decoration-none small text-muted">
            ¿No tienes cuenta en WeCommerce? <span class="text-primary fw-bold">Regístrate</span>
          </router-link>
        </div>
      </form>
    </div>
  </div>
</template>

<script>
import { reactive, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'

export default {
  name: 'Login',
  setup() {
    const router = useRouter()
    const authStore = useAuthStore()
    const credentials = reactive({ email: '', password: '' })
    const handleLogin = async () => {
      try {
        await authStore.login(credentials)
        router.push(authStore.isStaff ? '/admin/products' : '/')
      } catch (e) {}
    }
    onUnmounted(() => authStore.clearError())
    return { credentials, authStore, handleLogin }
  }
}
</script>

<style scoped>
.auth-page {
  background-color: #ffffff;
  min-height: calc(100vh - 76px);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 1rem;
}

.auth-card {
  width: 100%;
  max-width: 380px;
  padding: 2.5rem;
  background: #ffffff;
  box-shadow: 0 10px 40px -10px rgba(0,0,0,0.08);
  border: 1px solid #f1f5f9;
  border-radius: 1.5rem;
}

.form-control {
  background-color: #f8fafc;
  border: 1px solid #e2e8f0;
  color: #0f172a;
  padding: 0.7rem 1rem;
  transition: all 0.2s;
}

.form-control:focus {
  background-color: #ffffff;
  border-color: #0f172a;
  box-shadow: 0 0 0 4px rgba(15, 23, 42, 0.05);
}
</style>