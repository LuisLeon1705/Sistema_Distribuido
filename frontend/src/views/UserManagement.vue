<template>
  <div class="user-management">
    <div class="container">
      <div class="d-flex justify-content-between align-items-center mb-4">
        <h2>Gestión de Usuarios</h2>
        <button class="btn btn-primary" @click="showCreateModal">
          <i class="fas fa-user-plus me-2"></i>
          Nuevo Usuario
        </button>
      </div>

      <!-- Stats Cards -->
      <div class="row mb-4">
        <div class="col-md-3">
          <div class="card bg-primary text-white">
            <div class="card-body">
              <h6 class="card-title">Total Usuarios</h6>
              <h3>{{ users.length }}</h3>
            </div>
          </div>
        </div>
        <div class="col-md-3">
          <div class="card bg-danger text-white">
            <div class="card-body">
              <h6 class="card-title">Administradores</h6>
              <h3>{{ getUsersByRole(1).length }}</h3>
            </div>
          </div>
        </div>
        <div class="col-md-3">
          <div class="card bg-success text-white">
            <div class="card-body">
              <h6 class="card-title">Clientes</h6>
              <h3>{{ getUsersByRole(2).length }}</h3>
            </div>
          </div>
        </div>
        <div class="col-md-3">
          <div class="card bg-info text-white">
            <div class="card-body">
              <h6 class="card-title">Inventario</h6>
              <h3>{{ getUsersByRole(3).length }}</h3>
            </div>
          </div>
        </div>
      </div>

      <!-- Filters -->
      <div class="card mb-4">
        <div class="card-body">
          <div class="row g-2">
            <div class="col-md-3">
              <select v-model="filters.role" class="form-select">
                <option value="">Todos los roles</option>
                <option value="1">Admin</option>
                <option value="2">Cliente</option>
                <option value="3">Inventario</option>
              </select>
            </div>
            <div class="col-md-3">
              <select v-model="filters.status" class="form-select">
                <option value="">Todos los estados</option>
                <option value="true">Activos</option>
                <option value="false">Inactivos</option>
              </select>
            </div>
            <div class="col-md-4">
              <input 
                type="text" 
                v-model="filters.search" 
                placeholder="Buscar por nombre, email o teléfono..." 
                class="form-control"
              >
            </div>
            <div class="col-md-2">
              <button @click="clearFilters" class="btn btn-outline-secondary w-100">Limpiar</button>
            </div>
          </div>
        </div>
      </div>

      <!-- Users Table -->
      <div class="card">
        <div class="card-body">
          <div v-if="isLoading" class="text-center py-4">
            <div class="spinner-border"></div>
            <p class="mt-2">Cargando usuarios...</p>
          </div>
          <div v-else-if="error" class="alert alert-danger">{{ error }}</div>
          <div v-else class="table-responsive">
            <table class="table table-hover align-middle">
              <thead>
                <tr>
                  <th>Usuario</th>
                  <th>Email</th>
                  <th>Teléfono</th>
                  <th>Rol</th>
                  <th>Estado</th>
                  <th>Verificado</th>
                  <th>Fecha Registro</th>
                  <th>Acciones</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="user in paginatedUsers" :key="user.id">
                  <td>
                    <div class="fw-medium">{{ user.username }}</div>
                    <small class="text-muted">ID: {{ user.id.substring(0, 8) }}...</small>
                  </td>
                  <td>{{ user.email }}</td>
                  <td>{{ user.phone_number }}</td>
                  <td>
                    <span class="badge" :class="getRoleBadgeClass(user.role?.id)">
                      {{ getRoleName(user.role?.name) }}
                    </span>
                  </td>
                  <td>
                    <span class="badge" :class="user.is_active ? 'bg-success' : 'bg-secondary'">
                      {{ user.is_active ? 'Activo' : 'Inactivo' }}
                    </span>
                  </td>
                  <td>
                    <i class="fas" :class="user.is_verified ? 'fa-check-circle text-success' : 'fa-times-circle text-muted'"></i>
                  </td>
                  <td>{{ formatDate(user.created_at) }}</td>
                  <td>
                    <div class="btn-group btn-group-sm">
                      <button class="btn btn-outline-primary" @click="editUser(user)" title="Editar">
                        <i class="fas fa-edit"></i>
                      </button>
                      <button class="btn btn-outline-info" @click="viewUserDetails(user)" title="Ver detalles">
                        <i class="fas fa-eye"></i>
                      </button>
                      <button 
                        class="btn btn-outline-danger" 
                        @click="toggleUserStatus(user)" 
                        :title="user.is_active ? 'Desactivar' : 'Activar'"
                      >
                        <i class="fas" :class="user.is_active ? 'fa-ban' : 'fa-check'"></i>
                      </button>
                    </div>
                  </td>
                </tr>
              </tbody>
            </table>
            
            <div v-if="filteredUsers.length === 0" class="text-center py-4">
              <i class="fas fa-users fa-3x text-muted mb-3"></i>
              <h5>No se encontraron usuarios</h5>
              <p class="text-muted">Ajusta los filtros o crea un nuevo usuario</p>
            </div>

            <!-- Pagination -->
            <nav v-if="!isLoading && filteredUsers.length > 0 && totalPages > 1" aria-label="User pagination" class="mt-3">
              <ul class="pagination justify-content-center mb-0">
                <li class="page-item" :class="{ disabled: currentPage === 1 }">
                  <button class="page-link" @click="goToPage(currentPage - 1)" :disabled="currentPage === 1">
                    <i class="fas fa-chevron-left"></i>
                  </button>
                </li>
                <li v-for="page in totalPages" :key="page" class="page-item" :class="{ active: currentPage === page }">
                  <button class="page-link" @click="goToPage(page)">{{ page }}</button>
                </li>
                <li class="page-item" :class="{ disabled: currentPage === totalPages }">
                  <button class="page-link" @click="goToPage(currentPage + 1)" :disabled="currentPage === totalPages">
                    <i class="fas fa-chevron-right"></i>
                  </button>
                </li>
              </ul>
            </nav>
            <p v-if="filteredUsers.length > itemsPerPage" class="text-center text-muted small mt-2 mb-0">
              Mostrando {{ (currentPage - 1) * itemsPerPage + 1 }} - {{ Math.min(currentPage * itemsPerPage, filteredUsers.length) }} de {{ filteredUsers.length }} usuarios
            </p>
          </div>
        </div>
      </div>
    </div>

    <!-- User Form Modal -->
    <div class="modal fade" id="userModal" tabindex="-1">
      <div class="modal-dialog">
        <div class="modal-content">
          <div class="modal-header">
            <h5 class="modal-title">{{ isEditing ? 'Editar Usuario' : 'Nuevo Usuario' }}</h5>
            <button type="button" class="btn-close" data-bs-dismiss="modal"></button>
          </div>
          <form @submit.prevent="saveUser">
            <div class="modal-body">
              <div class="mb-3">
                <label class="form-label">Nombre de Usuario *</label>
                <input type="text" v-model="userForm.username" class="form-control" required :disabled="isEditing">
              </div>
              <div class="mb-3">
                <label class="form-label">Email *</label>
                <input type="email" v-model="userForm.email" class="form-control" required>
              </div>
              <div class="mb-3">
                <label class="form-label">Teléfono *</label>
                <input type="tel" v-model="userForm.phone_number" class="form-control" required>
              </div>
              <div class="mb-3" v-if="!isEditing">
                <label class="form-label">Contraseña *</label>
                <input type="password" v-model="userForm.password" class="form-control" required>
              </div>
              <div class="mb-3">
                <label class="form-label">Rol *</label>
                <select v-model.number="userForm.role_id" class="form-select" required>
                  <option :value="1">Admin</option>
                  <option :value="2">Cliente</option>
                  <option :value="3">Inventario</option>
                </select>
              </div>
              <div class="mb-3">
                <div class="form-check form-switch">
                  <input class="form-check-input" type="checkbox" v-model="userForm.is_active" id="isActive">
                  <label class="form-check-label" for="isActive">Usuario Activo</label>
                </div>
              </div>
              <div class="mb-3">
                <div class="form-check form-switch">
                  <input class="form-check-input" type="checkbox" v-model="userForm.is_verified" id="isVerified">
                  <label class="form-check-label" for="isVerified">Usuario Verificado</label>
                </div>
              </div>
              <div v-if="formError" class="alert alert-danger">{{ formError }}</div>
            </div>
            <div class="modal-footer">
              <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Cancelar</button>
              <button type="submit" class="btn btn-primary" :disabled="isSaving">
                <span v-if="isSaving" class="spinner-border spinner-border-sm me-2"></span>
                Guardar
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>

    <!-- User Details Modal -->
    <div class="modal fade" id="userDetailsModal" tabindex="-1">
      <div class="modal-dialog">
        <div class="modal-content" v-if="selectedUser">
          <div class="modal-header">
            <h5 class="modal-title">Detalles de Usuario</h5>
            <button type="button" class="btn-close" data-bs-dismiss="modal"></button>
          </div>
          <div class="modal-body">
            <div class="row mb-3">
              <div class="col-12">
                <h6 class="text-muted">Información Personal</h6>
                <p class="mb-1"><strong>Usuario:</strong> {{ selectedUser.username }}</p>
                <p class="mb-1"><strong>Email:</strong> {{ selectedUser.email }}</p>
                <p class="mb-1"><strong>Teléfono:</strong> {{ selectedUser.phone_number }}</p>
                <p class="mb-1"><strong>ID:</strong> <code>{{ selectedUser.id }}</code></p>
              </div>
            </div>
            <hr>
            <div class="row mb-3">
              <div class="col-12">
                <h6 class="text-muted">Estado y Permisos</h6>
                <p class="mb-1">
                  <strong>Rol:</strong> 
                  <span class="badge" :class="getRoleBadgeClass(selectedUser.role?.id)">
                    {{ getRoleName(selectedUser.role?.name) }}
                  </span>
                </p>
                <p class="mb-1">
                  <strong>Estado:</strong> 
                  <span class="badge" :class="selectedUser.is_active ? 'bg-success' : 'bg-secondary'">
                    {{ selectedUser.is_active ? 'Activo' : 'Inactivo' }}
                  </span>
                </p>
                <p class="mb-1">
                  <strong>Verificado:</strong> 
                  <i class="fas" :class="selectedUser.is_verified ? 'fa-check-circle text-success' : 'fa-times-circle text-danger'"></i>
                  {{ selectedUser.is_verified ? 'Sí' : 'No' }}
                </p>
              </div>
            </div>
            <hr>
            <div class="row">
              <div class="col-12">
                <h6 class="text-muted">Fechas</h6>
                <p class="mb-1"><strong>Registro:</strong> {{ formatDate(selectedUser.created_at) }}</p>
                <p class="mb-1"><strong>Última Actualización:</strong> {{ formatDate(selectedUser.updated_at) }}</p>
                <p class="mb-1" v-if="selectedUser.last_login_at">
                  <strong>Último Acceso:</strong> {{ formatDate(selectedUser.last_login_at) }}
                </p>
              </div>
            </div>
          </div>
          <div class="modal-footer">
            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Cerrar</button>
            <button type="button" class="btn btn-primary" @click="editFromDetails">
              <i class="fas fa-edit me-2"></i>
              Editar
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, reactive, computed, onMounted, watch } from 'vue'
import api from '../services/api'
import * as bootstrap from 'bootstrap'

