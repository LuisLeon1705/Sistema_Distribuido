<template>
  <div class="verification-page">
    <div class="verification-card">
      <div class="text-center mb-4">
        <h4 class="fw-bold text-dark mb-1">Verificar Correo</h4>
        <p class="text-muted small">
          Enviamos un código a <strong>{{ email }}</strong>
        </p>
      </div>

      <form @submit.prevent="handleVerify">
        <div class="mb-3">
          <label for="code" class="form-label small fw-bold text-muted">Código de 6 dígitos</label>
          <input
            id="code"
            type="text"
            class="form-control text-center tracking-widest fw-bold"
            v-model="code"
            maxlength="6"
            autocomplete="one-time-code"
            placeholder="000000"
            :class="{ 'is-invalid': codeError }"
            required
            style="letter-spacing: 0.5rem; font-size: 1.2rem;"
          >
          <div v-if="codeError" class="invalid-feedback text-center">
            {{ codeError }}
          </div>
        </div>

        <div v-if="authStore.error" class="alert alert-danger py-2 small text-center mb-3 border-0 bg-danger bg-opacity-10 text-danger">
          {{ authStore.error }}
        </div>

        <div v-if="successMessage" class="alert alert-success py-2 small text-center mb-3 border-0 bg-success bg-opacity-10 text-success">
          {{ successMessage }}
        </div>

        <button
          type="submit"
          class="btn btn-primary w-100 fw-bold py-2 mb-3 shadow-sm"
          :disabled="authStore.isLoading"
        >
          <span v-if="authStore.isLoading" class="spinner-border spinner-border-sm me-2"></span>
          {{ authStore.isLoading ? 'Verificando...' : 'Confirmar Código' }}
        </button>
      </form>

      <div class="text-center">
        <button
          type="button"
          class="btn btn-link text-decoration-none small text-muted p-0 mb-3"
          :disabled="authStore.isLoading || isResending"
          @click="handleResend"
        >
          <span v-if="isResending" class="spinner-border spinner-border-sm me-1"></span>
          {{ isResending ? 'Enviando...' : '¿No recibiste el código? Reenviar' }}
        </button>
      </div>

      <div class="text-center border-top pt-3">
        <router-link to="/login" class="text-decoration-none small text-dark fw-bold">
          &larr; Volver al inicio
        </router-link>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'

export default {
  name: 'EmailVerification',
  setup() {
    const route = useRoute()
    const router = useRouter()
    const authStore = useAuthStore()

    const email = computed(() => route.query.email || '')
    const code = ref('')
    const codeError = ref('')
    const successMessage = ref('')
    const isResending = ref(false)

    const validateCode = () => {
      codeError.value = ''
      if (!code.value) {
        codeError.value = 'Ingresa el código'
      } else if (!/^\d{6}$/.test(code.value)) {
        codeError.value = 'Deben ser 6 números'
      }
      return !codeError.value
    }

    const handleVerify = async () => {
      if (!email.value) {
        authStore.error = 'Error: Falta el correo.'
        return
      }

      if (!validateCode()) return

      try {
        await authStore.verifyEmail(email.value, code.value)
        successMessage.value = '¡Verificado! Redirigiendo...'
        code.value = ''
        setTimeout(() => { router.push('/login') }, 2000)
      } catch (error) { console.error(error) }
    }

    const handleResend = async () => {
      if (!email.value) return
      isResending.value = true
      successMessage.value = ''
      try {
        await authStore.sendVerificationCode(email.value)
        successMessage.value = 'Código reenviado.'
      } catch (error) { console.error(error) } 
      finally { isResending.value = false }
    }

    onMounted(() => {
      if (!email.value) router.push('/register')
    })

    onUnmounted(() => authStore.clearError())

    return {
      email, code, codeError, successMessage, isResending, authStore,
      handleVerify, handleResend
    }
  }
}
</script>

<style scoped>
.verification-page {
  background-color: #ffffff;
  min-height: calc(100vh - 76px);
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 1rem;
}

.verification-card {
  width: 100%;
  max-width: 380px;
  padding: 2.5rem;
  background: #ffffff;
  box-shadow: 0 10px 40px -10px rgba(0,0,0,0.08);
  border: 1px solid #f1f5f9;
  border-radius: 1.5rem;
}

.form-control {
  background-color: #f8fafc;
  border: 1px solid #e2e8f0;
  color: #334155;
}

.form-control:focus {
  background-color: #fff;
  border-color: #3b82f6;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.btn-primary {
  background: #0f172a;
  border: none;
  transition: opacity 0.2s;
}

.btn-primary:hover {
  background: #1e293b;
}
</style>