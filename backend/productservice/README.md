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
| `GET` | `/productos/{id}` | Obtiene los detalles de un producto específico por su ID. |
| `GET` | `/productos/categoria/{id}` | Filtra y lista los productos asociados a una categoría específica. |
| `GET` | `/productos/activos` | Lista únicamente los productos visibles (`estado = true`). |
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
| `precio` | Numérico | Sí | Precio del producto. Debe ser un número válido (ej. 10.50). |
| `id_categoria` | Entero | Sí | ID válido de una categoría existente en la base de datos. |
| `descripcion` | Texto | No | Descripción detallada del producto. |
| `estado` | Booleano | No | `1` para Activo (visible), `0` para Inactivo (oculto). Por defecto es true si no se envía. |
| `imagen` | **Archivo** | No | Archivo de imagen (jpeg, png, jpg, webp). Tamaño máximo: 100MB. Se subirá automáticamente a AWS S3. |

**Ejemplo de Respuesta Exitosa (201 Created):**

```json
{
    "mensaje": "Producto creado exitosamente",
    "producto": {
        "nombre": "Laptop Gamer",
        "precio": 1500,
        "id_categoria": "1",
        "descripcion": "Potente laptop para juegos",
        "estado": "1",
        "imagen_url": "[https://bucket.s3.amazonaws.com/productos/a1b2c3d4.jpg](https://bucket.s3.amazonaws.com/productos/a1b2c3d4.jpg)",
        "updated_at": "2023-11-27T16:00:00.000000Z",
        "created_at": "2023-11-27T16:00:00.000000Z",
        "id": 15
    }
}
```