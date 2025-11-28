<template>
  <div class="container">
    <div class="row justify-content-center">
      <div class="col-md-6 col-lg-4">
        <div class="card shadow">
          <div class="card-header text-center">
            <h4>Verificar Correo Electrónico</h4>
          </div>
          <div class="card-body">
            <p class="mb-3">
              Hemos enviado un código de verificación de 6 dígitos a:
              <strong>{{ email }}</strong>
            </p>

            <form @submit.prevent="handleVerify">
              <div class="mb-3">
                <label for="code" class="form-label">Código de verificación</label>
                <input
                  id="code"
                  type="text"
                  class="form-control"
                  v-model="code"
                  maxlength="6"
                  autocomplete="one-time-code"
                  :class="{ 'is-invalid': codeError }"
                  required
                >
                <div v-if="codeError" class="invalid-feedback">
                  {{ codeError }}
                </div>
              </div>

              <div v-if="authStore.error" class="alert alert-danger">
                {{ authStore.error }}
              </div>

              <div v-if="successMessage" class="alert alert-success">
                {{ successMessage }}
              </div>

              <button
                type="submit"
                class="btn btn-primary w-100 mb-2"
                :disabled="authStore.isLoading"
              >
                <span v-if="authStore.isLoading" class="spinner-border spinner-border-sm me-2"></span>
                {{ authStore.isLoading ? 'Verificando...' : 'Verificar correo' }}
              </button>
            </form>

            <button
              type="button"
              class="btn btn-link w-100"
              :disabled="authStore.isLoading || isResending"
              @click="handleResend"
            >
              <span v-if="isResending" class="spinner-border spinner-border-sm me-2"></span>
              Reenviar código
            </button>

            <div class="text-center mt-3">
              <router-link to="/login" class="text-decoration-none">
                Volver al inicio de sesión
              </router-link>
            </div>
          </div>
        </div>
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
        codeError.value = 'El código es requerido'
      } else if (!/^\d{6}$/.test(code.value)) {
        codeError.value = 'El código debe tener 6 dígitos numéricos'
      }
      return !codeError.value
    }

    const handleVerify = async () => {
      if (!email.value) {
        authStore.error = 'Falta el correo a verificar. Vuelve a registrarte.'
        return
      }

      if (!validateCode()) return

      try {
        await authStore.verifyEmail(email.value, code.value)
        successMessage.value = 'Correo verificado correctamente. Ahora puedes iniciar sesión.'
        code.value = ''

        setTimeout(() => {
          router.push('/login')
        }, 2000)
      } catch (error) {
        console.error('Email verification error:', error)
      }
    }

    const handleResend = async () => {
      if (!email.value) {
        authStore.error = 'Falta el correo a verificar. Vuelve a registrarte.'
        return
      }
      isResending.value = true
      successMessage.value = ''
      try {
        await authStore.sendVerificationCode(email.value)
        successMessage.value = 'Se ha reenviado el código de verificación a tu correo.'
      } catch (error) {
        console.error('Resend code error:', error)
      } finally {
        isResending.value = false
      }
    }

    onMounted(() => {
      if (!email.value) {
        // Si se accede directamente sin email, redirigimos al registro
        router.push('/register')
      }
    })

    onUnmounted(() => {
      authStore.clearError()
    })

    return {
      email,
      code,
      codeError,
      successMessage,
      isResending,
      authStore,
      handleVerify,
      handleResend
    }
  }
}
</script>

<style scoped>
.card {
  border: none;
  border-radius: 10px;
}

.card-header {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border-radius: 10px 10px 0 0 !important;
}

.btn-primary {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border: none;
}

.btn-primary:hover {
  background: linear-gradient(135deg, #5a67c7 0%, #684a8f 100%);
}
</style>