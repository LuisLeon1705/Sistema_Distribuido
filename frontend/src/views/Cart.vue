<template>
  <div class="cart-page">
    <div class="container py-5">
      
      <div class="d-flex justify-content-between align-items-center mb-5">
        <div>
          <h2 class="fw-bold text-dark mb-1">Mi Carrito</h2>
          <p class="text-muted small mb-0">Revisa tus artículos antes de pagar</p>
        </div>
        <router-link to="/products" class="btn btn-outline-dark btn-sm rounded-pill px-4">
          <i class="fas fa-arrow-left me-2"></i>Seguir comprando
        </router-link>
      </div>

      <div v-if="cartItems.length === 0" class="text-center py-5 empty-cart-container">
        <div class="mb-4">
          <div class="bg-light d-inline-block p-4 rounded-circle">
            <i class="fas fa-shopping-basket fa-4x text-muted opacity-50"></i>
          </div>
        </div>
        <h3 class="fw-bold text-dark">Tu carrito está vacío</h3>
        <p class="text-muted mb-4">¡Explora nuestro catálogo y encuentra lo que buscas!</p>
        <router-link to="/products" class="btn btn-dark rounded-pill px-5 py-2 fw-bold shadow-sm">
          Ver Productos
        </router-link>
      </div>

      <div v-else class="row g-4">
        
        <div class="col-lg-8">
          <div class="card border-0 shadow-sm rounded-4 overflow-hidden mb-4">
            <div class="card-header bg-white border-bottom py-3 px-4">
              <h6 class="mb-0 fw-bold text-dark">Artículos ({{ totalItems }})</h6>
            </div>
            
            <div class="card-body p-0">
              <div v-for="item in cartItems" :key="item.product_id" class="cart-item p-4 border-bottom last-no-border">
                <div class="row align-items-center gy-3">
                  
                  <div class="col-4 col-md-2">
                    <img 
                      :src="item.image || '/placeholder-product.jpg'" 
                      :alt="item.name"
                      class="img-fluid rounded-3 border bg-light"
                      style="width: 80px; height: 80px; object-fit: cover;"
                    >
                  </div>
                  
                  <div class="col-8 col-md-4">
                    <h6 class="fw-bold text-dark mb-1 text-truncate">{{ item.name }}</h6>
                    <small class="text-muted d-block">Unitario: ${{ item.price.toFixed(2) }}</small>
                  </div>
                  
                  <div class="col-6 col-md-3">
                    <div class="input-group input-group-sm bg-light rounded-pill p-1" style="width: 120px;">
                      <button 
                        class="btn btn-link text-dark text-decoration-none border-0 px-2"
                        @click="updateQuantity(item.product_id, item.quantity - 1)"
                        :disabled="item.quantity <= 1"
                      >
                        <i class="fas fa-minus small"></i>
                      </button>
                      <input 
                        type="text" 
                        class="form-control border-0 bg-transparent text-center fw-bold p-0" 
                        :value="item.quantity"
                        readonly
                      >
                      <button 
                        class="btn btn-link text-dark text-decoration-none border-0 px-2"
                        @click="updateQuantity(item.product_id, item.quantity + 1)"
                      >
                        <i class="fas fa-plus small"></i>
                      </button>
                    </div>
                  </div>
                  
                  <div class="col-6 col-md-3 text-end d-flex flex-column align-items-end justify-content-center">
                    <span class="fw-bold text-dark mb-2">${{ (item.price * item.quantity).toFixed(2) }}</span>
                    <button 
                      @click="removeFromCart(item.product_id)" 
                      class="btn btn-link text-danger p-0 text-decoration-none small opacity-75 hover-opacity-100"
                    >
                      <i class="fas fa-trash-alt me-1"></i> Eliminar
                    </button>
                  </div>

                </div>
              </div>
            </div>
          </div>

          <div class="text-end">
            <button @click="clearCart" class="btn btn-link text-muted text-decoration-none small hover-text-danger">
              <i class="fas fa-trash me-1"></i> Vaciar carrito por completo
            </button>
          </div>
        </div>

        <div class="col-lg-4">
          <div class="card border-0 shadow-sm rounded-4 p-4 sticky-top" style="top: 100px;">
            <h5 class="fw-bold text-dark mb-4">Resumen</h5>
            
            <div class="d-flex justify-content-between mb-2">
              <span class="text-muted">Subtotal</span>
              <span class="fw-bold text-dark">${{ totalPrice.toFixed(2) }}</span>
            </div>
            
            <div class="d-flex justify-content-between mb-4">
              <span class="text-muted">Envío</span>
              <span class="text-success fw-bold">Gratis</span>
            </div>
            
            <div class="border-top border-dashed my-3"></div>
            
            <div class="d-flex justify-content-between mb-4 align-items-center">
              <span class="fw-bold h5 mb-0">Total</span>
              <span class="fw-bold h4 text-primary mb-0">${{ totalPrice.toFixed(2) }}</span>
            </div>
            
            <button 
              class="btn btn-dark w-100 py-3 rounded-pill fw-bold shadow-lg mb-3"
              @click="checkout"
              :disabled="cartStore.isLoading"
            >
              <span v-if="cartStore.isLoading" class="spinner-border spinner-border-sm me-2"></span>
              {{ cartStore.isLoading ? 'Procesando...' : 'Finalizar Compra' }}
            </button>

            <div v-if="cartStore.error" class="alert alert-danger mt-3 py-2 small text-center border-0 bg-danger bg-opacity-10 text-danger">
              {{ cartStore.error }}
            </div>
            
            <div v-if="checkoutSuccess" class="alert alert-success mt-3 py-2 small text-center border-0 bg-success bg-opacity-10 text-success">
              ¡Compra exitosa! Redirigiendo...
            </div>

          </div>
        </div>

      </div>
    </div>
  </div>

  <div class="modal fade" id="paymentModal" tabindex="-1">
    <div class="modal-dialog modal-dialog-centered">
      <div class="modal-content border-0 shadow-lg rounded-4 overflow-hidden">
        <div class="modal-header border-bottom bg-light px-4 py-3">
          <h5 class="modal-title fw-bold text-dark">
            <i class="fas fa-credit-card me-2 text-primary"></i>
            Información de Pago
          </h5>
          <button type="button" class="btn-close" data-bs-dismiss="modal"></button>
        </div>

        <div class="modal-body p-4">
          <form @submit.prevent="processPayment">
            <div class="mb-4">
              <label class="small fw-bold text-muted mb-2">Método de Pago</label>
              <div class="row">
                <div class="col-4 d-flex">
                  <div class="form-check w-100">
                    <input class="form-check-input" type="radio" name="paymentMethod" id="creditCard" value="credit" v-model="paymentMethod" required>
                    <label class="form-check-label d-flex flex-column align-items-center justify-content-center p-2 border rounded-3 cursor-pointer w-100 h-100 text-center" for="creditCard">
                      <i class="fas fa-credit-card fa-2x text-primary mb-2"></i>
                      <small class="fw-bold">Tarjeta</small>
                    </label>
                  </div>
                </div>

                <div class="col-4 d-flex">
                  <div class="form-check w-100">
                    <input class="form-check-input" type="radio" name="paymentMethod" id="debitCard" value="debit" v-model="paymentMethod" required>
                    <label class="form-check-label d-flex flex-column align-items-center justify-content-center p-2 border rounded-3 cursor-pointer w-100 h-100 text-center" for="debitCard">
                      <i class="fas fa-credit-card fa-2x text-success mb-2"></i>
                      <small class="fw-bold">Débito</small>
                    </label>
                  </div>
                </div>

                <div class="col-4 d-flex">
                  <div class="form-check w-100">
                    <input class="form-check-input" type="radio" name="paymentMethod" id="paypal" value="paypal" v-model="paymentMethod" required>
                    <label class="form-check-label d-flex flex-column align-items-center justify-content-center p-2 border rounded-3 cursor-pointer w-100 h-100 text-center" for="paypal">
                      <i class="fab fa-paypal fa-2x text-info mb-2"></i>
                      <small class="fw-bold">PayPal</small>
                    </label>
                  </div>
                </div>
              </div>
            </div>

            <div v-if="paymentMethod === 'credit' || paymentMethod === 'debit'" class="mb-4">
              <label class="small fw-bold text-muted mb-2">Número de Tarjeta</label>
              <div class="input-group mb-3">
                <span class="input-group-text bg-light border-0">
                  <i class="fas fa-credit-card text-muted"></i>
                </span>
                <input 
                  type="text" 
                  class="form-control border-0 bg-light" 
                  placeholder="1234 5678 9012 3456"
                  v-model="cardNumber"
                  maxlength="19"
                  @input="formatCardNumber"
                  required
                >
              </div>

              <div class="row g-2">
                <div class="col-6">
                  <label class="small fw-bold text-muted mb-2">Fecha Vencimiento</label>
                  <input 
                    type="text" 
                    class="form-control border-0 bg-light" 
                    placeholder="MM/AA"
                    v-model="expiryDate"
                    maxlength="5"
                    @input="formatExpiryDate"
                    required
                  >
                </div>
                <div class="col-6">
                  <label class="small fw-bold text-muted mb-2">CVV</label>
                  <input 
                    type="text" 
                    class="form-control border-0 bg-light" 
                    placeholder="123"
                    v-model="cvv"
                    maxlength="4"
                    required
                  >
                </div>
              </div>
            </div>

            <div v-if="paymentMethod === 'paypal'" class="mb-4">
              <label class="small fw-bold text-muted mb-2">Email de PayPal</label>
              <div class="input-group">
                <span class="input-group-text bg-light border-0">
                  <i class="fas fa-envelope text-muted"></i>
                </span>
                <input 
                  type="email" 
                  class="form-control border-0 bg-light" 
                  placeholder="tu@email.com"
                  v-model="paypalEmail"
                  required
                >
              </div>
            </div>

            <div class="mb-4">
              <label class="small fw-bold text-muted mb-2">Dirección de Envío</label>
              <div class="input-group mb-2">
                <span class="input-group-text bg-light border-0">
                  <i class="fas fa-home text-muted"></i>
                </span>
                <input 
                  type="text" 
                  class="form-control border-0 bg-light" 
                  placeholder="Calle y número"
                  v-model="shippingAddress"
                  required
                >
              </div>
              <div class="row g-2">
                <div class="col-6">
                  <input 
                    type="text" 
                    class="form-control border-0 bg-light" 
                    placeholder="Ciudad"
                    v-model="shippingCity"
                    required
                  >
                </div>
                <div class="col-6">
                  <input 
                    type="text" 
                    class="form-control border-0 bg-light" 
                    placeholder="Código Postal"
                    v-model="shippingPostal"
                    required
                  >
                </div>
              </div>
            </div>

            <div class="bg-light rounded-3 p-3 mb-3">
              <div class="d-flex justify-content-between mb-2">
                <span class="text-muted small">Subtotal</span>
                <span class="fw-bold">${{ totalPrice.toFixed(2) }}</span>
              </div>
              <div class="d-flex justify-content-between mb-2">
                <span class="text-muted small">Envío</span>
                <span class="text-success fw-bold">Gratis</span>
              </div>
              <div class="border-top pt-2 d-flex justify-content-between">
                <span class="fw-bold">Total</span>
                <span class="fw-bold text-primary h5 mb-0">${{ totalPrice.toFixed(2) }}</span>
              </div>
            </div>

            <div class="d-flex gap-2">
              <button type="button" class="btn btn-outline-dark flex-fill rounded-pill" data-bs-dismiss="modal">
                Cancelar
              </button>
              <button type="submit" class="btn btn-primary flex-fill rounded-pill" :disabled="isProcessingPayment">
                <span v-if="isProcessingPayment" class="spinner-border spinner-border-sm me-2"></span>
                {{ isProcessingPayment ? 'Procesando...' : `Pagar $${totalPrice.toFixed(2)}` }}
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { useCartStore } from '../stores/cart'
import { useAuthStore } from '../stores/auth'
import api from '../services/api'
import * as bootstrap from 'bootstrap'

