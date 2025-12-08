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
          <!-- Links para Customers -->
          <template v-if="!isStaff">
            <li class="nav-item">
              <router-link class="nav-link" to="/">Inicio</router-link>
            </li>
            <li class="nav-item">
              <router-link class="nav-link" to="/products">Productos</router-link>
            </li>
          </template>
          
          <!-- Links de carrito y órdenes solo para customers autenticados -->
          <template v-if="isAuthenticated && canUseCart">
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
          
          <!-- Links para Staff (Admin e Inventory) -->
          <template v-if="isStaff">
            <li class="nav-item">
              <router-link class="nav-link" to="/admin/products">
                <i class="fas fa-box me-1"></i>
                Gestión de Productos
              </router-link>
            </li>
            <li class="nav-item">
              <router-link class="nav-link" to="/admin/orders">
                <i class="fas fa-clipboard-list me-1"></i>
                Gestión de Órdenes
              </router-link>
            </li>
            <!-- Solo Admin puede ver gestión de usuarios -->
            <li class="nav-item" v-if="isAdmin">
              <router-link class="nav-link" to="/admin/users">
                <i class="fas fa-users me-1"></i>
                Gestión de Usuarios
              </router-link>
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
                <span class="badge ms-1" :class="getRoleBadgeClass">{{ getRoleDisplayName }}</span>
              </a>
              <ul class="dropdown-menu dropdown-menu-end">
                <li>
                  <router-link class="dropdown-item" to="/profile">
                    <i class="fas fa-user me-2"></i>Mi Perfil
                  </router-link>
                </li>
                <li><hr class="dropdown-divider"></li>
                <li>
                  <a class="dropdown-item text-danger" href="#" @click="handleLogout">
                    <i class="fas fa-sign-out-alt me-2"></i>Cerrar Sesión
                  </a>
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
    const isStaff = computed(() => authStore.isStaff)
    const canUseCart = computed(() => authStore.canUseCart)
    const user = computed(() => authStore.user)
    const userRole = computed(() => authStore.userRole)
    const cartItemsCount = computed(() => cartStore.totalItems)

    const getRoleBadgeClass = computed(() => {
      const role = authStore.userRole
      switch(role) {
        case 'admin':
          return 'bg-danger'
        case 'inventory':
          return 'bg-warning text-dark'
        case 'customer':
        default:
          return 'bg-secondary'
      }
    })

    const getRoleDisplayName = computed(() => {
      const role = authStore.userRole
      switch(role) {
        case 'admin':
          return 'Administrador'
        case 'inventory':
          return 'Inventario'
        case 'customer':
        default:
          return 'Cliente'
      }
    })

    const handleLogout = async () => {
      try {
        await authStore.logout()
        cartStore.clearCart()
        router.push('/login')
      } catch (error) {
        console.error('Logout error:', error)
      }
    }

    return {
      isAuthenticated,
      isAdmin,
      isStaff,
      canUseCart,
      user,
      userRole,
      cartItemsCount,
      getRoleBadgeClass,
      getRoleDisplayName,
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

.dropdown-item i {
  width: 20px;
}
</style>
