<template>
  <div class="product-management">
    <div class="container-fluid">
      <div class="d-flex justify-content-between align-items-center mb-4">
        <h2>Gestión de Productos</h2>
        <div class="d-flex gap-2">
           <button class="btn btn-success" @click="handleGenerateSampleData" :disabled="isSeeding">
            <span v-if="isSeeding" class="spinner-border spinner-border-sm me-2"></span>
            <i v-else class="fas fa-magic me-2"></i>
            Generar Productos de Muestra
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
                <tr v-for="product in filteredProducts" :key="product.id">
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
                  <input type="number" step="0.01" min="0" v-model.number="productForm.precio" class="form-control" required>
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
                  <label class="form-label">URL de Imagen</label>
                  <input type="url" v-model="productForm.imagen" class="form-control" placeholder="https://ejemplo.com/imagen.jpg">
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
  </div>
</template>

<script>
import { ref, reactive, computed, onMounted } from 'vue';
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
    let productModal = null;
    
    const filters = reactive({ category: '', status: '', search: '' });
    
    const productForm = reactive({
      id: null, nombre: '', descripcion: '', precio: 0, categoria_id: '',
      imagen: '', estado: 'activo', quantity: 0, warehouse_location: ''
    });

    const filteredProducts = computed(() => {
        return products.value.filter(p =>
            (!filters.category || p.categoria_id == filters.category) &&
            (!filters.status || p.estado === filters.status) &&
            (!filters.search || p.nombre.toLowerCase().includes(filters.search.toLowerCase()))
        );
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

    const getCategoryName = (catId) => categories.value.find(c => c.id === catId)?.nombre || 'N/A';
    const truncateText = (text, len) => text?.length > len ? `${text.substring(0, len)}...` : text;
    const clearFilters = () => { filters.category = ''; filters.status = ''; filters.search = ''; };
    
    const resetForm = () => {
        Object.assign(productForm, {
            id: null, nombre: '', descripcion: '', precio: 0, categoria_id: '',
            imagen: '', estado: 'activo', quantity: 0, warehouse_location: ''
        });
        formError.value = null;
    };
    
    const showCreateModal = () => {
        isEditing.value = false;
        resetForm();
        productModal?.show();
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
        try {
            const { id, ...productData } = productForm;
            let savedProduct;
            if (isEditing.value) {
                savedProduct = await api.updateProduct(id, productData);
            } else {
                savedProduct = await api.createProduct(productData);
            }

            const stockPayload = {
                product_id: savedProduct.producto?.id || id,
                quantity: productForm.quantity,
                warehouse_location: productForm.warehouse_location,
            };

            if (isEditing.value) {
                await api.updateStock(stockPayload.product_id, stockPayload);
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

                const productPayload = { ...prod.data, categoria_id: newCatId };
                
                // Use a try-catch for each product to be more resilient
                try {
                    const newProductResponse = await api.createProduct(productPayload);
                    if (newProductResponse && newProductResponse.producto) {
                        await api.addStock({
                            product_id: newProductResponse.producto.id,
                            quantity: prod.stock,
                            warehouse_location: 'Almacén Principal'
                        });
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
    
    onMounted(() => {
        fetchAll();
        productModal = new bootstrap.Modal(document.getElementById('productModal'));
    });
    
    return { 
        products, categories, isLoading, isSaving, isSeeding, error, formError, isEditing, 
        filters, productForm, filteredProducts,
        getCategoryName, truncateText, clearFilters, handleGenerateSampleData,
        showCreateModal, editProduct, saveProduct, deleteProduct
    };
  }
}
</script>

<style scoped>
.table-img { width: 50px; height: 50px; object-fit: cover; border-radius: 4px; }
.table td, .table th { vertical-align: middle; }
</style>