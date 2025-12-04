<template>
  <div class="product-management">
    <div class="container-fluid">
      <div class="d-flex justify-content-between align-items-center mb-4">
        <h2>Gestión de Productos</h2>
        <div class="flex flex-row justify-content-end align-items-center gap-4">
          <button class="btn btn-primary mx-4" @click="showCreateModalCategory">
            <i class="fas fa-plus me-2"></i>
            Nueva Categoría
          </button>
          <button class="btn btn-primary mx-4" @click="showCreateModal">
            <i class="fas fa-plus me-2"></i>
            Nuevo Producto
          </button>
        </div>
        <button v-if="isAdmin" class="btn btn-outline-secondary ms-2" @click="showCategoriesModal">
          <i class="fas fa-folder-plus me-2"></i>
          Gestión de Categorías
        </button>
      </div>
      
      <!-- Filters -->
      <div class="card mb-4">
        <div class="card-body">
          <div class="row">
            <div class="col-md-3">
              <select v-model="filters.category" @change="applyFilters" class="form-select">
                <option value="">Todas las categorías</option>
                <option v-for="category in categories" :key="category.id" :value="category.id">
                  {{ category.nombre }}
                </option>
              </select>
            </div>
            <div class="col-md-3">
              <select v-model="filters.status" @change="applyFilters" class="form-select">
                <option value="">Todos los estados</option>
                <option value="activo">Activo</option>
                <option value="inactivo">Inactivo</option>
              </select>
            </div>
            <div class="col-md-4">
              <input 
                type="text" 
                v-model="filters.search" 
                @input="applyFilters"
                placeholder="Buscar productos..." 
                class="form-control"
              >
            </div>
            <div class="col-md-2">
              <button @click="clearFilters" class="btn btn-outline-secondary w-100">
                Limpiar
              </button>
            </div>
          </div>
        </div>
      </div>
      
      <!-- Products Table -->
      <div class="card">
        <div class="card-body">
          <!-- Loading State -->
          <div v-if="isLoading" class="text-center py-4">
            <div class="spinner-border" role="status">
              <span class="visually-hidden">Cargando...</span>
            </div>
          </div>
          
          <!-- Error State -->
          <div v-else-if="error" class="alert alert-danger">
            {{ error }}
          </div>
          
          <!-- Products Table -->
          <div v-else class="table-responsive">
            <table class="table table-hover">
              <thead>
                <tr>
                  <th>Código</th>
                  <th>Imagen</th>
                  <th>Nombre</th>
                  <th>Categoría</th>
                  <th>Precio</th>
                  <th>Estado</th>
                  <th>Acciones</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="product in filteredProducts" :key="product.id">
                  <td>{{ product.codigo }}</td>
                  <td>
                    <img 
                      :src="product.imagen || '/placeholder-product.jpg'" 
                      :alt="product.nombre"
                      style="width: 50px; height: 50px; object-fit: cover; border-radius: 4px;"
                    >
                  </td>
                  <td>
                    <div class="fw-medium">{{ product.nombre }}</div>
                    <small class="text-muted">{{ truncateText(product.descripcion, 50) }}</small>
                  </td>
                  <td>{{ getCategoryName(product.categoria_id) }}</td>
                  <td>${{ parseFloat(product.precio).toFixed(2) }}</td>
                  <td>
                    <span 
                      class="badge"
                      :class="product.estado === 'activo' ? 'bg-success' : 'bg-secondary'"
                    >
                      {{ product.estado }}
                    </span>
                  </td>
                  <td>
                    <div class="btn-group btn-group-sm">
                      <button 
                        class="btn btn-outline-primary"
                        @click="editProduct(product)"
                        title="Editar"
                      >
                        <i class="fas fa-edit"></i>
                      </button>
                      <button 
                        class="btn btn-outline-danger"
                        @click="deleteProduct(product.id)"
                        title="Eliminar"
                      >
                        <i class="fas fa-trash"></i>
                      </button>
                    </div>
                  </td>
                </tr>
              </tbody>
            </table>
            
            <!-- Empty State -->
            <div v-if="filteredProducts.length === 0" class="text-center py-4">
              <i class="fas fa-box-open fa-3x text-muted mb-3"></i>
              <h5>No se encontraron productos</h5>
              <p class="text-muted">Ajusta los filtros o crea un nuevo producto</p>
            </div>
          </div>
        </div>
      </div>
    </div>
    
    <!-- Product Modal -->
    <div class="modal fade" id="productModal" tabindex="-1">
      <div class="modal-dialog modal-lg">
        <div class="modal-content">
          <div class="modal-header">
            <h5 class="modal-title">
              {{ isEditing ? 'Editar Producto' : 'Nuevo Producto' }}
            </h5>
            <button type="button" class="btn-close" data-bs-dismiss="modal"></button>
          </div>
          <form @submit.prevent="saveProduct">
            <div class="modal-body">
              <div class="row">
                <div class="col-md-6">
                  <div class="mb-3">
                    <label class="form-label">Nombre *</label>
                    <input 
                      type="text" 
                      v-model="productForm.nombre"
                      class="form-control"
                      required
                    >
                  </div>
                </div>
                <div class="col-md-6">
                  <div class="mb-3">
                    <label class="form-label">Categoría *</label>
                    <select v-model="productForm.categoria_id" class="form-select" required>
                      <option value="">Seleccionar categoría</option>
                      <option v-for="category in categories" :key="category.id" :value="category.id">
                        {{ category.nombre }}
                      </option>
                    </select>
                  </div>
                </div>
              </div>
              
              <div class="mb-3">
                <label class="form-label">Descripción</label>
                <textarea 
                  v-model="productForm.descripcion"
                  class="form-control"
                  rows="3"
                ></textarea>
              </div>
              
              <div class="row">
                <div class="col-md-6">
                  <div class="mb-3">
                    <label class="form-label">Precio *</label>
                    <input 
                      type="number" 
                      step="0.01"
                      min="0"
                      v-model.number="productForm.precio"
                      @keydown="bloquearSignos"
                      class="form-control"
                      required
                    >
                  </div>
                </div>
                <div class="col-md-6">
                  <div class="mb-3">
                    <label class="form-label">Estado</label>
                    <select v-model="productForm.estado" class="form-select">
                      <option value="activo">Activo</option>
                      <option value="inactivo">Inactivo</option>
                    </select>
                  </div>
                </div>
              </div>
              
              <div class="mb-3">
                <label class="form-label">URL de Imagen</label>
                <input 
                  type="url" 
                  v-model="productForm.imagen"
                  class="form-control"
                  placeholder="https://ejemplo.com/imagen.jpg"
                >
              </div>
              <div class="mb-3">
                <label class="form-label">Subir imagen</label>
                <input
                  type="file"
                  accept="image/*"
                  @change="onImageFileChange"
                  class="form-control"
                  ref="fileInput"
                >
                <div v-if="imagePreviewUrl || productForm.imagen" class="mt-2">
                  <img :src="imagePreviewUrl || productForm.imagen" alt="Preview" style="max-width:200px; object-fit:cover; border-radius:4px;">
                </div>
              </div>
              
              <div v-if="formError" class="alert alert-danger">
                {{ formError }}
              </div>
            </div>
            <div class="modal-footer">
              <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">
                Cancelar
              </button>
              <button type="submit" class="btn btn-primary" :disabled="isSaving">
                <span v-if="isSaving" class="spinner-border spinner-border-sm me-2"></span>
                {{ isSaving ? 'Guardando...' : 'Guardar' }}
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
    <div class="modal fade" id="categoryModal" tabindex="-1">
      <div class="modal-dialog">
        <div class="modal-content">
          <div class="modal-header">
            <h5 class="modal-title">Nueva Categoría</h5>
            <button type="button" class="btn-close" data-bs-dismiss="modal"></button>
          </div>
          <form @submit.prevent="saveCategory">
            <div class="modal-body">
              <div class="mb-3">
                <label class="form-label">Nombre *</label>
                <input 
                  type="text" 
                  v-model="categoryForm.nombre"
                  class="form-control"
                  required
                >
              </div>
              
              <div class="mb-3">
                <label class="form-label">Descripción</label>
                <textarea 
                  v-model="categoryForm.descripcion"
                  class="form-control"
                  rows="3"
                ></textarea>
              </div>
              
              <div v-if="categoryFormError" class="alert alert-danger">
                {{ categoryFormError }}
              </div>
            </div>
            <div class="modal-footer">
              <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">
                Cancelar
              </button>
              <button type="submit" class="btn btn-primary" :disabled="isSavingCategory">
                <span v-if="isSavingCategory" class="spinner-border spinner-border-sm me-2"></span>
                {{ isSavingCategory ? 'Guardando...' : 'Guardar' }}
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
  </div>
