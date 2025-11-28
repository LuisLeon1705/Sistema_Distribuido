# AuthService

Servicio de autenticación y gestión de usuarios para el sistema distribuido. Expone una API basada en FastAPI para registro, login, verificación de correo, gestión de roles y usuarios.

## Tecnologías utilizadas

- **Lenguaje**: Python 3.12
- **Framework web**: FastAPI
- **ORM**: SQLAlchemy
- **Base de datos**: PostgreSQL
- **Autenticación**: JWT (JSON Web Tokens) + cookies HTTP-only
- **Gestión de configuración**: variables de entorno (.env) y modelo Pydantic (`Settings`)
- **Servidor ASGI**: Uvicorn

## Arquitectura del servicio

Estructura principal del directorio `app/`:

- `main.py`: punto de entrada de FastAPI, registra routers y crea el usuario admin por defecto si no existe.
- `config.py`: carga de variables de entorno usando Pydantic (`Settings`).
- `database.py`: configuración de SQLAlchemy (engine, sesión, `get_db`).
- `models.py`: modelos ORM (tablas `users`, `roles`, etc.).
- `schemas.py`: modelos Pydantic (requests/responses) y validaciones.
- `security.py`: utilidades de seguridad (hash de contraseñas, JWT, dependencias de seguridad).
- `routers/`:
  - `auth.py`: endpoints de autenticación y verificación de correo.
  - `users.py`: endpoints de gestión de usuarios (admin y self-service).

## Modelo de datos (resumen)

Tablas principales en PostgreSQL (ver `db/schema.sql`):

- **roles**
  - `id` (PK, serial)
  - `name` (único, ej: `admin`, `customer`, `inventory`)
  - `description`
  - `created_at`

- **users**
  - `id` (UUID, PK)
  - `username` (único)
  - `email` (único)
  - `password_hash` (hash bcrypt u otro algoritmo configurado)
  - `phone_number` (único, validado por regex)
  - `role_id` (FK a `roles.id`)
  - `is_verified` (bool)
  - `is_active` (bool)
  - `failed_login_attempts`, `locked_until`, `last_login_at`
  - `verification_code`, `verification_expires_at`
  - `created_at`, `updated_at`

## Variables de entorno

El servicio utiliza variables de entorno leídas en `app/config.py`.

### Base de datos

- `DATABASE_URL` (obligatoria en producción/docker)
  - Formato general: `postgresql://USUARIO:PASSWORD@HOST:PUERTO/BASE`.
  - En Docker Compose (desde `docker-compose.yml` raíz):
    - `postgresql://user:password@db/auth_db`

Ejemplos:

- Ejecución **dentro de Docker** (tal como viene en el proyecto):
  ```env
  DATABASE_URL=postgresql://user:password@db:5432/auth_db
  ```
- Ejecución **local** conectando al contenedor de Postgres (puerto mapeado 5432):
  ```env
  DATABASE_URL=postgresql://user:password@localhost:5432/auth_db
  ```

Si `DATABASE_URL` no se define, `Settings` tiene un valor por defecto (solo útil para desarrollo local y puede no coincidir con Docker).

### JWT y seguridad

- `JWT_SECRET_KEY`: clave secreta para firmar los tokens.
- `JWT_ALGORITHM`: algoritmo JWT (por defecto `HS256`).
- `ACCESS_TOKEN_EXPIRE_MINUTES`: expiración del token en minutos.
- `COOKIE_SECURE`: `true`/`false` para marcar cookies como **secure** (solo HTTPS).

### Correo electrónico (verificación de usuario)

- `SMTP_HOST`: host SMTP (por defecto `smtp.gmail.com`).
- `SMTP_PORT`: puerto SMTP (por defecto `587`).
- `SMTP_USER`: usuario de la cuenta de correo.
- `SMTP_PASSWORD`: contraseña o app password.
- `SMTP_USE_TLS`: `true`/`false`.
- `EMAIL_FROM`: dirección `From` de los correos.
- `EMAIL_VERIFICATION_CODE_MINUTES`: minutos de validez del código de verificación.

## Endpoints principales

Prefijo base del router: `/auth` (ver `routers/auth.py`).

### Registro de usuario

