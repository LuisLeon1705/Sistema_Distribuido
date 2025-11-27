<template>
  <nav class="navbar navbar-expand-lg navbar-dark bg-dark">
    <div class="container-fluid">
      <router-link class="navbar-brand" to="/">
        Sistema Distribuido
      </router-link>
      
      <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNav">
        <span class="navbar-toggler-icon"></span>
      </button>
      
      <div class="collapse navbar-collapse" id="navbarNav">
        <ul class="navbar-nav me-auto">
          <li class="nav-item">
            <router-link class="nav-link" to="/">Inicio</router-link>
          </li>
          <li class="nav-item">
            <router-link class="nav-link" to="/products">Productos</router-link>
          </li>
          
          <!-- Authenticated user links -->
          <template v-if="isAuthenticated">
            <li class="nav-item">
              <router-link class="nav-link" to="/cart">
                Carrito
                <span v-if="cartItemsCount > 0" class="badge bg-danger ms-1">
                  {{ cartItemsCount }}
                </span>
              </router-link>
            </li>
            <li class="nav-item">
              <router-link class="nav-link" to="/orders">Mis Órdenes</router-link>
            </li>
          </template>
          
          <!-- Admin links -->
          <template v-if="isAdmin">
            <li class="nav-item dropdown">
              <a class="nav-link dropdown-toggle" href="#" role="button" data-bs-toggle="dropdown">
                Administración
              </a>
              <ul class="dropdown-menu">
                <li>
                  <router-link class="dropdown-item" to="/admin/products">
                    Gestión de Productos
                  </router-link>
                </li>
                <li>
                  <router-link class="dropdown-item" to="/admin/orders">
                    Gestión de Órdenes
                  </router-link>
                </li>
                <li>
                  <router-link class="dropdown-item" to="/admin/users">
                    Gestión de Usuarios
                  </router-link>
                </li>
              </ul>
            </li>
          </template>
        </ul>
        
        <!-- Auth buttons -->
        <ul class="navbar-nav">
          <template v-if="!isAuthenticated">
            <li class="nav-item">
              <router-link class="nav-link" to="/login">Iniciar Sesión</router-link>
            </li>
            <li class="nav-item">
              <router-link class="nav-link" to="/register">Registrarse</router-link>
            </li>
          </template>
          
          <template v-else>
            <li class="nav-item dropdown">
              <a class="nav-link dropdown-toggle" href="#" role="button" data-bs-toggle="dropdown">
                {{ user?.username }}
                <span class="badge bg-secondary ms-1">{{ userRole }}</span>
              </a>
              <ul class="dropdown-menu dropdown-menu-end">
                <li>
                  <router-link class="dropdown-item" to="/profile">Mi Perfil</router-link>
                </li>
                <li><hr class="dropdown-divider"></li>
                <li>
                  <a class="dropdown-item" href="#" @click="handleLogout">Cerrar Sesión</a>
                </li>
              </ul>
            </li>
          </template>
        </ul>
      </div>
    </div>
  </nav>
</template>

<script>
import { computed } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'
import { useCartStore } from '../stores/cart'

export default {
  name: 'Navbar',
  setup() {
    const router = useRouter()
    const authStore = useAuthStore()
    const cartStore = useCartStore()

    const isAuthenticated = computed(() => authStore.isAuthenticated)
    const isAdmin = computed(() => authStore.isAdmin)
    const user = computed(() => authStore.user)
    const userRole = computed(() => authStore.userRole)
    const cartItemsCount = computed(() => cartStore.totalItems)

    const handleLogout = async () => {
      try {
        await authStore.logout()
        cartStore.clearCart()
        router.push('/')
      } catch (error) {
        console.error('Logout error:', error)
      }
    }

    return {
      isAuthenticated,
      isAdmin,
      user,
      userRole,
      cartItemsCount,
      handleLogout
    }
  }
}
</script>

<style scoped>
.navbar-brand {
  font-weight: bold;
}

.nav-link.router-link-active {
  color: #fff !important;
  font-weight: bold;
}

.badge {
  font-size: 0.7em;
}
</style>
