<template>
  <div class="user-management">
    <div class="container-fluid">
      <div class="d-flex justify-content-between align-items-center mb-4">
        <h2>Gestión de Usuarios</h2>
        <button class="btn btn-primary" @click="showCreateModal">
          <i class="fas fa-user-plus me-2"></i>
          Nuevo Usuario
        </button>
      </div>
      
      <!-- Filters -->
      <div class="card mb-4">
        <div class="card-body">
          <div class="row">
            <div class="col-md-3">
              <select v-model="filters.role" @change="applyFilters" class="form-select">
                <option value="">Todos los roles</option>
                <option value="admin">Administradores</option>
                <option value="customer">Clientes</option>
              </select>
            </div>
            <div class="col-md-3">
              <select v-model="filters.status" @change="applyFilters" class="form-select">
                <option value="">Todos los estados</option>
                <option value="true">Activos</option>
                <option value="false">Inactivos</option>
              </select>
            </div>
            <div class="col-md-4">
              <input 
                type="text" 
                v-model="filters.search" 
                @input="applyFilters"
                placeholder="Buscar por usuario, email..." 
                class="form-control"
              >
            </div>
            <div class="col-md-2">
              <button @click="clearFilters" class="btn btn-outline-secondary w-100">
                Limpiar
              </button>
            </div>
          </div>
        </div>
      </div>
      
      <!-- Users Table -->
      <div class="card">
        <div class="card-body">
          <!-- Loading State -->
          <div v-if="isLoading" class="text-center py-4">
            <div class="spinner-border" role="status">
              <span class="visually-hidden">Cargando...</span>
            </div>
          </div>
          
          <!-- Error State -->
          <div v-else-if="error" class="alert alert-danger">
            {{ error }}
          </div>
          
          <!-- Users Table -->
          <div v-else class="table-responsive">
            <table class="table table-hover">
              <thead>
                <tr>
                  <th>ID</th>
                  <th>Usuario</th>
                  <th>Email</th>
                  <th>Teléfono</th>
                  <th>Rol</th>
                  <th>Estado</th>
                  <th>Fecha</th>
                  <th>Acciones</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="user in users" :key="user.id">
                  <td>{{ user.id }}</td>
                  <td>
                    <div class="fw-medium">{{ user.username }}</div>
                    <small v-if="user.is_verified" class="text-success">
                      <i class="fas fa-check-circle"></i> Verificado
                    </small>
                    <small v-else class="text-warning">
                      <i class="fas fa-exclamation-circle"></i> No verificado
                    </small>
                  </td>
                  <td>{{ user.email }}</td>
                  <td>{{ user.phone_number }}</td>
                  <td>
                    <span 
                      class="badge"
                      :class="user.role?.name === 'admin' ? 'bg-danger' : 'bg-primary'"
                    >
                      {{ user.role?.name || 'customer' }}
                    </span>
                  </td>
                  <td>
                    <span 
                      class="badge"
                      :class="user.is_active ? 'bg-success' : 'bg-secondary'"
                    >
                      {{ user.is_active ? 'Activo' : 'Inactivo' }}
                    </span>
                  </td>
                  <td>
                    <div>{{ formatDate(user.created_at) }}</div>
                    <small v-if="user.last_login_at" class="text-muted">
                      Último acceso: {{ formatDate(user.last_login_at) }}
                    </small>
                  </td>
                  <td>
                    <div class="btn-group btn-group-sm">
                      <button 
                        class="btn btn-outline-primary"
                        @click="editUser(user)"
                        title="Editar"
                      >
                        <i class="fas fa-edit"></i>
                      </button>
                      <button 
                        class="btn btn-outline-danger"
                        @click="deleteUser(user.id)"
                        title="Eliminar"
                        :disabled="user.id === currentUserId"
                      >
                        <i class="fas fa-trash"></i>
                      </button>
                    </div>
                  </td>
                </tr>
              </tbody>
            </table>
            
            <!-- Empty State -->
            <div v-if="users.length === 0" class="text-center py-4">
              <i class="fas fa-users fa-3x text-muted mb-3"></i>
              <h5>No se encontraron usuarios</h5>
              <p class="text-muted">Ajusta los filtros para ver usuarios</p>
            </div>
          </div>
        </div>
      </div>
    </div>
    
    <!-- User Modal -->
    <div class="modal fade" id="userModal" tabindex="-1">
      <div class="modal-dialog modal-lg">
        <div class="modal-content">
          <div class="modal-header">
            <h5 class="modal-title">
              {{ isEditing ? 'Editar Usuario' : 'Nuevo Usuario' }}
            </h5>
            <button type="button" class="btn-close" data-bs-dismiss="modal"></button>
          </div>
          <form @submit.prevent="saveUser">
            <div class="modal-body">
              <div class="row">
                <div class="col-md-6">
                  <div class="mb-3">
                    <label class="form-label">Usuario *</label>
                    <input 
                      type="text" 
                      v-model="userForm.username"
                      class="form-control"
                      required
                    >
                  </div>
                </div>
                <div class="col-md-6">
                  <div class="mb-3">
                    <label class="form-label">Email *</label>
                    <input 
                      type="email" 
                      v-model="userForm.email"
                      class="form-control"
                      required
                    >
                  </div>
                </div>
              </div>
              
              <div class="row">
                <div class="col-md-6">
                  <div class="mb-3">
                    <label class="form-label">Teléfono *</label>
                    <input 
                      type="tel" 
                      v-model="userForm.phone_number"
                      class="form-control"
                      required
                    >
                  </div>
                </div>
                <div class="col-md-6">
                  <div class="mb-3">
                    <label class="form-label">Rol</label>
                    <select v-model="userForm.role_name" class="form-select">
                      <option value="customer">Cliente</option>
                      <option value="admin">Administrador</option>
                    </select>
                  </div>
                </div>
              </div>
              
              <div class="mb-3">
                <label class="form-label">
                  {{ isEditing ? 'Nueva Contraseña (opcional)' : 'Contraseña *' }}
                </label>
                <input 
                  type="password" 
                  v-model="userForm.password"
                  class="form-control"
                  :required="!isEditing"
                  :placeholder="isEditing ? 'Dejar vacío para mantener la actual' : ''"
                >
              </div>
              
              <div class="row">
                <div class="col-md-6">
                  <div class="mb-3">
                    <div class="form-check">
                      <input 
                        class="form-check-input" 
                        type="checkbox" 
                        v-model="userForm.is_active"
                        id="isActive"
                      >
                      <label class="form-check-label" for="isActive">
                        Usuario activo
                      </label>
                    </div>
                  </div>
                </div>
                <div class="col-md-6">
                  <div class="mb-3">
                    <div class="form-check">
                      <input 
                        class="form-check-input" 
                        type="checkbox" 
                        v-model="userForm.is_verified"
                        id="isVerified"
                      >
                      <label class="form-check-label" for="isVerified">
                        Usuario verificado
                      </label>
                    </div>
                  </div>
                </div>
              </div>
              
              <div v-if="formError" class="alert alert-danger">
                {{ formError }}
              </div>
            </div>
            <div class="modal-footer">
              <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">
                Cancelar
              </button>
              <button type="submit" class="btn btn-primary" :disabled="isSaving">
                <span v-if="isSaving" class="spinner-border spinner-border-sm me-2"></span>
                {{ isSaving ? 'Guardando...' : 'Guardar' }}
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, reactive, computed, onMounted } from 'vue'
import { useAuthStore } from '../stores/auth'
import { userService } from '../services/api'

