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
    max_retries = 10
    retry_interval = 3

    for attempt in range(max_retries):
        db = SessionLocal()
        try:
            db.execute(text("SELECT 1"))

            admin_role = db.query(models.Role).filter(models.Role.name == "admin").first()
            if not admin_role:
                admin_role = models.Role(name="admin", description="Administrador del sistema")
                db.add(admin_role)
                db.commit()
                db.refresh(admin_role)

            admin_user = db.query(models.User).filter(models.User.email == "admin@admin.com").first()
            
            if not admin_user:
                admin_user = models.User(
                    username="admin",
                    email="admin@admin.com",
                    phone_number="+10000000000",
                    password_hash=get_password_hash("-Admin123-"), 
                    role_id=admin_role.id,
                    is_verified=True,
                    is_active=True,
                )
                db.add(admin_user)
                db.commit()
            else:
                pass
            
            break 
            
        except OperationalError:
            time.sleep(retry_interval)
            
        except Exception as e:
            break
            
        finally:
            db.close()

    else:
        pass

app.include_router(auth.router, prefix="/auth", tags=["auth"])
app.include_router(users.router, prefix="/users", tags=["users"])