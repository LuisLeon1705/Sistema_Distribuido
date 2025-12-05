<template>
  <div class="cart">
    <div class="container">
      <div class="row">
        <div class="col-12">
          <div class="d-flex justify-content-between align-items-center mb-4">
            <h2>Mi Carrito</h2>
            <router-link to="/products" class="btn btn-outline-primary">
              <i class="fas fa-arrow-left me-2"></i>
              Continuar Comprando
            </router-link>
          </div>
          
          <!-- Empty Cart -->
          <div v-if="cartItems.length === 0" class="text-center py-5">
            <i class="fas fa-shopping-cart fa-5x text-muted mb-4"></i>
            <h3>Tu carrito está vacío</h3>
            <p class="text-muted mb-4">Agrega algunos productos para comenzar</p>
            <router-link to="/products" class="btn btn-primary btn-lg">
              Ver Productos
            </router-link>
          </div>
          
          <!-- Cart Items -->
          <div v-else>
            <div class="row">
              <div class="col-lg-8">
                <div class="card mb-4">
                  <div class="card-header">
                    <h5 class="mb-0">Productos ({{ totalItems }} {{ totalItems === 1 ? 'artículo' : 'artículos' }})</h5>
                  </div>
                  <div class="card-body p-0">
                    <div v-for="item in cartItems" :key="item.product_id" class="cart-item">
                      <div class="row align-items-center g-3 p-3 border-bottom">
                        <div class="col-md-2">
                          <img 
                            :src="item.image || '/placeholder-product.jpg'" 
                            :alt="item.name"
                            class="img-fluid rounded"
                            style="height: 80px; width: 80px; object-fit: cover;"
                          >
                        </div>
                        
                        <div class="col-md-4">
                          <h6 class="mb-1">{{ item.name }}</h6>
                          <small class="text-muted">Precio unitario: ${{ item.price.toFixed(2) }}</small>
                        </div>
                        
                        <div class="col-md-3">
                          <div class="quantity-controls d-flex align-items-center">
                            <button 
                              class="btn btn-outline-secondary btn-sm"
                              @click="updateQuantity(item.product_id, item.quantity - 1)"
                              :disabled="item.quantity <= 1"
                            >
                              <i class="fas fa-minus"></i>
                            </button>
                            <input 
                              type="number" 
                              v-model.number="item.quantity"
                              @change="updateQuantity(item.product_id, item.quantity)"
                              class="form-control quantity-input mx-2 text-center"
                              min="1"
                              style="width: 70px;"
                            >
                            <button 
                              class="btn btn-outline-secondary btn-sm"
                              @click="updateQuantity(item.product_id, item.quantity + 1)"
                            >
                              <i class="fas fa-plus"></i>
                            </button>
                          </div>
                        </div>
                        
                        <div class="col-md-2 text-end">
                          <div class="fw-bold text-primary">
                            ${{ (item.price * item.quantity).toFixed(2) }}
                          </div>
                        </div>
                        
                        <div class="col-md-1 text-end">
                          <button 
                            class="btn btn-outline-danger btn-sm"
                            @click="removeFromCart(item.product_id)"
                            title="Eliminar del carrito"
                          >
                            <i class="fas fa-trash"></i>
                          </button>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
                
                <!-- Clear Cart Button -->
                <div class="text-end mb-4">
                  <button 
                    class="btn btn-outline-danger"
                    @click="clearCart"
                  >
                    <i class="fas fa-trash me-2"></i>
                    Vaciar Carrito
                  </button>
                </div>
              </div>
              
              <!-- Order Summary -->
              <div class="col-lg-4">
                <div class="card sticky-top" style="top: 20px;">
                  <div class="card-header">
                    <h5 class="mb-0">Resumen de la Orden</h5>
                  </div>
                  <div class="card-body">
                    <div class="d-flex justify-content-between mb-3">
                      <span>Subtotal ({{ totalItems }} {{ totalItems === 1 ? 'artículo' : 'artículos' }}):</span>
                      <span>${{ totalPrice.toFixed(2) }}</span>
                    </div>
                    
                    <div class="d-flex justify-content-between mb-3">
                      <span>Envío:</span>
                      <span class="text-success">Gratis</span>
                    </div>
                    
                    <hr>
                    
                    <div class="d-flex justify-content-between mb-4">
                      <strong>Total:</strong>
                      <strong class="text-primary">${{ totalPrice.toFixed(2) }}</strong>
                    </div>
                    
                    <button 
                      class="btn btn-primary w-100 btn-lg"
                      @click="checkout"
                      :disabled="cartStore.isLoading || cartItems.length === 0"
                    >
                      <span v-if="cartStore.isLoading" class="spinner-border spinner-border-sm me-2"></span>
                      {{ cartStore.isLoading ? 'Procesando...' : 'Proceder al Pago' }}
                    </button>
                    
                    <div v-if="cartStore.error" class="alert alert-danger mt-3">
                      {{ cartStore.error }}
                    </div>
                    
                    <div v-if="checkoutSuccess" class="alert alert-success mt-3">
                      ¡Orden realizada exitosamente! Redirigiendo...
                    </div>
                  </div>
                </div>
              </div>
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
      if (confirm('¿Estás seguro de que deseas vaciar el carrito?')) {
        cartStore.clearCart()
      }
    }
    
    const checkout = async () => {
      if ( !confirm("Las unidades disponibles en el inventario pueden variar al momento de realizar el checkout. Por favor, asegúrate de que los productos en tu carrito estén disponibles antes de proceder."))
        {return}
        try {
        
        await cartStore.checkout()
        checkoutSuccess.value = true
        
        setTimeout(() => {
          router.push('/orders')
        }, 2000)
      } catch (error) {
        console.error('Checkout error:', error)
      }
    }
    
    onUnmounted(() => {
      cartStore.clearError()
    })
    
    return {
      cartStore,
      cartItems,
      totalItems,
      totalPrice,
      checkoutSuccess,
      updateQuantity,
      removeFromCart,
      clearCart,
      checkout
    }
  }
}
</script>

<style scoped>
.cart-item {
  transition: background-color 0.3s ease;
}

.cart-item:hover {
  background-color: #f8f9fa;
}

.quantity-controls {
  max-width: 150px;
}

.quantity-input {
  border-radius: 0;
  border-left: none;
  border-right: none;
}

.quantity-input:focus {
  box-shadow: none;
  border-color: #ced4da;
}

.btn-outline-secondary {
  border-color: #ced4da;
}

.sticky-top {
  position: sticky;
  top: 20px;
  z-index: 1020;
}

@media (max-width: 768px) {
  .quantity-controls {
    justify-content: center;
    margin: 10px 0;
  }
  
  .cart-item .row > div {
    text-align: center;
    margin-bottom: 10px;
  }
  
  .cart-item .row > div:last-child {
    margin-bottom: 0;
  }
}
</style>
