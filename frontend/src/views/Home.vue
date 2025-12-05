<template>
  <div class="home">
    <!-- Hero Section -->
    <div class="hero-section bg-primary text-white py-5 mb-4">
      <div class="container text-center">
        <h1 class="display-4 fw-bold">Bienvenido al Sistema Distribuido</h1>
        <p class="lead">Tu tienda online con arquitectura de microservicios</p>
        <div class="mt-4">
          <router-link to="/products" class="btn btn-light btn-lg me-3">
            Ver Productos
          </router-link>
          <router-link v-if="!isAuthenticated" to="/register" class="btn btn-outline-light btn-lg">
            Registrarse
          </router-link>
        </div>
      </div>
    </div>

    <div class="container">
      <!-- Features Section -->
      <div class="row mb-5">
        <div class="col-12 text-center mb-4">
          <h2>Características del Sistema</h2>
          <p class="text-muted">Sistema basado en microservicios para escalabilidad y mantenibilidad</p>
        </div>
        
        <div class="col-md-4 text-center mb-4">
          <div class="feature-card p-4">
            <div class="feature-icon mb-3">
              <i class="fas fa-shield-alt fa-3x text-primary"></i>
            </div>
            <h4>Autenticación Segura</h4>
            <p class="text-muted">
              Sistema de autenticación con JWT, gestión de usuarios y roles administrativos
            </p>
          </div>
        </div>
        
        <div class="col-md-4 text-center mb-4">
          <div class="feature-card p-4">
            <div class="feature-icon mb-3">
              <i class="fas fa-boxes fa-3x text-success"></i>
            </div>
            <h4>Gestión de Inventario</h4>
            <p class="text-muted">
              Control completo del inventario con seguimiento de órdenes y stock
            </p>
          </div>
        </div>
        
        <div class="col-md-4 text-center mb-4">
          <div class="feature-card p-4">
            <div class="feature-icon mb-3">
              <i class="fas fa-shopping-cart fa-3x text-warning"></i>
            </div>
            <h4>Gestión de Productos</h4>
            <p class="text-muted">
              Catálogo completo de productos con categorías, precios y descripciones
            </p>
          </div>
        </div>
      </div>

      <!-- Recent Products Section -->
      <template v-if="!isAdmin">
        <div class="row" v-if="recentProducts.length > 0">
          <div class="col-12 text-center mb-4">
            <h2>Productos Destacados</h2>
          </div>
          
          <div class="col-md-4 mb-4" v-for="product in recentProducts" :key="product.id">
            <div class="card h-100 shadow-sm">
              <div class="card-img-container">
                <img 
                  :src="product.imagen || '/placeholder-product.jpg'" 
                  class="card-img-top" 
                  :alt="product.nombre"
                  style="height: 200px; object-fit: cover;"
                >
              </div>
              <div class="card-body d-flex flex-column">
                <h5 class="card-title">{{ product.nombre }}</h5>
                <p class="card-text flex-grow-1">{{ product.descripcion }}</p>
                <div class="d-flex justify-content-between align-items-center mt-auto">
                  <span class="h5 text-primary mb-0">${{ parseFloat(product.precio).toFixed(2) }}</span>
                  <!--
                  <button 
                    v-if="isAuthenticated" 
                    class="btn btn-primary btn-sm"
                    @click="addToCart(product)"
                  >
                    Agregar
                  </button>
                -->
                </div>
              </div>
            </div>
          </div>
          
          <div class="col-12 text-center mt-3">
            <router-link to="/products" class="btn btn-outline-primary">
              Ver Todos los Productos
            </router-link>
          </div>
        </div>

        <!-- Stats Section (for authenticated users) -->
        <div v-if="isAuthenticated" class="row mt-5 mb-4">
          <div class="col-12 text-center mb-4">
            <h3>Mi Dashboard</h3>
          </div>
          
          <div class="col-md-6 text-center mb-3">
            <div class="dashboard-card p-4 bg-light rounded">
              <h4 class="text-primary">{{ cartItemsCount }}</h4>
              <p class="mb-0">Artículos en Carrito</p>
            </div>
          </div>
          
          <div class="col-md-6 text-center mb-3">
            <div class="dashboard-card p-4 bg-light rounded">
              <h4 class="text-success">{{ userOrdersCount }}</h4>
              <p class="mb-0">Órdenes Realizadas</p>
            </div>
          </div>
        </div>
      </template>
    </div>
  </div>
</template>

<script>
import { ref, computed, onMounted } from 'vue'
import { useAuthStore } from '../stores/auth'
import { useCartStore } from '../stores/cart'
import api from '../services/api'

export default {
  name: 'Home',
  setup() {
    const authStore = useAuthStore()
    const cartStore = useCartStore()
    
    const recentProducts = ref([])
    const userOrdersCount = ref(0)
    
    const isAuthenticated = computed(() => authStore.isAuthenticated)
    const isAdmin = computed(() => authStore.isAdmin)
    const cartItemsCount = computed(() => cartStore.totalItems)
    
    const fetchRecentProducts = async () => {
      try {
        const products = await api.getActiveProducts()
        recentProducts.value = products.slice(0, 3) // Show only first 3 products
      } catch (error) {
        console.error('Error fetching products:', error)
      }
    }
    
    const fetchUserStats = async () => {
      if (!isAuthenticated.value) return
      
      try {
        const orders = await api.getOrders()
        userOrdersCount.value = orders.length
      } catch (error) {
        console.error('Error fetching user stats:', error)
      }
    }
    
    const addToCart = (product) => {
      cartStore.addToCart(product)
    }
    
    onMounted(async () => {
      await fetchRecentProducts()
      await fetchUserStats()
    })
    
    return {
      isAuthenticated,
      cartItemsCount,
      userOrdersCount,
      recentProducts,
      addToCart
    }
  }
}
</script>

<style scoped>
.hero-section {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.feature-card {
  transition: transform 0.3s ease;
  border-radius: 10px;
  background: #f8f9fa;
}

.feature-card:hover {
  transform: translateY(-5px);
}

.dashboard-card {
  transition: all 0.3s ease;
}

.dashboard-card:hover {
  transform: scale(1.05);
  box-shadow: 0 4px 15px rgba(0,0,0,0.1);
}

.card {
  transition: transform 0.3s ease;
}

.card:hover {
  transform: translateY(-2px);
}

.card-img-container {
  overflow: hidden;
  border-radius: 0.375rem 0.375rem 0 0;
}

.card-img-top {
  transition: transform 0.3s ease;
}

.card:hover .card-img-top {
  transform: scale(1.1);
}
</style>
