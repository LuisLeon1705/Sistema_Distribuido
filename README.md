
# Documentación del Proyecto

Este documento proporciona una visión general de la arquitectura de microservicios para el proyecto Gestor.

## Servicios

El backend se compone de tres servicios principales:

1.  **Servicio de Autenticación**: Gestiona la autenticación y autorización de usuarios.
2.  **Servicio de Productos**: Administra la información de los productos.
3.  **Servicio de Inventario**: Administra los niveles de stock y los pedidos de los clientes.

El proyecto también incluye una aplicación frontend simple.

---

### 1. Servicio de Autenticación (`/backend/authservice`)

*   **Estado**: Incompleto
*   **Descripción**: Este servicio es responsable del registro de usuarios, inicio de sesión y gestión de sesiones. Emitirá tokens JWT para ser consumidos por otros servicios. El esquema de la base de datos incluye una tabla de `users` con roles (`admin`, `customer`).

---

### 2. Servicio de Productos (`/backend/productservice`)

*   **Estado**: Incompleto
*   **Descripción**: Este servicio gestionará el catálogo de productos. Está destinado a proporcionar operaciones CRUD para los productos, incluyendo detalles como nombre, descripción, precio y categoría. El esquema de la base de datos define una tabla de `products`.

---

### 3. Servicio de Inventario (`/backend/inventoryservice`)

*   **Estado**: Funcional
*   **Descripción**: Este servicio, construido en Rust utilizando el framework Axum, es el núcleo del sistema de gestión de inventario y pedidos. Se conecta a una base de datos PostgreSQL para administrar el stock y procesar los pedidos de los clientes.

#### Características Clave:

*   **Gestión de Stock**: Proporciona endpoints para que los administradores listen, aumenten y disminuyan el stock de productos.
*   **Procesamiento de Pedidos**: Implementa un proceso de creación de pedidos en dos fases para garantizar la consistencia y fiabilidad de los datos.
*   **Autenticación**: Asegura los endpoints mediante tokens JWT, con rutas específicas restringidas a roles de `admin`.

#### Proceso de Pedido en Dos Fases:

Para evitar problemas como condiciones de carrera o pedidos abandonados que bloquean el stock, se utiliza un proceso similar a un "two-phase commit":
1.  **Crear Pedido Temporal**: Un usuario primero crea un pedido temporal con los artículos deseados. Este pedido se mantiene en memoria y tiene un Tiempo de Vida (TTL) limitado. No afecta el stock de la base de datos.
2.  **Confirmar Pedido**: El usuario luego confirma el pedido temporal. En este punto, el servicio se comunica con el Servicio de Productos para obtener los precios finales, valida la disponibilidad de stock en su propia base de datos, crea el pedido oficial y reduce el stock de forma atómica dentro de una transacción de base de datos.

#### Endpoints de la API:

*   `GET /inventory`: Lista el inventario completo. (Solo admin)
*   `POST /inventory/increase`: Aumenta el stock de un producto determinado. (Solo admin)
*   `POST /inventory/decrease`: Disminuye el stock de un producto determinado. (Solo admin)
*   `POST /orders`: Crea un pedido temporal. Devuelve un `temporal_order_id`. (Usuario autenticado)
*   `POST /orders/confirm/:id`: Confirma un pedido temporal usando su ID, creando un pedido permanente y actualizando el stock. (Usuario autenticado)
*   `GET /orders`: Recupera el historial de pedidos del usuario autenticado. (Usuario autenticado)
*   `GET /orders/:id`: Recupera los detalles de un pedido específico. (Usuario autenticado)
*   `GET /admin/orders`: Recupera todos los pedidos del sistema. (Solo admin)

---

### 4. Frontend (`/frontend`)

*   **Estado**: En construcción
*   **Descripción**: Un simple `index.html` de marcador de posición servido por un contenedor Nginx. Eventualmente se desarrollará en una interfaz de usuario completa para interactuar con los servicios del backend.

---

## Cómo Ejecutar

Toda la pila de la aplicación se puede iniciar usando Docker Compose.

```bash
docker-compose up --build
```

Este comando construirá las imágenes para cada servicio e iniciará los contenedores.