export default {
  name: 'Cart',
  setup() {
    const router = useRouter()
    const cartStore = useCartStore()
    const authStore = useAuthStore()
    const checkoutSuccess = ref(false)
    let paymentModalInstance = null
    
    const paymentMethod = ref('')
    const cardNumber = ref('')
    const expiryDate = ref('')
    const cvv = ref('')
    const paypalEmail = ref('')
    const shippingAddress = ref('')
    const shippingCity = ref('')
    const shippingPostal = ref('')
    const isProcessingPayment = ref(false)
    
    const cartItems = computed(() => cartStore.cartItems)
    const totalItems = computed(() => cartStore.totalItems)
    const totalPrice = computed(() => cartStore.totalPrice)
    
    const updateQuantity = (productId, newQuantity) => {
      if (newQuantity < 1) return
      cartStore.updateQuantity(productId, newQuantity)
    }
    
    const removeFromCart = (productId) => {
      cartStore.removeFromCart(productId)
    }
    
    const clearCart = () => {
      if (confirm('¿Vaciar todo el carrito?')) {
        cartStore.clearCart()
      }
    }
    
    const checkout = () => {
      paymentModalInstance?.show()
    }
    
    const formatCardNumber = (event) => {
      let value = event.target.value.replace(/\s/g, '')
      let formattedValue = value.match(/.{1,4}/g)?.join(' ') || value
      cardNumber.value = formattedValue
    }
    
    const formatExpiryDate = (event) => {
      let value = event.target.value.replace(/\D/g, '')
      if (value.length >= 2) {
        value = value.slice(0, 2) + '/' + value.slice(2, 4)
      }
      expiryDate.value = value
    }
    
    const processPayment = async () => {
      isProcessingPayment.value = true
      
      try {
        if (!authStore.isAuthenticated) {
          throw new Error('Debes estar autenticado para procesar el pago')
        }
        
        if (!authStore.user?.id) {
          throw new Error('Usuario no encontrado')
        }
        
        if (!paymentMethod.value) {
          throw new Error('Debes seleccionar un método de pago')
        }
        
        console.log('Creando orden...')
        const order = await cartStore.checkout({
          shippingAddress: shippingAddress.value,
          shippingCity: shippingCity.value,
          shippingPostal: shippingPostal.value
        })
        console.log('Orden creada:', order)
        
        const paymentData = {
          order_id: order.id,
          user_id: authStore.user.id,
          amount: parseFloat(order.total) || parseFloat(totalPrice.value) || 0,
          method: paymentMethod.value,
          status: 'pending',
          metadata: {
            shipping_address: shippingAddress.value,
            shipping_city: shippingCity.value,
            shipping_postal: shippingPostal.value,
            payment_details: paymentMethod.value === 'paypal' ? { email: paypalEmail.value } : {
              card_number: cardNumber.value.replace(/\s/g, ''),
              expiry_date: expiryDate.value,
              cvv: cvv.value
            }
          }
        }
        
        console.log('Datos de pago:', paymentData)
        
        const payment = await api.processPayment(paymentData)
        console.log('Pago procesado:', payment)
        
        paymentModalInstance?.hide()
        
        cartStore.setPaymentId(payment.id)
        
        checkoutSuccess.value = true
        
        resetPaymentForm()
        
        setTimeout(() => { 
          router.push('/orders') 
        }, 2000)
        
      } catch (error) {
        console.error('Error en el pago:', error)
        console.error('Detalles del error:', error.response?.data || error.message)
        alert(`Error al procesar el pago: ${error.message || 'Error desconocido'}`)
      } finally {
        isProcessingPayment.value = false
      }
    }
    
    const resetPaymentForm = () => {
      paymentMethod.value = ''
      cardNumber.value = ''
      expiryDate.value = ''
      cvv.value = ''
      paypalEmail.value = ''
      shippingAddress.value = ''
      shippingCity.value = ''
      shippingPostal.value = ''
    }
    
    onMounted(() => {
      const modalElement = document.getElementById('paymentModal')
      if (modalElement) {
        paymentModalInstance = new bootstrap.Modal(modalElement)
      }
    })
    
    onUnmounted(() => { 
      cartStore.clearError()
      paymentModalInstance?.dispose()
    })
    
    return {
      cartStore, cartItems, totalItems, totalPrice, checkoutSuccess,
      updateQuantity, removeFromCart, clearCart, checkout,
      paymentMethod, cardNumber, expiryDate, cvv, paypalEmail,
      shippingAddress, shippingCity, shippingPostal, isProcessingPayment,
      formatCardNumber, formatExpiryDate, processPayment
    }
  }
}
</script>

