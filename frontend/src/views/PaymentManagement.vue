<template>
  <div class="payment-management-page">
    <div class="container py-5">
      
      <div class="d-flex justify-content-between align-items-center mb-5">
        <div>
          <h2 class="fw-bold text-dark mb-1">Gestión de Pagos</h2>
          <p class="text-muted small mb-0">Administra todos los pagos y transacciones</p>
        </div>
        <div class="d-flex gap-2">
          <button @click="refreshPayments" class="btn btn-outline-dark btn-sm rounded-pill px-4">
            <i class="fas fa-sync-alt me-2"></i>Actualizar
          </button>
          <button @click="exportPayments" class="btn btn-primary btn-sm rounded-pill px-4">
            <i class="fas fa-download me-2"></i>Exportar
          </button>
        </div>
      </div>

      <div class="row g-4 mb-5">
        <div class="col-md-6 col-lg-3">
          <div class="stat-card p-4 h-100 d-flex align-items-center">
            <div class="icon-circle bg-success bg-opacity-10 text-success me-3">
              <i class="fas fa-dollar-sign fa-lg"></i>
            </div>
            <div>
              <h6 class="text-muted small text-uppercase fw-bold mb-1">Ingresos Totales</h6>
              <h3 class="fw-bold text-dark mb-0">${{ totalRevenue.toFixed(2) }}</h3>
            </div>
          </div>
        </div>

        <div class="col-md-6 col-lg-3">
          <div class="stat-card p-4 h-100 d-flex align-items-center">
            <div class="icon-circle bg-primary bg-opacity-10 text-primary me-3">
              <i class="fas fa-credit-card fa-lg"></i>
            </div>
            <div>
              <h6 class="text-muted small text-uppercase fw-bold mb-1">Total Pagos</h6>
              <h3 class="fw-bold text-dark mb-0">{{ totalPayments }}</h3>
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
              <h3 class="fw-bold text-dark mb-0">{{ pendingPayments }}</h3>
            </div>
          </div>
        </div>

        <div class="col-md-6 col-lg-3">
          <div class="stat-card p-4 h-100 d-flex align-items-center">
            <div class="icon-circle bg-danger bg-opacity-10 text-danger me-3">
              <i class="fas fa-exclamation-circle fa-lg"></i>
            </div>
            <div>
              <h6 class="text-muted small text-uppercase fw-bold mb-1">Fallidos</h6>
              <h3 class="fw-bold text-dark mb-0">{{ failedPayments }}</h3>
            </div>
          </div>
        </div>
      </div>

      <div class="card border-0 shadow-sm rounded-4 mb-4 p-3 bg-white">
        <div class="row g-3 align-items-center">
          <div class="col-md-3">
            <div class="input-group input-group-sm">
              <span class="input-group-text bg-light border-0 text-muted"><i class="fas fa-filter"></i></span>
              <select v-model="statusFilter" @change="applyFilters" class="form-select bg-light border-0 text-dark">
                <option value="">Todos los estados</option>
                <option value="completed">Completado</option>
                <option value="pending">Pendiente</option>
                <option value="failed">Fallido</option>
              </select>
            </div>
          </div>

          <div class="col-md-3">
            <div class="input-group input-group-sm">
              <span class="input-group-text bg-light border-0 text-muted"><i class="fas fa-calendar-alt"></i></span>
              <input type="date" v-model="dateFilter" @change="applyFilters" class="form-control bg-light border-0 text-dark">
            </div>
          </div>

          <div class="col-md-4">
            <div class="input-group input-group-sm">
              <span class="input-group-text bg-light border-0 text-muted"><i class="fas fa-search"></i></span>
              <input type="text" v-model="searchTerm" @input="applyFilters" placeholder="ID, usuario, método..." class="form-control bg-light border-0 text-dark">
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
            <div class="spinner-border text-dark opacity-25"></div>
          </div>
          
          <div v-else-if="filteredPayments.length === 0" class="text-center py-5">
            <div class="mb-3 opacity-25">
              <i class="fas fa-credit-card fa-3x text-muted"></i>
            </div>
            <h6 class="text-dark">No se encontraron pagos</h6>
          </div>
          
          <div v-else class="table-responsive">
            <table class="table table-hover align-middle mb-0">
              <thead class="bg-light">
                <tr>
                  <th class="ps-4 py-3 text-muted small text-uppercase fw-bold border-0">Código</th>
                  <th class="py-3 text-muted small text-uppercase fw-bold border-0">Usuario</th>
                  <th class="py-3 text-muted small text-uppercase fw-bold border-0">Orden</th>
                  <th class="py-3 text-muted small text-uppercase fw-bold border-0">Monto</th>
                  <th class="py-3 text-muted small text-uppercase fw-bold border-0">Método</th>
                  <th class="py-3 text-muted small text-uppercase fw-bold border-0">Estado</th>
                  <th class="py-3 text-muted small text-uppercase fw-bold border-0">Fecha</th>
                  <th class="pe-4 py-3 text-muted small text-uppercase fw-bold border-0 text-end">Acción</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="payment in paginatedPayments" :key="payment.id" class="cursor-pointer" @click="viewPaymentDetails(payment)">
                  <td class="ps-4 fw-bold text-dark">{{ payment.code || payment.id }}</td>
                  <td>
                    <div class="d-flex align-items-center">
                      <div class="avatar-sm bg-secondary bg-opacity-10 text-secondary rounded-circle me-2 d-flex align-items-center justify-content-center fw-bold" style="width: 30px; height: 30px; font-size: 0.8rem;">
                        {{ getUserName(payment.user_id).charAt(0).toUpperCase() }}
                      </div>
                      <span class="text-dark">{{ getUserName(payment.user_id) }}</span>
                    </div>
                  </td>
                  <td class="fw-bold">{{ payment.order_code || payment.order_id }}</td>
                  <td class="fw-bold text-dark">${{ parseFloat(payment.amount).toFixed(2) }}</td>
                  <td>
                    <div class="d-flex align-items-center">
                      <i :class="getPaymentMethodIcon(payment.method)" class="me-2"></i>
                      <span>{{ getPaymentMethodName(payment.method) }}</span>
                    </div>
                  </td>
                  <td>
                    <span class="badge rounded-pill fw-normal px-3 py-1" :class="getStatusBadgeClass(payment.status)">
                      {{ getStatusText(payment.status) }}
                    </span>
                  </td>
                  <td class="text-muted small">{{ formatDate(payment.created_at) }}</td>
                  <td class="text-center pe-4">
                    <div class="btn-group btn-group-sm">
                      <button class="btn btn-light rounded-circle" @click="viewPaymentDetails(payment)">
                        <i class="fas fa-eye"></i>
                      </button>
                    </div>
                  </td>
                </tr>
              </tbody>
            </table>
          </div>
        </div>
        
        <div v-if="!isLoading && totalPages > 1" class="d-flex justify-content-between align-items-center p-4 border-top">
          <small class="text-muted">
            Mostrando {{ (currentPage - 1) * itemsPerPage + 1 }} - {{ Math.min(currentPage * itemsPerPage, filteredPayments.length) }} de {{ filteredPayments.length }}
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
    
    <div class="modal fade" id="paymentDetailsModal" tabindex="-1">
      <div class="modal-dialog modal-dialog-centered modal-lg">
        <div class="modal-content border-0 shadow-lg rounded-4 overflow-hidden" v-show="selectedPayment">
          <div v-if="selectedPayment">
            <div class="modal-header border-bottom bg-light px-4 py-3">
              <div>
                <h5 class="modal-title fw-bold text-dark">Detalles del Pago</h5>
                <small class="text-muted">{{ formatDate(selectedPayment.created_at) }}</small>
              </div>
              <button type="button" class="btn-close" data-bs-dismiss="modal"></button>
            </div>
            
            <div class="modal-body p-4">
            <div class="row g-4 mb-4">
              <div class="col-md-6">
                <div class="p-3 bg-light rounded-3 h-100">
                  <h6 class="fw-bold text-dark mb-3">Información del Pago</h6>
                  <div class="mb-2">
                    <small class="text-muted">Código:</small>
                    <span class="fw-bold text-dark d-block">{{ selectedPayment.code || selectedPayment.id }}</span>
                  </div>
                  <div class="mb-2">
                    <small class="text-muted">Monto:</small>
                    <span class="fw-bold text-dark d-block">${{ parseFloat(selectedPayment.amount).toFixed(2) }}</span>
                  </div>
                  <div class="mb-2">
                    <small class="text-muted">Método:</small>
                    <span class="fw-bold text-dark d-block">{{ getPaymentMethodName(selectedPayment.method) }}</span>
                  </div>
                  <div class="mb-2">
                    <small class="text-muted">Estado:</small>
                    <div class="mt-1">
                      <span class="badge rounded-pill px-3 py-1" :class="getStatusBadgeClass(selectedPayment.status)">
                        {{ getStatusText(selectedPayment.status) }}
                      </span>
                    </div>
                  </div>
                </div>
              </div>
              
              <div class="col-md-6">
                <div class="p-3 bg-light rounded-3 h-100">
                  <h6 class="fw-bold text-dark mb-3">Información del Cliente</h6>
                  <div class="mb-2">
                    <small class="text-muted">Usuario:</small>
                    <span class="fw-bold text-dark d-block">{{ getUserName(selectedPayment.user_id) }}</span>
                  </div>
                  <div class="mb-2">
                    <small class="text-muted">Orden:</small>
                    <div class="fw-bold text-dark d-block">{{ (selectedPayment.orderDetails && selectedPayment.orderDetails.code) || selectedPayment.order_code || selectedPayment.order_id }}</div>
                  </div>
                </div>
              </div>
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
</template>

