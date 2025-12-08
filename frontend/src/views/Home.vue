<template>
  <div class="home">
    <!-- Hero Section para Staff (Admin/Inventory) -->
    <template v-if="isStaff">
      <div class="hero-section bg-dark text-white py-5 mb-4">
        <div class="container text-center">
          <h1 class="display-4 fw-bold">Panel de Administración</h1>
          <p class="lead">Sistema de gestión para {{ getRoleDisplayName }}</p>
          <div class="mt-4">
            <router-link to="/admin/products" class="btn btn-primary btn-lg me-3">
              <i class="fas fa-box me-2"></i>Gestión de Productos
            </router-link>
            <router-link to="/admin/orders" class="btn btn-outline-light btn-lg">
              <i class="fas fa-clipboard-list me-2"></i>Gestión de Órdenes
            </router-link>
          </div>
        </div>
      </div>

      <div class="container">
        <!-- Dashboard para Staff -->
        <div class="row mb-5">
          <div class="col-12 text-center mb-4">
            <h2>Acceso Rápido</h2>
            <p class="text-muted">Herramientas de gestión del sistema</p>
          </div>
          
          <div class="col-md-4 text-center mb-4">
            <router-link to="/admin/products" class="text-decoration-none">
              <div class="feature-card p-4 bg-primary text-white">
                <div class="feature-icon mb-3">
                  <i class="fas fa-box fa-3x"></i>
                </div>
                <h4>Productos</h4>
                <p>Gestionar catálogo de productos</p>
              </div>
            </router-link>
          </div>
          
          <div class="col-md-4 text-center mb-4">
            <router-link to="/admin/orders" class="text-decoration-none">
              <div class="feature-card p-4 bg-success text-white">
                <div class="feature-icon mb-3">
                  <i class="fas fa-clipboard-list fa-3x"></i>
                </div>
                <h4>Órdenes</h4>
                <p>Gestionar pedidos del sistema</p>
              </div>
            </router-link>
          </div>
          
          <div class="col-md-4 text-center mb-4" v-if="isAdmin">
            <router-link to="/admin/users" class="text-decoration-none">
              <div class="feature-card p-4 bg-warning text-white">
                <div class="feature-icon mb-3">
                  <i class="fas fa-users fa-3x"></i>
                </div>
                <h4>Usuarios</h4>
                <p>Administrar cuentas de usuario</p>
              </div>
            </router-link>
          </div>
          
          <div class="col-md-4 text-center mb-4" v-if="!isAdmin">
            <router-link to="/profile" class="text-decoration-none">
              <div class="feature-card p-4 bg-info text-white">
                <div class="feature-icon mb-3">
                  <i class="fas fa-user fa-3x"></i>
                </div>
                <h4>Mi Perfil</h4>
                <p>Ver y editar mi información</p>
              </div>
            </router-link>
          </div>
        </div>
      </div>
    </template>

    <!-- Hero Section para Customers -->
    <template v-else>
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
                  <button 
                    v-if="isAuthenticated && canUseCart" 
                    class="btn btn-primary btn-sm"
                    @click="addToCart(product)"
                  >
                    Agregar
                  </button>
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

        <!-- Stats Section (only for authenticated customers) -->
        <div v-if="isAuthenticated && canUseCart" class="row mt-5 mb-4">
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
      </div>
    </template>
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
    const isStaff = computed(() => authStore.isStaff)
    const canUseCart = computed(() => authStore.canUseCart)
    const cartItemsCount = computed(() => cartStore.totalItems)

    const getRoleDisplayName = computed(() => {
      const role = authStore.userRole
      switch(role) {
        case 'admin':
          return 'Administrador'
        case 'inventory':
          return 'Gestor de Inventario'
        case 'customer':
        default:
          return 'Cliente'
      }
    })
    
    const fetchRecentProducts = async () => {
      // Solo cargar productos si no es staff
      if (authStore.isStaff) return
      
      try {
        const products = await api.getActiveProducts()
        recentProducts.value = products.slice(0, 3) // Show only first 3 products
      } catch (error) {
        console.error('Error fetching products:', error)
      }
    }
    
    const fetchUserStats = async () => {
      // Solo cargar stats si es customer autenticado
      if (!isAuthenticated.value || !authStore.canUseCart) return
      
      try {
        const orders = await api.getOrders()
        userOrdersCount.value = orders.length
      } catch (error) {
        console.error('Error fetching user stats:', error)
      }
    }
    
    const addToCart = (product) => {
      if (authStore.canUseCart) {
        cartStore.addToCart(product)
      }
    }
    
    onMounted(async () => {
      await fetchRecentProducts()
      await fetchUserStats()
    })
    
    return {
      isAuthenticated,
      isAdmin,
      isStaff,
      canUseCart,
      cartItemsCount,
      userOrdersCount,
      recentProducts,
      getRoleDisplayName,
      addToCart
    }
  }
}
</script>

<style scoped>
.hero-section {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
}

.hero-section.bg-dark {
  background: linear-gradient(135deg, #2c3e50 0%, #34495e 100%);
}

.feature-card {
  transition: transform 0.3s ease;
  border-radius: 10px;
  background: #f8f9fa;
}

.feature-card:hover {
  transform: translateY(-5px);
  box-shadow: 0 8px 25px rgba(0,0,0,0.15);
}

.feature-card.bg-primary:hover,
.feature-card.bg-success:hover,
.feature-card.bg-warning:hover,
.feature-card.bg-info:hover {
  opacity: 0.9;
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