<style scoped>
.cart-page {
  background-color: #ffffff;
  min-height: 100vh;
}

.cart-item {
  transition: background-color 0.2s;
}

.cart-item:hover {
  background-color: #f8fafc;
}

.last-no-border:last-child {
  border-bottom: none !important;
}

.border-dashed {
  border-top: 1px dashed #e2e8f0;
}

.hover-opacity-100:hover {
  opacity: 1 !important;
}

.hover-text-danger:hover {
  color: #dc3545 !important;
}

.sticky-top {
  z-index: 100;
}

.form-check-input:checked + .form-check-label {
  background-color: #f8f9fa;
  border-color: #0d6efd;
  color: #0d6efd;
}

.form-check-label {
  transition: all 0.18s cubic-bezier(.2,.9,.2,1);
  cursor: pointer;
  background: #fff;
  border: 1px solid rgba(15,23,42,0.06);
  box-shadow: none;
  position: relative;
}

.form-check-label:hover {
  background-color: #fbfdff;
  transform: translateY(-3px);
  box-shadow: 0 8px 20px rgba(2,6,23,0.06);
}

.form-check-input {
  display: none;
}

.form-check-input:checked + .form-check-label {
  background: linear-gradient(180deg, rgba(13,110,253,0.06), rgba(13,110,253,0.02));
  border-color: #0d6efd;
  box-shadow: 0 10px 30px rgba(13,110,253,0.12);
  transform: translateY(-4px);
}

.form-check-input:checked + .form-check-label i {
  color: #0d6efd !important;
}

.form-check-input:checked + .form-check-label::after {
  content: '✓';
  position: absolute;
  top: 8px;
  right: 10px;
  background: #fff;
  color: #0d6efd;
  border-radius: 50%;
  width: 22px;
  height: 22px;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  font-size: 12px;
  box-shadow: 0 4px 10px rgba(2,6,23,0.08);
}

.form-check-label.w-100.h-100 {
  min-height: 84px;
}

.payment-method-row {
  margin-left: 0;
  margin-right: 0;
}

.payment-method-row > .col-4 {
  padding-left: 0;
  padding-right: 0.5rem;
}

.payment-method-row > .col-4:first-child {
  padding-left: 0;
}

.cursor-pointer {
  cursor: pointer;
}
</style>