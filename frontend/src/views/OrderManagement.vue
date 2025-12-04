<template>
  <div class="order-management">
    <div class="container">
      <div class="d-flex justify-content-between align-items-center mb-4">
        <h2>Gestión de Órdenes</h2>
        <div class="d-flex gap-2">
          <button class="btn btn-info" @click="handleSeedStock" :disabled="isSeeding">
            <span v-if="isSeeding" class="spinner-border spinner-border-sm me-2"></span>
            <i v-else class="fas fa-seedling me-2"></i>
            Generar Stock de Productos
          </button>
          <button class="btn btn-outline-secondary" @click="refreshOrders">
            <i class="fas fa-sync me-2"></i>
            Actualizar
          </button>
        </div>
      </div>

      <!-- Stats Cards -->
      <div class="row mb-4">
        <div class="col-md-3">
          <div class="card bg-primary text-white">
            <div class="card-body">
              <h6 class="card-title">Total Órdenes</h6>
              <h3>{{ orders.length }}</h3>
            </div>
          </div>
        </div>
        <div class="col-md-3">
          <div class="card bg-warning text-white">
            <div class="card-body">
              <h6 class="card-title">Pendientes</h6>
              <h3>{{ getOrdersByStatus('pending').length }}</h3>
            </div>
          </div>
        </div>
        <div class="col-md-3">
          <div class="card bg-success text-white">
            <div class="card-body">
              <h6 class="card-title">Completadas</h6>
              <h3>{{ getOrdersByStatus('completed').length }}</h3>
            </div>
          </div>
        </div>
        <div class="col-md-3">
          <div class="card bg-danger text-white">
            <div class="card-body">
              <h6 class="card-title">Canceladas</h6>
              <h3>{{ getOrdersByStatus('cancelled').length }}</h3>
            </div>
          </div>
        </div>
      </div>
      
      <!-- Filters -->
      <div class="card mb-4">
        <div class="card-body">
          <div class="row g-2">
            <div class="col-md-3">
              <select v-model="filters.status" @change="applyFilters" class="form-select">
                <option value="">Todos los estados</option>
                <option value="pending">Pendientes</option>
                <option value="completed">Completadas</option>
                <option value="cancelled">Canceladas</option>
              </select>
            </div>
            <div class="col-md-3">
              <input type="date" v-model="filters.date" @change="applyFilters" class="form-control">
            </div>
            <div class="col-md-4">
              <input type="text" v-model="filters.search" @input="applyFilters" placeholder="Buscar por ID de orden o usuario..." class="form-control">
            </div>
            <div class="col-md-2">
              <button @click="clearFilters" class="btn btn-outline-secondary w-100">Limpiar</button>
            </div>
          </div>
        </div>
      </div>
      
      <!-- Orders Table -->
      <div class="card">
        <div class="card-body">
          <div v-if="isLoading" class="text-center py-4">
            <div class="spinner-border" role="status"><span class="visually-hidden">Cargando...</span></div>
          </div>
          <div v-else-if="error" class="alert alert-danger">{{ error }}</div>
          <div v-else class="table-responsive">
            <table class="table table-hover align-middle">
              <thead>
                <tr>
                  <th>ID</th>
                  <th>Usuario</th>
                  <th>Fecha</th>
                  <th>Total</th>
                  <th>Estado</th>
                  <th>Acciones</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="order in filteredOrders" :key="order.id">
                  <td><span class="fw-bold">#{{ order.id }}</span></td>
                  <td>{{ getUsername(order.user_id) }}</td>
                  <td>{{ formatDate(order.created_at) }}</td>
                  <td><span class="fw-bold text-primary">${{ parseFloat(order.total_price).toFixed(2) }}</span></td>
                  <td>
                    <span class="badge" :class="statusBadgeClass(order.status)">{{ getStatusText(order.status) }}</span>
                  </td>
                  <td class="actions-cell">
                    <button class="btn btn-outline-primary btn-sm" @click="viewOrderDetails(order)" title="Ver detalles">
                      <i class="fas fa-eye me-1"></i> Ver / Editar
                    </button>
                  </td>
                </tr>
              </tbody>
            </table>
            <div v-if="filteredOrders.length === 0" class="text-center py-4">
              <i class="fas fa-clipboard-list fa-3x text-muted mb-3"></i>
              <h5>No se encontraron órdenes</h5>
              <p class="text-muted">Ajusta los filtros para ver órdenes</p>
            </div>
          </div>
        </div>
      </div>
    </div>
    
    <!-- Order Details Modal -->
    <div class="modal fade" id="orderDetailsModal" tabindex="-1">
      <div class="modal-dialog modal-lg">
        <div class="modal-content" v-if="selectedOrder">
          <div class="modal-header">
            <h5 class="modal-title">Detalles de la Orden #{{ selectedOrder.id }}</h5>
            <button type="button" class="btn-close" data-bs-dismiss="modal"></button>
          </div>
          <div class="modal-body">
            <!-- Order Info -->
            <div class="row mb-4">
              <div class="col-md-6">
                <h6>Información General</h6>
                <p class="mb-1"><strong>Usuario:</strong> {{ getUsername(selectedOrder.user_id) }}</p>
                <p class="mb-1"><strong>Fecha:</strong> {{ formatDate(selectedOrder.created_at) }}</p>
                <p class="mb-1"><strong>Total:</strong> <span class="fw-bold text-primary">${{ parseFloat(selectedOrder.total_price).toFixed(2) }}</span></p>
              </div>
              <div class="col-md-6">
                <h6>Cambiar Estado</h6>
                <div v-if="selectedOrder.status === 'pending'">
                  <select v-model="selectedOrderForm.status" class="form-select">
                    <option value="pending">Pendiente</option>
                    <option value="completed">Completada</option>
                    <option value="cancelled">Cancelada</option>
                  </select>
                </div>
                <p v-else class="mb-1">
                  <strong>Estado:</strong> <span class="badge" :class="statusBadgeClass(selectedOrder.status)">{{ getStatusText(selectedOrder.status) }}</span>
                </p>
              </div>
            </div>
            
            <!-- Order Items -->
            <hr>
            <h6 class="mb-3">Productos</h6>
            <div v-if="isFetchingDetails" class="text-center"><div class="spinner-border spinner-border-sm"></div></div>
            <div v-else-if="selectedOrderItems.length > 0">
              <ul class="list-group list-group-flush">
                <li v-for="item in selectedOrderItems" :key="item.id" class="list-group-item d-flex justify-content-between align-items-center px-0">
                  <div>
                    <div class="fw-bold">{{ item.productName }}</div>
                    <small class="text-muted">{{ item.quantity }} x ${{ parseFloat(item.price_at_time_of_purchase).toFixed(2) }}</small>
                  </div>
                  <span class="fw-bold">${{ (item.quantity * parseFloat(item.price_at_time_of_purchase)).toFixed(2) }}</span>
                </li>
              </ul>
            </div>

          </div>
          <div class="modal-footer">
            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Cerrar</button>
            <button 
              type="button" 
              class="btn btn-primary" 
              @click="saveOrderChanges" 
              v-if="selectedOrder.status === 'pending'"
              :disabled="isSaving"
            >
              <span v-if="isSaving" class="spinner-border spinner-border-sm me-2"></span>
              Guardar Cambios
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, reactive, computed, onMounted } from 'vue'
import api from '../services/api'
import * as bootstrap from 'bootstrap'

