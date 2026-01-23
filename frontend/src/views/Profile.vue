<template>
  <div class="profile-page">
    <div class="container py-5">
      
      <div class="mb-5">
        <h2 class="fw-bold text-dark mb-1">Mi Perfil</h2>
        <p class="text-muted small mb-0">Gestiona tu información personal y seguridad</p>
      </div>

      <div class="row g-4">
        
        <div class="col-lg-4">
          <div class="card border-0 shadow-sm rounded-4 text-center p-4 h-100">
            <div class="card-body">
              <div class="mb-4">
                <div class="avatar-xl bg-dark text-white rounded-circle mx-auto d-flex align-items-center justify-content-center fw-bold display-4 mb-3 shadow-sm" style="width: 100px; height: 100px;">
                  {{ user?.username?.charAt(0).toUpperCase() }}
                </div>
                <h5 class="fw-bold text-dark mb-1">{{ user?.username }}</h5>
                <span class="badge bg-light text-dark border rounded-pill px-3">
                  {{ user?.role?.name || 'Cliente' }}
                </span>
              </div>

              <div class="border-top border-light py-4 text-start">  
                <div class="mb-3">
                  <small class="text-muted text-uppercase fw-bold d-block mb-1" style="font-size: 0.7rem;">Estado de Cuenta</small>
                  <div class="d-flex gap-2">
                    <span :class="user?.is_active ? 'badge bg-success bg-opacity-10 text-success border border-success border-opacity-25' : 'badge bg-danger'">
                      {{ user?.is_active ? 'Activo' : 'Inactivo' }}
                    </span>
                    <span :class="user?.is_verified ? 'badge bg-primary bg-opacity-10 text-primary border border-primary border-opacity-25' : 'badge bg-warning text-dark'">
                      {{ user?.is_verified ? 'Verificado' : 'No verificado' }}
                    </span>
                  </div>
                </div>

                <div>
                  <small class="text-muted text-uppercase fw-bold d-block mb-1" style="font-size: 0.7rem;">Miembro Desde</small>
                  <span class="text-dark">{{ formatDate(user?.created_at) }}</span>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div class="col-lg-8">
          <div class="card border-0 shadow-sm rounded-4 p-4">
            <div class="card-header bg-white border-0 px-0 pb-3">
              <h5 class="fw-bold text-dark mb-0">Editar Información</h5>
            </div>
            
            <div class="card-body px-0">
              <form @submit.prevent="handleUpdateProfile">
                
                <h6 class="text-muted small fw-bold text-uppercase mb-3">Datos Básicos</h6>
                <div class="row g-3 mb-4">
                  <div class="col-md-6">
                    <label class="form-label small text-muted">Usuario</label>
                    <input 
                      type="text" 
                      class="form-control bg-light border-0" 
                      v-model="profileData.username"
                      :class="{ 'is-invalid': errors.username }"
                      required
                    >
                    <div v-if="errors.username" class="invalid-feedback">{{ errors.username }}</div>
                  </div>
                  
                  <div class="col-md-6">
                    <label class="form-label small text-muted">Email</label>
                    <input 
                      type="email" 
                      class="form-control bg-light border-0" 
                      v-model="profileData.email"
                      :class="{ 'is-invalid': errors.email }"
                      required
                    >
                    <div v-if="errors.email" class="invalid-feedback">{{ errors.email }}</div>
                  </div>

                  <div class="col-md-12">
                    <label class="form-label small text-muted">Teléfono</label>
                    <input 
                      type="tel" 
                      class="form-control bg-light border-0" 
                      v-model="profileData.phone_number"
                      :class="{ 'is-invalid': errors.phone_number }"
                      required
                    >
                    <div v-if="errors.phone_number" class="invalid-feedback">{{ errors.phone_number }}</div>
                  </div>
                </div>

                <hr class="border-light my-4">

                <h6 class="text-muted small fw-bold text-uppercase mb-3">Seguridad</h6>
                <div class="row g-3 mb-4">
                  <div class="col-md-6">
                    <label class="form-label small text-muted">Nueva Contraseña</label>
                    <input 
                      type="password" 
                      class="form-control bg-light border-0" 
                      v-model="profileData.password"
                      placeholder="Dejar en blanco para mantener"
                      :class="{ 'is-invalid': errors.password }"
                    >
                    <div v-if="errors.password" class="invalid-feedback">{{ errors.password }}</div>
                  </div>
                  
                  <div class="col-md-6">
                    <label class="form-label small text-muted">Confirmar Contraseña</label>
                    <input 
                      type="password" 
                      class="form-control bg-light border-0" 
                      v-model="confirmPassword"
                      :class="{ 'is-invalid': errors.confirmPassword }"
                      :disabled="!profileData.password"
                    >
                    <div v-if="errors.confirmPassword" class="invalid-feedback">{{ errors.confirmPassword }}</div>
                  </div>
                </div>

                <div v-if="authStore.error" class="alert alert-danger border-0 bg-danger bg-opacity-10 text-danger py-2 small mb-3">
                  <i class="fas fa-exclamation-circle me-2"></i>{{ authStore.error }}
                </div>
                
                <div v-if="successMessage" class="alert alert-success border-0 bg-success bg-opacity-10 text-success py-2 small mb-3">
                  <i class="fas fa-check-circle me-2"></i>{{ successMessage }}
                </div>

                <div class="d-flex justify-content-end gap-2 mt-2">
                  <button 
                    type="button" 
                    class="btn btn-outline-dark rounded-pill px-4" 
                    @click="resetForm"
                  >
                    Restaurar
                  </button>
                  <button 
                    type="submit" 
                    class="btn btn-dark rounded-pill px-4 shadow-sm"
                    :disabled="authStore.isLoading"
                  >
                    <span v-if="authStore.isLoading" class="spinner-border spinner-border-sm me-2"></span>
                    Guardar Cambios
                  </button>
                </div>

              </form>
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
      if (!profileData.username) errors.value.username = 'Requerido'
      else if (profileData.username.length < 3) errors.value.username = 'Mínimo 3 caracteres'
      
      if (!profileData.email) errors.value.email = 'Requerido'
      else if (!/\S+@\S+\.\S+/.test(profileData.email)) errors.value.email = 'Email inválido'
      
      if (!profileData.phone_number) errors.value.phone_number = 'Requerido'
      else if (!/^\+?[\d\s\-\(\)]+$/.test(profileData.phone_number)) errors.value.phone_number = 'Teléfono inválido'
      
      if (profileData.password) {
        if (profileData.password.length < 6) errors.value.password = 'Mínimo 6 caracteres'
        if (!confirmPassword.value) errors.value.confirmPassword = 'Confirma la contraseña'
        else if (profileData.password !== confirmPassword.value) errors.value.confirmPassword = 'No coinciden'
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
        if (profileData.password) updateData.password = profileData.password
        
        await authStore.updateProfile(updateData)
        successMessage.value = 'Perfil actualizado correctamente'
        
        profileData.password = ''
        confirmPassword.value = ''
        
        setTimeout(() => { successMessage.value = '' }, 5000)
      } catch (error) { console.error('Profile update error:', error) }
    }
    
    const resetForm = () => {
      loadUserData()
      profileData.password = ''
      confirmPassword.value = ''
      errors.value = {}
      successMessage.value = ''
    }
    
    const formatDate = (dateString) => {
      const date = new Date(dateString);
      const time = new Date(date.getTime() - (date.getTimezoneOffset() * 60000));
      return time.toLocaleDateString('es-ES', { year: 'numeric', month: 'long', day: 'numeric', hour: '2-digit', minute: '2-digit' });
    };
    
    onMounted(() => { loadUserData() })
    onUnmounted(() => { authStore.clearError() })
    
    return {
      authStore, profileData, confirmPassword, errors, successMessage, user,
      handleUpdateProfile, resetForm, formatDate
    }
  }
}
</script>

<style scoped>
.profile-page {
  background-color: #ffffff;
  min-height: 100vh;
}

.form-control {
  transition: all 0.2s;
}

.form-control:focus {
  background-color: #ffffff;
  border: 1px solid #0f172a !important;
  box-shadow: none;
}

.avatar-xl {
  font-size: 2.5rem;
  background: linear-gradient(135deg, #1e293b 0%, #0f172a 100%);
}
</style>