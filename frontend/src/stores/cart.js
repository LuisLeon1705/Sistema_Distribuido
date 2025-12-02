import { defineStore } from 'pinia'
import { orderService } from '../services/api'

export const useCartStore = defineStore('cart', {
    state: () => ({
        items: [],
        isLoading: false,
        error: null
    }),

    getters: {
        totalItems: (state) => state.items.reduce((total, item) => total + item.quantity, 0),
        totalPrice: (state) => state.items.reduce((total, item) => total + (item.price * item.quantity), 0),
        cartItems: (state) => state.items
    },

    actions: {
        addToCart(product, quantity = 1) {
            const existingItem = this.items.find(item => item.product_id === product.id)

            if (existingItem) {
                existingItem.quantity += quantity
            } else {
                this.items.push({
                    product_id: product.id,
                    name: product.nombre,
                    price: parseFloat(product.precio),
                    quantity: quantity,
                    image: product.imagen
                })
            }
        },

        removeFromCart(productId) {
            this.items = this.items.filter(item => item.product_id !== productId)
        },

        updateQuantity(productId, quantity) {
            const item = this.items.find(item => item.product_id === productId)
            if (item && quantity > 0) {
                item.quantity = quantity
            } else if (item && quantity <= 0) {
                this.removeFromCart(productId)
            }
        },

        clearCart() {
            this.items = []
        },

        async checkout() {
            if (this.items.length === 0) {
                throw new Error('El carrito está vacío')
            }

            this.isLoading = true
            this.error = null

            try {
                const orderData = {
                    items: this.items.map(item => ({
                        product_id: item.product_id,
                        quantity: item.quantity
                    }))
                }

                const order = await orderService.createOrder(orderData)
                this.clearCart()
                return order
            } catch (error) {
                this.error = error.response?.data?.detail || 'Error al procesar la orden'
                throw error
            } finally {
                this.isLoading = false
            }
        },

        clearError() {
            this.error = null
        }
    },

    persist: {
        enabled: true,
        strategies: [
            {
                key: 'cart',
                storage: localStorage,
                paths: ['items']
            }
        ]
    }
})