<script>
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useAuthStore } from '../stores/auth'
import api from '../services/api'
import * as bootstrap from 'bootstrap'

export default {
  name: 'PaymentManagement',
  setup() {
    const authStore = useAuthStore()
    const payments = ref([])
    const users = ref({})
    const isLoading = ref(false)
    const selectedPayment = ref(null)
    let paymentDetailsModal = null
    
    const searchTerm = ref('')
    const statusFilter = ref('')
    const methodFilter = ref('')
    const dateFilter = ref('')
    const currentPage = ref(1)
    const itemsPerPage = ref(10)
    
    const totalRevenue = computed(() => {
      return payments.value
        .filter(p => p.status === 'completed')
        .reduce((sum, p) => sum + parseFloat(p.amount), 0)
    })
    
    const totalPayments = computed(() => payments.value.length)
    const pendingPayments = computed(() => payments.value.filter(p => p.status === 'pending').length)
    const failedPayments = computed(() => payments.value.filter(p => p.status === 'failed').length)
    
    const filteredPayments = computed(() => {
      let filtered = payments.value

      if (searchTerm.value) {
        const search = searchTerm.value.toLowerCase()
        filtered = filtered.filter(payment => 
          payment.id.toString().includes(search) ||
          ((payment.order_code && payment.order_code.toLowerCase().includes(search)) || payment.order_id.toString().includes(search)) ||
          getUserName(payment.user_id).toLowerCase().includes(search) ||
          payment.method.toLowerCase().includes(search)
        )
      }
      
      if (statusFilter.value) {
        filtered = filtered.filter(payment => payment.status === statusFilter.value)
      }
      
      if (methodFilter.value) {
        filtered = filtered.filter(payment => payment.method === methodFilter.value)
      }
      
      if (dateFilter.value) {
        const toLocalYMD = (d) => {
          const dt = new Date(d)
          return `${dt.getFullYear()}-${String(dt.getMonth() + 1).padStart(2, '0')}-${String(dt.getDate()).padStart(2, '0')}`
        }
        const filterYMD = dateFilter.value
        filtered = filtered.filter(payment => {
          const paymentYMD = toLocalYMD(payment.created_at)
          return paymentYMD === filterYMD
        })
      }
      
      return filtered.sort((a, b) => new Date(b.created_at) - new Date(a.created_at))
    })

    const totalPages = computed(() => Math.ceil(filteredPayments.value.length / itemsPerPage.value))
    
    const paginatedPayments = computed(() => {
      const start = (currentPage.value - 1) * itemsPerPage.value
      const end = start + itemsPerPage.value
      return filteredPayments.value.slice(start, end)
    })
    
    const fetchPayments = async () => {
      isLoading.value = true
      try {
        const paymentsData = await api.getPayments()
        payments.value = (paymentsData || []).slice().sort((a, b) => new Date(b.created_at) - new Date(a.created_at))

        const userIds = [...new Set(paymentsData.map(p => p.user_id))].filter(id => !users.value[id])
        if (userIds.length > 0) {
          const userPromises = userIds.map(id => api.getUserById(id))
          const usersData = await Promise.all(userPromises)
          usersData.forEach(user => {
            if (user) users.value[user.id] = user.username
          })
        }

        const orderIds = [...new Set(paymentsData.map(p => p.order_id))].filter(Boolean)
        if (orderIds.length > 0) {
          const orderPromises = orderIds.map(id =>
            api.getOrderCode(id)
              .then(res => ({ id, code: res && res.code ? res.code : null }))
              .catch(() => ({ id, code: null }))
          )
          const ordersData = await Promise.all(orderPromises)
          const ordersById = {}
          ordersData.forEach(o => { if (o && o.id) ordersById[o.id] = { code: o.code } })

          console.debug('Fetched order codes for payments:', ordersById)

          payments.value = payments.value.map(p => ({
            ...p,
            order_code: ordersById[p.order_id]?.code || p.order_code || p.metadata?.order_code || p.metadata?.shipping_info?.order_code || null,
            orderDetails: p.orderDetails
          }))
        }

      } catch (error) {
        console.error('Error fetching payments:', error)
      } finally {
        isLoading.value = false
      }
    }
    
    const getUserName = (userId) => {
      return users.value[userId] || `Usuario ${userId}`
    }
    
    const getPaymentMethodName = (method) => {
      const methods = {
        credit: 'Tarjeta Crédito',
        debit: 'Tarjeta Débito',
        paypal: 'PayPal',
        transfer: 'Transferencia'
      }
      return methods[method] || method
    }
    
    const getPaymentMethodIcon = (method) => {
      const icons = {
        credit: 'fas fa-credit-card text-primary',
        debit: 'fas fa-credit-card text-success',
        paypal: 'fab fa-paypal text-info',
        transfer: 'fas fa-exchange-alt text-warning'
      }
      return icons[method] || 'fas fa-money-bill text-muted'
    }
    
    const getStatusText = (status) => {
      const statuses = {
        completed: 'Completado',
        pending: 'Pendiente',
        failed: 'Fallido',
        refunded: 'Reembolsado'
      }
      return statuses[status] || status
    }
    
    const getStatusBadgeClass = (status) => {
      const classes = {
        completed: 'bg-success bg-opacity-10 text-success border border-success border-opacity-25',
        pending: 'bg-warning bg-opacity-10 text-warning border border-warning border-opacity-25',
        failed: 'bg-danger bg-opacity-10 text-danger border border-danger border-opacity-25',
        refunded: 'bg-info bg-opacity-10 text-info border border-info border-opacity-25'
      }
      return classes[status] || 'bg-secondary bg-opacity-10 text-secondary'
    }
    
    const formatDate = (dateString) => {
      const date = new Date(dateString);
      const time = new Date(date.getTime());
      return time.toLocaleDateString('es-ES', { year: 'numeric', month: 'long', day: 'numeric', hour: '2-digit', minute: '2-digit' });
    };
    
    const applyFilters = () => {
      currentPage.value = 1
    }
    
    const clearFilters = () => {
      searchTerm.value = ''
      statusFilter.value = ''
      methodFilter.value = ''
      dateFilter.value = ''
      currentPage.value = 1
    }
    
    const goToPage = (page) => {
      if (page >= 1 && page <= totalPages.value) {
        currentPage.value = page
      }
    }
    
    const viewPaymentDetails = async (payment) => {
      selectedPayment.value = payment

      try {
        if (!paymentDetailsModal) {
          const el = document.getElementById('paymentDetailsModal')
          if (el) paymentDetailsModal = new bootstrap.Modal(el)
        }
      } catch (e) {
        console.debug('Error initializing modal instance', e)
      }

      (async () => {
        try {
          const order = await api.getOrderById(payment.order_id)
          if (order) selectedPayment.value.orderDetails = order
        } catch (err) {
          console.debug('No order details for payment', payment.order_id)
        }
      })()

      try {
        paymentDetailsModal?.show()
      } catch (e) {
        console.error('Unable to show payment modal', e)
      }
    }
    
    const approvePayment = async (payment) => {
      if (confirm(`¿Aprobar pago ${payment.id} por $${payment.amount}?`)) {
        try {
          await api.updatePaymentStatus(payment.id, 'completed')
          await fetchPayments()
          paymentDetailsModal?.hide()
          alert('Pago aprobado exitosamente')
        } catch (error) {
          console.error('Error approving payment:', error)
          alert('Error al aprobar el pago')
        }
      }
    }
    
    const refundPayment = async (payment) => {
      if (confirm(`¿Reembolsar pago ${payment.id} por $${payment.amount}?`)) {
        try {
          await api.refundPayment(payment.id)
          await fetchPayments()
          paymentDetailsModal?.hide()
          alert('Reembolso procesado exitosamente')
        } catch (error) {
          console.error('Error refunding payment:', error)
          alert('Error al procesar el reembolso')
        }
      }
    }
    
    const retryPayment = async (payment) => {
      try {
        await api.retryPayment(payment.id)
        await fetchPayments()
        paymentDetailsModal?.hide()
        alert('Reintento de pago iniciado')
      } catch (error) {
        console.error('Error retrying payment:', error)
        alert('Error al reintentar el pago')
      }
    }
    
    const refreshPayments = () => {
      fetchPayments()
    }
    
    const exportPayments = () => {
      const csvContent = 'data:text/csv;charset=utf-8,' + 
        'ID,Usuario,Orden,Monto,Método,Estado,Fecha\n' +
        filteredPayments.value.map(p => 
          `${p.id},${getUserName(p.user_id)},${p.order_code || p.order_id},${p.amount},${p.method},${p.status},${p.created_at}`
        ).join('\n')
      
      const encodedUri = encodeURI(csvContent)
      const link = document.createElement('a')
      link.setAttribute('href', encodedUri)
      link.setAttribute('download', `payments_${new Date().toISOString().split('T')[0]}.csv`)
      document.body.appendChild(link)
      link.click()
      document.body.removeChild(link)
    }
    
    onMounted(() => {
      if (!authStore.isAdmin) {
        alert('Acceso denegado. Esta página es solo para administradores.')
        return
      }
      
      fetchPayments()
      
      const modalElement = document.getElementById('paymentDetailsModal')
      if (modalElement) {
        paymentDetailsModal = new bootstrap.Modal(modalElement)
      }
    })
    
    onUnmounted(() => {
      paymentDetailsModal?.dispose()
    })
    
    return {
      payments, users, isLoading, selectedPayment,
      searchTerm, statusFilter, methodFilter, dateFilter, currentPage, itemsPerPage,
      totalRevenue, totalPayments, pendingPayments, failedPayments,
      filteredPayments, paginatedPayments, totalPages,
      getUserName, getPaymentMethodName, getPaymentMethodIcon, getStatusText, getStatusBadgeClass,
      formatDate, applyFilters, clearFilters, goToPage, viewPaymentDetails,
      approvePayment, refundPayment, retryPayment, refreshPayments, exportPayments
    }
  }
}
</script>

<style scoped>
.payment-management-page {
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

.cursor-pointer {
  cursor: pointer;
}

.avatar-sm {
  width: 30px;
  height: 30px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.8rem;
}

.btn-group .btn {
  border-radius: 50%;
  width: 32px;
  height: 32px;
  padding: 0;
  display: flex;
  align-items: center;
  justify-content: center;
}

pre {
  font-size: 0.8rem;
  max-height: 200px;
  overflow-y: auto;
}

.payment-management-page .table.table-hover.align-middle tbody tr td,
.payment-management-page .table.table-hover.align-middle tbody tr th {
  padding-top: 1rem;
  padding-bottom: 1rem;
}
</style>
