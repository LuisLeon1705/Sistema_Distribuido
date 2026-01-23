<template>
  <div class="product-management-page">
    <div class="container py-5">
      
      <div class="d-flex justify-content-between align-items-center mb-5">
        <div>
          <h2 class="fw-bold text-dark mb-1">Inventario</h2>
          <p class="text-muted small mb-0">Gestión de productos y categorías</p>
        </div>
        <div class="d-flex gap-2">
           <!-- <button class="btn btn-outline-dark btn-sm rounded-pill px-3" @click="handleGenerateSampleData" :disabled="isSeeding">
            <span v-if="isSeeding" class="spinner-border spinner-border-sm me-1"></span>
            <i v-else class="fas fa-magic me-2"></i>Seed Data
          </button> -->
          <button class="btn btn-outline-dark btn-sm rounded-pill px-3" @click="showCategoryModal">
            <i class="fas fa-tags me-2"></i>Categoría
          </button>
          <button class="btn btn-dark btn-sm rounded-pill px-4 shadow-sm" @click="showCreateModal">
            <i class="fas fa-plus me-2"></i>Registrar Producto
          </button>
        </div>
      </div>
      
      <div class="card border-0 shadow-sm rounded-4 mb-4 p-3 bg-white">
        <div class="row g-3">
          <div class="col-md-3">
            <div class="input-group input-group-sm">
               <span class="input-group-text bg-light border-0 text-muted"><i class="fas fa-folder"></i></span>
               <select v-model="filters.category" class="form-select bg-light border-0 text-dark">
                  <option value="">Todas las categorías</option>
                  <option v-for="cat in categories" :key="cat.id" :value="cat.id">{{ cat.nombre }}</option>
               </select>
            </div>
          </div>
          <div class="col-md-3">
             <div class="input-group input-group-sm">
               <span class="input-group-text bg-light border-0 text-muted"><i class="fas fa-toggle-on"></i></span>
               <select v-model="filters.status" class="form-select bg-light border-0 text-dark">
                  <option value="">Todos los estados</option>
                  <option value="activo">Activo</option>
                  <option value="inactivo">Inactivo</option>
               </select>
            </div>
          </div>
          <div class="col-md-4">
             <div class="input-group input-group-sm">
               <span class="input-group-text bg-light border-0 text-muted"><i class="fas fa-search"></i></span>
               <input type="text" v-model="filters.search" placeholder="Buscar por nombre..." class="form-control bg-light border-0 text-dark">
            </div>
          </div>
          <div class="col-md-2">
            <button @click="clearFilters" class="btn btn-link text-muted text-decoration-none small w-100 hover-text-dark">
              Limpiar filtros
            </button>
          </div>
        </div>
      </div>
      
      <div class="card border-0 shadow-sm rounded-4 overflow-hidden">
        <div class="card-body p-0">
          
          <div v-if="isLoading" class="text-center py-5">
            <div class="spinner-border text-primary opacity-50"></div>
            <p class="text-muted mt-2 small">Cargando inventario...</p>
          </div>

          <div v-else-if="error" class="alert alert-danger m-3 border-0 bg-danger bg-opacity-10 text-danger">
             <i class="fas fa-exclamation-triangle me-2"></i>{{ error }}
          </div>

          <div v-else class="table-responsive">
            <table class="table table-hover align-middle mb-0">
              <thead class="bg-light">
                <tr>
                  <th class="ps-4 py-3 text-muted small text-uppercase fw-bold border-0">Producto</th>
                  <th class="py-3 text-muted small text-uppercase fw-bold border-0">Categoría</th>
                  <th class="py-3 text-muted small text-uppercase fw-bold border-0">Precio</th>
                  <th class="py-3 text-muted small text-uppercase fw-bold border-0">Estado</th>
                  <th class="pe-4 py-3 text-muted small text-uppercase fw-bold border-0 text-end">Acciones</th>
                </tr>
              </thead>
              <tbody>
                <tr v-for="product in paginatedProducts" :key="product.id">
                  <td class="ps-4 py-3">
                    <div class="d-flex align-items-center">
                      <img 
                        :src="product.imagen || '/placeholder-product.jpg'" 
                        :alt="product.nombre" 
                        class="rounded-3 border bg-white me-3"
                        style="width: 48px; height: 48px; object-fit: cover;"
                      >
                      <div>
                        <div class="fw-bold text-dark">{{ product.nombre }}</div>
                        <small class="text-muted d-block" style="font-size: 0.75rem;">{{ product.codigo || 'N/A' }}</small>
                      </div>
                    </div>
                  </td>
                  <td>
                    <span class="badge bg-light text-dark border fw-normal">{{ getCategoryName(product.categoria_id) }}</span>
                  </td>
                  <td class="fw-bold text-dark">${{ parseFloat(product.precio).toFixed(2) }}</td>
                  <td>
                    <span class="badge rounded-pill fw-normal px-2" 
                          :class="product.estado === 'activo' ? 'bg-success bg-opacity-10 text-success border border-success border-opacity-25' : 'bg-secondary bg-opacity-10 text-secondary border border-secondary border-opacity-25'">
                      {{ product.estado === 'activo' ? 'Activo' : 'Inactivo' }}
                    </span>
                  </td>
                  <td class="pe-4 text-end">
                    <div class="btn-group">
                      <button class="btn btn-light btn-sm text-muted hover-bg-gray" @click="viewStock(product)" title="Stock">
                        <i class="fas fa-boxes"></i>
                      </button>
                      <button class="btn btn-light btn-sm text-primary hover-bg-primary-soft" @click="editProduct(product)" title="Editar">
                        <i class="fas fa-pen"></i>
                      </button>
                      <button class="btn btn-light btn-sm text-danger hover-bg-danger-soft" @click="deleteProduct(product.id)" title="Eliminar">
                        <i class="fas fa-trash"></i>
                      </button>
                    </div>
                  </td>
                </tr>
              </tbody>
            </table>
            
            <div v-if="filteredProducts.length === 0" class="text-center py-5">
               <div class="mb-3 opacity-25">
                 <i class="fas fa-box-open fa-3x text-muted"></i>
               </div>
               <h6 class="text-dark">Inventario vacío</h6>
               <p class="text-muted small">No hay productos que coincidan con la búsqueda.</p>
            </div>
          </div>
          
          <div v-if="!isLoading && filteredProducts.length > 0 && totalPages > 1" class="d-flex justify-content-between align-items-center p-4 border-top">
             <small class="text-muted">
               Mostrando {{ (currentPage - 1) * itemsPerPage + 1 }} - {{ Math.min(currentPage * itemsPerPage, filteredProducts.length) }} de {{ filteredProducts.length }}
             </small>
             <nav aria-label="Pagination">
              <ul class="pagination pagination-sm mb-0">
                <li class="page-item" :class="{ disabled: currentPage === 1 }">
                  <button class="page-link border-0 text-dark bg-transparent" @click="goToPage(currentPage - 1)">
                    <i class="fas fa-chevron-left"></i>
                  </button>
                </li>
                <li v-for="page in totalPages" :key="page" class="page-item" :class="{ active: currentPage === page }">
                  <button class="page-link border-0 rounded-circle mx-1" :class="currentPage === page ? 'bg-dark text-white' : 'text-dark bg-transparent'" @click="goToPage(page)">
                    {{ page }}
                  </button>
                </li>
                <li class="page-item" :class="{ disabled: currentPage === totalPages }">
                  <button class="page-link border-0 text-dark bg-transparent" @click="goToPage(currentPage + 1)">
                    <i class="fas fa-chevron-right"></i>
                  </button>
                </li>
              </ul>
            </nav>
          </div>

        </div>
      </div>
    </div>
    
    <div class="modal fade" id="productModal" tabindex="-1">
      <div class="modal-dialog modal-dialog-centered modal-lg">
        <div class="modal-content border-0 shadow-lg rounded-4">
          <div class="modal-header border-bottom px-4 py-3">
            <h5 class="modal-title fw-bold text-dark">{{ isEditing ? 'Editar Producto' : 'Registrar Producto' }}</h5>
            <button type="button" class="btn-close" data-bs-dismiss="modal"></button>
          </div>
          <form @submit.prevent="saveProduct">
            <div class="modal-body p-4">
              <div class="row g-3 mb-3">
                <div class="col-md-8">
                  <label class="form-label small fw-bold text-muted">Nombre</label>
                  <input type="text" v-model="productForm.nombre" class="form-control" required>
                </div>
                <div class="col-md-4">
                  <label class="form-label small fw-bold text-muted">Estado</label>
                  <select v-model="productForm.estado" class="form-select">
                    <option value="activo">Activo</option>
                    <option value="inactivo">Inactivo</option>
                  </select>
                </div>
              </div>
              
              <div class="row g-3 mb-3">
                <div class="col-md-6">
                  <label class="form-label small fw-bold text-muted">Categoría</label>
                  <select v-model="productForm.categoria_id" class="form-select" required>
                    <option disabled value="">Seleccionar...</option>
                    <option v-for="cat in categories" :key="cat.id" :value="cat.id">{{ cat.nombre }}</option>
                  </select>
                </div>
                <div class="col-md-6">
                  <label class="form-label small fw-bold text-muted">Precio</label>
                  <div class="input-group">
                    <span class="input-group-text bg-light text-muted border">$</span>
                    <input type="number" step="0.01" min="0" v-model.number="productForm.precio" class="form-control" required>
                  </div>
                </div>
              </div>
              
              <div class="mb-4">
                <label class="form-label small fw-bold text-muted">Descripción</label>
                <textarea v-model="productForm.descripcion" class="form-control" rows="2"></textarea>
              </div>
              
              <div class="p-3 bg-light rounded-3 mb-4">
                <h6 class="fw-bold text-dark mb-3"><i class="fas fa-boxes me-2 text-muted"></i>Inventario Inicial</h6>
                <div class="row g-3">
                   <div class="col-md-6">
                      <label class="form-label small fw-bold text-muted">Stock Cantidad</label>
                      <input type="number" min="0" v-model.number="productForm.quantity" class="form-control">
                   </div>
                   <div class="col-md-6">
                      <label class="form-label small fw-bold text-muted">Ubicación</label>
                      <input type="text" v-model="productForm.warehouse_location" class="form-control" placeholder="Ej: Pasillo A-1">
                   </div>
                </div>
              </div>

              <div>
                <label class="form-label small fw-bold text-muted">Imagen</label>
                <div class="d-flex gap-2 mb-2">
                   <button type="button" class="btn btn-sm rounded-pill px-3" :class="uploadMode === 'file' ? 'btn-dark' : 'btn-outline-dark'" @click="uploadMode='file'">Subir</button>
                   <button type="button" class="btn btn-sm rounded-pill px-3" :class="uploadMode === 'url' ? 'btn-dark' : 'btn-outline-dark'" @click="uploadMode='url'">URL</button>
                </div>
                
                <input v-if="uploadMode === 'file'" type="file" @change="handleFileChange" class="form-control" accept="image/*">
                <input v-if="uploadMode === 'url'" type="url" v-model="productForm.imagen" class="form-control" placeholder="https://...">
                
                <div v-if="imagePreview" class="mt-3 p-2 border rounded bg-light d-inline-block">
                  <img :src="imagePreview" alt="Preview" style="height: 80px; width: 80px; object-fit: cover; border-radius: 4px;">
                </div>
              </div>
              
              <div v-if="formError" class="alert alert-danger mt-3 py-2 small border-0 bg-danger bg-opacity-10 text-danger">{{ formError }}</div>

            </div>
            <div class="modal-footer border-0 bg-light px-4 py-3">
              <button type="button" class="btn btn-outline-dark rounded-pill px-4" data-bs-dismiss="modal">Cancelar</button>
              <button type="submit" class="btn btn-dark rounded-pill px-4" :disabled="isSaving">
                <span v-if="isSaving" class="spinner-border spinner-border-sm me-2"></span>Guardar
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>
    
    <div class="modal fade" id="stockModal" tabindex="-1">
      <div class="modal-dialog modal-dialog-centered">
        <div class="modal-content border-0 shadow-lg rounded-4">
          <div class="modal-header border-bottom px-4 py-3">
            <h5 class="modal-title fw-bold text-dark">Detalle de Stock</h5>
            <button type="button" class="btn-close" data-bs-dismiss="modal"></button>
          </div>
          <div class="modal-body p-4 text-center" v-if="isLoadingStock">
             <div class="spinner-border text-primary"></div>
          </div>
          <div class="modal-body p-4" v-else-if="selectedStock">
             <div class="text-center mb-4">
                <h5 class="fw-bold mb-0">{{ selectedStock.productName }}</h5>
                <small class="text-muted">{{ selectedStock.productCode }}</small>
             </div>
             
             <div class="row g-3 text-center">
                <div class="col-6">
                   <div class="p-3 bg-light rounded-3">
                      <small class="text-muted text-uppercase fw-bold d-block mb-1">Cantidad</small>
                      <h2 class="text-primary fw-bold mb-0">{{ selectedStock.quantity }}</h2>
                   </div>
                </div>
                <div class="col-6">
                   <div class="p-3 bg-light rounded-3 h-100 d-flex flex-column justify-content-center">
                      <small class="text-muted text-uppercase fw-bold d-block mb-1">Ubicación</small>
                      <h6 class="fw-bold text-dark mb-0">{{ selectedStock.warehouse_location }}</h6>
                   </div>
                </div>
             </div>
             
             <div class="mt-4 pt-3 border-top text-center">
                <button class="btn btn-link text-primary text-decoration-none fw-bold" @click="editFromStockView">
                   <i class="fas fa-edit me-1"></i> Editar Producto
                </button>
             </div>
          </div>
        </div>
      </div>
    </div>

    <div class="modal fade" id="categoryModal" tabindex="-1">
      <div class="modal-dialog modal-dialog-centered">
        <div class="modal-content border-0 shadow-lg rounded-4">
          <div class="modal-header border-bottom px-4 py-3">
            <h5 class="modal-title fw-bold text-dark">Nueva Categoría</h5>
            <button type="button" class="btn-close" data-bs-dismiss="modal"></button>
          </div>
          <form @submit.prevent="saveCategory">
             <div class="modal-body p-4">
                <div class="mb-3">
                   <label class="form-label small fw-bold text-muted">Nombre</label>
                   <input type="text" v-model="categoryForm.nombre" class="form-control" required>
                </div>
                <div class="mb-3">
                   <label class="form-label small fw-bold text-muted">Descripción</label>
                   <textarea v-model="categoryForm.descripcion" class="form-control" rows="3"></textarea>
                </div>
                <div v-if="categoryFormError" class="alert alert-danger py-2 small border-0 bg-danger bg-opacity-10 text-danger">{{ categoryFormError }}</div>
             </div>
             <div class="modal-footer border-0 bg-light px-4 py-3">
                <button type="button" class="btn btn-outline-dark rounded-pill px-4" data-bs-dismiss="modal">Cancelar</button>
                <button type="submit" class="btn btn-dark rounded-pill px-4" :disabled="isSavingCategory">
                   <span v-if="isSavingCategory" class="spinner-border spinner-border-sm me-2"></span>Guardar
                </button>
             </div>
          </form>
        </div>
      </div>
    </div>

  </div>
