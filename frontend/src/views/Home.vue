<template>
  <div class="home-wrapper">
    <template v-if="isStaff">
      <div class="admin-hero py-5 text-white position-relative overflow-hidden">
        <div class="absolute-bg"></div>
        <div class="container position-relative z-1 text-center">
          <span class="badge bg-white text-dark bg-opacity-75 px-3 py-2 rounded-pill mb-3 fw-bold text-uppercase tracking-wider">
            Panel de Control
          </span>
          <h1 class="display-4 fw-800 mb-2">Bienvenido, {{ user?.username }}</h1>
          <p class="lead text-white-50 mb-4">Gestión centralizada para {{ getRoleDisplayName }}</p>
          
          <div class="d-flex justify-content-center gap-3">
            <router-link to="/admin/products" class="btn btn-light btn-lg rounded-pill px-4 shadow-lg fw-bold text-primary">
              <i class="fas fa-box me-2"></i>Inventario
            </router-link>
            <router-link to="/admin/orders" class="btn btn-outline-light btn-lg rounded-pill px-4 fw-bold backdrop-blur">
              <i class="fas fa-clipboard-list me-2"></i>Órdenes
            </router-link>
          </div>
        </div>
      </div>

      <div class="container mt-n5 position-relative z-2">
        <div class="row g-4 justify-content-center">
          <div class="col-md-4">
            <router-link to="/admin/products" class="text-decoration-none">
              <div class="admin-card h-100 p-4 bg-white rounded-4 shadow-sm border-0 d-flex flex-column align-items-center text-center">
                <div class="icon-circle bg-primary-subtle text-primary mb-3">
                  <i class="fas fa-box fa-2x"></i>
                </div>
                <h4 class="text-dark fw-bold">Productos</h4>
                <p class="text-muted small">Administrar catálogo, precios y stock disponible.</p>
                <div class="mt-auto text-primary fw-bold small">Gestionar <i class="fas fa-arrow-right ms-1"></i></div>
              </div>
            </router-link>
          </div>
          
          <div class="col-md-4">
            <router-link to="/admin/orders" class="text-decoration-none">
              <div class="admin-card h-100 p-4 bg-white rounded-4 shadow-sm border-0 d-flex flex-column align-items-center text-center">
                <div class="icon-circle bg-success-subtle text-success mb-3">
                  <i class="fas fa-clipboard-list fa-2x"></i>
                </div>
                <h4 class="text-dark fw-bold">Órdenes</h4>
                <p class="text-muted small">Revisar pedidos entrantes y estados de envío.</p>
                <div class="mt-auto text-success fw-bold small">Ver pedidos <i class="fas fa-arrow-right ms-1"></i></div>
              </div>
            </router-link>
          </div>
          
          <div class="col-md-4" v-if="isAdmin">
            <router-link to="/admin/users" class="text-decoration-none">
              <div class="admin-card h-100 p-4 bg-white rounded-4 shadow-sm border-0 d-flex flex-column align-items-center text-center">
                <div class="icon-circle bg-warning-subtle text-warning-emphasis mb-3">
                  <i class="fas fa-users fa-2x"></i>
                </div>
                <h4 class="text-dark fw-bold">Usuarios</h4>
                <p class="text-muted small">Gestión de roles, permisos y cuentas de acceso.</p>
                <div class="mt-auto text-warning-emphasis fw-bold small">Administrar <i class="fas fa-arrow-right ms-1"></i></div>
              </div>
            </router-link>
          </div>

           <div class="col-md-4" v-if="!isAdmin">
            <router-link to="/profile" class="text-decoration-none">
              <div class="admin-card h-100 p-4 bg-white rounded-4 shadow-sm border-0 d-flex flex-column align-items-center text-center">
                <div class="icon-circle bg-info-subtle text-info-emphasis mb-3">
                  <i class="fas fa-user-cog fa-2x"></i>
                </div>
                <h4 class="text-dark fw-bold">Mi Cuenta</h4>
                <p class="text-muted small">Configuración de perfil y seguridad.</p>
                <div class="mt-auto text-info-emphasis fw-bold small">Ver Perfil <i class="fas fa-arrow-right ms-1"></i></div>
              </div>
            </router-link>
          </div>
        </div>
      </div>
    </template>

    <template v-else>
      <div class="customer-hero py-5 mb-5">
        <div class="container py-5">
          <div class="row align-items-center">
            <div class="col-lg-7 text-center text-lg-start">
              <h1 class="display-3 fw-800 text-dark mb-3 tracking-tight">
                Tecnología distribuida <br>
                <span class="text-gradient">a tu alcance.</span>
              </h1>
              <p class="lead text-muted mb-4 pe-lg-5">
                Experimenta la velocidad de una arquitectura basada en microservicios. Compra artículos de forma segura, rápida y escalable.
              </p>
              <div class="d-flex gap-3 justify-content-center justify-content-lg-start">
                <router-link to="/products" class="btn btn-primary btn-lg rounded-pill px-5 shadow-primary-lg">
                  Explorar Tienda
                </router-link>
                <router-link v-if="!isAuthenticated" to="/register" class="btn btn-outline-dark btn-lg rounded-pill px-4">
                  Crear Cuenta
                </router-link>
              </div>
            </div>
            <div class="col-lg-5 d-none d-lg-block position-relative">
              <div class="hero-shape-1"></div>
              <div class="hero-shape-2"></div>
              
              <img 
                src="/box.png" 
                alt="Paquete de envío" 
                class="w-90 h-90 object-fit-cover"
                style="height: 400px; margin-left: 50px;"
              >
            </div>
          </div>
        </div>
      </div>

      <div class="container pb-5">
        <div class="row g-4 mb-5">
          <div class="col-md-4">
            <div class="feature-box p-4 rounded-4 bg-light h-100 border border-light">
              <i class="fas fa-shield-alt fa-2x text-primary mb-3"></i>
              <h5 class="fw-bold">100% Seguro</h5>
              <p class="text-muted small mb-0">Autenticación robusta vía JWT y protección de datos en cada transacción.</p>
            </div>
          </div>
          <div class="col-md-4">
            <div class="feature-box p-4 rounded-4 bg-light h-100 border border-light">
              <i class="fas fa-bolt fa-2x text-warning mb-3"></i>
              <h5 class="fw-bold">Alta Velocidad</h5>
              <p class="text-muted small mb-0">Inventario sincronizado en tiempo real gracias a nuestra arquitectura.</p>
            </div>
          </div>
          <div class="col-md-4">
            <div class="feature-box p-4 rounded-4 bg-light h-100 border border-light">
              <i class="fas fa-headset fa-2x text-success mb-3"></i>
              <h5 class="fw-bold">Soporte 24/7</h5>
              <p class="text-muted small mb-0">Gestión de órdenes transparente y seguimiento detallado.</p>
            </div>
          </div>
        </div>

        <div v-if="recentProducts.length > 0" class="mb-5">
          <div class="d-flex justify-content-between align-items-end mb-4">
            <div>
              <h2 class="fw-bold mb-1">Tendencias</h2>
              <p class="text-muted mb-0">Lo más reciente en nuestro catálogo</p>
            </div>
            <router-link to="/products" class="btn btn-link text-decoration-none fw-bold">
              Ver todo <i class="fas fa-arrow-right ms-1"></i>
            </router-link>
          </div>

          <div class="row g-4">
            <div class="col-md-4" v-for="product in recentProducts" :key="product.id">
              <div class="product-card card h-100 border-0 shadow-sm rounded-4 overflow-hidden">
                <div class="card-img-wrapper position-relative">
                  <img 
                    :src="product.imagen || '/placeholder-product.jpg'" 
                    class="card-img-top" 
                    :alt="product.nombre"
                  >
                  <span class="badge bg-dark position-absolute top-0 start-0 m-3 rounded-pill">Nuevo</span>
                </div>
                <div class="card-body p-4 d-flex flex-column">
                  <h5 class="card-title fw-bold mb-1">{{ product.nombre }}</h5>
                  <p class="card-text text-muted small flex-grow-1 line-clamp-2">{{ product.descripcion }}</p>
                  
                  <div class="d-flex justify-content-between align-items-center mt-3 pt-3 border-top">
                    <span class="h5 fw-800 text-dark mb-0">${{ parseFloat(product.precio).toFixed(2) }}</span>
                    <button 
                      v-if="isAuthenticated && canUseCart" 
                      class="btn btn-primary btn-sm rounded-pill px-3 fw-bold btn-hover-effect"
                      @click="addToCart(product)"
                    >
                      <i class="fas fa-plus me-1"></i> Agregar
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>

        <div v-if="isAuthenticated && canUseCart" class="user-dashboard-section p-4 p-md-5 rounded-5 bg-dark text-white position-relative overflow-hidden">
          <div class="position-relative z-1">
            <div class="text-center mb-5">
              <h3 class="fw-bold">Tu Actividad</h3>
              <p class="text-white-50">Resumen de tu cuenta</p>
            </div>
            
            <div class="row g-4 justify-content-center">
              <div class="col-md-5">
                <div class="stat-card p-4 rounded-4 bg-white bg-opacity-10 backdrop-blur text-center border border-white border-opacity-10">
                  <div class="display-4 fw-bold text-primary mb-2">{{ cartItemsCount }}</div>
                  <div class="text-white fw-medium">Artículos en Carrito</div>
                  <router-link to="/cart" class="btn btn-sm btn-outline-light rounded-pill mt-3 px-4">Ir al Carrito</router-link>
                </div>
              </div>
              
              <div class="col-md-5">
                <div class="stat-card p-4 rounded-4 bg-white bg-opacity-10 backdrop-blur text-center border border-white border-opacity-10">
                  <div class="display-4 fw-bold text-success mb-2">{{ userOrdersCount }}</div>
                  <div class="text-white fw-medium">Órdenes Completadas</div>
                  <router-link to="/orders" class="btn btn-sm btn-outline-light rounded-pill mt-3 px-4">Ver Historial</router-link>
                </div>
              </div>
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
        case 'admin': return 'Administrador'
        case 'inventory': return 'Gestor de Inventario'
        case 'customer': default: return 'Cliente'
      }
    })
    
    const fetchRecentProducts = async () => {
      if (authStore.isStaff) return
      try {
        const products = await api.getActiveProducts()
        recentProducts.value = products.slice(0, 3) 
      } catch (error) {
        console.error('Error fetching products:', error)
      }
    }
    
    const fetchUserStats = async () => {
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
      addToCart,
      user: computed(() => authStore.user)
    }
  }
}
</script>

