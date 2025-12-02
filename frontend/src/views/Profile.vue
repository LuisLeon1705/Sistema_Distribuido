<template>
  <div class="profile">
    <div class="container">
      <div class="row justify-content-center">
        <div class="col-md-8">
          <div class="card">
            <div class="card-header">
              <h4 class="mb-0">Mi Perfil</h4>
            </div>
            <div class="card-body">
              <form @submit.prevent="handleUpdateProfile">
                <div class="row">
                  <div class="col-md-6">
                    <div class="mb-3">
                      <label for="username" class="form-label">Usuario</label>
                      <input
                        type="text"
                        class="form-control"
                        id="username"
                        v-model="profileData.username"
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
                        v-model="profileData.email"
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
                    v-model="profileData.phone_number"
                    :class="{ 'is-invalid': errors.phone_number }"
                    required
                  >
                  <div v-if="errors.phone_number" class="invalid-feedback">
                    {{ errors.phone_number }}
                  </div>
                </div>
                
                <div class="mb-3">
                  <label for="password" class="form-label">Nueva Contraseña (opcional)</label>
                  <input
                    type="password"
                    class="form-control"
                    id="password"
                    v-model="profileData.password"
                    :class="{ 'is-invalid': errors.password }"
                    placeholder="Dejar vacío para mantener la actual"
                  >
                  <div v-if="errors.password" class="invalid-feedback">
                    {{ errors.password }}
                  </div>
                </div>
                
                <div v-if="profileData.password" class="mb-3">
                  <label for="confirmPassword" class="form-label">Confirmar Nueva Contraseña</label>
                  <input
                    type="password"
                    class="form-control"
                    id="confirmPassword"
                    v-model="confirmPassword"
                    :class="{ 'is-invalid': errors.confirmPassword }"
                  >
                  <div v-if="errors.confirmPassword" class="invalid-feedback">
                    {{ errors.confirmPassword }}
                  </div>
                </div>
                
                <div v-if="authStore.error" class="alert alert-danger">
                  {{ authStore.error }}
                </div>
                
                <div v-if="successMessage" class="alert alert-success">
                  {{ successMessage }}
                </div>
                
                <div class="d-grid gap-2 d-md-flex justify-content-md-end">
                  <button 
                    type="button" 
                    class="btn btn-outline-secondary me-md-2"
                    @click="resetForm"
                  >
                    Cancelar
                  </button>
                  <button 
                    type="submit" 
                    class="btn btn-primary"
                    :disabled="authStore.isLoading"
                  >
                    <span v-if="authStore.isLoading" class="spinner-border spinner-border-sm me-2"></span>
                    {{ authStore.isLoading ? 'Guardando...' : 'Guardar Cambios' }}
                  </button>
                </div>
              </form>
            </div>
          </div>
          
          <!-- Account Information -->
          <div class="card mt-4">
            <div class="card-header">
              <h5 class="mb-0">Información de la Cuenta</h5>
            </div>
            <div class="card-body">
              <div class="row">
                <div class="col-md-6">
                  <div class="info-item">
                    <label class="text-muted small">ID de Usuario:</label>
                    <div>{{ user?.id }}</div>
                  </div>
                </div>
                
                <div class="col-md-6">
                  <div class="info-item">
                    <label class="text-muted small">Rol:</label>
                    <div>
                      <span class="badge bg-primary">{{ user?.role?.name || 'customer' }}</span>
                    </div>
                  </div>
                </div>
                
                <div class="col-md-6">
                  <div class="info-item">
                    <label class="text-muted small">Estado:</label>
                    <div>
                      <span :class="user?.is_active ? 'badge bg-success' : 'badge bg-danger'">
                        {{ user?.is_active ? 'Activo' : 'Inactivo' }}
                      </span>
                    </div>
                  </div>
                </div>
                
                <div class="col-md-6">
                  <div class="info-item">
                    <label class="text-muted small">Verificado:</label>
                    <div>
                      <span :class="user?.is_verified ? 'badge bg-success' : 'badge bg-warning'">
                        {{ user?.is_verified ? 'Verificado' : 'No verificado' }}
                      </span>
                    </div>
                  </div>
                </div>
                
                <div class="col-md-6">
                  <div class="info-item">
                    <label class="text-muted small">Miembro desde:</label>
                    <div>{{ formatDate(user?.created_at) }}</div>
                  </div>
                </div>
                
                <div class="col-md-6">
                  <div class="info-item">
                    <label class="text-muted small">Último acceso:</label>
                    <div>{{ user?.last_login_at ? formatDate(user.last_login_at) : 'Nunca' }}</div>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, reactive, computed, onMounted, onUnmounted } from 'vue'
