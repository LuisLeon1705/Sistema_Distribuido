<template>
  <div class="user-management-page">
    <div class="container py-5">
      
      <div class="d-flex justify-content-between align-items-center mb-5">
        <div>
          <h2 class="fw-bold text-dark mb-1">Gestión de Usuarios</h2>
          <p class="text-muted small mb-0">Administra cuentas y roles</p>
        </div>
        <button class="btn btn-dark btn-sm rounded-pill px-4 shadow-sm" @click="showCreateModal">
          <i class="fas fa-user-plus me-2"></i>Registrar Usuario
        </button>
      </div>

      <div class="row g-4 mb-5">
        <div class="col-md-6 col-lg-3">
          <div class="stat-card p-4 h-100 d-flex align-items-center">
            <div class="icon-circle bg-primary bg-opacity-10 text-primary me-3">
              <i class="fas fa-users fa-lg"></i>
            </div>
            <div>
              <h6 class="text-muted small text-uppercase fw-bold mb-1">Total</h6>
              <h3 class="fw-bold text-dark mb-0">{{ users.length }}</h3>
            </div>
          </div>
        </div>
        <div class="col-md-6 col-lg-3">
          <div class="stat-card p-4 h-100 d-flex align-items-center">
            <div class="icon-circle bg-danger bg-opacity-10 text-danger me-3">
              <i class="fas fa-user-shield fa-lg"></i>
            </div>
            <div>
              <h6 class="text-muted small text-uppercase fw-bold mb-1">Administrador</h6>
              <h3 class="fw-bold text-dark mb-0">{{ getUsersByRole(1).length }}</h3>
            </div>
          </div>
        </div>
        <div class="col-md-6 col-lg-3">
          <div class="stat-card p-4 h-100 d-flex align-items-center">
            <div class="icon-circle bg-success bg-opacity-10 text-success me-3">
              <i class="fas fa-user-check fa-lg"></i>
            </div>
            <div>
              <h6 class="text-muted small text-uppercase fw-bold mb-1">Clientes</h6>
              <h3 class="fw-bold text-dark mb-0">{{ getUsersByRole(2).length }}</h3>
            </div>
          </div>
        </div>
        <div class="col-md-6 col-lg-3">
          <div class="stat-card p-4 h-100 d-flex align-items-center">
            <div class="icon-circle bg-info bg-opacity-10 text-info me-3">
              <i class="fas fa-boxes fa-lg"></i>
            </div>
            <div>
              <h6 class="text-muted small text-uppercase fw-bold mb-1">Inventario</h6>
              <h3 class="fw-bold text-dark mb-0">{{ getUsersByRole(3).length }}</h3>
            </div>
          </div>
        </div>
      </div>

      <div class="card border-0 shadow-sm rounded-4 mb-4 p-3 bg-white">
        <div class="row g-3">
          <div class="col-md-3">
            <div class="input-group input-group-sm">
              <span class="input-group-text bg-light border-0 text-muted"><i class="fas fa-id-badge"></i></span>
              <select v-model="filters.role" class="form-select bg-light border-0 text-dark">
                <option value="">Todos los roles</option>
                <option value="1">Administrador</option>
                <option value="2">Cliente</option>
                <option value="3">Inventario</option>
              </select>
            </div>
          </div>
          <div class="col-md-3">
            <div class="input-group input-group-sm">
              <span class="input-group-text bg-light border-0 text-muted"><i class="fas fa-toggle-on"></i></span>
              <select v-model="filters.status" class="form-select bg-light border-0 text-dark">
                <option value="">Todos los estados</option>
                <option value="true">Activos</option>
                <option value="false">Inactivos</option>
              </select>
            </div>
          </div>
          <div class="col-md-4">
            <div class="input-group input-group-sm">
              <span class="input-group-text bg-light border-0 text-muted"><i class="fas fa-search"></i></span>
              <input type="text" v-model="filters.search" placeholder="Buscar usuario..." class="form-control bg-light border-0 text-dark">
            </div>
          </div>
          <div class="col-md-2">
            <button @click="clearFilters" class="btn btn-link text-muted text-decoration-none small w-100 hover-text-dark">
              Limpiar filtros
            </button>
          </div>
        </div>
      </div>

      <div class="card border-0 shadow-sm rounded-4 overflow-hidden">
        <div class="card-body p-0">
          
          <div v-if="isLoading" class="text-center py-5">
            <div class="spinner-border text-primary opacity-50" role="status"></div>
            <p class="text-muted mt-2 small">Cargando usuarios...</p>
          </div>

          <div v-else-if="error" class="alert alert-danger m-3 border-0 bg-danger bg-opacity-10 text-danger">
            <i class="fas fa-exclamation-triangle me-2"></i>{{ error }}
          </div>

          <div v-else class="table-responsive">
            <table class="table table-hover align-middle mb-0">
              <thead class="bg-light">
                <tr>
                  <th class="ps-4 py-3 text-muted small text-uppercase fw-bold border-0">Usuario</th>
                  <th class="py-3 text-muted small text-uppercase fw-bold border-0">Contacto</th>
                  <th class="py-3 text-muted small text-uppercase fw-bold border-0">Rol</th>
                  <th class="py-3 text-muted small text-uppercase fw-bold border-0">Estado</th>
                  <th class="pe-4 py-3 text-muted small text-uppercase fw-bold border-0 text-end">Acciones</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="user in paginatedUsers" :key="user.id">
                  <td class="ps-4 py-3">
                    <div class="d-flex align-items-center">
                      <div class="avatar-circle bg-dark text-white me-3" style="width: 40px; height: 40px; font-size: 0.9rem;">
                        {{ user.username.charAt(0).toUpperCase() }}
                      </div>
                      <div>
                        <div class="fw-bold text-dark">{{ user.username }}</div>
                        <small class="text-muted d-block" style="font-size: 0.75rem;">
                          <i class="fas fa-check-circle me-1" :class="user.is_verified ? 'text-success' : 'text-muted opacity-50'"></i>
                          {{ user.is_verified ? 'Verificado' : 'No verificado' }}
                        </small>
                      </div>
                    </div>
                  </td>
                  <td>
                    <div class="text-dark small mb-1"><i class="far fa-envelope me-2 text-muted"></i>{{ user.email }}</div>
                    <div class="text-muted small"><i class="fas fa-phone me-2 text-muted opacity-50"></i>{{ user.phone_number }}</div>
                  </td>
                  <td>
                    <span class="badge rounded-pill fw-normal px-2" :class="getRoleBadgeClass(user.role?.id)">
                      {{ getRoleName(user.role?.name) }}
                    </span>
                  </td>
                  <td>
                    <span class="badge rounded-pill fw-normal px-2" :class="user.is_active ? 'bg-success bg-opacity-10 text-success border border-success border-opacity-25' : 'bg-secondary bg-opacity-10 text-secondary border border-secondary border-opacity-25'">
                      {{ user.is_active ? 'Activo' : 'Inactivo' }}
                    </span>
                  </td>
                  <td class="pe-4 text-end">
                    <div class="btn-group">
                      <button class="btn btn-light btn-sm text-muted hover-bg-gray" @click="viewUserDetails(user)" title="Ver">
                        <i class="fas fa-eye"></i>
                      </button>
                      <button class="btn btn-light btn-sm text-primary hover-bg-primary-soft" @click="editUser(user)" title="Editar">
                        <i class="fas fa-pen"></i>
                      </button>
                      <button class="btn btn-light btn-sm" :class="user.is_active ? 'text-danger hover-bg-danger-soft' : 'text-success hover-bg-success-soft'" @click="toggleUserStatus(user)" :title="user.is_active ? 'Desactivar' : 'Activar'">
                        <i class="fas" :class="user.is_active ? 'fa-ban' : 'fa-check'"></i>
                      </button>
                    </div>
                  </td>
                </tr>
              </tbody>
            </table>

            <div v-if="filteredUsers.length === 0" class="text-center py-5">
              <div class="mb-3 opacity-25">
                <i class="fas fa-users-slash fa-3x text-muted"></i>
              </div>
              <h6 class="text-dark">No se encontraron usuarios</h6>
              <p class="text-muted small">Intenta ajustar los filtros de búsqueda</p>
            </div>
          </div>

          <div v-if="!isLoading && filteredUsers.length > 0 && totalPages > 1" class="d-flex justify-content-between align-items-center p-4 border-top">
             <small class="text-muted">
               Mostrando {{ (currentPage - 1) * itemsPerPage + 1 }} - {{ Math.min(currentPage * itemsPerPage, filteredUsers.length) }} de {{ filteredUsers.length }}
             </small>
             <nav aria-label="Pagination">
              <ul class="pagination pagination-sm mb-0">
                <li class="page-item" :class="{ disabled: currentPage === 1 }">
                  <button class="page-link border-0 text-dark bg-transparent" @click="goToPage(currentPage - 1)">
                    <i class="fas fa-chevron-left"></i>
                  </button>
                </li>
                <li v-for="page in totalPages" :key="page" class="page-item" :class="{ active: currentPage === page }">
                  <button class="page-link border-0 rounded-circle mx-1" :class="currentPage === page ? 'bg-dark text-white' : 'text-dark bg-transparent'" @click="goToPage(page)">
                    {{ page }}
                  </button>
                </li>
                <li class="page-item" :class="{ disabled: currentPage === totalPages }">
                  <button class="page-link border-0 text-dark bg-transparent" @click="goToPage(currentPage + 1)">
                    <i class="fas fa-chevron-right"></i>
                  </button>
                </li>
              </ul>
            </nav>
          </div>

        </div>
      </div>
    </div>

    <div class="modal fade" id="userModal" tabindex="-1">
      <div class="modal-dialog modal-dialog-centered">
        <div class="modal-content border-0 shadow-lg rounded-4">
          <div class="modal-header border-bottom px-4 py-3">
            <h5 class="modal-title fw-bold text-dark">{{ isEditing ? 'Editar Usuario' : 'Registrar Usuario' }}</h5>
            <button type="button" class="btn-close" data-bs-dismiss="modal"></button>
          </div>
          <form @submit.prevent="saveUser">
            <div class="modal-body p-4">
              
              <div class="mb-3">
                <label class="form-label small fw-bold text-muted">Usuario</label>
                <div class="input-group">
                  <span class="input-group-text bg-light text-muted border"><i class="fas fa-user"></i></span>
                  <input type="text" v-model="userForm.username" class="form-control" required :disabled="isEditing">
                </div>
              </div>

              <div class="row g-3 mb-3">
                <div class="col-md-6">
                  <label class="form-label small fw-bold text-muted">Email</label>
                  <input type="email" v-model="userForm.email" class="form-control" required>
                </div>
                <div class="col-md-6">
                  <label class="form-label small fw-bold text-muted">Teléfono</label>
                  <input type="tel" v-model="userForm.phone_number" class="form-control" required>
                </div>
              </div>

              <div class="mb-3" v-if="!isEditing">
                <label class="form-label small fw-bold text-muted">Contraseña</label>
                <div class="input-group">
                  <span class="input-group-text bg-light text-muted border"><i class="fas fa-lock"></i></span>
                  <input type="password" v-model="userForm.password" class="form-control" required>
                </div>
              </div>

              <div class="mb-4">
                <label class="form-label small fw-bold text-muted">Rol de Acceso</label>
                <select v-model.number="userForm.role_id" class="form-select">
                  <option :value="1">Administrador</option>
                  <option :value="2">Cliente</option>
                  <option :value="3">Inventario</option>
                </select>
              </div>

              <div class="p-3 bg-light rounded-3 mb-3">
                <div class="form-check form-switch mb-2">
                  <input class="form-check-input" type="checkbox" v-model="userForm.is_active" id="isActive">
                  <label class="form-check-label small" for="isActive">Cuenta Activa</label>
                </div>
                <div class="form-check form-switch">
                  <input class="form-check-input" type="checkbox" v-model="userForm.is_verified" id="isVerified">
                  <label class="form-check-label small" for="isVerified">Email Verificado</label>
                </div>
              </div>

              <div v-if="formError" class="alert alert-danger py-2 small border-0 bg-danger bg-opacity-10 text-danger">{{ formError }}</div>

            </div>
            <div class="modal-footer border-0 bg-light px-4 py-3">
              <button type="button" class="btn btn-outline-dark rounded-pill px-4" data-bs-dismiss="modal">Cancelar</button>
              <button type="submit" class="btn btn-dark rounded-pill px-4" :disabled="isSaving">
                <span v-if="isSaving" class="spinner-border spinner-border-sm me-2"></span>Guardar
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>

    <div class="modal fade" id="userDetailsModal" tabindex="-1">
      <div class="modal-dialog modal-dialog-centered">
        <div class="modal-content border-0 shadow-lg rounded-4" v-show="selectedUser">
          <div v-if="selectedUser">
            <div class="modal-header border-bottom px-4 py-3">
              <h5 class="modal-title fw-bold text-dark">Detalle de Cuenta</h5>
              <button type="button" class="btn-close" data-bs-dismiss="modal"></button>
            </div>
            <div class="modal-body p-4">
            <div class="text-center mb-4">
              <div class="avatar-circle bg-dark text-white mx-auto mb-3" style="width: 80px; height: 80px; font-size: 2rem;">
                {{ selectedUser.username.charAt(0).toUpperCase() }}
              </div>
              <h5 class="fw-bold text-dark mb-1">{{ selectedUser.username }}</h5>
              <span class="badge rounded-pill fw-normal px-3" :class="getRoleBadgeClass(selectedUser.role?.id)">
                {{ getRoleName(selectedUser.role?.name) }}
              </span>
            </div>

            <div class="row g-3 text-center mb-4">
               <div class="col-6">
                  <div class="p-3 bg-light rounded-3 h-100">
                     <small class="text-muted d-block mb-1">Estado</small>
                     <span class="fw-bold" :class="selectedUser.is_active ? 'text-success' : 'text-danger'">
                        {{ selectedUser.is_active ? 'Activo' : 'Inactivo' }}
                     </span>
                  </div>
               </div>
               <div class="col-6">
                  <div class="p-3 bg-light rounded-3 h-100">
                     <small class="text-muted d-block mb-1">Verificación</small>
                     <span class="fw-bold" :class="selectedUser.is_verified ? 'text-primary' : 'text-warning'">
                        {{ selectedUser.is_verified ? 'Sí' : 'No' }}
                     </span>
                  </div>
               </div>
            </div>

            <ul class="list-group list-group-flush small">
              <li class="list-group-item px-0 d-flex justify-content-between">
                <span class="text-muted">Email</span>
                <span class="fw-bold text-dark">{{ selectedUser.email }}</span>
              </li>
              <li class="list-group-item px-0 d-flex justify-content-between">
                <span class="text-muted">Teléfono</span>
                <span class="fw-bold text-dark">{{ selectedUser.phone_number }}</span>
              </li>
              <li class="list-group-item px-0 d-flex justify-content-between">
                <span class="text-muted">Registro</span>
                <span class="text-dark">{{ formatDate(selectedUser.created_at) }}</span>
              </li>
            </ul>

          </div>
          <div class="modal-footer border-0 bg-light px-4 py-3">
            <button type="button" class="btn btn-outline-dark rounded-pill px-4" data-bs-dismiss="modal">Cerrar</button>
            <button type="button" class="btn btn-dark rounded-pill px-4" @click="editFromDetails">
               <i class="fas fa-pen me-2"></i>Editar
            </button>
            </div>
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

    const filters = reactive({ role: '', status: '', search: '' })
    
    const currentPage = ref(1)
    const itemsPerPage = ref(10)

    const userForm = reactive({
      id: null, username: '', email: '', phone_number: '', password: '',
      role_id: 2, is_active: true, is_verified: false
    })

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
    
    watch([() => filters.role, () => filters.status, () => filters.search], () => { currentPage.value = 1 })

    const fetchUsers = async () => {
      isLoading.value = true
      error.value = null
      try { users.value = await api.getUsers() } catch (err) { error.value = 'Error al cargar usuarios'; console.error(err); } finally { isLoading.value = false }
    }

    const getUsersByRole = (roleId) => users.value.filter(u => u.role?.id === roleId)
    
    const getRoleName = (roleNameOrId) => {
      if (typeof roleNameOrId === 'string') { const map = { 'admin': 'Admin', 'customer': 'Cliente', 'inventory': 'Inventario' }; return map[roleNameOrId] || roleNameOrId; }
      const roles = { 1: 'Admin', 2: 'Cliente', 3: 'Inventario' }; return roles[roleNameOrId] || 'Desconocido';
    }

    const getRoleBadgeClass = (roleId) => {
      const classes = { 
         1: 'bg-danger bg-opacity-10 text-danger border border-danger border-opacity-25', 
         2: 'bg-primary bg-opacity-10 text-primary border border-primary border-opacity-25', 
         3: 'bg-warning bg-opacity-10 text-warning border border-warning border-opacity-25 text-dark' 
      }
      return classes[roleId] || 'bg-light text-dark'
    }

    const formatDate = (dateString) => {
      const date = new Date(dateString);
      const time = new Date(date.getTime() - (date.getTimezoneOffset() * 60000));
      return time.toLocaleDateString('es-ES', { year: 'numeric', month: 'long', day: 'numeric', hour: '2-digit', minute: '2-digit' });
    };

    const resetForm = () => {
      Object.assign(userForm, { id: null, username: '', email: '', phone_number: '', password: '', role_id: 2, is_active: true, is_verified: false })
      formError.value = null
    }

    const showCreateModal = () => { isEditing.value = false; resetForm(); userModal?.show(); }
    
    const editUser = (user) => {
      isEditing.value = true; resetForm();
      Object.assign(userForm, { id: user.id, username: user.username, email: user.email, phone_number: user.phone_number, role_id: user.role?.id || 2, is_active: user.is_active, is_verified: user.is_verified });
      userModal?.show();
    }
    
    const getRoleNameForAPI = (roleId) => { const map = { 1: 'admin', 2: 'customer', 3: 'inventory' }; return map[roleId] || 'customer'; }

    const saveUser = async () => {
      isSaving.value = true; formError.value = null;
      try {
        if (isEditing.value) {
          const { id, password, username, role_id, ...updateData } = userForm
          await api.updateUser(id, { ...updateData, role_name: getRoleNameForAPI(role_id) })
        } else {
          await api.createUserAdmin({ ...userForm, role_name: getRoleNameForAPI(userForm.role_id) })
        }
        await fetchUsers(); userModal?.hide();
      } catch (err) { formError.value = err.response?.data?.detail || 'Error al guardar'; console.error(err); } finally { isSaving.value = false; }
    }

    const viewUserDetails = (user) => { selectedUser.value = user; detailsModal?.show(); }
    const editFromDetails = () => { detailsModal?.hide(); if (selectedUser.value) editUser(selectedUser.value); }

    const toggleUserStatus = async (user) => {
      if (!confirm(`¿${user.is_active ? 'Desactivar' : 'Activar'} a ${user.username}?`)) return;
      try { await api.updateUser(user.id, { is_active: !user.is_active }); await fetchUsers(); } catch (err) { console.error(err); }
    }
    
    const clearFilters = () => { filters.role = ''; filters.status = ''; filters.search = ''; }

    onMounted(() => {
      fetchUsers();
      userModal = new bootstrap.Modal(document.getElementById('userModal'));
      detailsModal = new bootstrap.Modal(document.getElementById('userDetailsModal'));
    })

    return {
      users, selectedUser, filteredUsers, paginatedUsers, isLoading, isSaving, error, formError, isEditing, filters, userForm,
      currentPage, itemsPerPage, totalPages, getUsersByRole, getRoleName, getRoleBadgeClass, formatDate,
      showCreateModal, editUser, saveUser, viewUserDetails, editFromDetails, toggleUserStatus, clearFilters, goToPage
    }
  }
}
</script>

<style scoped>
.user-management-page {
  background-color: #ffffff;
  min-height: 100vh;
}

.stat-card {
  background: #ffffff;
  border-radius: 1rem;
  border: 1px solid #f1f5f9;
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.05);
  transition: transform 0.2s;
}

.stat-card:hover { 
  transform: translateY(-3px); 
  box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.08); 
}

.icon-circle { 
  width: 50px; 
  height: 50px; 
  border-radius: 50%; 
  display: flex; 
  align-items: center; 
  justify-content: center; 
}

.avatar-circle { 
  border-radius: 50%; 
  display: flex; 
  align-items: center; 
  justify-content: center; 
  font-weight: 600; 
}

.hover-bg-gray:hover { 
  background-color: #e2e8f0; 
}

.hover-bg-primary-soft:hover { 
  background-color: rgba(59, 130, 246, 0.1); 
  color: #3b82f6 !important; 
}

.hover-bg-danger-soft:hover { 
  background-color: rgba(220, 53, 69, 0.1); 
  color: #dc3545 !important; 
}

.hover-bg-success-soft:hover { 
  background-color: rgba(25, 135, 84, 0.1); 
  color: #198754 !important; 
}

.hover-text-dark:hover { 
  color: #000 !important; 
}
</style>