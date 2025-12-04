import { defineStore } from 'pinia';
import { useAuthStore } from './auth';
import api from '../services/api';
import { debounce } from 'lodash-es';

export const useCartStore = defineStore('cart', {
    state: () => ({
        items: [],
        isLoading: false,
        error: null,
    }),

    getters: {
        totalItems: (state) => state.items.reduce((total, item) => total + item.quantity, 0),
        totalPrice: (state) => state.items.reduce((total, item) => total + (item.price * item.quantity), 0),
        cartItems: (state) => state.items,
    },

    actions: {
        // Private action to sync the cart with the backend.
        // Debounced to prevent too many API calls in rapid succession.
        _syncCartWithBackend: debounce(async function () {
            const authStore = useAuthStore();
            if (!authStore.isAuthenticated || !authStore.user?.id) {
                return; // Don't sync if user is not logged in
            }

            this.isLoading = true;
            try {
                const payload = {
                    user_id: authStore.user.id,
                    items: this.items.map(item => ({
                        product_id: item.product_id,
                        quantity: item.quantity,
                        price: item.price,
                    })),
                };
                await api.addTempOrder(payload);
            } catch (err) {
                this.error = 'Error al sincronizar el carrito.';
                console.error('Error syncing cart:', err);
            } finally {
                this.isLoading = false;
            }
        }, 1000), // Debounce for 1 second

        async initializeCart() {
            const authStore = useAuthStore();
            if (!authStore.isAuthenticated || !authStore.user?.id) {
                this.items = [];
                return;
            }

            this.isLoading = true;
            try {
                const tempOrders = await api.getTempOrdersByUserId(authStore.user.id);
                // A user might have multiple temp orders if something went wrong.
                // We'll load the most recent one. The backend returns them sorted.
                if (tempOrders && tempOrders.length > 0) {
                    this.items = tempOrders[0].items.map(item => ({
                        ...item,
                        // Ensure price is a number, as JSON can sometimes be stringy
                        price: parseFloat(item.price)
                    }));
                } else {
                    this.items = [];
                }
            } catch (err) {
                this.items = []; // Start with an empty cart on error
                this.error = 'Error al cargar el carrito.';
                console.error('Error fetching cart:', err);
            } finally {
                this.isLoading = false;
            }
        },

        addToCart(product, quantity = 1) {
            const existingItem = this.items.find(item => item.product_id === product.id);

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
            this._syncCartWithBackend();
        },

        removeFromCart(productId) {
            this.items = this.items.filter(item => item.product_id !== productId);
            this._syncCartWithBackend();
        },

        updateQuantity(productId, quantity) {
            const item = this.items.find(item => item.product_id === productId);
            if (item) {
                if (quantity > 0) {
                    item.quantity = quantity;
                } else {
                    this.items = this.items.filter(i => i.product_id !== productId);
                }
            }
            this._syncCartWithBackend();
        },

        async clearCart() {
            this.items = [];
            // Sync the empty cart to the backend
            await this._syncCartWithBackend();
            // Cancel any pending debounced calls since we just cleared everything
            this._syncCartWithBackend.cancel();
        },

        async checkout() {
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
                    user_id: authStore.user.id,
                    items: this.items.map(item => ({
                        product_id: item.product_id,
                        quantity: item.quantity,
                        price: item.price,
                    })),
                };
                const order = await api.createOrder(orderData);
                // After successful checkout, clear the cart on frontend and backend
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
    },
});