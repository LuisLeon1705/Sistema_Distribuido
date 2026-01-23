<template>
  <div class="orders-page">
    <div class="container py-5">
      
      <div class="d-flex justify-content-between align-items-center mb-5">
        <div>
          <h2 class="fw-bold text-dark mb-1">Mis Órdenes</h2>
          <p class="text-muted small mb-0">Historial de tus compras recientes</p>
        </div>
        <router-link to="/products" class="btn btn-dark rounded-pill px-4 shadow-sm">
          <i class="fas fa-shopping-bag me-2"></i>Nueva Compra
        </router-link>
      </div>
      
      <div v-if="isLoading" class="text-center py-5">
        <div class="spinner-border text-primary opacity-50" role="status" style="width: 3rem; height: 3rem;"></div>
        <p class="text-muted mt-3">Cargando historial...</p>
      </div>
      
      <div v-else-if="error" class="alert alert-danger bg-danger bg-opacity-10 border-danger text-danger border-0 m-3">
        <i class="fas fa-exclamation-circle me-2"></i>{{ error }}
      </div>
      
      <div v-else-if="orders.length === 0" class="text-center py-5 empty-state">
        <div class="mb-4">
          <div class="bg-light d-inline-block p-4 rounded-circle">
            <i class="fas fa-receipt fa-4x text-muted opacity-50"></i>
          </div>
        </div>
        <h3 class="fw-bold text-dark">Sin órdenes recientes</h3>
        <p class="text-muted mb-4">Aún no has realizado ninguna compra.</p>
        <router-link to="/products" class="btn btn-outline-dark rounded-pill px-4">
          Explorar Catálogo
        </router-link>
      </div>
      
      <div v-else class="row g-4">
        <div class="col-lg-8 mx-auto">
          <div v-for="order in orders" :key="order.id" class="order-card card border-0 shadow-sm rounded-4 mb-3 overflow-hidden">
            <div class="card-body p-4">
              <div class="d-flex justify-content-between align-items-start mb-3">
                <div>
                  <div class="d-flex align-items-center gap-2 mb-1">
                    <h5 class="fw-bold text-dark mb-0">Orden #{{ order.id }}</h5>
                    <span class="badge rounded-pill fw-normal px-3 py-1" :class="statusBadgeClass(order.status)">
                      {{ getStatusText(order.status) }}
                    </span>
                  </div>
                  <small class="text-muted">
                    <i class="far fa-calendar-alt me-1"></i> {{ formatDate(order.createdAt) }}
                  </small>
                </div>
                <div class="text-end">
                  <h4 class="fw-bold text-dark mb-0">${{ parseFloat(order.total).toFixed(2) }}</h4>
                </div>
              </div>
              
              <div class="d-flex justify-content-between align-items-center pt-3 border-top border-light">
                 <div class="d-flex align-items-center">
                    <div class="bg-light rounded p-2 me-2" style="width: 40px; height: 40px; display: flex; align-items: center; justify-content: center;">
                       <i class="fas fa-box text-muted opacity-50"></i>
                    </div>
                    <span class="text-muted small">Ver productos...</span>
                 </div>
                 <button 
                  class="btn btn-link text-primary text-decoration-none fw-bold small p-0"
                  @click="viewOrderDetails(order.id)"
                >
                  Ver Detalles <i class="fas fa-arrow-right ms-1"></i>
                </button>
              </div>
            </div>
          </div>
        </div>
      </div>

    </div>
    
    <div class="modal fade" id="orderDetailsModal" tabindex="-1">
      <div class="modal-dialog modal-dialog-centered modal-lg">
        <div class="modal-content border-0 shadow-lg rounded-4 overflow-hidden" v-if="selectedOrder">
          <div class="modal-header border-bottom bg-light px-4 py-3">
            <div>
              <h5 class="modal-title fw-bold text-dark">Detalles Orden #{{ selectedOrder.id }}</h5>
              <small class="text-muted">{{ formatDate(selectedOrder.createdAt) }}</small>
            </div>
            <button type="button" class="btn-close" data-bs-dismiss="modal"></button>
          </div>
          
          <div class="modal-body p-4">
            <div class="row g-4 mb-4">
              <div class="col-md-6">
                <div class="p-3 bg-light rounded-3 h-100">
                  <small class="text-muted text-uppercase fw-bold d-block mb-2">Estado</small>
                  <span class="badge rounded-pill px-3 py-2 fs-6" :class="statusBadgeClass(selectedOrder.status)">
                      {{ getStatusText(selectedOrder.status) }}
                  </span>
                </div>
              </div>
              <div class="col-md-6">
                 <div class="p-3 bg-light rounded-3 h-100 d-flex align-items-center justify-content-between">
                    <div>
                      <small class="text-muted text-uppercase fw-bold d-block mb-1">Total Pagado</small>
                      <span class="fw-bold text-dark h5 mb-0">${{ parseFloat(selectedOrder.total).toFixed(2) }}</span>
                    </div>
                    <i class="fas fa-receipt text-muted opacity-25 fa-2x"></i>
                 </div>
              </div>
            </div>

            <h6 class="fw-bold text-dark mb-3">Productos Comprados</h6>
            
            <div v-if="isFetchingDetails" class="text-center py-4">
               <div class="spinner-border spinner-border-sm text-primary"></div>
            </div>
            
            <div v-else class="table-responsive border rounded-3 overflow-hidden">
               <table class="table table-borderless mb-0">
                  <thead class="bg-light border-bottom">
                     <tr>
                        <th class="ps-3 text-muted small fw-bold">Producto</th>
                        <th class="text-center text-muted small fw-bold">Cant.</th>
                        <th class="pe-3 text-end text-muted small fw-bold">Subtotal</th>
                     </tr>
                  </thead>
                  <tbody>
                     <tr v-for="item in selectedOrderItemsWithDetails" :key="item.id" class="border-bottom-dashed">
                        <td class="ps-3 py-3">
                           <div class="d-flex align-items-center">
                              <img :src="item.productImage || '/placeholder-product.jpg'" class="rounded border bg-white me-3" style="width: 48px; height: 48px; object-fit: cover;">
                              <div>
                                 <span class="fw-bold text-dark d-block">{{ item.productName }}</span>
                                 <small class="text-muted">${{ parseFloat(item.price).toFixed(2) }} unitario</small>
                              </div>
                           </div>
                        </td>
                        <td class="text-center align-middle">{{ item.quantity }}</td>
                        <td class="text-end align-middle pe-3 fw-bold text-dark">${{ (item.quantity * parseFloat(item.price)).toFixed(2) }}</td>
                     </tr>
                  </tbody>
               </table>
            </div>

          </div>
          <div class="modal-footer border-0 bg-light px-4 py-3">
            <button type="button" class="btn btn-outline-dark rounded-pill px-4" data-bs-dismiss="modal">Cerrar</button>
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
    
    let orderDetailsModalInstance = null;

    const fetchOrders = async () => {
      isLoading.value = true;
      error.value = null;
      try { orders.value = await api.getOrdersByUserId(); } 
      catch (err) { orders.value = []; error.value = 'No se pudo cargar el historial.'; console.error(err); } 
      finally { isLoading.value = false; }
    }
    
    const viewOrderDetails = async (orderId) => {
      selectedOrder.value = orders.value.find(o => o.id === orderId) || null;
      if (!selectedOrder.value) return;

      if (orderDetailsModalInstance) orderDetailsModalInstance.show();
      
      isFetchingDetails.value = true;
      selectedOrderItemsWithDetails.value = [];
      try {
        const items = await api.getOrderItems(orderId);
        selectedOrderItemsWithDetails.value = await Promise.all(items.map(async (item) => {
            try {
               const product = await api.getProductById(item.productId);
               return { ...item, productName: product.nombre, productImage: product.imagen };
            } catch {
               return { ...item, productName: `Producto #${item.productId}`, productImage: null };
            }
        }));
      } catch(err) { console.error(err) } 
      finally { isFetchingDetails.value = false; }
    }
    
    const getStatusText = (status) => {
      const map = { CREADO: 'Creado', PENDING: 'Procesando', COMPLETED: 'Pagado', PAGADO: 'Pagado', CANCELADO: 'Cancelada', FAILED: 'Fallido' };
      return map[status] || status;
    }

    const statusBadgeClass = (status) => ({
      'bg-info bg-opacity-10 text-info border border-info border-opacity-25': status === 'CREADO',
      'bg-warning bg-opacity-10 text-warning border border-warning border-opacity-25': status === 'PENDING',
      'bg-success bg-opacity-10 text-success border border-success border-opacity-25': status === 'COMPLETED' || status === 'PAGADO',
      'bg-danger bg-opacity-10 text-danger border border-danger border-opacity-25': status === 'CANCELADO' || status === 'FAILED',
    });
    
    const formatDate = (dateString) => new Date(dateString).toLocaleDateString('es-ES', { year: 'numeric', month: 'long', day: 'numeric', hour: '2-digit', minute: '2-digit' });
    
    onMounted(() => {
      fetchOrders();
      const modalElement = document.getElementById('orderDetailsModal');
      if (modalElement) {
        orderDetailsModalInstance = new bootstrap.Modal(modalElement);
        modalElement.addEventListener('hidden.bs.modal', () => {
          selectedOrder.value = null;
          document.body.focus();
        });
      }
    })
    
    return {
      orders, selectedOrder, selectedOrderItemsWithDetails, isLoading, isFetchingDetails, error,
      viewOrderDetails, getStatusText, statusBadgeClass, formatDate
    }
  }
}
</script>

<style scoped>
.orders-page {
  background-color: #ffffff;
  min-height: 100vh;
}

.order-card {
  transition: transform 0.2s, box-shadow 0.2s;
}

.order-card:hover {
  transform: translateY(-4px);
  box-shadow: 0 10px 20px rgba(0,0,0,0.08) !important;
}

.border-bottom-dashed {
    border-bottom: 1px dashed #e2e8f0;
}
</style>