export default {
  name: 'UserManagement',
  setup() {
    const users = ref([])
    const selectedUser = ref(null)
    const isLoading = ref(false)
    const isSaving = ref(false)
    const error = ref(null)
    const formError = ref(null)
    const isEditing = ref(false)

    let userModal = null
    let detailsModal = null

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
      role_id: 2,
      is_active: true,
      is_verified: false
    })

    // Pagination
    const currentPage = ref(1)
    const itemsPerPage = ref(15) // 15 usuarios por página

    const filteredUsers = computed(() => {
      return users.value.filter(user => {
        const roleMatch = !filters.role || user.role?.id == filters.role
        const statusMatch = !filters.status || user.is_active.toString() === filters.status
        const searchMatch = !filters.search || 
          user.username.toLowerCase().includes(filters.search.toLowerCase()) ||
          user.email.toLowerCase().includes(filters.search.toLowerCase()) ||
          user.phone_number.includes(filters.search)
        return roleMatch && statusMatch && searchMatch
      })
    })

    // Pagination computed
    const totalPages = computed(() => Math.ceil(filteredUsers.value.length / itemsPerPage.value))

    const paginatedUsers = computed(() => {
      const start = (currentPage.value - 1) * itemsPerPage.value
      const end = start + itemsPerPage.value
      return filteredUsers.value.slice(start, end)
    })

    const goToPage = (page) => {
      if (page >= 1 && page <= totalPages.value) {
        currentPage.value = page
        window.scrollTo({ top: 0, behavior: 'smooth' })
      }
    }

    // Reset pagination when filters change
    watch([() => filters.role, () => filters.status, () => filters.search], () => {
      currentPage.value = 1
    })

    const fetchUsers = async () => {
      isLoading.value = true
      error.value = null
      try {
        users.value = await api.getUsers()
      } catch (err) {
        error.value = 'Error al cargar usuarios'
        console.error('Error fetching users:', err)
      } finally {
        isLoading.value = false
      }
    }

    const getUsersByRole = (roleId) => users.value.filter(u => u.role?.id === roleId)

    const getRoleName = (roleNameOrId) => {
      // Puede recibir nombre de rol (string) o ID (number)
      if (typeof roleNameOrId === 'string') {
        const nameMap = { 'admin': 'Admin', 'customer': 'Cliente', 'inventory': 'Inventario' }
        return nameMap[roleNameOrId] || roleNameOrId
      }
      const roles = { 1: 'Admin', 2: 'Cliente', 3: 'Inventario' }
      return roles[roleNameOrId] || 'Desconocido'
    }

    const getRoleBadgeClass = (roleId) => {
      const classes = { 1: 'bg-danger', 2: 'bg-success', 3: 'bg-info' }
      return classes[roleId] || 'bg-secondary'
    }

    const formatDate = (dateString) => {
      return new Date(dateString).toLocaleString('es-ES', { 
        dateStyle: 'short', 
        timeStyle: 'short' 
      })
    }

    const resetForm = () => {
      Object.assign(userForm, {
        id: null,
        username: '',
        email: '',
        phone_number: '',
        password: '',
        role_id: 2,
        is_active: true,
        is_verified: false
      })
      formError.value = null
    }

    const showCreateModal = () => {
      isEditing.value = false
      resetForm()
      userModal?.show()
    }

    const editUser = (user) => {
      isEditing.value = true
      resetForm()
      Object.assign(userForm, {
        id: user.id,
        username: user.username,
        email: user.email,
        phone_number: user.phone_number,
        role_id: user.role?.id || 2,
        is_active: user.is_active,
        is_verified: user.is_verified
      })
      userModal?.show()
    }

    // Helper para convertir role_id a role_name
    const getRoleNameForAPI = (roleId) => {
      const roleMap = { 1: 'admin', 2: 'customer', 3: 'inventory' }
      return roleMap[roleId] || 'customer'
    }

    const saveUser = async () => {
      isSaving.value = true
      formError.value = null
      
      try {
        if (isEditing.value) {
          const { id, password, username, role_id, ...updateData } = userForm
          // Convertir role_id a role_name para el backend
          const dataToSend = {
            ...updateData,
            role_name: getRoleNameForAPI(role_id)
          }
          await api.updateUser(id, dataToSend)
        } else {
          // Para crear usuario, usar la ruta de admin con role_name
          await api.createUserAdmin({
            username: userForm.username,
            email: userForm.email,
            password: userForm.password,
            phone_number: userForm.phone_number,
            role_name: getRoleNameForAPI(userForm.role_id),
            is_active: userForm.is_active,
            is_verified: userForm.is_verified
          })
        }
        
        await fetchUsers()
        userModal?.hide()
      } catch (err) {
        formError.value = err.response?.data?.detail || 'Error al guardar usuario'
        console.error('Error saving user:', err)
      } finally {
        isSaving.value = false
      }
    }

    const viewUserDetails = (user) => {
      selectedUser.value = user
      detailsModal?.show()
    }

    const editFromDetails = () => {
      detailsModal?.hide()
      if (selectedUser.value) {
        editUser(selectedUser.value)
      }
    }

    const toggleUserStatus = async (user) => {
      const newStatus = !user.is_active
      const action = newStatus ? 'activar' : 'desactivar'
      
      if (!confirm(`¿Estás seguro de que deseas ${action} a ${user.username}?`)) {
        return
      }

      try {
        await api.updateUser(user.id, { is_active: newStatus })
        await fetchUsers()
      } catch (err) {
        error.value = `Error al ${action} usuario`
        console.error('Error toggling user status:', err)
      }
    }

    const clearFilters = () => {
      filters.role = ''
      filters.status = ''
      filters.search = ''
    }

    onMounted(() => {
      fetchUsers()
      userModal = new bootstrap.Modal(document.getElementById('userModal'))
      detailsModal = new bootstrap.Modal(document.getElementById('userDetailsModal'))
    })

    return {
      users,
      selectedUser,
      filteredUsers,
      paginatedUsers,
      isLoading,
      isSaving,
      error,
      formError,
      isEditing,
      filters,
      userForm,
      currentPage,
      itemsPerPage,
      totalPages,
      getUsersByRole,
      getRoleName,
      getRoleBadgeClass,
      formatDate,
      showCreateModal,
      editUser,
      saveUser,
      viewUserDetails,
      editFromDetails,
      toggleUserStatus,
      clearFilters,
      goToPage
    }
  }
}
</script>

<style scoped>
.card {
  box-shadow: 0 2px 4px rgba(0,0,0,0.05);
  border: none;
}

.table td, .table th {
  vertical-align: middle;
}

.badge {
  font-size: 0.85em;
  padding: .4em .6em;
}

code {
  font-size: 0.85em;
  background-color: #f8f9fa;
  padding: 2px 6px;
  border-radius: 3px;
}
</style>
