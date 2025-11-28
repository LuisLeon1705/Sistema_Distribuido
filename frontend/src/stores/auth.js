import { defineStore } from 'pinia'
import { authService } from '../services/api'

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
        isCustomer: (state) => state.user?.role?.name === 'customer',
        userRole: (state) => state.user?.role?.name || null
    },

    actions: {
        async login(credentials) {
            this.isLoading = true
            this.error = null
            try {
                const response = await authService.login(credentials)
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
                const response = await authService.register(userData)
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
                const response = await authService.sendVerificationCode(email)
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
                const response = await authService.verifyEmail(email, code)
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
                await authService.logout()
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
                this.user = await authService.getCurrentUser()
                localStorage.setItem('user_role', this.user.role?.name || 'customer')
            } catch (error) {
                console.error('Error fetching current user:', error)
                this.logout()
            }
        },

        async updateProfile(userData) {
            this.isLoading = true
            this.error = null
            try {
                const updatedUser = await authService.updateProfile(userData)
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