</template>

<script>
import { ref, reactive, computed, onMounted, watch } from 'vue';
import * as bootstrap from 'bootstrap';
import api from '../services/api';

const sampleCategories = [
    { old_id: '11111111-1111-1111-1111-111111111111', codigo: 'BEB', nombre: 'Bebidas', descripcion: 'Bebidas, Jugos, Refrescos' },
    { old_id: '22222222-2222-2222-2222-222222222222', codigo: 'VIB', nombre: 'Víveres', descripcion: 'Alimentos no perecederos' },
    { old_id: '33333333-3333-3333-3333-333333333333', codigo: 'DUL', nombre: 'Dulces', descripcion: 'Golosinas y Snacks' },
];

const sampleProducts = [
    { stock: 150, data: { codigo: 'BEB-000001', nombre: 'Coca Cola Original', precio: 3.00, id_categoria: '11111111-1111-1111-1111-111111111111', imagen: 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/cocacola.jpg', descripcion: 'Bebida gaseosa clásica'}},
    { stock: 120, data: { codigo: 'BEB-000002', nombre: 'Jugo de Naranja', precio: 2.50, id_categoria: '11111111-1111-1111-1111-111111111111', imagen: 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/naranja.jpg', descripcion: 'Saludable y natrual'}},
    { stock: 180, data: { codigo: 'VIB-000001', nombre: 'Harina', precio: 1.10, id_categoria: '22222222-2222-2222-2222-222222222222', imagen: 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/harina.jpg', descripcion: 'Ideal para repostería'}},
    { stock: 160, data: { codigo: 'VIB-000002', nombre: 'Pasta', precio: 2.20, id_categoria: '22222222-2222-2222-2222-222222222222', imagen: 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/pasta.jpg', descripcion: 'Fácil de cocinar'}},
    { stock: 300, data: { codigo: 'DUL-000001', nombre: 'Galleta Oreo', precio: 2.50, id_categoria: '33333333-3333-3333-3333-333333333333', imagen: 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/oreo.jpg', descripcion: 'Clásica y deliciosa'}},
    { stock: 90, data: { codigo: 'DUL-000002', nombre: 'Chocolate Savoy', precio: 3.60, id_categoria: '33333333-3333-3333-3333-333333333333', imagen: 'https://productos-sistema-distribuido.s3.us-east-1.amazonaws.com/chocolate.jpg', descripcion: 'Delicioso chocolate oscuro'}},
];

export default {
  name: 'ProductManagement',
  setup() {
    const products = ref([]);
    const categories = ref([]);
    const isLoading = ref(false);
    const isSaving = ref(false);
    const isSeeding = ref(false);
    const error = ref(null);
    const formError = ref(null);
    const isEditing = ref(false);
    const isLoadingStock = ref(false);
    const selectedStock = ref(null);
    let productModal = null;
    let stockModal = null;
    let categoryModal = null;

    const imageFile = ref(null);
    const imagePreview = ref(null);
    const uploadMode = ref('file');
    
    const filters = reactive({ category: '', status: '', search: '' });
    
    const currentPage = ref(1);
    const itemsPerPage = ref(10);
    
    const productForm = reactive({
      id: null, nombre: '', descripcion: '', precio: 0, categoria_id: '',
      imagen: '', estado: 'activo', quantity: 0, warehouse_location: ''
    });

    const categoryForm = reactive({ nombre: '', descripcion: '' });
    const isSavingCategory = ref(false);
    const categoryFormError = ref(null);

    watch(() => productForm.imagen, (newUrl) => {
        if (uploadMode.value === 'url') {
            imagePreview.value = newUrl;
        }
    });

    const filteredProducts = computed(() => {
        return products.value.filter(p =>
            (!filters.category || p.categoria_id == filters.category) &&
            (!filters.status || p.estado === filters.status) &&
            (!filters.search || p.nombre.toLowerCase().includes(filters.search.toLowerCase()))
        );
    });
    
    const totalPages = computed(() => Math.ceil(filteredProducts.value.length / itemsPerPage.value));
    
    const paginatedProducts = computed(() => {
        const start = (currentPage.value - 1) * itemsPerPage.value;
        const end = start + itemsPerPage.value;
        return filteredProducts.value.slice(start, end);
    });
    
    const goToPage = (page) => {
        if (page >= 1 && page <= totalPages.value) {
            currentPage.value = page;
            window.scrollTo({ top: 0, behavior: 'smooth' });
        }
    };
    
    watch([() => filters.category, () => filters.status, () => filters.search], () => {
        currentPage.value = 1;
    });

    const fetchAll = async () => {
        isLoading.value = true;
        error.value = null;
        try {
            const [prods, cats] = await Promise.all([
                api.getProducts(),
                api.getCategories(),
            ]);
            products.value = prods;
            const uniqueCategories = cats.filter((category, index, self) =>
                index === self.findIndex((c) => c.nombre.toLowerCase() === category.nombre.toLowerCase())
            );
            categories.value = uniqueCategories;
        } catch (err) {
            error.value = 'Error al cargar datos.';
            console.error(err);
        } finally {
            isLoading.value = false;
        }
    };

    const bloquearSignos = (e) => {
        if (['-', '+', 'e', 'E'].includes(e.key)) {
            e.preventDefault();
        }
    };
    
    const getCategoryName = (catId) => categories.value.find(c => c.id === catId)?.nombre || 'N/A';
    const clearFilters = () => { filters.category = ''; filters.status = ''; filters.search = ''; };
    
    const resetForm = () => {
        Object.assign(productForm, {
            id: null, nombre: '', descripcion: '', precio: 0, categoria_id: '',
            imagen: '', estado: 'activo', quantity: 0, warehouse_location: ''
        });
        imageFile.value = null;
        imagePreview.value = null;
        formError.value = null;
        uploadMode.value = 'file';
    };

    const handleFileChange = (event) => {
        const file = event.target.files[0];
        if (file) {
            imageFile.value = file;
            imagePreview.value = URL.createObjectURL(file);
            productForm.imagen = '';
        }
    };
    
    const showCreateModal = () => {
        isEditing.value = false;
        resetForm();
        productModal?.show();
    };

    const showCategoryModal = () => {
        Object.assign(categoryForm, { nombre: '', descripcion: ''});
        categoryFormError.value = null;
        categoryModal?.show();
    };

    const editProduct = async (product) => {
        isEditing.value = true;
        resetForm();
        Object.assign(productForm, {
            id: product.id,
            nombre: product.nombre,
            descripcion: product.descripcion || '',
            precio: parseFloat(product.precio),
            categoria_id: product.categoria_id,
            imagen: product.imagen || '',
            estado: product.estado
        });
        
        if (product.imagen) {
            imagePreview.value = product.imagen;
            uploadMode.value = 'url';
        }
        
        try {
            const stockResponse = await api.getStock(product.id);
            if (stockResponse.data && stockResponse.data.length > 0) {
                const { quantity, warehouse_location } = stockResponse.data[0];
                productForm.quantity = quantity;
                productForm.warehouse_location = warehouse_location;
            }
        } catch (err) {
            console.error("Error fetching stock info:", err);
        }
        productModal?.show();
    };

    const saveProduct = async () => {
        isSaving.value = true;
        formError.value = null;

        const formData = new FormData();
        Object.keys(productForm).forEach(key => {
            if (key !== 'id') formData.append(key, productForm[key]);
        });

        if (uploadMode.value === 'file' && imageFile.value) {
            formData.append('imagen', imageFile.value);
        }

        try {
            let savedProduct;
            if (isEditing.value) {
                savedProduct = await api.updateProduct(productForm.id, formData);
            } else {
                savedProduct = await api.createProduct(formData);
            }

            const stockPayload = {
                product_id: savedProduct.producto?.id || productForm.id,
                quantity: productForm.quantity,
                warehouse_location: productForm.warehouse_location,
            };

            if (isEditing.value) {
                const { product_id, ...updateData } = stockPayload;
                await api.updateStock(stockPayload.product_id, updateData);
            } else {
                await api.addStock(stockPayload);
            }
            
            await fetchAll();
            productModal?.hide();
        } catch (err) {
            formError.value = err.response?.data?.message || 'Error al guardar.';
            console.error(err);
        } finally {
            isSaving.value = false;
        }
    };

    const saveCategory = async () => {
        isSavingCategory.value = true;
        categoryFormError.value = null;
        try {
            await api.createCategory(categoryForm);
            await fetchAll();
            categoryModal?.hide();
        } catch (err) {
            categoryFormError.value = err.response?.data?.message || 'Error al guardar la categoría.';
            console.error(err);
        } finally {
            isSavingCategory.value = false;
        }
    };

    const deleteProduct = async (productId) => {
        if (!confirm('¿Seguro que quieres eliminar este producto y su stock?')) return;
        try {
            await Promise.all([
                api.deleteProduct(productId),
                api.deleteStock(productId)
            ]);
            await fetchAll();
        } catch (err) {
            console.error('Error deleting product/stock:', err);
            error.value = 'Error al eliminar el producto.';
        }
    };

    const handleGenerateSampleData = async () => {
        if (!confirm('¿Seguro que quieres generar datos de muestra?')) return;
        isSeeding.value = true;
        try {
            const categoryIdMap = new Map();
            for (const cat of sampleCategories) {
                try {
                    const newCat = await api.createCategory({ nombre: cat.nombre, descripcion: cat.descripcion });
                    categoryIdMap.set(cat.old_id, newCat.id);
                } catch (e) {
                    const existingCats = await api.getCategories();
                    const existingCat = existingCats.find(c => c.nombre === cat.nombre);
                    if (existingCat) categoryIdMap.set(cat.old_id, existingCat.id);
                    else throw e;
                }
            }
            for (const prod of sampleProducts) {
                const newCatId = categoryIdMap.get(prod.data.id_categoria);
                if (!newCatId) continue;
                const { id_categoria, imagen, ...productData } = prod.data;
                const productPayload = { ...productData, categoria_id: newCatId, imagen_url: imagen };
                try {
                    const newProductResponse = await api.createProduct(productPayload);
                    if (newProductResponse && newProductResponse.producto) {
                         await api.addStock({ product_id: newProductResponse.producto.id, quantity: prod.stock, warehouse_location: 'Almacén Principal' });
                    }
                } catch(e) { console.error(e); }
            }
            await fetchAll();
        } catch (err) { alert('Error seeding data'); console.error(err); } 
        finally { isSeeding.value = false; }
    };
    
    const viewStock = async (product) => {
        isLoadingStock.value = true;
        selectedStock.value = null;
        stockModal?.show();
        try {
            const stockResponse = await api.getStock(product.id);
            if (stockResponse.data && stockResponse.data.length > 0) {
                selectedStock.value = { ...stockResponse.data[0], productName: product.nombre, productCode: product.codigo };
            }
        } catch (err) { console.error('Error fetching stock:', err); } 
        finally { isLoadingStock.value = false; }
    };

    const editFromStockView = () => {
        stockModal?.hide();
        const product = products.value.find(p => p.id === selectedStock.value.product_id);
        if (product) editProduct(product);
    };
    
    onMounted(() => {
        fetchAll();
        productModal = new bootstrap.Modal(document.getElementById('productModal'));
        stockModal = new bootstrap.Modal(document.getElementById('stockModal'));
        categoryModal = new bootstrap.Modal(document.getElementById('categoryModal'));
    });
    
    return { 
        products, categories, isLoading, isSaving, isSeeding, error, formError, isEditing, 
        isLoadingStock, selectedStock, filters, productForm, filteredProducts, paginatedProducts, 
        imagePreview, uploadMode, currentPage, itemsPerPage, totalPages,
        categoryForm, isSavingCategory, categoryFormError,
        getCategoryName, clearFilters, handleGenerateSampleData, bloquearSignos,
        showCreateModal, showCategoryModal, editProduct, saveProduct, saveCategory, deleteProduct, handleFileChange,
        viewStock, editFromStockView, goToPage
    };
  }
}
</script>

<style scoped>
.product-management-page {
  background-color: #ffffff;
  min-height: 100vh;
}

.hover-bg-gray:hover {
  background-color: #e2e8f0;
}

.hover-bg-primary-soft:hover {
  background-color: rgba(59, 130, 246, 0.1);
  color: #3b82f6 !important;
}

.hover-bg-danger-soft:hover {
  background-color: rgba(220, 53, 69, 0.1);
  color: #dc3545 !important;
}

.hover-text-dark:hover {
  color: #000 !important;
}
</style>