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
            
            <div class="text-center">
              <small class="text-muted d-block mb-1"><i class="fas fa-lock me-1"></i> Pago 100% Seguro</small>
            </div>

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
</template>

<script>
import { ref, computed, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { useCartStore } from '../stores/cart'

export default {
  name: 'Cart',
  setup() {
    const router = useRouter()
    const cartStore = useCartStore()
    const checkoutSuccess = ref(false)
    
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
    
    const checkout = async () => {
      try {
        await cartStore.checkout()
        checkoutSuccess.value = true
        setTimeout(() => { router.push('/orders') }, 2000)
      } catch (error) { console.error(error) }
    }
    
    onUnmounted(() => { cartStore.clearError() })
    
    return {
      cartStore, cartItems, totalItems, totalPrice, checkoutSuccess,
      updateQuantity, removeFromCart, clearCart, checkout
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
</style>