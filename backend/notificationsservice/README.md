# Notifications Service

Microservicio de notificaciones desarrollado en Go con Gin framework. Proporciona funcionalidades para enviar notificaciones por email y mantener un registro de todas las notificaciones del sistema.

## Tecnologías

- **Go 1.21**
- **Gin Framework** - Web framework
- **JWT (golang-jwt/jwt)** - Autenticación y autorización
- **GoMail** - Envío de emails
- **UUID** - Generación de identificadores únicos

## Características

- ✅ Envío de notificaciones por email con plantillas HTML
- ✅ Notificaciones de creación de pedidos
- ✅ Notificaciones de cambio de estado de pedidos
- ✅ Almacenamiento en memoria de historial de notificaciones
- ✅ Validación JWT para endpoints protegidos
- ✅ CORS habilitado
- ✅ Dockerizado

## Endpoints

### Públicos (Para microservicios internos)

```bash
POST /notifications/order-created
POST /notifications/status-change
POST /notifications
GET /notifications
```

### Protegidos con JWT

```bash
POST /api/notifications          # Crear notificación (requiere autenticación)
GET /api/notifications           # Obtener todas (solo admin/staff)
GET /api/notifications/:id       # Obtener por ID
```

### Health Check

```bash
GET /health
```

## Ejemplos de Uso

### 1. Notificar creación de pedido

```bash
POST http://localhost:8004/notifications/order-created
Content-Type: application/json

{
  "order_id": "ORD-12345",
  "customer_email": "cliente@example.com",
  "customer_name": "Juan Pérez",
  "total_amount": 150.00,
  "items": [
    {
      "product_name": "Producto 1",
      "quantity": 2,
      "price": 50.00
    },
    {
      "product_name": "Producto 2",
      "quantity": 1,
      "price": 50.00
    }
  ]
}
```

### 2. Notificar cambio de estado

```bash
POST http://localhost:8004/notifications/status-change
Content-Type: application/json

{
  "order_id": "ORD-12345",
  "customer_email": "cliente@example.com",
  "old_status": "pending",
  "new_status": "shipped",
  "message": "Tu pedido ha sido enviado y está en camino"
}
```

### 3. Crear notificación personalizada

```bash
POST http://localhost:8004/notifications
Content-Type: application/json

{
  "type": "general",
  "recipient": "usuario@example.com",
  "subject": "Notificación importante",
  "message": "<h1>Hola</h1><p>Este es un mensaje importante.</p>",
  "order_id": "ORD-12345"
}
```

### 4. Obtener todas las notificaciones

```bash
GET http://localhost:8004/notifications
```

## Variables de Entorno

```env
PORT=8000
JWT_SECRET_KEY=tu_clave_secreta_jwt
JWT_ALGORITHM=HS256

# SMTP Configuration
SMTP_HOST=mail.ejemplo.com
SMTP_PORT=465
SMTP_USER=servicio@ejemplo.com
SMTP_PASSWORD=tu_password
SMTP_USE_TLS=false
EMAIL_FROM=servicio@ejemplo.com
```

## Docker

### Construir imagen

```bash
docker build -t notificationsservice .
```

### Ejecutar contenedor

```bash
docker run -p 8004:8000 --env-file .env notificationsservice
```

### Docker Compose

El servicio está incluido en el docker-compose.yml principal:

```bash
docker-compose up notificationsservice
```

## Desarrollo Local

### Prerrequisitos

- Go 1.21 o superior

### Instalación

```bash
# Clonar el repositorio y navegar al directorio
cd backend/notificationsservice

# Descargar dependencias
go mod download

# Ejecutar en modo desarrollo
go run .
```

El servicio estará disponible en `http://localhost:8000`

## Estructura del Proyecto

```
notificationsservice/
├── main.go              # Punto de entrada
├── models.go            # Modelos de datos
├── database.go          # Base de datos en memoria
├── handlers.go          # Manejadores HTTP
├── routes.go            # Definición de rutas
├── middleware.go        # Middleware JWT
├── email.go             # Funcionalidad de email
├── Dockerfile           # Configuración Docker
├── .env                 # Variables de entorno
├── go.mod               # Dependencias
└── README.md            # Documentación
```

## Autenticación

Los endpoints bajo `/api/*` requieren un token JWT válido en el header:

```
Authorization: Bearer <token>
```

El token debe contener los claims:
- `sub`: ID del usuario
- `role`: Rol del usuario (admin, staff, customer)

## Configuración SMTP

El servicio soporta:
- SMTP con SSL/TLS (puerto 465)
- SMTP con STARTTLS (puerto 587)

Si no se configura SMTP, las notificaciones se registrarán en los logs en lugar de enviarse.

## Integración con Otros Servicios

### Desde Orders Service (Java/Spring)

```java
@Service
public class NotificationClient {
    private final RestTemplate restTemplate;
    
    public void notifyOrderCreated(OrderCreatedRequest request) {
        String url = "http://notificationsservice:8000/notifications/order-created";
        restTemplate.postForEntity(url, request, String.class);
    }
}
```

### Desde cualquier servicio

```bash
curl -X POST http://notificationsservice:8000/notifications/order-created \
  -H "Content-Type: application/json" \
  -d '{
    "order_id": "123",
    "customer_email": "cliente@example.com",
    "customer_name": "Juan",
    "total_amount": 100.00,
    "items": []
  }'
```

## Base de Datos

El servicio utiliza una base de datos en memoria (mapa concurrente) para almacenar las notificaciones. Los datos se pierden al reiniciar el servicio.

Para persistencia permanente, considera:
- Integrar con PostgreSQL
- Usar Redis para cache
- Implementar event sourcing

## Plantillas de Email

Las notificaciones incluyen plantillas HTML responsivas con:
- Diseño profesional
- Colores temáticos según el tipo de notificación
- Emojis y formato atractivo
- Compatible con la mayoría de clientes de email

## Estados de Notificación

- `pending`: Notificación creada, esperando envío
- `sent`: Email enviado exitosamente
- `failed`: Error al enviar email

## Mejoras Futuras

- [ ] Persistencia en base de datos (PostgreSQL)
- [ ] Soporte para SMS
- [ ] Soporte para push notifications
- [ ] Plantillas personalizables
- [ ] Sistema de colas (RabbitMQ/Kafka)
- [ ] Reintentos automáticos para envíos fallidos
- [ ] Dashboard de métricas
- [ ] Webhooks para eventos