</template>

<script>
import { ref, reactive, computed, onMounted } from 'vue'
import * as bootstrap from 'bootstrap'
import { productService } from '../services/api'
import { useAuthStore } from '../stores/auth'

export default {
  name: 'ProductManagement',
  setup() {
    const products = ref([])
    const categories = ref([])
    const isLoading = ref(false)
    const isSaving = ref(false)
    const error = ref(null)
    const fileInput = ref(null)
    const formError = ref(null)
    const isEditing = ref(false)
    const authStore = useAuthStore()
    
    const filters = reactive({
      category: '',
      status: '',
      search: ''
    })
    
    const productForm = reactive({
      id: null,
      nombre: '',
      descripcion: '',
      precio: 0,
      categoria_id: '',
      imagen: '',
      imagenFile: null,
      estado: 'activo'
    })
    const categoryForm = reactive({
      nombre: '',
      descripcion: ''
    })
    const isSavingCategory = ref(false)
    const categoryFormError = ref(null)

    // Categories admin form
    const imagePreviewUrl = ref('')
    
    const filteredProducts = computed(() => {
      let filtered = products.value
      
      if (filters.category) {
        filtered = filtered.filter(p => p.categoria_id == filters.category)
      }
      
      if (filters.status) {
        filtered = filtered.filter(p => p.estado === filters.status)
      }
      
      if (filters.search) {
        const term = filters.search.toLowerCase()
        filtered = filtered.filter(p =>
          p.nombre.toLowerCase().includes(term) ||
          p.descripcion.toLowerCase().includes(term) ||
          p.codigo.toLowerCase().includes(term.toLowerCase())
        )
      }
      
      return filtered
    })
    
    const fetchProducts = async () => {
      isLoading.value = true
      error.value = null
      try {
        products.value = await productService.getProducts()
      } catch (err) {
        error.value = 'Error al cargar productos'
        console.error('Error fetching products:', err)
      } finally {
        isLoading.value = false
      }
    }
    
    const fetchCategories = async () => {
      try {
        categories.value = await productService.getCategories()
      } catch (err) {
        console.error('Error fetching categories:', err)
      }
    }
    
    const getCategoryName = (categoryId) => {
      const category = categories.value.find(cat => cat.id === categoryId)
      return category ? category.nombre : 'Sin categoría'
    }
    
    const truncateText = (text, length) => {
      if (!text) return ''
      return text.length > length ? text.substring(0, length) + '...' : text
    }
    
    const showCreateModal = () => {
      isEditing.value = false
      resetForm()
      const modal = new bootstrap.Modal(document.getElementById('productModal'))
      modal.show()
    }
    const resetCategoryForm = () => {
      categoryForm.nombre = ''
      categoryForm.descripcion = ''
      categoryFormError.value = null
    }

    const showCreateModalCategory = () => {
      resetCategoryForm()
      const modal = new bootstrap.Modal(document.getElementById('categoryModal'))
      modal.show()
    }

    const saveCategory = async () => {
      isSavingCategory.value = true
      categoryFormError.value = null
      
      try {
        const data = {
          nombre: categoryForm.nombre,
          descripcion: categoryForm.descripcion
        }
        await productService.createCategory(data)
        await fetchCategories()
        const modal = bootstrap.Modal.getInstance(document.getElementById('categoryModal'))
        modal.hide()
      } catch (err) {
        categoryFormError.value = err.response?.data?.message || 'Error al crear categoría'
        console.error('Error saving category:', err)
      } finally {
        isSavingCategory.value = false
      }
    }
    
    const editProduct = (product) => {
      if (imagePreviewUrl.value) {
        try { URL.revokeObjectURL(imagePreviewUrl.value) } catch (e) {}
        imagePreviewUrl.value = ''
      }
      productForm.imagenFile = null
      if (fileInput.value) {
        fileInput.value.value = ''
      }
      isEditing.value = true
      productForm.id = product.id
      productForm.nombre = product.nombre
      productForm.descripcion = product.descripcion || ''
      productForm.precio = parseFloat(product.precio)
      productForm.categoria_id = product.categoria_id
      productForm.imagen = product.imagen || ''
      productForm.imagenFile = product.imagenFile || null
      productForm.estado = product.estado
      
      const modal = new bootstrap.Modal(document.getElementById('productModal'))
      modal.show()
    }

    const onImageFileChange = (event) => {
      const file = event.target.files?.[0] || null
      if (file) {
        // Revoke old preview URL if present
        if (imagePreviewUrl.value) {
          try { URL.revokeObjectURL(imagePreviewUrl.value) } catch (e) {}
        }
        productForm.imagenFile = file
        imagePreviewUrl.value = URL.createObjectURL(file)
      } else {
        productForm.imagenFile = null
        imagePreviewUrl.value = ''
      }
    }
    
    const saveProduct = async () => {
      isSaving.value = true
      formError.value = null
      
      try {
        let data
        if (productForm.imagenFile || isEditing.value) {
          data = new FormData()
          data.append('nombre', productForm.nombre)
          data.append('descripcion', productForm.descripcion)
          data.append('precio', productForm.precio)
          data.append('categoria_id', productForm.categoria_id)
          data.append('estado', productForm.estado)
          if (productForm.imagenFile) {
            data.append('imagen', productForm.imagenFile)
          } else {
            data.append('imagen', productForm.imagen || '') 
          }
          if (isEditing.value) {
            data.append('_method', 'PUT')
          }
        } else {
          data = {
            nombre: productForm.nombre,
            descripcion: productForm.descripcion,
            precio: productForm.precio,
            categoria_id: productForm.categoria_id,
            imagen: productForm.imagen,
            estado: productForm.estado
          }
        }
        
        if (isEditing.value) {
          await productService.updateProduct(productForm.id, data)
        } else {
          await productService.createProduct(data)
        }
        
        await fetchProducts()
        resetForm()
        const modal = bootstrap.Modal.getInstance(document.getElementById('productModal'))
        modal.hide()
      } catch (err) {
        formError.value = err.response?.data?.message || 'Error al guardar producto'
        console.error('Error saving product:', err)
      } finally {
        isSaving.value = false
      }
    }
    
    const deleteProduct = async (productId) => {
      if (confirm('¿Estás seguro de que deseas eliminar este producto?')) {
        try {
          await productService.deleteProduct(productId)
          await fetchProducts()
        } catch (err) {
          error.value = 'Error al eliminar producto'
          console.error('Error deleting product:', err)
        }
      }
    }
    
    const resetForm = () => {
      productForm.id = null
      productForm.nombre = ''
      productForm.descripcion = ''
      productForm.precio = 0
      productForm.categoria_id = ''
      productForm.imagen = ''

      if (imagePreviewUrl.value) {
        try { URL.revokeObjectURL(imagePreviewUrl.value) } catch (e) {}
      }
      productForm.imagenFile = null
      imagePreviewUrl.value = ''
      if (fileInput.value) {
        fileInput.value.value = '' 
      }
      productForm.estado = 'activo'
      formError.value = null
    }
    
    const applyFilters = () => {
      // Filters are applied automatically via computed property
    }
    
    const clearFilters = () => {
      filters.category = ''
      filters.status = ''
      filters.search = ''
    }

    const bloquearSignos = (e) => {
      if (['-', '+', 'e', 'E'].includes(e.key)) {
        e.preventDefault();
      }
    };
    
    onMounted(async () => {
      await Promise.all([
        fetchProducts(),
        fetchCategories()
      ])
    })
    
    return {
      products,
      categories,
      filteredProducts,
      isLoading,
      isSaving,
      error,
      formError,
      isEditing,
      filters,
      productForm,
      imagePreviewUrl,
      getCategoryName,
      truncateText,
      showCreateModal,
      editProduct,
      saveProduct,
      onImageFileChange,
      deleteProduct,
      applyFilters,
      clearFilters,
      categoryForm,
      isSavingCategory,
      categoryFormError,
      showCreateModalCategory,
      saveCategory,
      fileInput,
      bloquearSignos
    }
  }
}
</script>

<style scoped>
.table th {
  border-top: none;
  font-weight: 600;
}

.btn-group-sm > .btn {
  padding: 0.25rem 0.5rem;
}

.modal-body {
  max-height: 80vh;
  overflow-y: auto;
}
</style>