import { useAuthStore } from '../stores/auth'

export default {
  name: 'Profile',
  setup() {
    const authStore = useAuthStore()
    
    const profileData = reactive({
      username: '',
      email: '',
      phone_number: '',
      password: ''
    })
    
    const confirmPassword = ref('')
    const errors = ref({})
    const successMessage = ref('')
    
    const user = computed(() => authStore.user)
    
    const loadUserData = () => {
      if (user.value) {
        profileData.username = user.value.username || ''
        profileData.email = user.value.email || ''
        profileData.phone_number = user.value.phone_number || ''
      }
    }
    
    const validateForm = () => {
      errors.value = {}
      
      if (!profileData.username) {
        errors.value.username = 'El usuario es requerido'
      } else if (profileData.username.length < 3) {
        errors.value.username = 'El usuario debe tener al menos 3 caracteres'
      }
      
      if (!profileData.email) {
        errors.value.email = 'El email es requerido'
      } else if (!/\S+@\S+\.\S+/.test(profileData.email)) {
        errors.value.email = 'El email no es válido'
      }
      
      if (!profileData.phone_number) {
        errors.value.phone_number = 'El teléfono es requerido'
      } else if (!/^\+?[\d\s\-\(\)]+$/.test(profileData.phone_number)) {
        errors.value.phone_number = 'El teléfono no es válido'
      }
      
      if (profileData.password) {
        if (profileData.password.length < 6) {
          errors.value.password = 'La contraseña debe tener al menos 6 caracteres'
        }
        
        if (!confirmPassword.value) {
          errors.value.confirmPassword = 'Confirma tu nueva contraseña'
        } else if (profileData.password !== confirmPassword.value) {
          errors.value.confirmPassword = 'Las contraseñas no coinciden'
        }
      }
      
      return Object.keys(errors.value).length === 0
    }
    
    const handleUpdateProfile = async () => {
      if (!validateForm()) return
      
      try {
        const updateData = {
          username: profileData.username,
          email: profileData.email,
          phone_number: profileData.phone_number
        }
        
        // Only include password if it's provided
        if (profileData.password) {
          updateData.password = profileData.password
        }
        
        await authStore.updateProfile(updateData)
        successMessage.value = 'Perfil actualizado exitosamente'
        
        // Clear password fields after successful update
        profileData.password = ''
        confirmPassword.value = ''
        
        // Clear success message after 5 seconds
        setTimeout(() => {
          successMessage.value = ''
        }, 5000)
      } catch (error) {
        console.error('Profile update error:', error)
      }
    }
    
    const resetForm = () => {
      loadUserData()
      profileData.password = ''
      confirmPassword.value = ''
      errors.value = {}
      successMessage.value = ''
    }
    
    const formatDate = (dateString) => {
      if (!dateString) return 'N/A'
      const date = new Date(dateString)
      return date.toLocaleDateString('es-ES', {
        year: 'numeric',
        month: 'long',
        day: 'numeric',
        hour: '2-digit',
        minute: '2-digit'
      })
    }
    
    onMounted(() => {
      loadUserData()
    })
    
    onUnmounted(() => {
      authStore.clearError()
    })
    
    return {
      authStore,
      profileData,
      confirmPassword,
      errors,
      successMessage,
      user,
      handleUpdateProfile,
      resetForm,
      formatDate
    }
  }
}
</script>

<style scoped>
.info-item {
  margin-bottom: 1rem;
}

.info-item label {
  display: block;
  margin-bottom: 0.25rem;
}

.card {
  border: none;
  box-shadow: 0 2px 10px rgba(0,0,0,0.1);
}

.badge {
  font-size: 0.8em;
}

.alert {
  margin-bottom: 0;
}
</style>
