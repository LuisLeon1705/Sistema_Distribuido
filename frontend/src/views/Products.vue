<template>
  <div class="products">
    <div class="container">
      <div class="row">
        <!-- Filters Sidebar -->
        <div class="col-md-3 mb-4">
          <div class="card">
            <div class="card-header">
              <h5 class="mb-0">Filtros</h5>
            </div>
            <div class="card-body">
              <!-- Category Filter -->
              <div class="mb-3">
                <label class="form-label">Categorías</label>
                <select v-model="selectedCategory" @change="applyFilters" class="form-select">
                  <option value="">Todas las categorías</option>
                  <option v-for="category in categories" :key="category.id" :value="category.id">
                    {{ category.nombre }}
                  </option>
                </select>
              </div>
              
              <!-- Price Range -->
              <div class="mb-3">
                <label class="form-label">Rango de Precios</label>
                <div class="row">
                  <div class="col-6">
                    <input 
                      type="number" 
                      v-model.number="priceRange.min" 
                      @input="applyFilters"
                      @keydown="bloquearSignos"
                      placeholder="Min" 
                      min="0"
                      max="99999999"
                      class="form-control form-control-sm"
                    >
                  </div>
                  <div class="col-6">
                    <input 
                      type="number" 
                      v-model.number="priceRange.max" 
                      @input="applyFilters"
                      @keydown="bloquearSignos"
                      placeholder="Max" 
                      min="0"
                      max="99999999"
                      class="form-control form-control-sm"
                    >
                  </div>
                </div>
              </div>
              
              <!-- Search -->
              <div class="mb-3">
                <label class="form-label">Buscar</label>
                <input 
                  type="text" 
                  v-model="searchTerm" 
                  @input="applyFilters"
                  placeholder="Nombre del producto..." 
                  class="form-control"
                >
              </div>
              
              <button @click="clearFilters" class="btn btn-outline-secondary btn-sm w-100">
                Limpiar Filtros
              </button>
            </div>
          </div>
        </div>

        <!-- Products Grid -->
        <div class="col-md-9">
          <div class="d-flex justify-content-between align-items-center mb-4">
            <h2>Productos</h2>
            <div class="d-flex align-items-center">
              <span class="text-muted me-3">{{ filteredProducts.length }} productos</span>
              <select v-model="sortBy" @change="applySorting" class="form-select" style="width: auto;">
                <option value="name">Ordenar por nombre</option>
                <option value="price-asc">Precio: menor a mayor</option>
                <option value="price-desc">Precio: mayor a menor</option>
              </select>
            </div>
          </div>
          
          <!-- Loading State -->
          <div v-if="isLoading" class="text-center py-5">
            <div class="spinner-border" role="status">
              <span class="visually-hidden">Cargando...</span>
            </div>
            <p class="mt-2">Cargando productos...</p>
          </div>
          
          <!-- Error State -->
          <div v-else-if="error" class="alert alert-danger">
            {{ error }}
          </div>
          
          <!-- Products Grid -->
          <div v-else class="row">
            <div class="col-md-4 mb-4" v-for="product in displayedProducts" :key="product.id">
              <div class="card h-100 product-card">
                <div class="product-image-container">
                  <img 
                    :src="product.imagen || '/placeholder-product.jpg'" 
                    class="card-img-top" 
                    :alt="product.nombre"
                    style="height: 250px; object-fit: cover;"
                  >
                  <div v-if="product.estado === 'inactivo'" class="product-badge bg-secondary">
                    No disponible
                  </div>
                </div>
                <div class="card-body d-flex flex-column">
                  <h5 class="card-title">{{ product.nombre }}</h5>
                  <p class="card-text text-muted small mb-2">
                    {{ getCategoryName(product.categoria_id) }}
                  </p>
                  <p class="card-text flex-grow-1">{{ product.descripcion }}</p>
                  <div class="mt-auto">
                    <div class="d-flex justify-content-between align-items-center">
                      <span class="h5 text-primary mb-0">
                        ${{ parseFloat(product.precio).toFixed(2) }}
                      </span>
                      <div v-if="product.stock_quantity > 0">
                        <button 
                          v-if="isAuthenticated && product.estado === 'activo'" 
                          class="btn btn-primary btn-sm"
                          @click="addToCart(product)"
                          :disabled="cartStore.isLoading"
                        >
                          <i class="fas fa-cart-plus me-1"></i>
                          Agregar
                        </button>
                        <button 
                          v-else-if="!isAuthenticated" 
                          class="btn btn-outline-primary btn-sm"
                          @click="$router.push('/login')"
                        >
                          Inicia sesión
                        </button>
                      </div>
                      <div v-else>
                          <span class="text-danger fw-bold">Fuera de stock</span>
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>
          </div>
          
          <!-- Empty State -->
          <div v-if="!isLoading && filteredProducts.length === 0" class="text-center py-5">
            <i class="fas fa-search fa-3x text-muted mb-3"></i>
            <h4>No se encontraron productos</h4>
            <p class="text-muted">Intenta ajustar los filtros de búsqueda</p>
          </div>
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
    const error = ref(null)
    
    // Filters
    const selectedCategory = ref('')
    const priceRange = ref({ min: null, max: null })
    const searchTerm = ref('')
    const sortBy = ref('name')
    
    const isAuthenticated = computed(() => authStore.isAuthenticated)

    const productsWithStock = computed(() => {
      return products.value.map(product => {
        const stock = stockLevels.value.get(product.id) || { quantity: 0 };
        return {
          ...product,
          stock_quantity: stock.quantity,
        };
      });
    });
    
    const bloquearSignos = (e) => {
      if (['-', '+', 'e', 'E'].includes(e.key)) {
        e.preventDefault();
      }
    };
    const filteredProducts = computed(() => {
      let filtered = productsWithStock.value
      
      // Filter by category
      if (selectedCategory.value) {
        filtered = filtered.filter(product => product.categoria_id == selectedCategory.value)
      }
      
      // Filter by price range
      if (priceRange.value.min !== null) {
        filtered = filtered.filter(product => parseFloat(product.precio) >= priceRange.value.min)
      }
      if (priceRange.value.max !== null) {
        filtered = filtered.filter(product => parseFloat(product.precio) <= priceRange.value.max)
      }

      if (priceRange.value.max !== null && (priceRange.value.min === null || priceRange.value.min == "")) {
        filtered = filtered.filter(product => parseFloat(product.precio) <= priceRange.value.max)
      }
      if (priceRange.value.min !== null && (priceRange.value.max === null || priceRange.value.max == "")) {
        filtered = filtered.filter(product => parseFloat(product.precio) >= priceRange.value.min)
      }
      if (priceRange.value.min == "" && priceRange.value.max == "") {
        filtered = productsWithStock.value
      }
      // Filter by search term
      if (searchTerm.value) {
        const term = searchTerm.value.toLowerCase()
        filtered = filtered.filter(product =>
          product.nombre.toLowerCase().includes(term) ||
          product.descripcion.toLowerCase().includes(term)
        )
      }
      
      return filtered
    })
    
    const displayedProducts = computed(() => {
      let sorted = [...filteredProducts.value]
      
      switch (sortBy.value) {
        case 'price-asc':
          sorted.sort((a, b) => parseFloat(a.precio) - parseFloat(b.precio))
          break
        case 'price-desc':
          sorted.sort((a, b) => parseFloat(b.precio) - parseFloat(a.precio))
          break
        case 'name':
        default:
          sorted.sort((a, b) => a.nombre.localeCompare(b.nombre))
          break
      }
      
      return sorted
    })
    
    const fetchProducts = async () => {
      isLoading.value = true
      error.value = null
      try {
        products.value = await api.getProducts()
      } catch (err) {
        error.value = 'Error al cargar los productos'
        console.error('Error fetching products:', err)
      } finally {
        isLoading.value = false
      }
    }
    
    const fetchCategories = async () => {
      try {
        categories.value = await api.getCategories()
      } catch (err) {
        console.error('Error fetching categories:', err)
      }
    }

    const fetchStockLevels = async () => {
      try {
        const stockData = await api.getStock(); // Assuming getStock() fetches all stock
        const stockMap = new Map();
        stockData.data.forEach(item => {
            stockMap.set(item.product_id, item);
        });
        stockLevels.value = stockMap;
      } catch (err) {
        console.error('Error fetching stock levels:', err);
      }
    };
    
    const getCategoryName = (categoryId) => {
      const category = categories.value.find(cat => cat.id === categoryId)
      return category ? category.nombre : 'Sin categoría'
    }
    
    const addToCart = (product) => {
      cartStore.addToCart(product)
    }
    
    const applyFilters = () => {
      // This will trigger the computed properties to update
    }
    
    const applySorting = () => {
      // This will trigger the computed properties to update
    }
    
    const clearFilters = () => {
      selectedCategory.value = ''
      priceRange.value = { min: null, max: null }
      searchTerm.value = ''
      sortBy.value = 'name'
    }
    
    onMounted(async () => {
      await Promise.all([
        fetchProducts(),
        fetchCategories(),
        fetchStockLevels()
      ])
    })
    
    return {
      products,
      categories,
      filteredProducts,
      displayedProducts,
      isLoading,
      error,
      selectedCategory,
      priceRange,
      searchTerm,
      sortBy,
      isAuthenticated,
      cartStore,
      getCategoryName,
      addToCart,
      applyFilters,
      applySorting,
      clearFilters,
      bloquearSignos
    }
  }
}
</script>

<style scoped>
.product-card {
  transition: transform 0.3s ease, box-shadow 0.3s ease;
  border: none;
  box-shadow: 0 2px 10px rgba(0,0,0,0.1);
}

.product-card:hover {
  transform: translateY(-5px);
  box-shadow: 0 8px 25px rgba(0,0,0,0.15);
}

.product-image-container {
  position: relative;
  overflow: hidden;
}

.product-badge {
  position: absolute;
  top: 10px;
  right: 10px;
  padding: 5px 10px;
  border-radius: 15px;
  font-size: 0.8em;
  font-weight: bold;
  color: white;
}

.card-img-top {
  transition: transform 0.3s ease;
}

.product-card:hover .card-img-top {
  transform: scale(1.1);
}

.form-select {
  cursor: pointer;
}

.btn-primary {
  transition: all 0.3s ease;
}

.btn-primary:hover {
  transform: scale(1.05);
}
</style>
