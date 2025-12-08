import { createRouter, createWebHistory } from 'vue-router'
import { useAuthStore } from '../stores/auth'

// Import components
import Home from '../views/Home.vue'
import Login from '../views/Login.vue'
import Register from '../views/Register.vue'
import EmailVerification from '../views/EmailVerification.vue'
import Products from '../views/Products.vue'
import ProductManagement from '../views/ProductManagement.vue'
import Orders from '../views/Orders.vue'
import OrderManagement from '../views/OrderManagement.vue'
import UserManagement from '../views/UserManagement.vue'
import Profile from '../views/Profile.vue'
import Cart from '../views/Cart.vue'

const routes = [
    {
        path: '/',
        name: 'Home',
        component: Home
    },
    {
        path: '/login',
        name: 'Login',
        component: Login,
        meta: { requiresGuest: true }
    },
    {
        path: '/register',
        name: 'Register',
        component: Register,
        meta: { requiresGuest: true }
    },
    {
        path: '/verify-email',
        name: 'EmailVerification',
        component: EmailVerification,
        meta: { requiresGuest: true }
    },
    {
        path: '/products',
        name: 'Products',
        component: Products,
        meta: { requiresCustomer: true } // Solo para customers
    },
    {
        path: '/cart',
        name: 'Cart',
        component: Cart,
        meta: { requiresAuth: true, requiresCustomer: true } // Solo para customers
    },
    {
        path: '/orders',
        name: 'Orders',
        component: Orders,
        meta: { requiresAuth: true, requiresCustomer: true } // Solo para customers
    },
    {
        path: '/profile',
        name: 'Profile',
        component: Profile,
        meta: { requiresAuth: true }
    },
    // Admin routes
    {
        path: '/admin/products',
        name: 'ProductManagement',
        component: ProductManagement,
        meta: { requiresAuth: true, requiresStaff: true } // Admin e Inventory
    },
    {
        path: '/admin/orders',
        name: 'OrderManagement',
        component: OrderManagement,
        meta: { requiresAuth: true, requiresStaff: true } // Admin e Inventory
    },
    {
        path: '/admin/users',
        name: 'UserManagement',
        component: UserManagement,
        meta: { requiresAuth: true, requiresAdmin: true } // Solo Admin
    }
]

const router = createRouter({
    history: createWebHistory(),
    routes
})

// Navigation guards
router.beforeEach(async (to, from, next) => {
    const authStore = useAuthStore()

    // Initialize auth if needed
    await authStore.initializeAuth()

    const isAuthenticated = authStore.isAuthenticated
    const requiresAuth = to.matched.some(record => record.meta.requiresAuth)
    const requiresGuest = to.matched.some(record => record.meta.requiresGuest)
    const requiresAdmin = to.matched.some(record => record.meta.requiresAdmin)
    const requiresStaff = to.matched.some(record => record.meta.requiresStaff)
    const requiresCustomer = to.matched.some(record => record.meta.requiresCustomer)

    // If route requires guest and user is authenticated
    if (requiresGuest && isAuthenticated) {
        // Redirigir según el rol
        if (authStore.isStaff) {
            return next('/admin/products')
        }
        return next('/')
    }

    // If route requires auth and user is not authenticated
    if (requiresAuth && !isAuthenticated) {
        return next('/login')
    }

    // If route requires admin and user is not admin
    if (requiresAdmin && !authStore.isAdmin) {
        if (authStore.isCustomer) {
            return next('/')
        }
        return next('/admin/products')
    }

    // If route requires staff (admin or inventory) and user is not staff
    if (requiresStaff && !authStore.isStaff) {
        return next('/')
    }

    // If route requires customer and user is staff (admin or inventory)
    if (requiresCustomer && authStore.isStaff) {
        return next('/admin/products')
    }

    next()
})

export default router