export default {
  name: 'UserManagement',
  setup() {
    const authStore = useAuthStore()
    const users = ref([])
    const isLoading = ref(false)
    const isSaving = ref(false)
    const error = ref(null)
    const formError = ref(null)
    const isEditing = ref(false)
    
    const filters = reactive({
      role: '',
      status: '',
      search: ''
    })
    
    const userForm = reactive({
      id: null,
      username: '',
      email: '',
      phone_number: '',
      password: '',
      role_name: 'customer',
      is_active: true,
      is_verified: false
    })
    
    const currentUserId = computed(() => authStore.user?.id)
    
    const fetchUsers = async () => {
      isLoading.value = true
      error.value = null
      try {
        const filterParams = {}
        if (filters.role) filterParams.role = filters.role
        if (filters.status !== '') filterParams.is_active = filters.status === 'true'
        if (filters.search) {
          filterParams.username = filters.search
          // Note: The API might not support searching by email, but we can try
        }
        
        users.value = await userService.getUsers(filterParams)
      } catch (err) {
        error.value = 'Error al cargar usuarios'
        console.error('Error fetching users:', err)
      } finally {
        isLoading.value = false
      }
    }
    
    const formatDate = (dateString) => {
      if (!dateString) return 'N/A'
      const date = new Date(dateString)
      return date.toLocaleDateString('es-ES', {
        year: 'numeric',
        month: 'short',
        day: 'numeric'
      })
    }
    
    const showCreateModal = () => {
      isEditing.value = false
      resetForm()
      const modal = new bootstrap.Modal(document.getElementById('userModal'))
      modal.show()
    }
    
    const editUser = (user) => {
      isEditing.value = true
      userForm.id = user.id
      userForm.username = user.username
      userForm.email = user.email
      userForm.phone_number = user.phone_number
      userForm.password = ''
      userForm.role_name = user.role?.name || 'customer'
      userForm.is_active = user.is_active
      userForm.is_verified = user.is_verified
      
      const modal = new bootstrap.Modal(document.getElementById('userModal'))
      modal.show()
    }
    
    const saveUser = async () => {
      isSaving.value = true
      formError.value = null
      
      try {
        const data = {
          username: userForm.username,
          email: userForm.email,
          phone_number: userForm.phone_number,
          role_name: userForm.role_name,
          is_active: userForm.is_active,
          is_verified: userForm.is_verified
        }
        
        // Only include password if it's provided
        if (userForm.password) {
          data.password = userForm.password
        }
        
        if (isEditing.value) {
          await userService.updateUser(userForm.id, data)
        } else {
          // Password is required for new users
          if (!userForm.password) {
            formError.value = 'La contraseña es requerida para nuevos usuarios'
            return
          }
          data.password = userForm.password
          await userService.createUser(data)
        }
        
        await fetchUsers()
        
        const modal = bootstrap.Modal.getInstance(document.getElementById('userModal'))
        modal.hide()
      } catch (err) {
        formError.value = err.response?.data?.detail || 'Error al guardar usuario'
        console.error('Error saving user:', err)
      } finally {
        isSaving.value = false
      }
    }
    
    const deleteUser = async (userId) => {
      if (userId === currentUserId.value) {
        alert('No puedes eliminar tu propia cuenta')
        return
      }
      
      if (confirm('¿Estás seguro de que deseas eliminar este usuario?')) {
        try {
          await userService.deleteUser(userId)
          await fetchUsers()
        } catch (err) {
          error.value = 'Error al eliminar usuario'
          console.error('Error deleting user:', err)
        }
      }
    }
    
    const resetForm = () => {
      userForm.id = null
      userForm.username = ''
      userForm.email = ''
      userForm.phone_number = ''
      userForm.password = ''
      userForm.role_name = 'customer'
      userForm.is_active = true
      userForm.is_verified = false
      formError.value = null
    }
    
    const applyFilters = () => {
      fetchUsers()
    }
    
    const clearFilters = () => {
      filters.role = ''
      filters.status = ''
      filters.search = ''
      fetchUsers()
    }
    
    onMounted(() => {
      fetchUsers()
    })
    
    return {
      users,
      isLoading,
      isSaving,
      error,
      formError,
      isEditing,
      filters,
      userForm,
      currentUserId,
      formatDate,
      showCreateModal,
      editUser,
      saveUser,
      deleteUser,
      applyFilters,
      clearFilters
    }
  }
}
</script>

<style scoped>
.table th {
  border-top: none;
  font-weight: 600;
}

.btn-group-sm > .btn {
  padding: 0.25rem 0.5rem;
}

.modal-body {
  max-height: 80vh;
  overflow-y: auto;
}

.badge {
  font-size: 0.8em;
}
</style>
