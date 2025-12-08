import { defineStore } from 'pinia'
import api from '../services/api'

export const useAuthStore = defineStore('auth', {
    state: () => ({
        user: null,
        token: localStorage.getItem('access_token') || null,
        isLoading: false,
        error: null
    }),

    getters: {
        isAuthenticated: (state) => !!state.token && !!state.user,
        isAdmin: (state) => state.user?.role?.name === 'admin',
        isInventory: (state) => state.user?.role?.name === 'inventory',
        isCustomer: (state) => state.user?.role?.name === 'customer',
        userRole: (state) => state.user?.role?.name || null,
        // Getter para identificar si puede usar el carrito (solo customers)
        canUseCart: (state) => state.user?.role?.name === 'customer',
        // Getter para identificar si es staff (admin o inventory)
        isStaff: (state) => ['admin', 'inventory'].includes(state.user?.role?.name)
    },

    actions: {
        async login(credentials) {
            this.isLoading = true
            this.error = null
            try {
                const response = await api.login(credentials)
                this.token = response.access_token
                localStorage.setItem('access_token', response.access_token)
                await this.fetchCurrentUser()
                return response
            } catch (error) {
                this.error = error.response?.data?.detail || 'Login failed'
                throw error
            } finally {
                this.isLoading = false
            }
        },

        async register(userData) {
            this.isLoading = true
            this.error = null
            try {
                const response = await api.register(userData)
                return response
            } catch (error) {
                this.error = error.response?.data?.detail || 'Registration failed'
                throw error
            } finally {
                this.isLoading = false
            }
        },

        async sendVerificationCode(email) {
            this.isLoading = true
            this.error = null
            try {
                const response = await api.sendVerificationCode(email)
                return response
            } catch (error) {
                this.error = error.response?.data?.detail || 'No se pudo enviar el código'
                throw error
            } finally {
                this.isLoading = false
            }
        },

        async verifyEmail(email, code) {
            this.isLoading = true
            this.error = null
            try {
                const response = await api.verifyEmail(email, code)
                return response
            } catch (error) {
                this.error = error.response?.data?.detail || 'Verificación fallida'
                throw error
            } finally {
                this.isLoading = false
            }
        },

        async logout() {
            try {
                await api.logout()
            } catch (error) {
                console.error('Logout error:', error)
            } finally {
                this.token = null
                this.user = null
                localStorage.removeItem('access_token')
                localStorage.removeItem('user_role')
            }
        },

        async fetchCurrentUser() {
            if (!this.token) return

            try {
                this.user = await api.getCurrentUser()
                localStorage.setItem('user_role', this.user.role?.name || 'customer')
            } catch (error) {
                console.error('Error fetching current user:', error)
                // this.logout()
            }
        },

        async updateProfile(userData) {
            this.isLoading = true
            this.error = null
            try {
                const updatedUser = await api.updateProfile(userData)
                this.user = updatedUser
                return updatedUser
            } catch (error) {
                this.error = error.response?.data?.detail || 'Update failed'
                throw error
            } finally {
                this.isLoading = false
            }
        },

        async initializeAuth() {
            if (this.token && !this.user) {
                await this.fetchCurrentUser()
            }
        },

        clearError() {
            this.error = null
        }
    }
})
