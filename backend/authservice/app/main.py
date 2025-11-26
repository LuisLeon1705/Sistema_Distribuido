from fastapi import FastAPI

from .routers import auth
from .routers import users

app = FastAPI(title="Auth Service")

app.include_router(auth.router, prefix="/auth", tags=["auth"])
app.include_router(users.router, prefix="/users", tags=["users"])
