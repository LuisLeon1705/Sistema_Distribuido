<template>
  <div class="orders">
    <div class="container">
      <div class="row">
        <div class="col-12">
          <div class="d-flex justify-content-between align-items-center mb-4">
            <h2>Mis Órdenes</h2>
            <router-link to="/products" class="btn btn-primary">
              <i class="fas fa-plus me-2"></i>
              Nueva Compra
            </router-link>
          </div>
          
          <!-- Loading State -->
          <div v-if="isLoading" class="text-center py-5">
            <div class="spinner-border" role="status">
              <span class="visually-hidden">Cargando...</span>
            </div>
            <p class="mt-2">Cargando órdenes...</p>
          </div>
          
          <!-- Error State -->
          <div v-else-if="error" class="alert alert-danger">
            {{ error }}
          </div>
          
          <!-- Empty State -->
          <div v-else-if="orders.length === 0" class="text-center py-5">
            <i class="fas fa-receipt fa-5x text-muted mb-4"></i>
            <h3>No tienes órdenes aún</h3>
            <p class="text-muted mb-4">Realiza tu primera compra para ver tus órdenes aquí</p>
            <router-link to="/products" class="btn btn-primary btn-lg">
              Explorar Productos
            </router-link>
          </div>
          
          <!-- Orders List -->
          <div v-else class="orders-list">
            <div v-for="order in orders" :key="order.id" class="card mb-4 order-card">
              <div class="card-header d-flex justify-content-between align-items-center">
                <div>
                  <h5 class="mb-1">
                    Orden #{{ order.id }}
                    <span 
                      class="badge ms-2"
                      :class="{
                        'bg-warning': order.status === 'pending',
                        'bg-success': order.status === 'completed',
                        'bg-danger': order.status === 'cancelled'
                      }"
                    >
                      {{ getStatusText(order.status) }}
                    </span>
                  </h5>
                  <small class="text-muted">
                    {{ formatDate(order.created_at) }}
                  </small>
                </div>
                <div class="text-end">
                  <div class="h5 text-primary mb-0">
                    ${{ parseFloat(order.total_price).toFixed(2) }}
                  </div>
                </div>
              </div>
              
              <div class="card-body">
                <div class="row">
                  <div class="col-md-8">
                    <!-- Order Items (if available) -->
                    <div v-if="order.items && order.items.length > 0">
                      <h6 class="mb-3">Productos:</h6>
                      <div class="order-items">
                        <div 
                          v-for="item in order.items" 
                          :key="item.id" 
                          class="d-flex align-items-center border-bottom pb-2 mb-2"
                        >
                          <div class="flex-grow-1">
                            <div class="fw-medium">Producto ID: {{ item.product_id }}</div>
                            <small class="text-muted">
                              Cantidad: {{ item.quantity }} x ${{ parseFloat(item.price_at_time_of_purchase).toFixed(2) }}
                            </small>
                          </div>
                          <div class="text-end">
                            <div class="fw-bold">
                              ${{ (item.quantity * parseFloat(item.price_at_time_of_purchase)).toFixed(2) }}
                            </div>
                          </div>
                        </div>
                      </div>
                    </div>
                  </div>
                  
                  <div class="col-md-4">
                    <div class="d-grid gap-2">
                      <button 
                        class="btn btn-outline-primary btn-sm"
                        @click="viewOrderDetails(order.id)"
                      >
                        <i class="fas fa-eye me-1"></i>
                        Ver Detalles
                      </button>
                      
                      <button 
                        v-if="order.status === 'pending'"
                        class="btn btn-outline-danger btn-sm"
                        @click="cancelOrder(order.id)"
                      >
                        <i class="fas fa-times me-1"></i>
                        Cancelar Orden
                      </button>
                    </div>
                  </div>
                </div>
              </div>
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
                <p class="mb-1"><strong>Estado:</strong> {{ getStatusText(selectedOrder.status) }}</p>
                <p class="mb-1"><strong>Fecha:</strong> {{ formatDate(selectedOrder.created_at) }}</p>
                <p class="mb-1"><strong>Total:</strong> ${{ parseFloat(selectedOrder.total_price).toFixed(2) }}</p>
              </div>
            </div>
            
            <div v-if="selectedOrder.items && selectedOrder.items.length > 0">
              <h6>Productos:</h6>
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
                      <td>${{ (item.quantity * parseFloat(item.price_at_time_of_purchase)).toFixed(2) }}</td>
                    </tr>
                  </tbody>
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
import { ref, onMounted } from 'vue'
import { orderService } from '../services/api'

export default {
  name: 'Orders',
  setup() {
    const orders = ref([])
    const selectedOrder = ref(null)
    const isLoading = ref(false)
    const error = ref(null)
    
    const fetchOrders = async () => {
      isLoading.value = true
      error.value = null
      try {
        orders.value = await orderService.getOrders()
      } catch (err) {
        error.value = 'Error al cargar las órdenes'
        console.error('Error fetching orders:', err)
      } finally {
        isLoading.value = false
      }
    }
    
    const viewOrderDetails = async (orderId) => {
      try {
        selectedOrder.value = await orderService.getOrderById(orderId)
        // Show modal
        const modal = new bootstrap.Modal(document.getElementById('orderDetailsModal'))
        modal.show()
      } catch (err) {
        console.error('Error fetching order details:', err)
      }
    }
    
    const cancelOrder = async (orderId) => {
      if (confirm('¿Estás seguro de que deseas cancelar esta orden?')) {
        try {
          // Note: This would require implementing a cancel order endpoint in the inventory service
          console.log('Cancel order:', orderId)
          // await orderService.cancelOrder(orderId)
          // await fetchOrders() // Refresh orders
        } catch (err) {
          console.error('Error cancelling order:', err)
        }
      }
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
        month: 'long',
        day: 'numeric',
        hour: '2-digit',
        minute: '2-digit'
      })
    }
    
    onMounted(() => {
      fetchOrders()
    })
    
    return {
      orders,
      selectedOrder,
      isLoading,
      error,
      viewOrderDetails,
      cancelOrder,
      getStatusText,
      formatDate
    }
  }
}
</script>

<style scoped>
.order-card {
  transition: transform 0.2s ease, box-shadow 0.2s ease;
  border: none;
  box-shadow: 0 2px 10px rgba(0,0,0,0.1);
}

.order-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 20px rgba(0,0,0,0.15);
}

.order-items {
  max-height: 200px;
  overflow-y: auto;
}

.badge {
  font-size: 0.75em;
}

@media (max-width: 768px) {
  .card-header {
    flex-direction: column;
    align-items: flex-start !important;
  }
  
  .card-header .text-end {
    margin-top: 10px;
    align-self: flex-end;
  }
}
</style>
