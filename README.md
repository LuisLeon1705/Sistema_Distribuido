
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

*   **Estado**: Funcional
*   **Descripción**: Este servicio es responsable del registro de usuarios, inicio de sesión y gestión de sesiones. Emitirá tokens JWT para ser consumidos por otros servicios. El esquema de la base de datos incluye una tabla de `users` con roles (`admin`, `customer`).

---

### 2. Servicio de Productos (`/backend/productservice`)

*   **Estado**: Funcional
*   **Descripción**: Este servicio gestionará el catálogo de productos. Está destinado a proporcionar operaciones CRUD para los productos, incluyendo detalles como nombre, descripción, precio y categoría. El esquema de la base de datos define una tabla de `products`.

---

### 3. Servicio de Inventario (`/backend/inventoryservice`)

*   **Estado**: Funcional
*   **Descripción**: Este servicio, construido en Rust utilizando el framework Axum, es el núcleo del sistema de gestión de inventario y pedidos. Se conecta a una base de datos PostgreSQL para administrar el stock y procesar los pedidos de los clientes.

#### Documentación de Endpoints de API del Servicio de Inventario

Este documento describe los endpoints de la API REST proporcionados por el Servicio de Inventario.

##### URL Base

Todos los endpoints son relativos a la URL base del servicio (por ejemplo, `http://localhost:8000`).

---

##### 1. Endpoints de Pedidos

###### `GET /orders`
Recupera una lista de todos los pedidos.
- **Método:** `GET`
- **URL:** `/orders`
- **Cuerpo de la Solicitud:** Ninguno
- **Respuesta:** `200 OK`
  ```json
  [
    {
      "id": 1,
      "user_id": "a1b2c3d4-e5f6-7890-1234-567890abcdef",
      "total_price": "123.45",
      "status": "pending",
      "created_at": "2023-10-27T10:00:00Z"
    }
  ]
  ```

###### `POST /orders`
Crea un nuevo pedido a partir de un pedido temporal asociado a un usuario.
- **Método:** `POST`
- **URL:** `/orders`
- **Cuerpo de la Solicitud:**
  ```json
  {
    "user_id": "a1b2c3d4-e5f6-7890-1234-567890abcdef",
    "items": []
  }
  ```
  *(Nota: El array `items` en el cuerpo de la solicitud es ignorado actualmente. Los ítems del pedido real se obtienen del pedido temporal asociado al `user_id`.)*
- **Respuesta:** `201 Created`
  ```json
  {
    "id": 1,
    "user_id": "a1b2c3d4-e5f6-7890-1234-567890abcdef",
    "total_price": "123.45",
    "status": "pending",
    "created_at": "2023-10-27T10:00:00Z"
  }
  ```
- **Respuestas de Error:**
  - `400 Bad Request`: Si no se encuentra un pedido temporal para el `user_id` o el pedido temporal no tiene ítems.
  - `500 Internal Server Error`: Para otros problemas del lado del servidor.

###### `GET /orders/:order_id`
Recupera un pedido específico por su ID.
- **Método:** `GET`
- **URL:** `/orders/{order_id}` (ej., `/orders/1`)
- **Cuerpo de la Solicitud:** Ninguno
- **Respuesta:** `200 OK`
  ```json
  {
    "id": 1,
    "user_id": "a1b2c3d4-e5f6-7890-1234-567890abcdef",
    "total_price": "123.45",
    "status": "pending",
    "created_at": "2023-10-27T10:00:00Z"
  }
  ```
- **Respuestas de Error:**
  - `404 Not Found`: Si el pedido con el ID dado no existe.
  - `500 Internal Server Error`: Para otros problemas del lado del servidor.

###### `GET /orders/:order_id/items`
Recupera todos los ítems pertenecientes a un pedido específico.
- **Método:** `GET`
- **URL:** `/orders/{order_id}/items` (ej., `/orders/1/items`)
- **Cuerpo de la Solicitud:** Ninguno
- **Respuesta:** `200 OK`
  ```json
  [
    {
      "id": 101,
      "order_id": 1,
      "product_id": "c1d2e3f4-g5h6-7890-1234-567890abcdef",
      "quantity": 2,
      "price_at_time_of_purchase": "50.00"
    }
  ]
  ```
