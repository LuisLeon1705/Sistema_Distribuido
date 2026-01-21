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
          
          <div v-if="isLoading" class="text-center py-5">
            <div class="spinner-border" role="status">
              <span class="visually-hidden">Cargando...</span>
            </div>
            <p class="mt-2">Cargando órdenes...</p>
          </div>
          
          <div v-else-if="error" class="alert alert-danger">
            {{ error }}
          </div>
          
          <div v-else-if="orders.length === 0" class="text-center py-5">
            <i class="fas fa-receipt fa-5x text-muted mb-4"></i>
            <h3>No tienes órdenes aún</h3>
            <p class="text-muted mb-4">Realiza tu primera compra para ver tus órdenes aquí</p>
            <router-link to="/products" class="btn btn-primary btn-lg">
              Explorar Productos
            </router-link>
          </div>
          
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
                  <div class="d-flex justify-content-end">
                    <button 
                      class="btn btn-outline-primary btn-sm"
                      @click="viewOrderDetails(order.id)"
                    >
                      <i class="fas fa-eye me-1"></i>
                      Ver Detalles
                    </button>
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
            
            <div v-if="isFetchingDetails" class="text-center">
              <div class="spinner-border spinner-border-sm" role="status">
                <span class="visually-hidden">Cargando productos...</span>
              </div>
            </div>
            <div v-else-if="selectedOrderItemsWithDetails.length > 0">
              <h6 class="mb-3">Productos en esta orden</h6>
              <ul class="list-group list-group-flush">
                <li v-for="item in selectedOrderItemsWithDetails" :key="item.id" class="list-group-item d-flex justify-content-between align-items-center">
                  <div class="d-flex align-items-center">
                    <img :src="item.productImage || '/placeholder-product.jpg'" :alt="item.productName" class="me-3" style="width: 50px; height: 50px; object-fit: cover; border-radius: 4px;">
                    <div>
                      <div class="fw-bold">{{ item.productName }}</div>
                      <small class="text-muted">{{ item.quantity }} x ${{ parseFloat(item.price_at_time_of_purchase).toFixed(2) }}</small>
                    </div>
                  </div>
                  <span class="fw-bold">${{ (item.quantity * parseFloat(item.price_at_time_of_purchase)).toFixed(2) }}</span>
                </li>
              </ul>
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
import { useAuthStore } from '../stores/auth'
import api from '../services/api'
import * as bootstrap from 'bootstrap'

export default {
  name: 'Orders',
  setup() {
    const orders = ref([])
    const selectedOrder = ref(null)
    const selectedOrderItemsWithDetails = ref([])
    const isLoading = ref(false)
    const isFetchingDetails = ref(false)
    const error = ref(null)
    const authStore = useAuthStore()
    
    let orderDetailsModalInstance = null;

    const fetchOrders = async () => {
      if (!authStore.user?.id) {
        error.value = 'No se pudo obtener el ID del usuario para cargar las órdenes.'
        return
      }
      isLoading.value = true;
      error.value = null;
      try {
        orders.value = await api.getOrdersByUserId(authStore.user.id);
      } catch (err) {
        orders.value = [];
        error.value = 'Error al cargar las órdenes';
        console.error('Error fetching orders:', err);
      } finally {
        isLoading.value = false;
      }
    }
    
    const viewOrderDetails = async (orderId) => {
      selectedOrder.value = orders.value.find(o => o.id === orderId) || null;
      if (!selectedOrder.value) return;

      if (orderDetailsModalInstance) {
        orderDetailsModalInstance.show();
      }
      
      isFetchingDetails.value = true;
      selectedOrderItemsWithDetails.value = [];
      try {
        const items = await api.getOrderItems(orderId);
        
        const detailedItems = await Promise.all(
          items.map(async (item) => {
            try {
              const product = await api.getProductById(item.product_id);
              return {
                ...item,
                productName: product.nombre,
                productImage: product.imagen
              };
            } catch (e) {
              console.error(`Error fetching product ${item.product_id}:`, e);
              return {
                ...item,
                productName: `Producto ID: ${item.product_id}`,
                productImage: '/placeholder-product.jpg'
              };
            }
          })
        );
        selectedOrderItemsWithDetails.value = detailedItems;
      } catch(err) {
        console.error('Error fetching order items:', err)
      } finally {
        isFetchingDetails.value = false;
      }
    }
    
    const cancelOrder = async (orderId) => {
      if (confirm('¿Estás seguro de que deseas cancelar esta orden?')) {
        try {
          console.log('Cancel order:', orderId)
        } catch (err) {
          console.error('Error cancelling order:', err)
        }
      }
    }
    
    const getStatusText = (status) => {
      const statusMap = { pending: 'Pendiente', completed: 'Completada', cancelled: 'Cancelada' }
      return statusMap[status] || status
    }
    
    const formatDate = (dateString) => {
      const date = new Date(dateString)
      return date.toLocaleDateString('es-ES', {
        year: 'numeric', month: 'long', day: 'numeric',
        hour: '2-digit', minute: '2-digit'
      })
    }
    
    onMounted(() => {
      fetchOrders()
      const modalElement = document.getElementById('orderDetailsModal');
      if (modalElement) {
        orderDetailsModalInstance = new bootstrap.Modal(modalElement);
        modalElement.addEventListener('hidden.bs.modal', () => {
          selectedOrder.value = null;
          selectedOrderItemsWithDetails.value = [];
          document.body.focus();
        });
      }
    })
    
    return {
      orders,
      selectedOrder,
      selectedOrderItemsWithDetails,
      isLoading,
      isFetchingDetails,
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
.badge {
  font-size: 0.75em;
}
</style>