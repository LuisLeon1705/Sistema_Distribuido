import time
from fastapi import FastAPI
from sqlalchemy.exc import OperationalError
from sqlalchemy.sql import text

from .routers import auth
from .routers import users
from .database import SessionLocal
from . import models
from .security import get_password_hash

app = FastAPI(title="Auth Service")

@app.on_event("startup")
def ensure_admin_user():
    # Configuración de reintentos
    max_retries = 10
    retry_interval = 3  # segundos

    print(f"Iniciando verificación de Admin (Reintentos máximos: {max_retries})...")

    for attempt in range(max_retries):
        db = SessionLocal()
        try:
            # 0. 'Ping' a la base de datos para ver si está lista
            # Esto lanzará error si la DB aún no acepta conexiones
            db.execute(text("SELECT 1"))

            # --- TU LÓGICA ORIGINAL ---
            
            # 1. Asegurar que el rol admin existe
            admin_role = db.query(models.Role).filter(models.Role.name == "admin").first()
            if not admin_role:
                admin_role = models.Role(name="admin", description="Administrador del sistema")
                db.add(admin_role)
                db.commit()
                db.refresh(admin_role)

            # 2. Buscar si YA existe el usuario admin por email
            admin_user = db.query(models.User).filter(models.User.email == "admin@admin.com").first()
            
            # 3. Si NO existe, lo creamos
            if not admin_user:
                print("Creando usuario administrador por defecto...")
                admin_user = models.User(
                    username="admin",
                    email="admin@admin.com",
                    phone_number="+10000000000",
                    # Asegúrate que esta clave cumpla tus requisitos de Regex
                    password_hash=get_password_hash("-Admin123-"), 
                    role_id=admin_role.id,
                    is_verified=True,
                    is_active=True,
                )
                db.add(admin_user)
                db.commit()
                print("✅ Usuario admin creado exitosamente: admin@admin.com / -Admin123-")
            else:
                print("ℹ️ El usuario admin ya existe. No se realizaron cambios.")
            
            # Si llegamos aquí, todo salió bien. Rompemos el bucle.
            break 
            
        except OperationalError:
            # Capturamos ESPECÍFICAMENTE el error de conexión de la DB
            print(f"⚠️ La base de datos no está lista (Intento {attempt + 1}/{max_retries}). Esperando {retry_interval}s...")
            time.sleep(retry_interval)
            
        except Exception as e:
            # Cualquier otro error (código, sintaxis) lo mostramos y paramos
            print(f"❌ Error crítico no relacionado con conexión: {e}")
            break
            
        finally:
            db.close()

    else:
        # Esto se ejecuta si el bucle 'for' termina sin el 'break' (se acabaron los intentos)
        print("❌ Se agotaron los intentos de conexión a la base de datos.")

app.include_router(auth.router, prefix="/auth", tags=["auth"])
app.include_router(users.router, prefix="/users", tags=["users"])