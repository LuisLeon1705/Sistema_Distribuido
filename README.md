# Sistema de Gestión Distribuido

Este proyecto es una aplicación web de comercio electrónico construida sobre una arquitectura de microservicios. Incluye funcionalidades para la gestión de productos, inventario, pedidos y autenticación de usuarios. El frontend está desarrollado con Vue.js y el backend se compone de múltiples servicios construidos con Python (FastAPI), PHP (Laravel) y Rust (Axum).

## Características

-   **Autenticación de Usuarios**: Registro, inicio de sesión y gestión de sesiones con tokens JWT.
-   **Verificación de Correo Electrónico**: Proceso de registro seguro con verificación por código.
-   **Gestión de Productos**: Operaciones CRUD para productos y categorías.
-   **Gestión de Inventario**: Seguimiento del stock de productos en tiempo real.
-   **Carrito de Compras Persistente**: El carrito de compras se guarda en el backend y se sincroniza entre sesiones.
-   **Gestión de Pedidos**: Creación y seguimiento de pedidos de clientes.
-   **Roles de Usuario**: Roles de Administrador y Cliente con diferentes permisos.
-   **Paneles de Administración**: Interfaces para gestionar productos, pedidos y usuarios.

## Arquitectura

El sistema está compuesto por los siguientes microservicios:

1.  **Frontend (`/frontend`)**: Una aplicación de una sola página (SPA) construida con **Vue.js**. Sirve como la interfaz de usuario para clientes y administradores.
2.  **Servicio de Autenticación (`/backend/authservice`)**: Construido con **Python (FastAPI)**, gestiona el registro de usuarios, inicio de sesión, validación de correo electrónico y roles.
3.  **Servicio de Productos (`/backend/productservice`)**: Construido con **PHP (Laravel)**, se encarga de toda la información relacionada con los productos y categorías (nombres, descripciones, precios, etc.).
4.  **Servicio de Inventario (`/backend/inventoryservice`)**: Construido con **Rust (Axum)**, gestiona el stock de los productos y los pedidos de los clientes. También maneja la lógica del carrito de compras temporal.

Todos los servicios están orquestados usando Docker y se comunican a través de una red definida en `docker-compose.yml`.

## Cómo Empezar

Sigue estos pasos para levantar el entorno de desarrollo completo.

### Prerrequisitos

