# Documentación API - Microservicio de Órdenes (Java)

Este documento detalla los endpoints disponibles en el servicio de Órdenes (`ordersservice`).
Este servicio se encarga de crear, listar y validar órdenes, comunicándose con el servicio de Productos (PHP) para obtener precios reales.

## Configuración Base

* **URL Base Local:** `http://localhost:8081/api/orders`
* **Formato de Datos:** JSON (`application/json`)

## Autenticación

Todos los endpoints están protegidos. Se debe enviar el Token JWT obtenido del `authservice`.

| Header | Valor | Descripción |
| :--- | :--- | :--- |
| `Authorization` | `Bearer <TU_TOKEN_JWT>` | Token de acceso (debe empezar con la palabra "Bearer " seguida de un espacio). |

---

## 1. Crear una Orden

Crea una nueva orden de compra. El sistema conectará internamente con el servicio de Productos para validar que el producto exista y obtener su precio real (ignorando cualquier precio enviado por el cliente para evitar fraudes).

* **Método:** `POST`
* **URL:** `/` (Raíz del servicio)
* **URL Completa:** `http://localhost:8081/api/orders`

### Body (Request)

```json
{
  "items": [
    {
      "productId": "550e8400-e29b-41d4-a716-446655440000",
      "quantity": 2
    },
    {
      "productId": "770e8400-e29b-41d4-a716-999999999999",
      "quantity": 1
    }
  ]
}
```

### Tabla de Parámetros:

| Campo | Tipo | Obligatorio | Descripcion |
| :--- | :--- | :--- | :--- |
| `items` | `Array` | `Sí` | Lista de objetos con los productos a comprar. |
| `items[].productId` | `String` | `Sí` | El UUID o ID del producto (debe existir en el servicio de PHP). |
| `items[].quantity` | `Integer` | `Sí` | Cantidad de unidades a comprar. |
| `items[].price` | `Decimal` | `No` | Ignorado por el backend. El precio se obtiene en tiempo real desde la BD de Productos. |

---

### Respuesta Exitosa (200 OK) 

```json
{
    "id": "c9f5d3a1-...",
    "userId": "a1b2c3d4-...",
    "total": 550.00,
    "status": "CREADO",
    "createdAt": "2026-01-10T03:30:00",
    "items": [
        {
            "id": 1,
            "productId": "550e8400-...",
            "quantity": 2,
            "price": 275.00
        }
    ]
}
```

### Respuesta de Error (404 Not Found)
Ocurre si el ID del producto no existe en el servicio de PHP.

```json
{
    "error": "Producto no encontrado",
    "mensaje": "El ID del producto solicitado no existe en el catálogo externo."
}
```
---

## 2. Obtener Mis Órdenes

Devuelve una lista de todas las órdenes creadas por el usuario autenticado. Filtra automáticamente usando el ID dentro del token JWT.

* **Método:** `GET`
* **URL:** `/` (Raíz del servicio)
* **URL Completa:** `http://localhost:8081/api/orders`

### Body
No requiere cuerpo

### Respuesta Exitosa (200 OK) 
Devuelve un Array de objetos Orden.

```json
[
    {
        "id": "c9f5d3a1-...",
        "total": 150.00,
        "status": "CREADO",
        "items": [...]
    },
    {
        "id": "d8e4f2b2-...",
        "total": 80.50,
        "status": "CREADO",
        "items": [...]
    }
]
```

---

## 3. Obtener Orden por ID

Obtiene el detalle de una orden específica. Restricción de Seguridad: Solo el dueño de la orden puede verla. Si intentas ver la orden de otro usuario, recibirás un error 403.

* **Método:** `GET`
* **URL:** `/{id}`
* **URL Completa:** `http://localhost:8081/api/orders/c9f5d3a1-4e7d-4e3f-83d9-c2effaf66cb3`

### Body

| Parámetro | Tipo | Descripción |
| :--- | :--- | :--- |
| `id` | `UUID` | El identificador único de la orden que se quiere consultar. |


### Respuesta Exitosa (200 OK) 
Devuelve el objeto Orden completo (igual que en Crear Orden).

### Errores Comunes

| Código | Error | Causa |
| :--- | :--- | :--- |
| `404` | `Not Found` | La orden con ese ID no existe en la base de datos. |
| `403` | `Forbidden` | La orden existe, pero pertenece a otro usuario. "No tienes permiso para ver esta orden". |

---

## 4. Obtener Todas las Órdenes (ADMIN)
Devuelve el historial completo de órdenes de todos los usuarios. Restricción: El token JWT debe contener el claim "role": "admin".

* **Método:** `GET`
* **URL:** `/all`
* **URL Completa:** `http://localhost:8081/api/orders/all`


### Body
No requiere cuerpo

### Respuesta
* **Si es Admin::** `Devuelve un Array JSON con todas las órdenes del sistema (200 OK).`
* **Si es Usuario Normal:** Devuelve 403 Forbidden con mensaje: "Acceso denegado. Se requiere rol de administrador.".