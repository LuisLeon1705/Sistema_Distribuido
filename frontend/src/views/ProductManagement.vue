<template>
  <div class="product-management">
    <div class="container-fluid">
      <div class="d-flex justify-content-between align-items-center mb-4">
        <h2>Gestión de Productos</h2>
        <div class="d-flex gap-2">
          <button class="btn btn-info" @click="showCategoryModal">
            <i class="fas fa-tags me-2"></i>
            Crear Categoría
          </button>
          <button class="btn btn-primary" @click="showCreateModal">
            <i class="fas fa-plus me-2"></i>
            Nuevo Producto
          </button>
        </div>
      </div>
      
      <!-- Filters -->
      <div class="card mb-4">
        <div class="card-body">
          <div class="row g-2">
            <div class="col-md-3">
              <select v-model="filters.category" class="form-select">
                <option value="">Todas las categorías</option>
                <option v-for="category in categories" :key="category.id" :value="category.id">
                  {{ category.nombre }}
                </option>
              </select>
            </div>
            <div class="col-md-3">
              <select v-model="filters.status" class="form-select">
                <option value="">Todos los estados</option>
                <option value="activo">Activo</option>
                <option value="inactivo">Inactivo</option>
              </select>
            </div>
            <div class="col-md-4">
              <input type="text" v-model="filters.search" placeholder="Buscar productos..." class="form-control">
            </div>
            <div class="col-md-2">
              <button @click="clearFilters" class="btn btn-outline-secondary w-100">Limpiar</button>
            </div>
          </div>
        </div>
      </div>
      
      <!-- Products Table -->
      <div class="card">
        <div class="card-body">
          <div v-if="isLoading" class="text-center py-4"><div class="spinner-border"></div></div>
          <div v-else-if="error" class="alert alert-danger">{{ error }}</div>
          <div v-else class="table-responsive">
            <table class="table table-hover align-middle">
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
                <tr v-for="product in paginatedProducts" :key="product.id">
                  <td>{{ product.codigo }}</td>
                  <td><img :src="product.imagen || '/placeholder-product.jpg'" :alt="product.nombre" class="table-img"></td>
                  <td>
                    <div class="fw-medium">{{ product.nombre }}</div>
                    <small class="text-muted">{{ truncateText(product.descripcion, 50) }}</small>
                  </td>
                  <td>{{ getCategoryName(product.categoria_id) }}</td>
                  <td>${{ parseFloat(product.precio).toFixed(2) }}</td>
                  <td><span class="badge" :class="product.estado === 'activo' ? 'bg-success' : 'bg-secondary'">{{ product.estado }}</span></td>
                  <td>
                    <div class="btn-group btn-group-sm">
                      <button class="btn btn-outline-info" @click="viewStock(product)" title="Ver Stock"><i class="fas fa-boxes"></i></button>
                      <button class="btn btn-outline-primary" @click="editProduct(product)" title="Editar"><i class="fas fa-edit"></i></button>
                      <button class="btn btn-outline-danger" @click="deleteProduct(product.id)" title="Eliminar"><i class="fas fa-trash"></i></button>
                    </div>
                  </td>
                </tr>
              </tbody>
            </table>
            <div v-if="filteredProducts.length === 0" class="text-center py-4">
              <i class="fas fa-box-open fa-3x text-muted mb-3"></i>
              <h5>No se encontraron productos</h5>
              <p class="text-muted">Ajusta los filtros o crea un nuevo producto.</p>
            </div>
            
            <!-- Pagination -->
            <nav v-if="!isLoading && filteredProducts.length > 0 && totalPages > 1" aria-label="Product pagination" class="mt-3">
              <ul class="pagination justify-content-center mb-0">
                <li class="page-item" :class="{ disabled: currentPage === 1 }">
                  <button class="page-link" @click="goToPage(currentPage - 1)" :disabled="currentPage === 1">
                    <i class="fas fa-chevron-left"></i>
                  </button>
                </li>
                
                <li v-for="page in totalPages" :key="page" class="page-item" :class="{ active: currentPage === page }">
                  <button class="page-link" @click="goToPage(page)">{{ page }}</button>
                </li>
                
                <li class="page-item" :class="{ disabled: currentPage === totalPages }">
                  <button class="page-link" @click="goToPage(currentPage + 1)" :disabled="currentPage === totalPages">
                    <i class="fas fa-chevron-right"></i>
                  </button>
                </li>
              </ul>
            </nav>
            <p v-if="filteredProducts.length > itemsPerPage" class="text-center text-muted small mt-2 mb-0">
              Mostrando {{ (currentPage - 1) * itemsPerPage + 1 }} - {{ Math.min(currentPage * itemsPerPage, filteredProducts.length) }} de {{ filteredProducts.length }} productos
            </p>
          </div>
        </div>
      </div>
    </div>
    
    <!-- Product Modal -->
    <div class="modal fade" id="productModal" tabindex="-1">
      <div class="modal-dialog modal-lg">
        <div class="modal-content">
          <div class="modal-header">
            <h5 class="modal-title">{{ isEditing ? 'Editar Producto' : 'Nuevo Producto' }}</h5>
            <button type="button" class="btn-close" data-bs-dismiss="modal"></button>
          </div>
          <form @submit.prevent="saveProduct">
            <div class="modal-body">
              <h6 class="mb-3">Detalles del Producto</h6>
              <div class="row">
                <div class="col-md-6 mb-3">
                  <label class="form-label">Nombre *</label>
                  <input type="text" v-model="productForm.nombre" class="form-control" required>
                </div>
                <div class="col-md-6 mb-3">
                  <label class="form-label">Categoría *</label>
                  <select v-model="productForm.categoria_id" class="form-select" required>
                    <option disabled value="">Seleccionar categoría</option>
                    <option v-for="cat in categories" :key="cat.id" :value="cat.id">{{ cat.nombre }}</option>
                  </select>
                </div>
              </div>
              <div class="mb-3">
                <label class="form-label">Descripción</label>
                <textarea v-model="productForm.descripcion" class="form-control" rows="3"></textarea>
              </div>
              <div class="row">
                <div class="col-md-6 mb-3">
                  <label class="form-label">Precio *</label>
                  <input 
                    type="number" 
                    step="0.01" 
                    min="0.01" 
                    v-model.number="productForm.precio" 
                    @keydown="bloquearSignos"
                    class="form-control" 
                    required
                  >
                  <small class="text-muted">El precio debe ser mayor a 0</small>
                </div>
                <div class="col-md-6 mb-3">
                  <label class="form-label">Estado</label>
                  <select v-model="productForm.estado" class="form-select">
                    <option value="activo">Activo</option>
                    <option value="inactivo">Inactivo</option>
                  </select>
                </div>
              </div>
              <div class="mb-3">
                <label class="form-label">Imagen del Producto</label>
                <div>
                    <div class="btn-group btn-group-sm mb-2">
                        <button type="button" class="btn" :class="uploadMode === 'file' ? 'btn-primary' : 'btn-outline-primary'" @click="uploadMode = 'file'">Subir Archivo</button>
                        <button type="button" class="btn" :class="uploadMode === 'url' ? 'btn-primary' : 'btn-outline-primary'" @click="uploadMode = 'url'">Desde URL</button>
                    </div>
                </div>

                <div v-if="uploadMode === 'file'">
                    <input type="file" @change="handleFileChange" class="form-control" accept="image/*">
                </div>
                <div v-if="uploadMode === 'url'">
                    <input type="url" v-model="productForm.imagen" class="form-control" placeholder="https://ejemplo.com/imagen.jpg">
                </div>

                <img v-if="imagePreview" :src="imagePreview" alt="Vista previa" class="img-thumbnail mt-2" style="max-height: 150px;">
              </div>

              <hr class="my-4">

              <h6 class="mb-3">Detalles del Stock</h6>
              <div class="row">
                <div class="col-md-6 mb-3">
                  <label class="form-label">Cantidad en Stock *</label>
                  <input type="number" min="0" v-model.number="productForm.quantity" class="form-control" required>
                </div>
                <div class="col-md-6 mb-3">
                  <label class="form-label">Ubicación en Almacén *</label>
                  <input type="text" v-model="productForm.warehouse_location" class="form-control" required>
                </div>
              </div>
              
              <div v-if="formError" class="alert alert-danger mt-3">{{ formError }}</div>
            </div>
            <div class="modal-footer">
              <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Cancelar</button>
              <button type="submit" class="btn btn-primary" :disabled="isSaving">
                <span v-if="isSaving" class="spinner-border spinner-border-sm me-2"></span>
                Guardar
              </button>
            </div>
          </form>
        </div>
      </div>
    </div>

    <!-- Stock Details Modal -->
    <div class="modal fade" id="stockModal" tabindex="-1">
      <div class="modal-dialog">
        <div class="modal-content">
          <div class="modal-header">
            <h5 class="modal-title">Detalles de Stock</h5>
            <button type="button" class="btn-close" data-bs-dismiss="modal"></button>
          </div>
          <div class="modal-body">
            <div v-if="isLoadingStock" class="text-center py-4">
              <div class="spinner-border"></div>
              <p class="mt-2">Cargando información de stock...</p>
            </div>
            <div v-else-if="selectedStock">
              <div class="mb-3">
                <h6>{{ selectedStock.productName }}</h6>
                <p class="text-muted mb-0">Código: {{ selectedStock.productCode }}</p>
              </div>
              <hr>
              <div class="row">
                <div class="col-6">
                  <p class="mb-1 text-muted">Cantidad en Stock</p>
                  <h4 class="text-primary">{{ selectedStock.quantity }}</h4>
                </div>
                <div class="col-6">
                  <p class="mb-1 text-muted">Ubicación</p>
                  <h5>{{ selectedStock.warehouse_location }}</h5>
                </div>
              </div>
              <hr>
              <div class="alert alert-info mb-0">
                <i class="fas fa-info-circle me-2"></i>
                <small>
                  <strong>ID del Producto:</strong> {{ selectedStock.product_id }}
                </small>
              </div>
            </div>
            <div v-else class="alert alert-warning">
              <i class="fas fa-exclamation-triangle me-2"></i>
              No hay información de stock disponible para este producto.
            </div>
          </div>
          <div class="modal-footer">
            <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Cerrar</button>
            <button type="button" class="btn btn-primary" @click="editFromStockView" v-if="selectedStock">
              <i class="fas fa-edit me-2"></i>
              Editar Producto
            </button>
          </div>
        </div>
      </div>
    </div>

    <!-- Category Modal -->
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
                <input type="text" v-model="categoryForm.nombre" class="form-control" required>
              </div>
              <div class="mb-3">
                <label class="form-label">Descripción</label>
                <textarea v-model="categoryForm.descripcion" class="form-control" rows="3"></textarea>
              </div>
              <div v-if="categoryFormError" class="alert alert-danger">{{ categoryFormError }}</div>
            </div>
            <div class="modal-footer">
              <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">Cancelar</button>
              <button type="submit" class="btn btn-primary" :disabled="isSavingCategory">
                <span v-if="isSavingCategory" class="spinner-border spinner-border-sm me-2"></span>
                Guardar
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
    { old_id: '22222222-2222-2222-2222-222222222222', codigo: 'VIB', nombre: 'Viveres', descripcion: 'Alimentos no perecederos' },
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
    
    // Pagination
    const currentPage = ref(1);
    const itemsPerPage = ref(12); // 12 productos por página
    
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
    
    // Pagination computed
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
    
    // Reset pagination when filters change
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
            categories.value = cats;
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
    const truncateText = (text, len) => text?.length > len ? `${text.substring(0, len)}...` : text;
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
            productForm.imagen = ''; // Clear URL if a file is selected
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
            if (key !== 'id') {
                 formData.append(key, productForm[key]);
            }
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
            // First, create categories and map their old IDs to new ones
            for (const cat of sampleCategories) {
                try {
                    const newCat = await api.createCategory({ nombre: cat.nombre, descripcion: cat.descripcion });
                    categoryIdMap.set(cat.old_id, newCat.id);
                } catch (e) {
                    // Try to find existing category if creation fails (e.g., unique constraint)
                    const existingCats = await api.getCategories();
                    const existingCat = existingCats.find(c => c.nombre === cat.nombre);
                    if (existingCat) {
                        categoryIdMap.set(cat.old_id, existingCat.id);
                    } else { throw e; } // Re-throw if not found
                }
            }
            
            // Then, create products and their stock
            for (const prod of sampleProducts) {
                const newCatId = categoryIdMap.get(prod.data.id_categoria);
                if (!newCatId) continue;

                const { id_categoria, imagen, ...productData } = prod.data;
                const productPayload = { 
                    ...productData, 
                    categoria_id: newCatId,
                    imagen_url: imagen
                };
                
                // Use a try-catch for each product to be more resilient
                try {
                    const newProductResponse = await api.createProduct(productPayload);
                    if (newProductResponse && newProductResponse.producto) {
                        try {
                            await api.addStock({
                                product_id: newProductResponse.producto.id,
                                quantity: prod.stock,
                                warehouse_location: 'Almacén Principal'
                            });
                        } catch (stockErr) {
                            console.error(`Failed to add stock for product ${prod.data.nombre}:`, stockErr);
                        }
                    }
                } catch(e) {
                     console.error(`Failed to create product ${prod.data.nombre}:`, e);
                }
            }
            
            alert('Datos de muestra generados exitosamente.');
            await fetchAll();

        } catch (err) {
            alert('Error al generar datos de muestra. Revisa la consola.');
            console.error(err);
        } finally {
            isSeeding.value = false;
        }
    };
    
    const viewStock = async (product) => {
        isLoadingStock.value = true;
        selectedStock.value = null;
        stockModal?.show();
        
        try {
            const stockResponse = await api.getStock(product.id);
            if (stockResponse.data && stockResponse.data.length > 0) {
                selectedStock.value = {
                    ...stockResponse.data[0],
                    productName: product.nombre,
                    productCode: product.codigo
                };
            }
        } catch (err) {
            console.error('Error fetching stock:', err);
        } finally {
            isLoadingStock.value = false;
        }
    };

    const editFromStockView = () => {
        stockModal?.hide();
        const product = products.value.find(p => p.id === selectedStock.value.product_id);
        if (product) {
            editProduct(product);
        }
    };
    
    onMounted(() => {
        fetchAll();
        productModal = new bootstrap.Modal(document.getElementById('productModal'));
        stockModal = new bootstrap.Modal(document.getElementById('stockModal'));
        categoryModal = new bootstrap.Modal(document.getElementById('categoryModal'));
    });
    
    return { 
        products, categories, isLoading, isSaving, isSeeding, error, formError, isEditing, 
        isLoadingStock, selectedStock,
        filters, productForm, filteredProducts, paginatedProducts, imagePreview, uploadMode,
        currentPage, itemsPerPage, totalPages,
        categoryForm, isSavingCategory, categoryFormError,
        getCategoryName, truncateText, clearFilters, handleGenerateSampleData, bloquearSignos,
        showCreateModal, showCategoryModal, editProduct, saveProduct, saveCategory, deleteProduct, handleFileChange,
        viewStock, editFromStockView, goToPage
    };
  }
}
</script>

<style scoped>
.table-img { width: 50px; height: 50px; object-fit: cover; border-radius: 4px; }
.table td, .table th { vertical-align: middle; }
</style>