<style scoped>
.fw-800 { 
  font-weight: 800; 
}

.tracking-tight { 
  letter-spacing: -1px; 
}

.tracking-wider { 
  letter-spacing: 1px; 
}

.backdrop-blur { 
  backdrop-filter: blur(10px); 
}

.admin-hero {
  background-color: #0f172a;
  min-height: 400px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.absolute-bg {
  position: absolute;
  top: 0; left: 0; right: 0; bottom: 0;
  background: radial-gradient(circle at top right, #3b82f6 0%, transparent 40%),
              radial-gradient(circle at bottom left, #06b6d4 0%, transparent 40%);
  opacity: 0.2;
}

.admin-card {
  transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
}

.admin-card:hover {
  transform: translateY(-8px);
  box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1), 0 10px 10px -5px rgba(0, 0, 0, 0.04) !important;
}

.icon-circle {
  width: 70px;
  height: 70px;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
}

.text-gradient {
  background: linear-gradient(135deg, #2563eb 0%, #06b6d4 100%);
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
}

.shadow-primary-lg {
  box-shadow: 0 10px 15px -3px rgba(37, 99, 235, 0.3);
}

.hero-shape-1 {
  position: absolute;
  width: 300px; height: 300px;
  background: #3b82f6;
  border-radius: 50%;
  filter: blur(80px);
  opacity: 0.1;
  top: -50px; right: 0;
}

.hero-shape-2 {
  position: absolute;
  width: 250px; height: 250px;
  background: #10b981;
  border-radius: 50%;
  filter: blur(60px);
  opacity: 0.1;
  bottom: 0; left: 0;
}

.glass-card {
  background: rgba(255, 255, 255, 0.7);
  backdrop-filter: blur(12px);
  border: 1px solid rgba(255, 255, 255, 0.5);
  transform: rotate(-3deg);
  transition: transform 0.5s ease;
}

.rotate-card:hover {
  transform: rotate(0deg) scale(1.02);
  transition: transform 0.5s ease;
}

.feature-box {
  transition: transform 0.3s ease;
}

.feature-box:hover {
  transform: translateY(-5px);
  background: #fff !important;
  box-shadow: 0 10px 15px -3px rgba(0, 0, 0, 0.05);
}

.product-card {
  transition: all 0.3s ease;
}

.product-card:hover {
  transform: translateY(-5px);
  box-shadow: 0 20px 25px -5px rgba(0, 0, 0, 0.1) !important;
}

.card-img-wrapper {
  height: 240px;
  overflow: hidden;
}

.card-img-top {
  width: 100%;
  height: 100%;
  object-fit: cover;
  transition: transform 0.5s ease;
}

.product-card:hover .card-img-top {
  transform: scale(1.1);
}

.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.btn-hover-effect {
  transition: transform 0.2s;
}

.btn-hover-effect:active {
  transform: scale(0.95);
}

.user-dashboard-section {
  background: linear-gradient(135deg, #0f172a 0%, #1e293b 100%);
}

.stat-card {
  transition: transform 0.3s ease;
}

.stat-card:hover {
  transform: translateY(-5px);
  background: rgba(255, 255, 255, 0.15);
}
</style>