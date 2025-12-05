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
        beforeEnter: (to, from, next) => {
            const authStore = useAuthStore();
            if (authStore.isAdmin) {
                next({ name: 'ProductManagement' });
            } else {
                next();
            }
        }
    },
    {
        path: '/cart',
        name: 'Cart',
        component: Cart,
        meta: { requiresAuth: true }
    },
    {
        path: '/orders',
        name: 'Orders',
        component: Orders,
        meta: { requiresAuth: true }
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
        meta: { requiresAuth: true, requiresAdmin: true }
    },
    {
        path: '/admin/orders',
        name: 'OrderManagement',
        component: OrderManagement,
        meta: { requiresAuth: true, requiresAdmin: true }
    },
    {
        path: '/admin/users',
        name: 'UserManagement',
        component: UserManagement,
        meta: { requiresAuth: true, requiresAdmin: true }
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

    // If route requires guest and user is authenticated
    if (requiresGuest && isAuthenticated) {
        return next('/')
    }

    // If route requires auth and user is not authenticated
    if (requiresAuth && !isAuthenticated) {
        return next('/login')
    }

    // If route requires admin and user is not admin
    if (requiresAdmin && !authStore.isAdmin) {
        return next('/')
    }

    next()
})

export default router
