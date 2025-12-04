<template>
  <div class="order-management">
    <div class="container-fluid">
      <div class="d-flex justify-content-between align-items-center mb-4">
        <h2>Gestión de Órdenes</h2>
        <div class="d-flex gap-2">
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
              <div class="d-flex justify-content-between">
                <div>
                  <h6>Total Órdenes</h6>
                  <h3>{{ orders.length }}</h3>
                </div>
                <div class="align-self-center">
                  <i class="fas fa-receipt fa-2x"></i>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div class="col-md-3">
          <div class="card bg-warning text-white">
            <div class="card-body">
              <div class="d-flex justify-content-between">
                <div>
                  <h6>Pendientes</h6>
                  <h3>{{ getOrdersByStatus('pending').length }}</h3>
                </div>
                <div class="align-self-center">
                  <i class="fas fa-clock fa-2x"></i>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div class="col-md-3">
          <div class="card bg-success text-white">
            <div class="card-body">
              <div class="d-flex justify-content-between">
                <div>
                  <h6>Completadas</h6>
                  <h3>{{ getOrdersByStatus('completed').length }}</h3>
                </div>
                <div class="align-self-center">
                  <i class="fas fa-check fa-2x"></i>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div class="col-md-3">
          <div class="card bg-danger text-white">
            <div class="card-body">
              <div class="d-flex justify-content-between">
                <div>
                  <h6>Canceladas</h6>
                  <h3>{{ getOrdersByStatus('cancelled').length }}</h3>
                </div>
                <div class="align-self-center">
                  <i class="fas fa-times fa-2x"></i>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
      
      <!-- Filters -->
      <div class="card mb-4">
        <div class="card-body">
          <div class="row">
            <div class="col-md-3">
              <select v-model="filters.status" @change="applyFilters" class="form-select">
                <option value="">Todos los estados</option>
                <option value="pending">Pendientes</option>
                <option value="completed">Completadas</option>
                <option value="cancelled">Canceladas</option>
              </select>
            </div>
            <div class="col-md-3">
              <input 
                type="date" 
                v-model="filters.date" 
                @change="applyFilters"
                class="form-control"
              >
            </div>
            <div class="col-md-4">
              <input 
                type="text" 
                v-model="filters.search" 
                @input="applyFilters"
                placeholder="Buscar por ID de orden o usuario..." 
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
      
      <!-- Orders Table -->
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
          
          <!-- Orders Table -->
          <div v-else class="table-responsive">
            <table class="table table-hover">
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
                  <td>
                    <span class="fw-bold">#{{ order.id }}</span>
                  </td>
                  <td>
                    <div>Usuario ID: {{ order.user_id }}</div>
                  </td>
                  <td>
                    <div>{{ formatDate(order.created_at) }}</div>
                  </td>
                  <td>
                    <span class="fw-bold text-primary">
                      ${{ parseFloat(order.total_price).toFixed(2) }}
                    </span>
                  </td>
                  <td>
                    <span 
                      class="badge"
                      :class="{
                        'bg-warning': order.status === 'pending',
                        'bg-success': order.status === 'completed',
                        'bg-danger': order.status === 'cancelled'
                      }"
                    >
                      {{ getStatusText(order.status) }}
                    </span>
                  </td>
                  <td>
                    <div class="btn-group btn-group-sm">
                      <button 
                        class="btn btn-outline-primary"
                        @click="viewOrderDetails(order)"
                        title="Ver detalles"
                      >
                        <i class="fas fa-eye"></i>
                      </button>
                      <div class="btn-group btn-group-sm" role="group">
                        <button 
                          type="button" 
                          class="btn btn-outline-secondary dropdown-toggle" 
                          data-bs-toggle="dropdown"
                          title="Cambiar estado"
                        >
                          <i class="fas fa-edit"></i>
                        </button>
                        <ul class="dropdown-menu">
                          <li>
                            <a 
                              class="dropdown-item" 
                              href="#" 
                              @click.prevent="updateOrderStatus(order.id, 'pending')"
                              :class="{ active: order.status === 'pending' }"
                            >
                              Pendiente
                            </a>
                          </li>
                          <li>
                            <a 
                              class="dropdown-item" 
                              href="#" 
                              @click.prevent="updateOrderStatus(order.id, 'completed')"
                              :class="{ active: order.status === 'completed' }"
                            >
                              Completada
                            </a>
                          </li>
                          <li>
                            <a 
                              class="dropdown-item" 
                              href="#" 
                              @click.prevent="updateOrderStatus(order.id, 'cancelled')"
                              :class="{ active: order.status === 'cancelled' }"
                            >
                              Cancelada
                            </a>
                          </li>
                        </ul>
                      </div>
                    </div>
                  </td>
                </tr>
              </tbody>
            </table>
            
            <!-- Empty State -->
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
            <h5 class="modal-title">
              Detalles de la Orden #{{ selectedOrder.id }}
            </h5>
            <button type="button" class="btn-close" data-bs-dismiss="modal"></button>
          </div>
          <div class="modal-body">
            <div class="row mb-4">
              <div class="col-md-6">
                <h6>Información General</h6>
                <table class="table table-sm">
                  <tbody>
                    <tr>
                      <td><strong>ID:</strong></td>
                      <td>#{{ selectedOrder.id }}</td>
                    </tr>
                    <tr>
                      <td><strong>Usuario ID:</strong></td>
                      <td>{{ selectedOrder.user_id }}</td>
                    </tr>
                    <tr>
                      <td><strong>Estado:</strong></td>
                      <td>
                        <span 
                          class="badge"
                          :class="{
                            'bg-warning': selectedOrder.status === 'pending',
                            'bg-success': selectedOrder.status === 'completed',
                            'bg-danger': selectedOrder.status === 'cancelled'
                          }"
                        >
                          {{ getStatusText(selectedOrder.status) }}
                        </span>
                      </td>
                    </tr>
                    <tr>
                      <td><strong>Fecha:</strong></td>
                      <td>{{ formatDate(selectedOrder.created_at) }}</td>
                    </tr>
                    <tr>
                      <td><strong>Total:</strong></td>
                      <td class="fw-bold text-primary">${{ parseFloat(selectedOrder.total_price).toFixed(2) }}</td>
                    </tr>
                  </tbody>
                </table>
              </div>
            </div>
            
            <div v-if="selectedOrder.items && selectedOrder.items.length > 0">
              <h6>Productos Ordenados:</h6>
              <div class="table-responsive">
                <table class="table table-sm">
                  <thead>
                    <tr>
                      <th>Producto ID</th>
                      <th>Cantidad</th>
                      <th>Precio Unitario</th>
                      <th>Subtotal</th>
                    </tr>
                  </thead>
                  <tbody>
                    <tr v-for="item in selectedOrder.items" :key="item.id">
                      <td>{{ item.product_id }}</td>
                      <td>{{ item.quantity }}</td>
                      <td>${{ parseFloat(item.price_at_time_of_purchase).toFixed(2) }}</td>
                      <td class="fw-bold">
                        ${{ (item.quantity * parseFloat(item.price_at_time_of_purchase)).toFixed(2) }}
                      </td>
                    </tr>
                  </tbody>
                  <tfoot>
                    <tr class="table-active">
                      <td colspan="3" class="fw-bold text-end">Total:</td>
                      <td class="fw-bold text-primary">
                        ${{ parseFloat(selectedOrder.total_price).toFixed(2) }}
                      </td>
                    </tr>
                  </tfoot>
                </table>
              </div>
            </div>
          </div>
          <div class="modal-footer">
            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">
              Cerrar
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, reactive, computed, onMounted } from 'vue'
import { orderService } from '../services/api'

