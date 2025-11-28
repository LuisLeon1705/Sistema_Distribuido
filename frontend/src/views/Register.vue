<template>
  <div class="container">
    <div class="row justify-content-center">
      <div class="col-md-8 col-lg-6">
        <div class="card shadow">
          <div class="card-header text-center">
            <h4>Crear Cuenta</h4>
          </div>
          <div class="card-body">
            <form @submit.prevent="handleRegister">
              <div class="row">
                <div class="col-md-6">
                  <div class="mb-3">
                    <label for="username" class="form-label">Usuario</label>
                    <input
                      type="text"
                      class="form-control"
                      id="username"
                      v-model="userData.username"
                      :class="{ 'is-invalid': errors.username }"
                      required
                    >
                    <div v-if="errors.username" class="invalid-feedback">
                      {{ errors.username }}
                    </div>
                  </div>
                </div>
                
                <div class="col-md-6">
                  <div class="mb-3">
                    <label for="email" class="form-label">Email</label>
                    <input
                      type="email"
                      class="form-control"
                      id="email"
                      v-model="userData.email"
                      :class="{ 'is-invalid': errors.email }"
                      required
                    >
                    <div v-if="errors.email" class="invalid-feedback">
                      {{ errors.email }}
                    </div>
                  </div>
                </div>
              </div>
              
              <div class="mb-3">
                <label for="phone_number" class="form-label">Teléfono</label>
                <input
                  type="tel"
                  class="form-control"
                  id="phone_number"
                  v-model="userData.phone_number"
                  :class="{ 'is-invalid': errors.phone_number }"
                  required
                >
                <div v-if="errors.phone_number" class="invalid-feedback">
                  {{ errors.phone_number }}
                </div>
              </div>
              
              <div class="row">
                <div class="col-md-6">
                  <div class="mb-3">
                    <label for="password" class="form-label">Contraseña</label>
                    <input
                      type="password"
                      class="form-control"
                      id="password"
                      v-model="userData.password"
                      :class="{ 'is-invalid': errors.password }"
                      required
                    >
                    <div v-if="errors.password" class="invalid-feedback">
                      {{ errors.password }}
                    </div>
                  </div>
                </div>
                
                <div class="col-md-6">
                  <div class="mb-3">
                    <label for="confirmPassword" class="form-label">Confirmar Contraseña</label>
                    <input
                      type="password"
                      class="form-control"
                      id="confirmPassword"
                      v-model="confirmPassword"
                      :class="{ 'is-invalid': errors.confirmPassword }"
                      required
                    >
                    <div v-if="errors.confirmPassword" class="invalid-feedback">
                      {{ errors.confirmPassword }}
                    </div>
                  </div>
                </div>
              </div>
              
              <div v-if="authStore.error" class="alert alert-danger">
                {{ authStore.error }}
              </div>
              
              <div v-if="successMessage" class="alert alert-success">
                {{ successMessage }}
              </div>
              
              <button 
                type="submit" 
                class="btn btn-primary w-100"
                :disabled="authStore.isLoading"
              >
                <span v-if="authStore.isLoading" class="spinner-border spinner-border-sm me-2"></span>
                {{ authStore.isLoading ? 'Registrando...' : 'Registrarse' }}
              </button>
              
              <div class="text-center mt-3">
                <p class="mb-0">
                  ¿Ya tienes cuenta? 
                  <router-link to="/login" class="text-decoration-none">
                    Inicia sesión aquí
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
  name: 'Register',
  setup() {
    const router = useRouter()
    const authStore = useAuthStore()
    
    const userData = reactive({
      username: '',
      email: '',
      phone_number: '',
      password: ''
    })
    
    const confirmPassword = ref('')
    const errors = ref({})
    const successMessage = ref('')
    
    const validateForm = () => {
      errors.value = {}
      
      if (!userData.username) {
        errors.value.username = 'El usuario es requerido'
      } else if (userData.username.length < 3) {
        errors.value.username = 'El usuario debe tener al menos 3 caracteres'
      }
      
      if (!userData.email) {
        errors.value.email = 'El email es requerido'
      } else if (!/\S+@\S+\.\S+/.test(userData.email)) {
        errors.value.email = 'El email no es válido'
      }
      
      if (!userData.phone_number) {
        errors.value.phone_number = 'El teléfono es requerido'
      } else if (!/^\+?[\d\s\-\(\)]+$/.test(userData.phone_number)) {
        errors.value.phone_number = 'El teléfono no es válido'
      }
      
      if (!userData.password) {
        errors.value.password = 'La contraseña es requerida'
      } else if (userData.password.length < 6) {
        errors.value.password = 'La contraseña debe tener al menos 6 caracteres'
      }
      
      if (!confirmPassword.value) {
        errors.value.confirmPassword = 'Confirma tu contraseña'
      } else if (userData.password !== confirmPassword.value) {
        errors.value.confirmPassword = 'Las contraseñas no coinciden'
      }
      
      return Object.keys(errors.value).length === 0
    }
    
    const handleRegister = async () => {
      if (!validateForm()) return
      
      try {
        await authStore.register(userData)
        successMessage.value = 'Cuenta creada exitosamente. Hemos enviado un código de verificación a tu correo.'
        
        setTimeout(() => {
          router.push({
            name: 'EmailVerification',
            query: { email: userData.email }
          })
        }, 2000)
      } catch (error) {
        console.error('Registration error:', error)
      }
    }
    
    onUnmounted(() => {
      authStore.clearError()
    })
    
    return {
      userData,
      confirmPassword,
      errors,
      successMessage,
      authStore,
      handleRegister
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
