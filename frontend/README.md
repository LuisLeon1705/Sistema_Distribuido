# Frontend - Sistema Distribuido

Este es el frontend de Vue.js que consume los tres microservicios del sistema distribuido.

## Estructura del Proyecto

- **AuthService (Port 8001)**: Manejo de autenticación, usuarios y roles
- **InventoryService (Port 8002)**: Gestión de órdenes e inventario
- **ProductService (Port 8003)**: Gestión de productos y categorías

## Características

### Para Usuarios (Customers):
- ✅ Registro e inicio de sesión
- ✅ Visualización de productos con filtros
- ✅ Carrito de compras
- ✅ Realizar órdenes
- ✅ Ver historial de órdenes
- ✅ Gestión de perfil

### Para Administradores:
- ✅ Gestión completa de productos
- ✅ Gestión de categorías
- ✅ Vista de todas las órdenes
- ✅ Gestión de usuarios
- ✅ Dashboard administrativo

## Instalación

```bash
# Navegar al directorio frontend
cd frontend

# Instalar dependencias
npm install

# Ejecutar en modo desarrollo
npm run dev
```

## Configuración de Microservicios

El frontend está configurado para hacer proxy a los microservicios:

- Auth API: `http://localhost:8001`
- Inventory API: `http://localhost:8002`
- Products API: `http://localhost:8003`

Asegúrate de que todos los microservicios estén ejecutándose antes de iniciar el frontend.

## Scripts Disponibles

```bash
# Desarrollo
npm run dev

# Build para producción
npm run build

# Preview del build
npm run preview
```

## Tecnologías Utilizadas

- **Vue 3**: Framework principal
- **Vue Router**: Navegación
- **Pinia**: Estado global
- **Axios**: Cliente HTTP
- **Bootstrap 5**: UI Framework
- **Vite**: Build tool

## Estructura de Componentes

```
src/
├── components/          # Componentes reutilizables
│   └── Navbar.vue      # Barra de navegación
├── stores/             # Estado global con Pinia
│   ├── auth.js        # Autenticación
│   └── cart.js        # Carrito de compras
├── services/          # Servicios API
│   └── api.js         # Configuración de Axios y servicios
├── views/             # Páginas principales
│   ├── Home.vue
│   ├── Login.vue
│   ├── Register.vue
│   ├── Products.vue
│   ├── Cart.vue
│   ├── Orders.vue
│   ├── Profile.vue
│   ├── ProductManagement.vue  # Admin
│   ├── OrderManagement.vue    # Admin
│   └── UserManagement.vue     # Admin
└── router/
    └── index.js       # Configuración de rutas
```

## Autenticación

El sistema utiliza JWT tokens para la autenticación:

1. El token se almacena en localStorage
2. Se incluye automáticamente en todas las peticiones
3. Se redirige a login si el token expira
4. Las rutas están protegidas por roles

## Rutas Disponibles

### Públicas:
- `/` - Página de inicio
- `/login` - Iniciar sesión
- `/register` - Registro
- `/products` - Lista de productos

### Autenticadas:
- `/cart` - Carrito de compras
- `/orders` - Mis órdenes
- `/profile` - Mi perfil

### Solo Administradores:
- `/admin/products` - Gestión de productos
- `/admin/orders` - Gestión de órdenes
- `/admin/users` - Gestión de usuarios

## API Endpoints Consumidos

### AuthService:
- `POST /register` - Registro de usuario
- `POST /login` - Inicio de sesión
- `POST /logout` - Cerrar sesión
- `GET /me` - Obtener usuario actual
- `PATCH /users/me` - Actualizar perfil
- `GET /users` - Listar usuarios (admin)
- `POST /users` - Crear usuario (admin)
- `PATCH /users/:id` - Actualizar usuario (admin)
- `DELETE /users/:id` - Eliminar usuario (admin)

### ProductService:
- `GET /productos` - Listar productos
- `GET /productos/activos` - Productos activos
- `POST /productos` - Crear producto (admin)
- `PUT /productos/:id` - Actualizar producto (admin)
- `DELETE /productos/:id` - Eliminar producto (admin)
- `GET /categorias` - Listar categorías

### InventoryService:
- `POST /orders` - Crear orden
- `GET /orders` - Mis órdenes
- `GET /orders/:id` - Detalle de orden
- `GET /admin/orders` - Todas las órdenes (admin)

## Variables de Entorno

El frontend utiliza las siguientes configuraciones:

- Puerto de desarrollo: `3000`
- Proxy automático a microservicios
- CORS configurado automáticamente

## Notas de Desarrollo

1. **Interceptores HTTP**: Configurados para manejar tokens JWT automáticamente
2. **Guards de Ruta**: Protección basada en autenticación y roles
3. **Estado Reactivo**: Pinia para manejo de estado global
4. **Componentes Reutilizables**: Bootstrap Vue para UI consistente
5. **Validación**: Validación del lado cliente con feedback visual

## Problemas Conocidos

1. El carrito persiste en localStorage (podría mejorarse con backend)
2. Las imágenes de productos usan URLs externas
3. No hay notificaciones en tiempo real
4. Falta paginación en listas largas

## Próximas Mejoras

- [ ] Notificaciones push
- [ ] Paginación en tablas
- [ ] Cache de resultados
- [ ] PWA capabilities
- [ ] Tests unitarios
- [ ] Internacionalización