- **POST** `/auth/register`
- Request body (`schemas.UserCreate`):
  ```json
  {
    "username": "juan",
    "email": "juan@example.com",
    "phone_number": "+1234567890",
    "password": "-Admin123-"
  }
  ```
- Validaciones importantes:
  - `phone_number`: regex `^[0-9+][0-9]{6,14}$`.
  - `password`: mínimo 8 caracteres, al menos una mayúscula, un número y un caracter especial.
- Respuesta: `schemas.UserRead` (usuario creado).
- También genera y envía un código de verificación por correo.

### Login

- **POST** `/auth/login`
- Request body (`schemas.UserLogin`):
  ```json
  {
    "email": "juan@example.com",
    "password": "-Admin123-"
  }
  ```
- Comportamiento:
  - Verifica credenciales.
  - Requiere que el correo esté verificado y el usuario esté activo.
  - Genera un JWT con rol del usuario.
  - Setea cookies:
    - `access_token` (HTTP-only, opcionalmente secure).
    - `user_role` (no HTTP-only) para front.

### Logout

- **POST** `/auth/logout`
- Borra las cookies `access_token` y `user_role`.

### Verificación de correo

- **POST** `/auth/send-verification-code`
  - Envía un nuevo código a un email registrado.
- **POST** `/auth/verify-email`
  - Verifica el código enviado y marca el usuario como verificado.

### Usuario actual

- **GET** `/auth/me`
  - Devuelve el usuario autenticado en base al token/cookie.

### Endpoint solo admin

- **GET** `/auth/admin-only`
  - Requiere rol `admin` (usa `require_role("admin")`).

## Usuario administrador por defecto

En `app/main.py`, durante el startup de la aplicación, se ejecuta `create_default_admin_user`:

- Si no hay usuarios en la tabla `users`, se crea:
  - `username`: `admin`
  - `email`: `admin@admin.com`
  - `phone_number`: `+10000000000`
  - `password`: `-Admin123-` (hash via `get_password_hash`)
  - `role`: `admin` (se crea el rol si no existe)
  - `is_verified = True`, `is_active = True`.

Este usuario permite acceder al panel administrativo desde el inicio.

## Ejecución con Docker Compose

Desde la raíz del proyecto `Sistema_Distribuido`:

```bash
# Construir imágenes
docker-compose build

# Levantar todos los servicios (incluyendo authservice)
docker-compose up -d
```

Servicios relevantes:

- `db` (PostgreSQL)
- `authservice` (este servicio, FastAPI en Python)
- `inventoryservice`, `productservice`, `frontend` (otros microservicios y UI)

El `docker-compose.yml` monta:

- `init-db.sh` y los archivos `schema.sql` para inicializar las bases:
  - `auth_db`
  - `inventory_db`
  - `products_db`

AuthService se expone internamente en el puerto `8000` del contenedor y se mapea (según `docker-compose.yml`) a un puerto del host.

## Ejecución local (sin Docker, solo authservice)

Requisitos:

- Python 3.12
- PostgreSQL corriendo y accesible.

Pasos:

1. Crear entorno virtual y activar (opcional pero recomendado).
2. Instalar dependencias:
   ```bash
   pip install -r requirements.txt
   ```
3. Exportar `DATABASE_URL` apuntando a tu Postgres local.
4. Ejecutar migraciones/esquema manualmente (usando `db/schema.sql`).
5. Lanzar el servidor:
   ```bash
   uvicorn app.main:app --reload
   ```

La API estará disponible en `http://localhost:8000`.

## Consideraciones de seguridad

- No usar en producción la `JWT_SECRET_KEY` por defecto; definir una clave fuerte por entorno.
- Habilitar `COOKIE_SECURE=true` cuando se use HTTPS.
- Configurar correctamente las credenciales SMTP en producción.
- Cambiar la contraseña del admin por defecto (`-Admin123-`) inmediatamente después del despliegue.

## Tests / Extensibilidad

- Nuevos endpoints deben reutilizar:
  - Esquemas Pydantic (`schemas.py`)
  - Dependencias de seguridad (`security.py`)
  - Sesión de BD (`get_db`).
- La validación de contraseña está centralizada en `schemas.py` para creaciones y actualizaciones.
