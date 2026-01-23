<template>
  <div class="auth-page">
    <div class="auth-card">
      <div class="text-center mb-4">
        <h4 class="fw-bold text-dark">Crea tu Cuenta</h4>
        <p class="text-muted small">Únete a WeCommerce</p>
      </div>

      <form @submit.prevent="handleRegister">
        <div class="row g-2 mb-3">
          <div class="col-6">
            <input type="text" class="form-control form-control-sm" placeholder="Usuario" v-model="userData.username" required>
          </div>
          <div class="col-6">
            <input type="tel" class="form-control form-control-sm" placeholder="Teléfono" v-model="userData.phone_number" required>
          </div>
        </div>

        <div class="mb-3">
          <input type="email" class="form-control form-control-sm" placeholder="Correo electrónico" v-model="userData.email" required>
        </div>

        <div class="row g-2 mb-4">
          <div class="col-6">
            <input type="password" class="form-control form-control-sm" placeholder="Contraseña" v-model="userData.password" required>
          </div>
          <div class="col-6">
            <input type="password" class="form-control form-control-sm" placeholder="Confirmar" v-model="confirmPassword" required>
          </div>
        </div>

        <div v-if="hasErrors" class="alert alert-danger py-1 px-2 mb-3 small text-center border-0 bg-danger bg-opacity-10 text-danger">
          {{ errorMessage }}
        </div>

        <button type="submit" class="btn btn-primary w-100 fw-bold py-2 mb-3" :disabled="authStore.isLoading">
          {{ authStore.isLoading ? '...' : 'Registrarse' }}
        </button>

        <div class="text-center border-top pt-3">
          <router-link to="/login" class="text-decoration-none small text-muted">
            ¿Ya tienes cuenta en WeCommerce? <span class="text-dark fw-bold">Inicia Sesión</span>
          </router-link>
        </div>
      </form>
    </div>
  </div>
</template>

<script>
import { ref, reactive, computed, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'

export default {
  name: 'Register',
  setup() {
    const router = useRouter()
    const authStore = useAuthStore()
    const userData = reactive({ username: '', email: '', phone_number: '', password: '' })
    const confirmPassword = ref('')
    const localError = ref('')
    const errorMessage = computed(() => localError.value || (authStore.error ? 'Error en registro' : ''))
    const hasErrors = computed(() => !!errorMessage.value)
    const handleRegister = async () => {
      localError.value = ''
      if (userData.password !== confirmPassword.value) { localError.value = 'Contraseñas no coinciden'; return }
      try { await authStore.register(userData); router.push({ name: 'EmailVerification', query: { email: userData.email } }) } catch (e) {}
    }
    onUnmounted(() => authStore.clearError())
    return { userData, confirmPassword, authStore, handleRegister, hasErrors, errorMessage }
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
  max-width: 400px;
  padding: 2.5rem;
  background: #ffffff;
  box-shadow: 0 20px 50px -12px rgba(0,0,0,0.1);
  border: 1px solid #f1f5f9;
  border-radius: 1.5rem;
}

.form-control {
  background-color: #f8fafc;
  border: 1px solid #e2e8f0;
  color: #334155;
}

.form-control:focus {
  background-color: #fff;
  border-color: #3b82f6;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}
</style>