export default {
  name: 'OrderManagement',
  setup() {
    const orders = ref([])
    const selectedOrder = ref(null)
    const isLoading = ref(false)
    const error = ref(null)
    
    const filters = reactive({
      status: '',
      date: '',
      search: ''
    })
    
    const filteredOrders = computed(() => {
      let filtered = orders.value
      
      if (filters.status) {
        filtered = filtered.filter(order => order.status === filters.status)
      }
      
      if (filters.date) {
        filtered = filtered.filter(order => {
          const orderDate = new Date(order.created_at).toDateString()
          const filterDate = new Date(filters.date).toDateString()
          return orderDate === filterDate
        })
      }
      
      if (filters.search) {
        const term = filters.search.toLowerCase()
        filtered = filtered.filter(order =>
          order.id.toString().includes(term) ||
          order.user_id.toString().includes(term)
        )
      }
      
      return filtered.sort((a, b) => new Date(b.created_at) - new Date(a.created_at))
    })
    
    const fetchOrders = async () => {
      isLoading.value = true
      error.value = null
      try {
        orders.value = await orderService.getAllOrders()
      } catch (err) {
        error.value = err
        console.error('Error fetching orders:', err)
      } finally {
        isLoading.value = false
      }
    }
    
    const refreshOrders = () => {
      fetchOrders()
    }
    
    const getOrdersByStatus = (status) => {
      return orders.value.filter(order => order.status === status)
    }
    
    const getStatusText = (status) => {
      const statusMap = {
        pending: 'Pendiente',
        completed: 'Completada',
        cancelled: 'Cancelada'
      }
      return statusMap[status] || status
    }
    
    const formatDate = (dateString) => {
      const date = new Date(dateString)
      return date.toLocaleDateString('es-ES', {
        year: 'numeric',
        month: 'short',
        day: 'numeric',
        hour: '2-digit',
        minute: '2-digit'
      })
    }
    
    const viewOrderDetails = async (order) => {
      try {
        selectedOrder.value = await orderService.getOrderById(order.id)
        const modal = new bootstrap.Modal(document.getElementById('orderDetailsModal'))
        modal.show()
      } catch (err) {
        console.error('Error fetching order details:', err)
        error.value = 'Error al cargar los detalles de la orden'
      }
    }
    
    const updateOrderStatus = async (orderId, newStatus) => {
      try {
        // Note: This would require implementing an update order status endpoint
        console.log(`Update order ${orderId} to status ${newStatus}`)
        // await orderService.updateOrderStatus(orderId, newStatus)
        // await fetchOrders()
        alert('Funcionalidad no implementada en el microservicio de inventario')
      } catch (err) {
        console.error('Error updating order status:', err)
        error.value = 'Error al actualizar el estado de la orden'
      }
    }
    
    const applyFilters = () => {
      // Filters are applied automatically via computed property
    }
    
    const clearFilters = () => {
      filters.status = ''
      filters.date = ''
      filters.search = ''
    }
    
    onMounted(() => {
      fetchOrders()
    })
    
    return {
      orders,
      selectedOrder,
      filteredOrders,
      isLoading,
      error,
      filters,
      fetchOrders,
      refreshOrders,
      getOrdersByStatus,
      getStatusText,
      formatDate,
      viewOrderDetails,
      updateOrderStatus,
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

.card {
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  border: none;
}

.badge {
  font-size: 0.8em;
}

.dropdown-item.active {
  background-color: #e9ecef;
  color: #495057;
}
</style>
