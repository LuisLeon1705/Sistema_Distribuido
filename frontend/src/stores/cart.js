import { defineStore } from 'pinia';
import { useAuthStore } from './auth';
import api from '../services/api';

export const useCartStore = defineStore('cart', {
    state: () => ({
        items: [],
        isLoading: false,
        error: null,
        lastPaymentId: null,
    }),

    getters: {
        totalItems: (state) => state.items.reduce((total, item) => total + item.quantity, 0),
        totalPrice: (state) => state.items.reduce((total, item) => total + (item.price * item.quantity), 0),
        cartItems: (state) => state.items,
    },

    actions: {
        async syncAndRefresh() {
            const authStore = useAuthStore();
            if (!authStore.isAuthenticated || !authStore.user?.id) return;

            try {
                // First, send the current optimistic state to the backend
                const payload = {
                    user_id: authStore.user.id,
                    items: this.items.map(item => ({
                        product_id: item.product_id,
                        quantity: item.quantity,
                        price: item.price,
                    })),
                };
                await api.addTempOrder(payload);

                // Then, fetch the authoritative state from the backend
                // This will overwrite the local state with the backend's calculated quantities
                await this.initializeCart();
            } catch (err) {
                this.error = 'Error al sincronizar el carrito.';
                console.error('Error syncing and refreshing cart:', err);
            }
        },

        async initializeCart() {
            const authStore = useAuthStore();
            if (!authStore.isAuthenticated || !authStore.user?.id) {
                this.items = [];
                return;
            }

            this.isLoading = true;
            try {
                const tempOrders = await api.getTempOrdersByUserId(authStore.user.id);

                if (tempOrders && tempOrders.length > 0) {
                    const mostRecentOrder = tempOrders[0];
                    const productPromises = mostRecentOrder.items.map(item =>
                        api.getProductById(item.product_id).catch(() => null)
                    );
                    const products = await Promise.all(productPromises);
                    const productsMap = new Map(products.filter(p => p).map(p => [p.id, p]));

                    this.items = mostRecentOrder.items.map(item => {
                        const product = productsMap.get(item.product_id);
                        return {
                            product_id: item.product_id,
                            name: product ? product.nombre : 'Producto no disponible',
                            price: parseFloat(item.price),
                            quantity: item.quantity,
                            image: product ? product.imagen : null,
                        };
                    });
                } else {
                    this.items = [];
                }
            } catch (err) {
                this.items = [];
                this.error = 'Error al cargar el carrito.';
                console.error('Error fetching cart:', err);
            } finally {
                this.isLoading = false;
            }
        },

        async addToCart(product, quantity = 1) {
            try {
                // Verificar stock disponible
                const stockResponse = await api.getStock(product.id);
                const availableStock = stockResponse.data?.[0]?.quantity || 0;

                const existingItem = this.items.find(item => item.product_id === product.id);
                const currentQuantity = existingItem ? existingItem.quantity : 0;
                const newTotalQuantity = currentQuantity + quantity;

                if (newTotalQuantity > availableStock) {
                    this.error = `Stock insuficiente. Disponible: ${availableStock}, en carrito: ${currentQuantity}`;
                    setTimeout(() => this.error = null, 5000);
                    return;
                }

                if (existingItem) {
                    existingItem.quantity += quantity;
                } else {
                    this.items.push({
                        product_id: product.id,
                        name: product.nombre,
                        price: parseFloat(product.precio),
                        quantity: quantity,
                        image: product.imagen,
                    });
                }
                this.syncAndRefresh();
            } catch (err) {
                this.error = 'Error al verificar el stock';
                console.error('Error checking stock:', err);
            }
        },

        removeFromCart(productId) {
            this.items = this.items.filter(item => item.product_id !== productId);
            this.syncAndRefresh();
        },

        async updateQuantity(productId, quantity) {
            const item = this.items.find(item => item.product_id === productId);
            if (!item) return;

            if (quantity <= 0) {
                this.items = this.items.filter(i => i.product_id !== productId);
                this.syncAndRefresh();
                return;
            }

            try {
                // Verificar stock disponible
                const stockResponse = await api.getStock(productId);
                const availableStock = stockResponse.data?.[0]?.quantity || 0;

                if (quantity > availableStock) {
                    this.error = `Stock insuficiente. Disponible: ${availableStock}`;
                    setTimeout(() => this.error = null, 5000);
                    // Mantener la cantidad anterior
                    return;
                }

                item.quantity = quantity;
                this.syncAndRefresh();
            } catch (err) {
                this.error = 'Error al verificar el stock';
                console.error('Error checking stock:', err);
            }
        },

        async clearCart() {
            this.items = [];
            this.syncAndRefresh();
        },

        async checkout(orderMeta = {}) {
            if (this.items.length === 0) throw new Error('El carrito está vacío');

            const authStore = useAuthStore();
            if (!authStore.user?.id) {
                this.error = "Debes iniciar sesión para comprar.";
                return;
            }

            this.isLoading = true;
            this.error = null;
            try {
                const orderData = {
                    items: this.items.map(item => ({
                        productId: item.product_id,
                        quantity: item.quantity,
                    })),
                    shippingAddress: orderMeta.shippingAddress || null,
                    shippingCity: orderMeta.shippingCity || null,
                    shippingPostal: orderMeta.shippingPostal || null
                };

                const order = await api.createOrder(orderData);

                await this.clearCart();
                return order;
            } catch (error) {
                this.error = error.response?.data?.detail || 'Error al procesar la orden';
                throw error;
            } finally {
                this.isLoading = false;
            }
        },

        clearError() {
            this.error = null;
        },
        setPaymentId(paymentId) {
            this.lastPaymentId = paymentId;
        },
    },
});
