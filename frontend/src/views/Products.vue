<template>
  <div class="products-page">
    <div class="container py-4">
      <div class="row g-4">
        
        <div class="col-lg-3">
          <div class="filter-card p-4 sticky-top" style="top: 120px;">
            <div class="d-flex align-items-center justify-content-between mb-4">
              <h6 class="fw-bold text-dark mb-0">Filtros</h6>
              <i class="fas fa-sliders-h text-muted"></i>
            </div>

            <div class="mb-4">
              <input 
                type="text" 
                v-model="searchTerm" 
                @input="applyFilters"
                placeholder="Buscar..." 
                class="form-control form-control-sm bg-light border-0"
              >
            </div>

            <div class="mb-4">
              <label class="small fw-bold text-muted mb-2">Categoría</label>
              <select v-model="selectedCategory" @change="applyFilters" class="form-select form-select-sm border-0 bg-light">
                <option value="">Todas</option>
                <option v-for="cat in categories" :key="cat.id" :value="cat.id">{{ cat.nombre }}</option>
              </select>
            </div>
            
            <div class="mb-4">
              <label class="small fw-bold text-muted mb-2">Precio</label>
              <div class="d-flex gap-2">
                <input type="number" v-model.number="priceRange.min" @input="applyFilters" placeholder="Min" class="form-control form-control-sm bg-light border-0">
                <input type="number" v-model.number="priceRange.max" @input="applyFilters" placeholder="Max" class="form-control form-control-sm bg-light border-0">
              </div>
            </div>
            
            <button @click="clearFilters" class="btn btn-outline-dark btn-sm w-100 rounded-pill">
              Limpiar
            </button>
          </div>
        </div>

        <div class="col-lg-9">
          <div class="d-flex justify-content-between align-items-center mb-4 pb-2 border-bottom">
            <span class="text-muted small">{{ filteredProducts.length }} productos</span>
            <select v-model="sortBy" class="form-select form-select-sm border-0 w-auto text-end fw-bold text-dark cursor-pointer">
              <option value="name">Nombre A-Z</option>
              <option value="price-asc">Precio: Menor a Mayor</option>
              <option value="price-desc">Precio: Mayor a Menor</option>
            </select>
          </div>
          
          <div v-if="isLoading" class="text-center py-5">
            <div class="spinner-border text-dark opacity-25"></div>
          </div>
          
          <div v-else-if="filteredProducts.length === 0" class="text-center py-5">
            <h5 class="text-dark">Sin resultados</h5>
            <p class="text-muted">Intenta con otros filtros</p>
          </div>

          <div v-else class="row g-4">
            <div class="col-md-6 col-xl-4" v-for="product in paginatedProducts" :key="product.id">
              <div class="card product-card h-100 border-0">
                <div class="position-relative">
                  <img :src="product.imagen || '/placeholder-product.jpg'" class="card-img-top" :alt="product.nombre">
                  <span v-if="product.stock_quantity <= 5" class="badge bg-warning text-dark position-absolute top-0 start-0 m-3 shadow-sm">
                    Poco stock
                  </span>
                </div>
                
                <div class="card-body d-flex flex-column p-4">
                  <div class="mb-2">
                    <small class="text-primary fw-bold text-uppercase" style="font-size: 0.7rem;">
                      {{ getCategoryName(product.categoria_id) }}
                    </small>
                  </div>
                  <h6 class="card-title fw-bold text-dark mb-2">{{ product.nombre }}</h6>
                  <p class="card-text text-muted small flex-grow-1 line-clamp-2">{{ product.descripcion }}</p>
                  
                  <div class="d-flex justify-content-between align-items-center mt-3">
                    <span class="h5 fw-bold text-dark mb-0">${{ parseFloat(product.precio).toFixed(2) }}</span>
                    <button 
                      v-if="isAuthenticated" 
                      @click="addToCart(product)"
                      class="btn btn-dark btn-sm rounded-circle shadow-sm"
                      style="width: 38px; height: 38px;"
                    >
                      <i class="fas fa-plus"></i>
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
          
          <nav v-if="!isLoading && totalPages > 1" class="mt-5 d-flex justify-content-center">
            <ul class="pagination pagination-sm">
              <li class="page-item" :class="{ disabled: currentPage === 1 }">
                <button class="page-link border-0 text-dark" @click="goToPage(currentPage - 1)"><i class="fas fa-chevron-left"></i></button>
              </li>
              <li class="page-item disabled"><span class="page-link border-0 bg-transparent text-muted">{{ currentPage }} / {{ totalPages }}</span></li>
              <li class="page-item" :class="{ disabled: currentPage === totalPages }">
                <button class="page-link border-0 text-dark" @click="goToPage(currentPage + 1)"><i class="fas fa-chevron-right"></i></button>
              </li>
            </ul>
          </nav>

        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, computed, onMounted, watch } from 'vue'
import { useAuthStore } from '../stores/auth'
import { useCartStore } from '../stores/cart'
import api from '../services/api'

export default {
  name: 'Products',
  setup() {
    const authStore = useAuthStore()
    const cartStore = useCartStore()
    const products = ref([])
    const categories = ref([])
    const stockLevels = ref(new Map())
    const isLoading = ref(false)
    const selectedCategory = ref('')
    const priceRange = ref({ min: null, max: null })
    const searchTerm = ref('')
    const sortBy = ref('name')
    const currentPage = ref(1)
    const itemsPerPage = ref(9)
    
    const filteredProducts = computed(() => products.value)
    const paginatedProducts = computed(() => products.value)
    const totalPages = computed(() => 1)
    
    const getCategoryName = () => 'General'
    const addToCart = () => {}
    const applyFilters = () => {}
    const clearFilters = () => {}
    const goToPage = () => {}

    onMounted(async () => {
       try { 
         products.value = await api.getProducts(); 
         const categoriesData = await api.getCategories();
         const uniqueCategories = categoriesData.filter((category, index, self) =>
           index === self.findIndex((c) => c.nombre.toLowerCase() === category.nombre.toLowerCase())
         );
         categories.value = uniqueCategories;
       } catch(e){}
    })

    return {
      products, categories, filteredProducts, paginatedProducts, isLoading,
      selectedCategory, priceRange, searchTerm, sortBy, currentPage, totalPages,
      authStore, getCategoryName, addToCart, applyFilters, clearFilters, goToPage,
      isAuthenticated: computed(() => authStore.isAuthenticated)
    }
  }
}
</script>

<style scoped>
.products-page {
  background-color: #ffffff;
  min-height: 100vh;
}

.filter-card {
  background: #ffffff;
  border-right: 1px solid #f1f5f9;
  z-index: 1;
}

.product-card {
  background: #ffffff;
  border-radius: 1rem;
  transition: all 0.3s ease;
  box-shadow: 0 2px 10px rgba(0,0,0,0.02);
  overflow: hidden;
}

.product-card:hover {
  transform: translateY(-5px);
  box-shadow: 0 15px 30px rgba(0,0,0,0.08);
}

.card-img-top {
  height: 220px;
  object-fit: cover;
}

.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.bg-light {
  background-color: #f8fafc !important;
}

.cursor-pointer {
  cursor: pointer;
}
</style>