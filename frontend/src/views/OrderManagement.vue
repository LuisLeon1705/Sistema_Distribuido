<template>
  <div class="order-management-page">
    <div class="container py-5">
      
      <div class="d-flex justify-content-between align-items-center mb-5">
        <div>
          <h2 class="fw-bold text-dark mb-1">Gestión de Órdenes</h2>
          <p class="text-muted small mb-0">Administra y monitorea las transacciones del sistema</p>
        </div>
        <div class="d-flex gap-2">
          <button class="btn btn-dark btn-sm rounded-pill px-4 shadow-sm" @click="refreshOrders">
            <i class="fas fa-sync-alt me-2" :class="{ 'fa-spin': isLoading }"></i>
            Actualizar
          </button>
        </div>
      </div>

      <div class="row g-4 mb-5">
        <div class="col-md-6 col-lg-3">
          <div class="stat-card p-4 h-100 d-flex align-items-center">
            <div class="icon-circle bg-primary bg-opacity-10 text-primary me-3">
              <i class="fas fa-clipboard-list fa-lg"></i>
            </div>
            <div>
              <h6 class="text-muted small text-uppercase fw-bold mb-1">Total</h6>
              <h3 class="fw-bold text-dark mb-0">{{ orders.length }}</h3>
            </div>
          </div>
        </div>
        <div class="col-md-6 col-lg-3">
          <div class="stat-card p-4 h-100 d-flex align-items-center">
            <div class="icon-circle bg-warning bg-opacity-10 text-warning me-3">
              <i class="fas fa-clock fa-lg"></i>
            </div>
            <div>
              <h6 class="text-muted small text-uppercase fw-bold mb-1">Pendientes</h6>
              <h3 class="fw-bold text-dark mb-0">{{ getOrdersByStatus('CREADO').length + getOrdersByStatus('PENDING').length }}</h3>
            </div>
          </div>
        </div>
        <div class="col-md-6 col-lg-3">
          <div class="stat-card p-4 h-100 d-flex align-items-center">
            <div class="icon-circle bg-success bg-opacity-10 text-success me-3">
              <i class="fas fa-check-circle fa-lg"></i>
            </div>
            <div>
              <h6 class="text-muted small text-uppercase fw-bold mb-1">Completadas</h6>
              <h3 class="fw-bold text-dark mb-0">{{ orders.filter(o => o.status === 'COMPLETED' || o.status === 'PAGADO').length }}</h3>
            </div>
          </div>
        </div>
        <div class="col-md-6 col-lg-3">
          <div class="stat-card p-4 h-100 d-flex align-items-center">
            <div class="icon-circle bg-danger bg-opacity-10 text-danger me-3">
              <i class="fas fa-times-circle fa-lg"></i>
            </div>
            <div>
              <h6 class="text-muted small text-uppercase fw-bold mb-1">Canceladas</h6>
              <h3 class="fw-bold text-dark mb-0">{{ getOrdersByStatus('CANCELADO').length }}</h3>
            </div>
          </div>
        </div>
      </div>
      
      <div class="card border-0 shadow-sm rounded-4 mb-4 p-3 bg-white">
        <div class="row g-3 align-items-center">
          <div class="col-md-3">
            <div class="input-group input-group-sm">
              <span class="input-group-text bg-light border-0 text-muted"><i class="fas fa-filter"></i></span>
              <select v-model="filters.status" @change="applyFilters" class="form-select bg-light border-0 text-dark">
                <option value="">Todos los estados</option>
                <option value="CREADO">Creado</option>
                <option value="PENDING">Procesando</option>
                <option value="COMPLETED">Pagado</option>
                <option value="CANCELADO">Cancelada</option>
              </select>
            </div>
          </div>
          <div class="col-md-3">
            <div class="input-group input-group-sm">
               <span class="input-group-text bg-light border-0 text-muted"><i class="fas fa-calendar-alt"></i></span>
               <input type="date" v-model="filters.date" @change="applyFilters" class="form-control bg-light border-0 text-dark">
            </div>
          </div>
          <div class="col-md-4">
            <div class="input-group input-group-sm">
              <span class="input-group-text bg-light border-0 text-muted"><i class="fas fa-search"></i></span>
              <input type="text" v-model="filters.search" @input="applyFilters" placeholder="Buscar orden o usuario..." class="form-control bg-light border-0 text-dark">
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
            <p class="text-muted mt-2 small">Cargando datos...</p>
          </div>
          
          <div v-else-if="error" class="alert alert-danger m-3 border-0 bg-danger bg-opacity-10 text-danger">
            <i class="fas fa-exclamation-circle me-2"></i>{{ error }}
          </div>

          <div v-else class="table-responsive">
            <table class="table table-hover align-middle mb-0">
              <thead class="bg-light">
                <tr>
                  <th class="ps-4 py-3 text-muted small text-uppercase fw-bold border-0">Código</th>
                  <th class="py-3 text-muted small text-uppercase fw-bold border-0">Usuario</th>
                  <th class="py-3 text-muted small text-uppercase fw-bold border-0">Fecha</th>
                  <th class="py-3 text-muted small text-uppercase fw-bold border-0">Total</th>
                  <th class="py-3 text-muted small text-uppercase fw-bold border-0">Estado</th>
                  <th class="pe-4 py-3 text-muted small text-uppercase fw-bold border-0 text-end">Acción</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="order in paginatedOrders" :key="order.id" class="cursor-pointer" @click="viewOrderDetails(order.id)">
                    <td class="ps-4 fw-bold text-dark">{{ order.code || order.id }}</td>
                  <td>
                    <div class="d-flex align-items-center">
                      <div class="avatar-sm bg-secondary bg-opacity-10 text-secondary rounded-circle me-2 d-flex align-items-center justify-content-center fw-bold" style="width: 30px; height: 30px; font-size: 0.8rem;">
                        {{ getUsername(order.userId).charAt(0).toUpperCase() }}
                      </div>
                      <span class="text-dark">{{ getUsername(order.userId) }}</span>
                    </div>
                  </td>
                  <td class="text-muted small">{{ formatDate(order.createdAt) }}</td>
                  <td class="fw-bold text-dark">${{ parseFloat(order.total).toFixed(2) }}</td>
                  <td>
                    <span class="badge rounded-pill fw-normal px-3 py-1" :class="statusBadgeClass(order.status)">
                      {{ getStatusText(order.status) }}
                    </span>
                  </td>
                  <td class="pe-4 text-end">
                    <button class="btn btn-light btn-sm rounded-circle text-muted hover-bg-gray" @click.stop="viewOrderDetails(order.id)">
                      <i class="fas fa-chevron-right"></i>
                    </button>
                  </td>
                </tr>
              </tbody>
            </table>

            <div v-if="filteredOrders.length === 0" class="text-center py-5">
              <div class="mb-3 opacity-25">
                <i class="fas fa-search fa-3x text-muted"></i>
              </div>
              <h6 class="text-dark">No se encontraron resultados</h6>
              <p class="text-muted small">Intenta cambiar los filtros de búsqueda</p>
            </div>
          </div>
          
          <div v-if="!isLoading && filteredOrders.length > 0 && totalPages > 1" class="d-flex justify-content-between align-items-center p-4 border-top">
             <small class="text-muted">
               Mostrando {{ (currentPage - 1) * itemsPerPage + 1 }} - {{ Math.min(currentPage * itemsPerPage, filteredOrders.length) }} de {{ filteredOrders.length }}
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
    
    <div class="modal fade" id="orderDetailsModal" tabindex="-1">
      <div class="modal-dialog modal-dialog-centered modal-lg">
        <div class="modal-content border-0 shadow-lg rounded-4 overflow-hidden" v-show="selectedOrder">
          <div v-if="selectedOrder">
            <div class="modal-header border-bottom bg-light px-4 py-3">
              <div>
                <h5 class="modal-title fw-bold text-dark">Orden {{ selectedOrder.code || selectedOrder.id }}</h5>
                <small class="text-muted">{{ formatDate(selectedOrder.createdAt) }}</small>
              </div>
              <button type="button" class="btn-close" data-bs-dismiss="modal"></button>
            </div>
            
            <div class="modal-body p-4">
            <div class="row g-4 mb-4">
              <div class="col-md-6">
                <div class="p-3 bg-light rounded-3 h-100">
                  <h6 class="fw-bold text-dark mb-3">Cliente</h6>
                  <div class="d-flex align-items-center">
                     <div class="avatar-md bg-primary text-white rounded-circle me-3 d-flex align-items-center justify-content-center fw-bold" style="width: 40px; height: 40px;">
                        {{ getUsername(selectedOrder.userId).charAt(0).toUpperCase() }}
                     </div>
                     <div>
                       <span class="fw-bold text-dark d-block">{{ getUsername(selectedOrder.userId) }}</span>
                     </div>
                  </div>
                </div>
              </div>
              <div class="col-md-6">
                <div class="p-3 bg-light rounded-3 h-100">
                  <h6 class="fw-bold text-dark mb-3">Gestión de Estado</h6>
                  <div v-if="selectedOrder.status === 'pending' || selectedOrder.status === 'CREADO'" class="d-flex gap-2">
                    <select v-model="selectedOrderForm.status" class="form-select border-0 bg-white shadow-sm">
                      <option value="pending">Pendiente</option>
                      <option value="completed">Completar (Pagado)</option>
                      <option value="cancelled">Cancelar</option>
                    </select>
                    <button 
                      class="btn btn-primary px-3 shadow-sm"
                      @click="saveOrderChanges" 
                      :disabled="isSaving"
                    >
                      <span v-if="isSaving" class="spinner-border spinner-border-sm"></span>
                      <span v-else>Guardar</span>
                    </button>
                  </div>
                  <div v-else class="d-flex align-items-center h-50">
                     <span class="badge rounded-pill px-3 py-2 fs-6" :class="statusBadgeClass(selectedOrder.status)">
                       {{ getStatusText(selectedOrder.status) }}
                     </span>
                  </div>
                </div>
              </div>
            </div>
            
            <h6 class="fw-bold text-dark mb-3">Detalle de Productos</h6>
            <div v-if="isFetchingDetails" class="text-center py-3">
              <div class="spinner-border spinner-border-sm text-primary"></div>
            </div>
            <div v-else>
               <div class="table-responsive border rounded-3 overflow-hidden">
                  <table class="table table-borderless mb-0">
                     <thead class="bg-light border-bottom">
                        <tr>
                           <th class="text-muted small fw-bold ps-3">Producto</th>
                           <th class="text-muted small fw-bold text-center">Cant.</th>
                           <th class="text-muted small fw-bold text-end pe-3">Subtotal</th>
                        </tr>
                     </thead>
                     <tbody>
                        <tr v-if="selectedOrderItemsWithDetails.length === 0">
                           <td colspan="3" class="text-center py-4 text-muted">
                              <small>No se encontraron productos para esta orden</small>
                           </td>
                        </tr>
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
                     <tfoot class="bg-light">
                        <tr>
                           <td colspan="2" class="text-end fw-bold text-dark pt-3">Total General</td>
                           <td class="text-end fw-bold text-primary fs-5 pe-3 pt-3">
                              ${{ parseFloat(selectedOrder.total).toFixed(2) }}
                           </td>
                        </tr>
                     </tfoot>
                  </table>
               </div>
            </div>

            <div class="mt-4">
              <h6 class="fw-bold text-dark mb-2">Dirección de Envío</h6>
              <div class="p-3 bg-light rounded-3">
                <div class="mb-1"><small class="text-muted">Dirección:</small> <span class="fw-bold">{{ selectedOrder.shippingAddress || 'No disponible' }}</span></div>
                <div class="mb-1"><small class="text-muted">Ciudad:</small> <span class="fw-bold">{{ selectedOrder.shippingCity || 'No disponible' }}</span></div>
                <div class="mb-1"><small class="text-muted">Código Postal:</small> <span class="fw-bold">{{ selectedOrder.shippingPostal || 'No disponible' }}</span></div>
              </div>
            </div>
            <div class="modal-footer border-0 bg-light px-4 py-3">
              <button type="button" class="btn btn-outline-dark rounded-pill px-4" data-bs-dismiss="modal">Cerrar</button>
            </div>
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
  name: 'OrderManagement',
  setup() {
    const orders = ref([])
    const usersCache = ref({})
    const selectedOrder = ref(null)
    const selectedOrderItems = ref([])
    const selectedOrderItemsWithDetails = ref([])
    const isLoading = ref(false)
    const isFetchingDetails = ref(false)
    const isSaving = ref(false)
    const error = ref(null)
    let orderDetailsModalInstance = null;

    const filters = reactive({ status: '', date: '', search: '' });
    const selectedOrderForm = reactive({ status: '' });
    
    const currentPage = ref(1);
    const itemsPerPage = ref(10);
    
    const filteredOrders = computed(() => {
      return orders.value
        .filter(order => {
          const statusMatch = !filters.status || order.status === filters.status;
          const toLocalYMD = (d) => {
            const dt = new Date(d)
            return `${dt.getFullYear()}-${String(dt.getMonth() + 1).padStart(2, '0')}-${String(dt.getDate()).padStart(2, '0')}`
          }
          const dateMatch = !filters.date || toLocalYMD(order.createdAt) === filters.date
          const searchMatch = !filters.search || 
            order.id.toString().includes(filters.search) || 
            getUsername(order.userId).toLowerCase().includes(filters.search.toLowerCase());
          return statusMatch && dateMatch && searchMatch;
        })
        .sort((a, b) => new Date(b.createdAt) - new Date(a.createdAt));
    });
    
    const totalPages = computed(() => Math.ceil(filteredOrders.value.length / itemsPerPage.value));

    const paginatedOrders = computed(() => {
      const start = (currentPage.value - 1) * itemsPerPage.value;
      const end = start + itemsPerPage.value;
      return filteredOrders.value.slice(start, end);
    });
    
    const goToPage = (page) => {
      if (page >= 1 && page <= totalPages.value) {
        currentPage.value = page;
        window.scrollTo({ top: 0, behavior: 'smooth' });
      }
    };
    
    watch([() => filters.status, () => filters.date, () => filters.search], () => { currentPage.value = 1 });
    
    const fetchOrders = async () => {
      isLoading.value = true
      error.value = null
      try {
        const ordersData = await api.getAllOrders();
        orders.value = (ordersData || []).slice().sort((a, b) => new Date(b.createdAt) - new Date(a.createdAt));

        const userIds = [...new Set(ordersData.map(o => o.userId))].filter(id => !usersCache.value[id]);
        if (userIds.length > 0) {
          const userPromises = userIds.map(id => api.getUserById(id));
          const usersData = await Promise.all(userPromises);
          usersData.forEach(user => {
            if (user) usersCache.value[user.id] = user.username;
          });
        }
      } catch (err) {
        error.value = 'Error al cargar las órdenes.'
        console.error(err)
      } finally {
        isLoading.value = false
      }
    };
    
    const getUsername = (userId) => usersCache.value[userId] || `Usuario ${userId}`;

    const handleSeedStock = async () => {
       if(!confirm('¿Generar stock inicial?')) return;
       try { await api.seedStock(); alert('Stock generado'); } catch(e) { console.error(e); }
    }

    const refreshOrders = () => fetchOrders();
    const getOrdersByStatus = (status) => {
      if (status === 'CANCELADO') {
        return orders.value.filter(order => order.status === 'CANCELADO' || order.status === 'FAILED')
      }
      return orders.value.filter(order => order.status === status)
    };
    
    const statusMap = { 
      CREADO: 'Creado',
      PENDING: 'Procesando', 
      COMPLETED: 'Pagado', 
      PAGADO: 'Pagado',
      CANCELADO: 'Cancelada',
      FAILED: 'Cancelada'
    };
    const getStatusText = (status) => statusMap[status] || status;

    const statusBadgeClass = (status) => ({
      'bg-info bg-opacity-10 text-info border border-info border-opacity-25': status === 'CREADO',
      'bg-warning bg-opacity-10 text-warning border border-warning border-opacity-25': status === 'PENDING',
      'bg-success bg-opacity-10 text-success border border-success border-opacity-25': status === 'COMPLETED' || status === 'PAGADO',
      'bg-danger bg-opacity-10 text-danger border border-danger border-opacity-25': status === 'CANCELADO' || status === 'FAILED',
    });
    
    const formatDate = (dateString) => {
      const date = new Date(dateString);
      const time = new Date(date.getTime() - (date.getTimezoneOffset() * 60000));
      return time.toLocaleDateString('es-ES', { year: 'numeric', month: 'long', day: 'numeric', hour: '2-digit', minute: '2-digit' });
    };
    
    const viewOrderDetails = async (orderId) => {
      selectedOrder.value = orders.value.find(o => o.id === orderId) || null;
      if (!selectedOrder.value) return;

      if (orderDetailsModalInstance) orderDetailsModalInstance.show();
      
      isFetchingDetails.value = true;
      selectedOrderItemsWithDetails.value = [];
      try {
        console.log('Fetching items for order:', orderId);
        const response = await api.getOrderItems(orderId);
        console.log('Raw API response:', response);
        
        let items = response;
        
        if (!items && response.data) {
          items = response.data.items || response.data;
        }
        
        console.log('Items extracted:', items);
        
        if (!items || items.length === 0) {
          console.log('No items found for order:', orderId);
          return;
        }
        
        selectedOrderItemsWithDetails.value = await Promise.all(items.map(async (item) => {
            try {
               console.log('Fetching product for ID:', item.productId);
               const product = await api.getProductById(item.productId);
               console.log('Product data:', product);
               return { ...item, productName: product.nombre, productImage: product.imagen };
            } catch (productError) {
               console.error('Error fetching product:', item.productId, productError);
               return { ...item, productName: `Producto #${item.productId}`, productImage: null };
            }
        }));
        console.log('Final items with details:', selectedOrderItemsWithDetails.value);
      } catch(err) { 
        console.error('Error fetching order items:', err); 
        
        if (err.response?.status === 404 || err.response?.status === 403) {
          console.log('Trying fallback method...');
          try {
            const orderResponse = await api.getOrderById(orderId);
            const items = orderResponse.items || orderResponse.data?.items || [];
            console.log('Fallback items:', items);
            
            selectedOrderItemsWithDetails.value = await Promise.all(items.map(async (item) => {
                try {
                   const product = await api.getProductById(item.productId);
                   return { ...item, productName: product.nombre, productImage: product.imagen };
                } catch {
                   return { ...item, productName: `Producto #${item.productId}`, productImage: null };
                }
            }));
          } catch(fallbackErr) {
            console.error('Fallback also failed:', fallbackErr);
            alert('No se pueden cargar los detalles de esta orden');
          }
        } else {
          alert('Error al cargar los detalles de la orden');
        }
      } 
      finally { isFetchingDetails.value = false; }
    }
    
    const saveOrderChanges = async () => {
      if (!selectedOrder.value || selectedOrder.value.status === selectedOrderForm.status) return;
      
      if (confirm(`¿Cambiar estado a "${getStatusText(selectedOrderForm.status)}"?`)) {
        isSaving.value = true;
        try {
          await api.updateOrderStatus(selectedOrder.value.id, selectedOrderForm.status)
          await fetchOrders();
          orderDetailsModalInstance?.hide();
        } catch (err) {
          console.error(err)
          alert('Error al actualizar estado')
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
        orderDetailsModalInstance = new bootstrap.Modal(modalElement);
        modalElement.addEventListener('hidden.bs.modal', () => {
          selectedOrder.value = null;
        });
      }
    });
    
    return {
      orders, selectedOrder, selectedOrderForm, selectedOrderItems, selectedOrderItemsWithDetails, filteredOrders, paginatedOrders,
      isLoading, isFetchingDetails, isSaving, error, filters, currentPage, itemsPerPage, totalPages,
      refreshOrders, getOrdersByStatus, getStatusText, statusBadgeClass, formatDate, getUsername,
      viewOrderDetails, saveOrderChanges, applyFilters, clearFilters, handleSeedStock, goToPage
    }
  }
}
</script>

<style scoped>
.order-management-page {
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

.hover-bg-gray:hover {
  background-color: #e2e8f0;
}

.hover-text-dark:hover {
  color: #000 !important;
}

.cursor-pointer {
  cursor: pointer;
}

.border-bottom-dashed {
    border-bottom: 1px dashed #e2e8f0;
}

.order-management-page .table.table-hover.align-middle tbody tr td,
.order-management-page .table.table-hover.align-middle tbody tr th {
  padding-top: 1rem;
  padding-bottom: 1rem;
}
</style>