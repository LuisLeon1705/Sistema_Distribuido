# Microservicio de Productos (Backend)

##### (El puerto por defecto es :49)

Este es el servicio encargado de la gestión del catálogo de productos e inventario lógico. Está construido con **PHP 8.2 (Laravel)**, utiliza **PostgreSQL** como base de datos y **AWS S3** para el almacenamiento de imágenes en la nube.

## Tecnologías

* **Framework:** Laravel 12
* **Lenguaje:** PHP 8.2
* **Base de Datos:** PostgreSQL 15
* **Contenedores:** Docker & Docker Compose
* **Almacenamiento:** AWS S3 (Flysystem)

---

## Configuración del Entorno (Variables)

Para que el proyecto funcione, debes crear un archivo `.env` en la raíz (basado en `.env.example`) y configurar las siguientes variables críticas:

### 1. Base de Datos
Asegúrate de que coincidan con lo definido en `docker-compose.yml`.
```ini
DB_CONNECTION=pgsql
DB_HOST=db
DB_PORT=5432
DB_DATABASE=products_db
DB_USERNAME=
DB_PASSWORD=
```

---

### Categorías

Gestión de las categorías para clasificar los productos.

| Método | Endpoint | Descripción |
| :--- | :--- | :--- |
| `GET` | `/categorias` | Obtiene la lista completa de categorías. |
| `GET` | `/categorias/{id}` | Obtiene los detalles de una categoría específica. |
| `POST` | `/categorias` | Crea una nueva categoría. |
| `PUT` | `/categorias/{id}` | Actualiza el nombre o descripción de una categoría. |
| `DELETE` | `/categorias/{id}` | Elimina una categoría. |

#### Crear / Editar Categoría (POST/PUT)
**Headers:**
* `Accept: application/json`
* `Content-Type: application/json`

**Body (JSON):**
```json
{
  "nombre": "Electrónica",
  "descripcion": "Dispositivos, gadgets y accesorios"
}
```

#### Respuesta correcta (200 - 201)
```json
{
    "id": 1,
    "nombre": "Electrónica",
    "codigo" : "ELE",
    "descripcion": "Dispositivos, gadgets y accesorios",
    "created_at": "2023-10-27T10:00:00.000000Z",
    "updated_at": "2023-10-27T10:00:00.000000Z"
}
```

---

### Productos

Gestión de productos, incluyendo carga de imágenes a AWS S3.

| Método | Endpoint | Descripción |
| :--- | :--- | :--- |
| `GET` | `/productos` | Lista todos los productos registrados. |
| `GET` | `/productos/activos` | Lista únicamente los productos visibles (`estado = true`). |
| `GET` | `/productos/{id}` | Obtiene los detalles de un producto específico por su ID. |
| `GET` | `/productos/codigo/{codigo}` | Obtiene el producto asociados a un codigo específico. |
| `GET` | `/productos/categoria/{id}` | Filtra y lista los productos asociados a una categoría específica. |
| `POST` | `/productos` | Crea un nuevo producto con imagen. |
| `PUT` | `/productos/{id}` | Actualiza los datos de un producto existente. |
| `DELETE` | `/productos/{id}` | Elimina un producto de la base de datos. |

#### Detalles del Endpoint: Crear Producto (POST)

Este endpoint es especial porque requiere subir un archivo (imagen). Por lo tanto, no se debe enviar como un JSON crudo (`application/json`), sino como un formulario multipart (`multipart/form-data`).

**Configuración de la Petición:**

* **URL:** `http://localhost:49/api/productos`
* **Método:** `POST`
* **Header Obligatorio:** `Accept: application/json`
* **Header de Contenido:** `Content-Type: multipart/form-data` (Generalmente Postman/Axios lo añaden automáticamente al detectar archivos).

**Parámetros del Cuerpo (Body - Form Data):**

A continuación se describen los campos que se deben enviar dentro del formulario:

| Campo | Tipo | Requerido | Descripción |
| :--- | :--- | :--- | :--- |
| `nombre` | Texto | Sí | Nombre comercial del producto. Máximo 255 caracteres. |
| `detalles` | Texto | No | Características del producto. |
| `precio` | Numérico | Sí | Precio del producto. Debe ser un número válido (ej. 10.50). |
| `id_categoria` | Entero | Sí (Si no existe categoria_id) | ID válido de una categoría existente en la base de datos. |
| `categoria_id` | Entero | Sí (Si no existe id_categoria)| ID válido de una categoría existente en la base de datos. |
| `descripcion` | Texto | No | Descripción detallada del producto. |
| `imagen` | **Archivo** | No | Archivo de imagen (jpeg, png, jpg, webp). Tamaño máximo: 100MB. Se subirá automáticamente a AWS S3. |
| `imagen_url` | URL | No | URL de una imagen representativa del producto |
| `estado` | Booleano | No | `1` para Activo (visible), `0` para Inactivo (oculto). Por defecto es true si no se envía. |

**Ejemplo de Respuesta Exitosa (201 Created):**

```json
{
    "mensaje": "Producto creado exitosamente",
    "producto": {
        "id": uuid
        "codigo" : "ELE-000001",
        "nombre": "Laptop Gamer",
        "detalles": "RGB",
        "precio": 1500,
        "id_categoria": "1",
        "descripcion": "Potente laptop para juegos",
        "imagen_url": "[https://bucket.s3.amazonaws.com/productos/a1b2c3d4.jpg](https://bucket.s3.amazonaws.com/productos/a1b2c3d4.jpg)",
        "estado": "1",
        "updated_at": "2023-11-27T16:00:00.000000Z",
        "created_at": "2023-11-27T16:00:00.000000Z",
    }
}
```

## Seguridad

Este microservicio utiliza **JWT (JSON Web Tokens)** generados por el Auth Service.

* **Rutas Públicas:** Accesibles por cualquier usuario sin necesidad de token.
* **Rutas Protegidas:** Requieren un token válido.

**Para acceder a rutas protegidas:**
Debes enviar el token en el encabezado de la petición:
`Authorization: Bearer <token>`

### JWT (JSON Web Tokens)
Para validar correctamente los tokens emitidos por el servicio de autenticación debes configurar las mismas variables utilizadas por `authservice`:

```
JWT_SECRET_KEY=auth-token-secret-key-123
JWT_ALGORITHM=HS256
```
Estas variables pueden establecerse en `docker-compose.yml` o en el `.env` local del servicio de productos.

---

## Rutas y su seguridad

El sistema valida el rol del usuario a través del token JWT. Las rutas de modificación requieren permisos de **Administrador**.

| Modo | Endpoint | Acceso |
| :--- | :--- | :--- |
| **GET** | `/api/productos` | Público |
| **GET** | `/api/productos/activos` | Público |
| **GET** | `/api/productos/{id}` | Público |
| **GET** | `/api/productos/codigo/{codigo}` | Público |
| **GET** | `/api/productos/categoria/{id}` | Público |
| **GET** | `/api/categorias` | Público |
| **GET** | `/api/categorias/{id}` | Público |
| **POST** | `/api/productos` | **Admin** |
| **PUT** | `/api/productos/{id}` | **Admin** |
| **DELETE** | `/api/productos/{id}` | **Admin** |
| **POST** | `/api/categorias` | **Admin** |
| **PUT** | `/api/categorias/{id}` | **Admin** |
| **DELETE** | `/api/categorias/{id}` | **Admin** |

> **Nota:** Para las rutas marcadas como **Admin**, se debe enviar el header:
> `Authorization: Bearer <token_admin>`