<template>
  <div class="container">
    <div class="row justify-content-center">
      <div class="col-md-6 col-lg-4">
        <div class="card shadow">
          <div class="card-header text-center">
            <h4>Iniciar Sesión</h4>
          </div>
          <div class="card-body">
            <form @submit.prevent="handleLogin">
              <div class="mb-3">
                <label for="email" class="form-label">Email</label>
                <input
                  type="email"
                  class="form-control"
                  id="email"
                  v-model="credentials.email"
                  :class="{ 'is-invalid': errors.email }"
                  required
                >
                <div v-if="errors.email" class="invalid-feedback">
                  {{ errors.email }}
                </div>
              </div>
              
              <div class="mb-3">
                <label for="password" class="form-label">Contraseña</label>
                <input
                  type="password"
                  class="form-control"
                  id="password"
                  v-model="credentials.password"
                  :class="{ 'is-invalid': errors.password }"
                  required
                >
                <div v-if="errors.password" class="invalid-feedback">
                  {{ errors.password }}
                </div>
              </div>
              
              <div v-if="authStore.error" class="alert alert-danger">
                {{ authStore.error }}
              </div>
              
              <button 
                type="submit" 
                class="btn btn-primary w-100"
                :disabled="authStore.isLoading"
              >
                <span v-if="authStore.isLoading" class="spinner-border spinner-border-sm me-2"></span>
                {{ authStore.isLoading ? 'Iniciando sesión...' : 'Iniciar Sesión' }}
              </button>
              
              <div class="text-center mt-3">
                <p class="mb-0">
                  ¿No tienes cuenta? 
                  <router-link to="/register" class="text-decoration-none">
                    Regístrate aquí
                  </router-link>
                </p>
              </div>
            </form>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, reactive, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'

export default {
  name: 'Login',
  setup() {
    const router = useRouter()
    const authStore = useAuthStore()
    
    const credentials = reactive({
      email: '',
      password: ''
    })
    
    const errors = ref({})
    
    const validateForm = () => {
      errors.value = {}
      
      if (!credentials.email) {
        errors.value.email = 'El email es requerido'
      } else if (!/\S+@\S+\.\S+/.test(credentials.email)) {
        errors.value.email = 'El email no es válido'
      }
      
      if (!credentials.password) {
        errors.value.password = 'La contraseña es requerida'
      } else if (credentials.password.length < 6) {
        errors.value.password = 'La contraseña debe tener al menos 6 caracteres'
      }
      
      return Object.keys(errors.value).length === 0
    }
    
    const handleLogin = async () => {
      if (!validateForm()) return
      
      try {
        await authStore.login(credentials)
        // Redirigir según el rol del usuario
        if (authStore.isStaff) {
          router.push('/admin/products')
        } else {
          router.push('/')
        }
      } catch (error) {
        console.error('Login error:', error)
      }
    }
    
    onUnmounted(() => {
      authStore.clearError()
    })
    
    return {
      credentials,
      errors,
      authStore,
      handleLogin
    }
  }
}
</script>

<style scoped>
.card {
  border: none;
  border-radius: 10px;
}

.card-header {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border-radius: 10px 10px 0 0 !important;
}

.btn-primary {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border: none;
}

.btn-primary:hover {
  background: linear-gradient(135deg, #5a67c7 0%, #684a8f 100%);
}
</style>