- **Respuestas de Error:**
  - `500 Internal Server Error`: Para problemas del lado del servidor.

###### `GET /orders/user/:user_id`
Recupera todos los pedidos realizados por un usuario específico.
- **Método:** `GET`
- **URL:** `/orders/user/{user_id}` (ej., `/orders/user/a1b2c3d4-e5f6-7890-1234-567890abcdef`)
- **Cuerpo de la Solicitud:** Ninguno
- **Respuesta:** `200 OK`
  ```json
  [
    {
      "id": 1,
      "user_id": "a1b2c3d4-e5f6-7890-1234-567890abcdef",
      "total_price": "123.45",
      "status": "pending",
      "created_at": "2023-10-27T10:00:00Z"
    }
  ]
  ```
- **Respuestas de Error:**
  - `500 Internal Server Error`: Para problemas del lado del servidor.

###### `POST /orders/status`
Actualiza el estado de un pedido. Solo los pedidos con estado "pendiente" pueden ser actualizados. Si el estado se cambia a "cancelado", los ítems se devuelven al stock.
- **Método:** `POST`
- **URL:** `/orders/status`
- **Cuerpo de la Solicitud:**
  ```json
  {
    "order_id": 1,
    "new_status": "completed"
  }
  ```
  Valores posibles para `new_status`: `"completed"`, `"cancelled"`.
- **Respuesta:** `200 OK`
  ```json
  {
    "id": 1,
    "user_id": "a1b2c3d4-e5f6-7890-1234-567890abcdef",
    "total_price": "123.45",
    "status": "completed",
    "created_at": "2023-10-27T10:00:00Z"
  }
  ```
- **Respuestas de Error:**
  - `400 Bad Request`: Si el estado del pedido no es "pendiente" o para transiciones de estado inválidas.
  - `404 Not Found`: Si el pedido con el ID dado no existe.
  - `500 Internal Server Error`: Para otros problemas del lado del servidor.

---

##### 2. Endpoints de Stock

###### `POST /stock`
Añade nuevo stock para un producto.
- **Método:** `POST`
- **URL:** `/stock`
- **Cuerpo de la Solicitud:**
  ```json
  {
    "product_id": "f1g2h3i4-j5k6-7890-1234-567890abcdef",
    "quantity": 100,
    "warehouse_location": "Warehouse A"
  }
  ```
- **Respuesta:** `200 OK`
  ```json
  {
    "id": 1,
    "product_id": "f1g2h3i4-j5k6-7890-1234-567890abcdef",
    "quantity": 100,
    "last_updated": "2023-10-27T10:00:00Z",
    "warehouse_location": "Warehouse A"
  }
  ```
- **Respuestas de Error:**
  - `500 Internal Server Error`: Para problemas del lado del servidor.

###### `GET /stock`
Recupera una lista de todas las entradas de stock.
- **Método:** `GET`
- **URL:** `/stock`
- **Cuerpo de la Solicitud:** Ninguno
- **Respuesta:** `200 OK`
  ```json
  [
    {
      "id": 1,
      "product_id": "f1g2h3i4-j5k6-7890-1234-567890abcdef",
      "quantity": 100,
      "last_updated": "2023-10-27T10:00:00Z",
      "warehouse_location": "Warehouse A"
    }
  ]
  ```
- **Respuestas de Error:**
  - `500 Internal Server Error`: Para problemas del lado del servidor.

###### `GET /stock/:product_id`
Recupera información de stock para un producto específico.
- **Método:** `GET`
- **URL:** `/stock/{product_id}` (ej., `/stock/f1g2h3i4-j5k6-7890-1234-567890abcdef`)
- **Cuerpo de la Solicitud:** Ninguno
- **Respuesta:** `200 OK`
  ```json
  [
    {
      "id": 1,
      "product_id": "f1g2h3i4-j5k6-7890-1234-567890abcdef",
      "quantity": 100,
      "last_updated": "2023-10-27T10:00:00Z",
      "warehouse_location": "Warehouse A"
    }
  ]
  ```
