from fastapi import FastAPI

from .routers import auth
from .routers import users
from .database import SessionLocal
from . import models
from .security import get_password_hash

app = FastAPI(title="Auth Service")

@app.on_event("startup")
def create_default_admin_user():
    db = SessionLocal()
    try:
        existing_users = db.query(models.User).count()
        if existing_users > 0:
            return

        admin_role = db.query(models.Role).filter(models.Role.name == "admin").first()
        if not admin_role:
            admin_role = models.Role(name="admin", description="Administrador del sistema")
            db.add(admin_role)
            db.commit()
            db.refresh(admin_role)

        admin_user = models.User(
            username="admin",
            email="admin@admin.com",
            phone_number="+10000000000",
            password_hash=get_password_hash("admin"),
            role_id=admin_role.id,
            is_verified=True,
            is_active=True,
        )

        db.add(admin_user)
        db.commit()
    finally:
        db.close()

app.include_router(auth.router, prefix="/auth", tags=["auth"])
app.include_router(users.router, prefix="/users", tags=["users"])
