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

const ordersAPI = axios.create({
    baseURL: '/api/orders',
    withCredentials: true
})

const paymentsAPI = axios.create({
    baseURL: '/api/payments',
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
[authAPI, usersAPI, inventoryAPI, productsAPI, ordersAPI, paymentsAPI].forEach(api => {
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
        const response = await usersAPI.patch('/me', userData)
        return response.data
    }
}

// User Management Service (Admin only)
const userService = {
    async getUsers(filters = {}) {
        const params = new URLSearchParams(filters)
        const queryString = params.toString()
        const url = queryString ? `/?${queryString}` : '/'
        const response = await usersAPI.get(url)
        return response.data
    },

    async getUserById(userId) {
        const response = await usersAPI.get(`/${userId}`)
        return response.data
    },

    async createUser(userData) {
        const response = await usersAPI.post('/', userData)
        return response.data
    },

    async createUserAdmin(userData) {
        const response = await usersAPI.post('/', userData)
        return response.data
    },

    async updateUser(userId, userData) {
        const response = await usersAPI.patch(`/${userId}`, userData)
        return response.data
    },

    async deleteUser(userId) {
        await usersAPI.delete(`/${userId}`)
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
        const response = await ordersAPI.post('', orderData)
        return response.data
    },

    async getOrdersByUserId() {
        const response = await ordersAPI.get('')
        return response.data
    },

    async getOrderById(id) {
        const response = await ordersAPI.get(`/${id}`)
        return response.data
    },

    async getOrderCode(id) {
        const response = await ordersAPI.get(`/${id}/code`)
        return response.data
    },

    async getOrderItems(orderId) {
        const response = await ordersAPI.get(`/${orderId}/items`);
        return response.data;
    },

    async updateOrderStatus(orderId, newStatus) {
        const payload = { status: newStatus };
        const response = await ordersAPI.put(`/${orderId}/status`, payload);
        return response.data;
    },

    async getAllOrders() {
        const response = await ordersAPI.get('/all')
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

const paymentService = {
    async processPayment(paymentData) {
        if (!paymentData.order_id) throw new Error('Order ID is required');
        if (!paymentData.user_id) throw new Error('User ID is required');
        if (!paymentData.amount || paymentData.amount <= 0) throw new Error('Valid amount is required');
        if (!paymentData.method) throw new Error('Payment method is required');
        
        const completePaymentData = {
            order_id: paymentData.order_id,
            user_id: paymentData.user_id,
            amount: parseFloat(paymentData.amount),
            method: paymentData.method,
            status: paymentData.status || 'pending',
            currency: paymentData.currency || 'USD',
            transaction_id: paymentData.transaction_id || null,
            created_at: new Date().toISOString(),
            metadata: {
                ...paymentData.metadata,
                payment_details: paymentData.metadata?.payment_details || {},
                shipping_info: paymentData.metadata?.shipping_info || {},
                browser_info: {
                    user_agent: navigator.userAgent,
                    timestamp: new Date().toISOString()
                }
            }
        };
        
        console.log('Enviando datos de pago:', completePaymentData);
        const response = await paymentsAPI.post('/', completePaymentData);
        console.log('Respuesta del servidor:', response.data);
        return response.data;
    },

    async getPayments(filters = {}) {
        const params = new URLSearchParams();
        if (filters.status) params.append('status', filters.status);
        if (filters.method) params.append('method', filters.method);
        if (filters.user_id) params.append('user_id', filters.user_id);
        if (filters.order_id) params.append('order_id', filters.order_id);
        if (filters.date_from) params.append('date_from', filters.date_from);
        if (filters.date_to) params.append('date_to', filters.date_to);
        if (filters.limit) params.append('limit', filters.limit);
        if (filters.offset) params.append('offset', filters.offset);
        
        const url = params.toString() ? `/?${params}` : '/';
        const response = await paymentsAPI.get(url);
        return response.data;
    },

    async getPaymentById(paymentId) {
        if (!paymentId) throw new Error('Payment ID is required');
        const response = await paymentsAPI.get(`/${paymentId}`);
        return response.data;
    },

    async getPaymentsByOrderId(orderId) {
        if (!orderId) throw new Error('Order ID is required');
        const response = await paymentsAPI.get(`/order/${orderId}`);
        return response.data;
    },

    async getPaymentsByUserId(userId) {
        if (!userId) throw new Error('User ID is required');
        const response = await paymentsAPI.get(`/user/${userId}`);
        return response.data;
    },

    async updatePaymentStatus(paymentId, status, reason = null) {
        if (!paymentId) throw new Error('Payment ID is required');
        if (!status) throw new Error('Status is required');
        
        const updateData = {
            status: status,
            updated_at: new Date().toISOString()
        };
        
        if (reason) {
            updateData.reason = reason;
        }
        
        if (status === 'completed') {
            updateData.paid_at = new Date().toISOString();
            updateData.transaction_id = `txn_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`;
        }
        
        console.log('Actualizando pago:', paymentId, updateData);
        const response = await paymentsAPI.put(`/${paymentId}/status`, updateData);
        return response.data;
    },

    async updatePaymentMetadata(paymentId, metadata) {
        if (!paymentId) throw new Error('Payment ID is required');
        if (!metadata) throw new Error('Metadata is required');
        
        const response = await paymentsAPI.put(`/${paymentId}/metadata`, { metadata });
        return response.data;
    },

    async refundPayment(paymentId, reason = null, amount = null) {
        if (!paymentId) throw new Error('Payment ID is required');
        
        const refundData = {
            status: 'refunded',
            refunded_at: new Date().toISOString(),
            refund_reason: reason || 'Customer request'
        };
        
        if (amount && amount > 0) {
            refundData.refund_amount = parseFloat(amount);
        }
        
        console.log('Procesando reembolso:', paymentId, refundData);
        const response = await paymentsAPI.post(`/${paymentId}/refund`, refundData);
        return response.data;
    },

    async retryPayment(paymentId) {
        if (!paymentId) throw new Error('Payment ID is required');
        
        const retryData = {
            status: 'pending',
            retry_count: 1,
            last_retry_at: new Date().toISOString()
        };
        
        console.log('Reintentando pago:', paymentId, retryData);
        const response = await paymentsAPI.post(`/${paymentId}/retry`, retryData);
        return response.data;
    },

    async cancelPayment(paymentId, reason = null) {
        if (!paymentId) throw new Error('Payment ID is required');
        
        const cancelData = {
            status: 'cancelled',
            cancelled_at: new Date().toISOString(),
            cancel_reason: reason || 'User cancellation'
        };
        
        console.log('Cancelando pago:', paymentId, cancelData);
        const response = await paymentsAPI.post(`/${paymentId}/cancel`, cancelData);
        return response.data;
    },

    async getPaymentStats(filters = {}) {
        const params = new URLSearchParams();
        if (filters.date_from) params.append('date_from', filters.date_from);
        if (filters.date_to) params.append('date_to', filters.date_to);
        
        const url = params.toString() ? `/stats?${params}` : '/stats';
        const response = await paymentsAPI.get(url);
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
    ...paymentService,
}

export default api;