export default {
  name: 'OrderManagement',
  setup() {
    const orders = ref([])
    const usersCache = ref({})
    const selectedOrder = ref(null)
    const selectedOrderItems = ref([])
    const isLoading = ref(false)
    const isFetchingDetails = ref(false)
    const isSaving = ref(false)
    const isSeeding = ref(false)
    const error = ref(null)
    
    let modalInstance = null;

    const filters = reactive({ status: '', date: '', search: '' });
    const selectedOrderForm = reactive({ status: '' });
    
    const filteredOrders = computed(() => {
      return orders.value
        .filter(order => {
          const statusMatch = !filters.status || order.status === filters.status;
          const dateMatch = !filters.date || new Date(order.created_at).toDateString() === new Date(filters.date).toDateString();
          const searchMatch = !filters.search || 
            order.id.toString().includes(filters.search) || 
            getUsername(order.user_id).toLowerCase().includes(filters.search.toLowerCase());
          return statusMatch && dateMatch && searchMatch;
        })
        .sort((a, b) => new Date(b.created_at) - new Date(a.created_at));
    });
    
    const fetchOrders = async () => {
      isLoading.value = true
      error.value = null
      try {
        const ordersData = await api.getAllOrders();
        orders.value = ordersData;

        const userIds = [...new Set(ordersData.map(o => o.user_id))].filter(id => !usersCache.value[id]);
        if (userIds.length > 0) {
          const userPromises = userIds.map(id => api.getUserById(id));
          const usersData = await Promise.all(userPromises);
          usersData.forEach(user => {
            if (user) usersCache.value[user.id] = user.username;
          });
        }
      } catch (err) {
        error.value = 'Error al cargar las órdenes'
        console.error('Error fetching orders:', err)
      } finally {
        isLoading.value = false
      }
    };
    
    const getUsername = (userId) => {
      return usersCache.value[userId] || `ID: ${userId}`;
    };

    const handleSeedStock = async () => {
      if (!confirm('¿Estás seguro de que deseas generar el stock inicial? Esto puede sobrescribir los datos de stock existentes.')) {
        return;
      }
      isSeeding.value = true;
      error.value = null;
      try {
        await api.seedStock();
        alert('El stock de productos se ha generado exitosamente.');
      } catch (err) {
        error.value = 'Error al generar el stock.';
        console.error('Error seeding stock:', err);
        alert('Hubo un error al generar el stock. Revisa la consola para más detalles.');
      } finally {
        isSeeding.value = false;
      }
    };

    const refreshOrders = () => fetchOrders();
    
    const getOrdersByStatus = (status) => orders.value.filter(order => order.status === status);
    
    const statusMap = { pending: 'Pendiente', completed: 'Completada', cancelled: 'Cancelada' };
    const getStatusText = (status) => statusMap[status] || status;

    const statusBadgeClass = (status) => ({
      'bg-warning': status === 'pending',
      'bg-success': status === 'completed',
      'bg-danger': status === 'cancelled',
    });
    
    const formatDate = (dateString) => new Date(dateString).toLocaleString('es-ES', { dateStyle: 'short', timeStyle: 'short' });
    
    const viewOrderDetails = async (order) => {
      selectedOrder.value = order;
      selectedOrderForm.status = order.status;
      modalInstance?.show();
      
      isFetchingDetails.value = true;
      selectedOrderItems.value = [];
      try {
        const items = await api.getOrderItems(order.id);
        selectedOrderItems.value = await Promise.all(
          items.map(async item => {
            try {
              const product = await api.getProductById(item.product_id);
              return { ...item, productName: product.nombre };
            } catch {
              return { ...item, productName: `ID: ${item.product_id}` };
            }
          })
        );
      } catch (err) {
        console.error('Error fetching order items:', err)
      } finally {
        isFetchingDetails.value = false;
      }
    };
    
    const saveOrderChanges = async () => {
      if (!selectedOrder.value) return;
      
      if (selectedOrder.value.status === selectedOrderForm.status) {
        modalInstance?.hide();
        return;
      }

      const statusText = getStatusText(selectedOrderForm.status).toLowerCase();
      if (confirm(`¿Estás seguro de que deseas marcar esta orden como "${statusText}"?`)) {
        isSaving.value = true;
        try {
          await api.updateOrderStatus(selectedOrder.value.id, selectedOrderForm.status)
          await fetchOrders();
          modalInstance?.hide();
        } catch (err) {
          console.error('Error updating order status:', err)
          error.value = 'Error al actualizar el estado de la orden'
        } finally {
          isSaving.value = false;
        }
      }
    };
    
    const applyFilters = () => {};
    
    const clearFilters = () => {
      filters.status = ''
      filters.date = ''
      filters.search = ''
    };
    
    onMounted(() => {
      fetchOrders();
      const modalElement = document.getElementById('orderDetailsModal');
      if (modalElement) {
        modalInstance = new bootstrap.Modal(modalElement);
        modalElement.addEventListener('hidden.bs.modal', () => {
          document.body.focus();
          selectedOrder.value = null;
          selectedOrderItems.value = [];
          selectedOrderForm.status = '';
        });
      }
    });
    
    return {
      orders,
      selectedOrder,
      selectedOrderForm,
      selectedOrderItems,
      filteredOrders,
      isLoading,
      isFetchingDetails,
      isSaving,
      isSeeding,
      error,
      filters,
      refreshOrders,
      getOrdersByStatus,
      getStatusText,
      statusBadgeClass,
      formatDate,
      getUsername,
      viewOrderDetails,
      saveOrderChanges,
      applyFilters,
      clearFilters,
      handleSeedStock,
    }
  }
}
</script>

<style scoped>
.table th {
  font-weight: 600;
}
.table td {
  vertical-align: middle;
}
.card {
  box-shadow: 0 2px 4px rgba(0,0,0,0.05);
  border: none;
}
.badge {
  font-size: 0.8em;
  padding: .4em .6em;
}
.actions-cell {
  min-width: 120px;
}
</style>