- **Respuestas de Error:**
  - `500 Internal Server Error`: Para problemas del lado del servidor.

###### `PUT /stock/:product_id`
Actualiza la cantidad o la ubicación del almacén del stock de un producto específico.
- **Método:** `PUT`
- **URL:** `/stock/{product_id}` (ej., `/stock/f1g2h3i4-j5k6-7890-1234-567890abcdef`)
- **Cuerpo de la Solicitud:**
  ```json
  {
    "quantity": 120,
    "warehouse_location": "Warehouse B"
  }
  ```
  (Los campos son opcionales; solo se actualizarán los campos proporcionados)
- **Respuesta:** `200 OK`
  ```json
  {
    "id": 1,
    "product_id": "f1g2h3i4-j5k6-7890-1234-567890abcdef",
    "quantity": 120,
    "last_updated": "2023-10-27T10:00:00Z",
    "warehouse_location": "Warehouse B"
  }
  ```
- **Respuestas de Error:**
  - `404 Not Found`: Si la entrada de stock para el ID de producto dado no existe.
  - `500 Internal Server Error`: Para otros problemas del lado del servidor.

###### `DELETE /stock/:product_id`
Elimina una entrada de stock para un producto específico.
- **Método:** `DELETE`
- **URL:** `/stock/{product_id}` (ej., `/stock/f1g2h3i4-j5k6-7890-1234-567890abcdef`)
- **Cuerpo de la Solicitud:** Ninguno
- **Respuesta:** `204 No Content`
- **Respuestas de Error:**
  - `404 Not Found`: Si la entrada de stock para el ID de producto dado no existe.
  - `500 Internal Server Error`: Para otros problemas del lado del servidor.

---

##### 3. Endpoints de Pedidos Temporales

Los pedidos temporales se almacenan en un archivo JSON local (`data/order_list.json`) y se utilizan para guardar la información del pedido antes de que se finalice y se mueva a la base de datos principal.

###### `POST /temp_orders`
Añade un nuevo pedido temporal. Si ya existe un pedido temporal con el mismo `user_id`, será reemplazado.
- **Método:** `POST`
- **URL:** `/temp_orders`
- **Cuerpo de la Solicitud:**
  ```json
  {
    "user_id": "a1b2c3d4-e5f6-7890-1234-567890abcdef",
    "items": [
      {
        "product_id": "b2c3d4e5-f6a7-8901-2345-67890abcdef1",
        "quantity": 1,
        "price": "10.00"
      },
      {
        "product_id": "c3d4e5f6-a7b8-9012-3456-7890abcdef21",
        "quantity": 3,
        "price": "5.50"
      }
    ]
  }
  ```
- **Respuesta:** `201 Created`
  ```json
  {
    "user_id": "a1b2c3d4-e5f6-7890-1234-567890abcdef",
    "items": [
      {
        "product_id": "b2c3d4e5-f6a7-8901-2345-67890abcdef1",
        "quantity": 1,
        "price": "10.00"
      }
    ],
    "created_at": "2023-10-27T10:00:00Z"
  }
  ```
  *(Nota: La `quantity` en la respuesta podría ajustarse si excede el stock disponible.)*
- **Respuestas de Error:**
  - `500 Internal Server Error`: Para problemas del lado del servidor.

###### `GET /temp_orders/user/:user_id`
Recupera todos los pedidos temporales asociados a un usuario específico.
- **Método:** `GET`
- **URL:** `/temp_orders/user/{user_id}` (ej., `/temp_orders/user/a1b2c3d4-e5f6-7890-1234-567890abcdef`)
- **Cuerpo de la Solicitud:** Ninguno
- **Respuesta:** `200 OK`
  ```json
  [
    {
      "user_id": "a1b2c3d4-e5f6-7890-1234-567890abcdef",
      "items": [
        {
          "product_id": "b2c3d4e5-f6a7-8901-2345-67890abcdef1",
          "quantity": 1,
          "price": "10.00"
        }
      ],
      "created_at": "2023-10-27T10:00:00Z"
    }
  ]
  ```
- **Respuestas de Error:**
  - `500 Internal Server Error`: Para problemas del lado del servidor.

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
