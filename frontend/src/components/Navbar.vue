<template>
  <nav class="navbar navbar-expand-lg modern-navbar sticky-top">
    <div class="container">
      <router-link class="navbar-brand d-flex align-items-center" to="/">
        <i class="fas fa-network-wired me-2 text-primary"></i>
        <span class="brand-text">We<span class="text-primary">Commerce</span></span>
      </router-link>
      
      <button class="navbar-toggler border-0 shadow-none" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNav">
        <span class="navbar-toggler-icon"></span>
      </button>
      
      <div class="collapse navbar-collapse" id="navbarNav">
        <ul class="navbar-nav mx-auto mb-2 mb-lg-0">
          <template v-if="!isStaff">
            <li class="nav-item">
              <router-link class="nav-link" to="/">Inicio</router-link>
            </li>
            <li class="nav-item">
              <router-link class="nav-link" to="/products">Productos</router-link>
            </li>
          </template>
          
          <template v-if="isAuthenticated && canUseCart">
            <li class="nav-item">
              <router-link class="nav-link" to="/orders">Mis Órdenes</router-link>
            </li>
          </template>
          
          <template v-if="isStaff">
            <li class="nav-item">
              <router-link class="nav-link" to="/admin/products">
                Inventario
              </router-link>
            </li>
            <li class="nav-item">
              <router-link class="nav-link" to="/admin/orders">
                Órdenes de Compra
              </router-link>
            </li>
            <li class="nav-item" v-if="isAdmin">
              <router-link class="nav-link" to="/admin/users">
                Usuarios
              </router-link>
            </li>
          </template>
        </ul>
        
        <ul class="navbar-nav align-items-lg-center">
          <template v-if="!isAuthenticated">
            <li class="nav-item">
              <router-link class="nav-link" to="/login">Ingresar</router-link>
            </li>
            <li class="nav-item ms-lg-2">
              <router-link class="btn btn-primary btn-sm rounded-pill px-4 fw-bold" to="/register">Registro</router-link>
            </li>
          </template>
          
          <template v-else>
            <li class="nav-item dropdown">
              <a class="nav-link dropdown-toggle d-flex align-items-center gap-2" href="#" role="button" data-bs-toggle="dropdown">
                <div class="avatar-circle">
                  {{ user?.username?.charAt(0).toUpperCase() }}
                </div>
                <span class="d-none d-lg-block">{{ user?.username }}</span>
              </a>
              <ul class="dropdown-menu dropdown-menu-end shadow-lg border-0 mt-2 p-2 rounded-3">
                <li class="px-3 py-2">
                  <span class="badge w-100 py-2" :class="getRoleBadgeClass">{{ getRoleDisplayName }}</span>
                </li>
                <li><hr class="dropdown-divider"></li>
                <li>
                  <router-link class="dropdown-item rounded-2" to="/profile">
                    <i class="fas fa-user-circle me-2"></i>Perfil
                  </router-link>
                </li>
                <li>
                  <a class="dropdown-item rounded-2 text-danger" href="#" @click="handleLogout">
                    <i class="fas fa-sign-out-alt me-2"></i>Salir
                  </a>
                </li>
              </ul>
            </li>
          </template>

          <template v-if="isAuthenticated && canUseCart">
            <li class="nav-item">
              <router-link class="nav-link position-relative" to="/cart">
                <i class="fas fa-shopping-cart me-1"></i>
                <span v-if="cartItemsCount > 0" class="position-absolute top-0 start-100 translate-middle badge rounded-pill bg-primary cart-badge">
                  {{ cartItemsCount }}
                </span>
              </router-link>
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
        case 'admin': return 'bg-danger-subtle text-danger border border-danger'
        case 'inventory': return 'bg-warning-subtle text-warning-emphasis border border-warning'
        case 'customer': default: return 'bg-primary-subtle text-primary border border-primary'
      }
    })

    const getRoleDisplayName = computed(() => {
      const role = authStore.userRole
      switch(role) {
        case 'admin': return 'ADMINISTRADOR'
        case 'inventory': return 'INVENTARIO'
        case 'customer': default: return 'CLIENTE'
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
.modern-navbar {
  background-color: #0f172a; 
  backdrop-filter: blur(10px);
  border-bottom: 1px solid rgba(255, 255, 255, 0.05);
  padding: 0.8rem 0;
  transition: all 0.3s ease;
}

.brand-text {
  font-weight: 700;
  font-size: 1.25rem;
  letter-spacing: -0.5px;
  color: #fff;
}

.nav-link {
  color: #94a3b8 !important;
  font-weight: 500;
  font-size: 0.95rem;
  padding: 0.5rem 1rem;
  transition: all 0.2s ease;
  position: relative;
}

.nav-link:hover, .nav-link.router-link-active {
  color: #fff !important;
}

.nav-link.router-link-active::after {
  content: '';
  position: absolute;
  bottom: 0;
  left: 1rem;
  right: 1rem;
  height: 2px;
  background-color: #3b82f6;
  border-radius: 2px;
}

.cart-badge {
  font-size: 0.65rem;
  padding: 0.35em 0.6em;
  transform: translate(-50%, -10%) !important;
}

.avatar-circle {
  width: 32px;
  height: 32px;
  background: linear-gradient(135deg, #3b82f6, #2563eb);
  color: white;
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-weight: 600;
  font-size: 0.9rem;
}

.dropdown-menu {
  background: #1e293b;
  border: 1px solid rgba(255, 255, 255, 0.1);
  min-width: 220px;
}

.dropdown-item {
  color: #cbd5e1;
  padding: 0.6rem 1rem;
  transition: all 0.2s;
}

.dropdown-item:hover {
  background-color: rgba(255, 255, 255, 0.05);
  color: #fff;
}

.dropdown-divider {
  border-color: rgba(255, 255, 255, 0.1);
}

@media (max-width: 991px) {
  .modern-navbar {
    padding: 0.5rem 0;
  }

  .navbar-collapse {
    background: #0f172a;
    padding: 1rem;
    border-radius: 0.5rem;
    margin-top: 0.5rem;
  }
  
  .nav-link.router-link-active::after {
    left: 0;
    right: auto;
    width: 3px;
    height: 100%;
    bottom: auto;
    top: 0;
  }
}
</style>