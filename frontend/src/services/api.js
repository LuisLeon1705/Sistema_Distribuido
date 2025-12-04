import axios from 'axios'

// Create axios instances for each service
const authAPI = axios.create({
    baseURL: '/api/auth',
    withCredentials: true
})

const inventoryAPI = axios.create({
    baseURL: '/api/inventory',
    withCredentials: true
})

const usersAPI = axios.create({
    baseURL: '/api/users',
    withCredentials: true
})

const productsAPI = axios.create({
    baseURL: '/api/products',
    withCredentials: true
})

// Request interceptor to add JWT token
const addAuthInterceptor = (apiInstance) => {
    apiInstance.interceptors.request.use(
        (config) => {
            const token = localStorage.getItem('access_token')
            if (token) {
                config.headers.Authorization = `Bearer ${token}`
            }
            return config
        },
        (error) => Promise.reject(error)
    )
}

// Response interceptor to handle token expiration
const addResponseInterceptor = (apiInstance) => {
    apiInstance.interceptors.response.use(
        (response) => response,
        (error) => {
            if (error.response?.status === 401) {
                localStorage.removeItem('access_token')
                localStorage.removeItem('user_role')
                window.location.href = '/login'
            }
            return Promise.reject(error)
        }
    )
}

// Add interceptors to all instances
[authAPI, usersAPI, inventoryAPI, productsAPI].forEach(api => {
    addAuthInterceptor(api)
    addResponseInterceptor(api)
})

// Auth Service
const authService = {
    async register(userData) {
        const response = await authAPI.post('/register', userData)
        return response.data
    },

    async login(credentials) {
        const response = await authAPI.post('/login', credentials)
        if (response.data.access_token) {
            localStorage.setItem('access_token', response.data.access_token)
        }
        return response.data
    },

    async sendVerificationCode(email) {
        const response = await authAPI.post('/send-verification-code', { email })
        return response.data
    },

    async verifyEmail(email, code) {
        const response = await authAPI.post('/verify-email', { email, code })
        return response.data
    },

    async logout() {
        try {
            await authAPI.post('/logout')
        } finally {
            localStorage.removeItem('access_token')
            localStorage.removeItem('user_role')
        }
    },

    async getCurrentUser() {
        const response = await usersAPI.get('/me')
        return response.data
    },

    async updateProfile(userData) {
        const response = await usersAPI.patch('/users/me', userData)
        return response.data
    }
}

// User Management Service (Admin only)
const userService = {
    async getUsers(filters = {}) {
        const params = new URLSearchParams(filters)
        const response = await usersAPI.get(`/${params}`)
        return response.data
    },

    async getUserById(userId) {
        const response = await usersAPI.get(`/users/${userId}`)
        return response.data
    },

    async createUser(userData) {
        const response = await usersAPI.post('/users', userData)
        return response.data
    },

    async updateUser(userId, userData) {
        const response = await usersAPI.patch(`/users/${userId}`, userData)
        return response.data
    },

    async deleteUser(userId) {
        await usersAPI.delete(`/users/${userId}`)
    }
}

// Product Service
const productService = {
    async getProducts() {
        const response = await productsAPI.get('/productos')
        return response.data
    },

    async getActiveProducts() {
        const response = await productsAPI.get('/productos/activos')
        return response.data
    },

    async getProductById(id) {
        const response = await productsAPI.get(`/productos/${id}`)
        return response.data
    },

    async getProductsByCategory(categoryId) {
        const response = await productsAPI.get(`/productos/categoria/${categoryId}`)
        return response.data
    },

    async createProduct(productData) {
        let response
        if (productData instanceof FormData) {
            response = await productsAPI.post('/productos', productData)
        } else {
            response = await productsAPI.post('/productos', productData)
        }
        return response.data
    },

    async updateProduct(id, productData) {
        let response
        if (productData instanceof FormData) {
            // Laravel may not handle multipart PUT properly in all setups; use POST + _method override
            productData.append('_method', 'PUT')
            response = await productsAPI.post(`/productos/${id}`, productData)
        } else {
            response = await productsAPI.put(`/productos/${id}`, productData)
        }
        return response.data
    },

    async deleteProduct(id) {
        await productsAPI.delete(`/productos/${id}`)
    },

    async getCategories() {
        const response = await productsAPI.get('/categorias')
        return response.data
    },

    async createCategory(categoryData) {
        const response = await productsAPI.post('/categorias', categoryData)
        return response.data
    }
    ,
    async updateCategory(id, categoryData) {
        const response = await productsAPI.put(`/categorias/${id}`, categoryData)
        return response.data
    },

    async deleteCategory(id) {
        await productsAPI.delete(`/categorias/${id}`)
    }
}

// Inventory/Orders Service
const orderService = {
    async createOrder(orderData) {
        const response = await inventoryAPI.post('/orders', orderData)
        return response.data
    },

    async getOrdersByUserId(userId) {
        const response = await inventoryAPI.get(`/orders/user/${userId}`)
        return response.data
    },

    async getOrderById(id) {
        const response = await inventoryAPI.get(`/orders/${id}`)
        return response.data
    },

    async getOrderItems(orderId) {
        const response = await inventoryAPI.get(`/orders/${orderId}/items`);
        return response.data;
    },

    async updateOrderStatus(orderId, newStatus) {
        const payload = { order_id: orderId, new_status: newStatus };
        const response = await inventoryAPI.post('/orders/status', payload);
        return response.data;
    },

    async getAllOrders() {
        const response = await inventoryAPI.get('/admin/orders')
        return response.data
    },

    async addTempOrder(orderData) {
        const response = await inventoryAPI.post('/temp_orders', orderData);
        return response.data;
    },

    async getTempOrdersByUserId(userId) {
        const response = await inventoryAPI.get(`/temp_orders/user/${userId}`);
        return response.data;
    }
}

// Stock Service
const stockService = {
    async getStock(productId) {
        const url = productId ? `/stock/${productId}` : '/stock';
        return await inventoryAPI.get(url);
    },

    async addStock(stockData) {
        return await inventoryAPI.post('/stock', stockData);
    },

    async updateStock(productId, stockData) {
        return await inventoryAPI.put(`/stock/${productId}`, stockData);
    },

    async deleteStock(productId) {
        return await inventoryAPI.delete(`/stock/${productId}`);
    },

    async seedStock() {
        return await inventoryAPI.post('/seed');
    }
};


const api = {
    ...authService,
    ...userService,
    ...productService,
    ...orderService,
    ...stockService,
}

export default api;