-   Tener instalado [Docker](https://www.docker.com/get-started/) y Docker Compose.

### Instalación y Ejecución

1.  Clona este repositorio en tu máquina local.
2.  Abre una terminal en el directorio raíz del proyecto.
3.  Ejecuta el siguiente comando para construir e iniciar todos los servicios en segundo plano:

    ```bash
    docker-compose up --build -d
    ```

4.  Una vez que todos los contenedores estén en funcionamiento, la aplicación estará accesible.

## Uso de la Aplicación

-   **Aplicación Frontend**: Abre tu navegador y ve a `http://localhost:3000`.
-   **API de Autenticación**: Accesible en `http://localhost:8001`.
-   **API de Inventario**: Accesible en `http://localhost:8002`.
-   **API de Productos**: Accesible en `http://localhost:8003`.

### Credenciales de Administrador

Se crea un usuario administrador por defecto para facilitar las pruebas y la gestión inicial.

-   **Usuario**: `admin@admin.com`
-   **Contraseña**: `-Admin123-`

### Proceso de Verificación de Correo

Cuando un nuevo usuario se registra, el sistema sigue estos pasos:

1.  El usuario se registra con su nombre de usuario, correo electrónico y contraseña.
2.  El servicio de autenticación envía un código de verificación de 6 dígitos al correo electrónico proporcionado.
3.  **Importante**: Dado que este es un entorno de desarrollo local sin un servidor de correo real, el código de verificación se **imprime en la consola del contenedor del `authservice`**.
4.  Para ver el código, revisa los logs del servicio de autenticación:
    ```bash
    docker logs authservice
    ```
5.  Busca una línea que se parezca a: `Verification code for ... is ...`.
6.  Introduce este código en la página de verificación de la aplicación frontend para activar la cuenta.

---

<details>
<summary>Documentación de la API del Servicio de Autenticación y Usuarios</summary>

El servicio de autenticación y usuarios se expone en `http://localhost:8001` y se divide en dos prefijos principales: `/auth` y `/users`.

### Endpoints de Autenticación (`/auth`)

-   **`POST /register`**: Registra un nuevo usuario (rol "customer" por defecto).
-   **`POST /login`**: Inicia sesión y devuelve un token JWT.
-   **`POST /logout`**: Cierra la sesión del usuario.
-   **`POST /send-verification-code`**: Reenvía un código de verificación al correo del usuario.
-   **`POST /verify-email`**: Verifica el correo de un usuario usando el código recibido.

### Endpoints de Usuarios (`/users`)

-   **`GET /me`**: Obtiene el perfil del usuario autenticado actualmente.
-   **`PATCH /me`**: Actualiza el perfil del usuario autenticado actualmente.
-   **`POST /`** `(Admin)`: Crea un nuevo usuario.
-   **`GET /`** `(Admin)`: Lista todos los usuarios con opciones de filtrado.
-   **`GET /{user_id}`** `(Admin)`: Obtiene un usuario específico por su ID.
-   **`PATCH /{user_id}`** `(Admin)`: Actualiza los datos de un usuario específico.
-   **`DELETE /{user_id}`** `(Admin)`: Elimina un usuario específico.

</details>

<details>
<summary>Documentación de la API del Servicio de Productos</summary>

El servicio de productos se expone en `http://localhost:8003` con un prefijo `/api`.

### Endpoints de Productos (`/api/productos`)

-   **`GET /productos`**: Obtiene una lista de todos los productos.
-   **`GET /productos/activos`**: Obtiene una lista de los productos con estado "activo".
-   **`GET /productos/{id}`**: Obtiene un producto específico por su ID.
-   **`GET /productos/codigo/{codigo}`**: Obtiene un producto por su código.
-   **`GET /productos/categoria/{id}`**: Obtiene todos los productos de una categoría específica.
-   **`POST /productos`** `(Requiere JWT)`: Crea un nuevo producto.
-   **`PUT /productos/{id}`** `(Requiere JWT)`: Actualiza un producto existente.
-   **`DELETE /productos/{id}`** `(Requiere JWT)`: Elimina un producto.

### Endpoints de Categorías (`/api/categorias`)

-   **`GET /categorias`**: Obtiene una lista de todas las categorías.
-   **`GET /categorias/{id}`**: Obtiene una categoría específica por su ID.
-   **`POST /categorias`** `(Requiere JWT)`: Crea una nueva categoría.
-   **`PUT /categorias/{id}`** `(Requiere JWT)`: Actualiza una categoría existente.
-   **`DELETE /categorias/{id}`** `(Requiere JWT)`: Elimina una categoría.

</details>

<details>
<summary>Documentación de la API del Servicio de Inventario</summary>

El servicio de inventario se expone en `http://localhost:8002`.

### 1. Endpoints de Pedidos

#### `GET /orders`
Recupera una lista de todos los pedidos.

#### `POST /orders`
Crea un nuevo pedido a partir de un pedido temporal.

#### `GET /orders/:order_id`
Recupera un pedido específico por su ID.

#### `GET /orders/:order_id/items`
Recupera todos los ítems de un pedido específico.

#### `GET /orders/user/:user_id`
Recupera todos los pedidos de un usuario específico.

#### `POST /orders/status`
Actualiza el estado de un pedido (`completed` o `cancelled`).

---

### 2. Endpoints de Stock

#### `POST /stock`
Añade nuevo stock para un producto.

#### `GET /stock`
Recupera una lista de todas las entradas de stock.

#### `GET /stock/:product_id`
Recupera el stock de un producto específico.

#### `PUT /stock/:product_id`
Actualiza el stock de un producto.

#### `DELETE /stock/:product_id`
Elimina una entrada de stock.

---

### 3. Endpoints de Pedidos Temporales

#### `POST /temp_orders`
Añade o actualiza un pedido temporal (carrito de compras) para un usuario. El backend ajusta las cantidades según el stock disponible y devuelve el estado final del carrito.

#### `GET /temp_orders/user/:user_id`
Recupera el pedido temporal de un usuario.

</details>