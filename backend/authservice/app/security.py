from datetime import datetime, timedelta, timezone
from typing import Optional
import uuid

from fastapi import Depends, HTTPException, status, Request
from fastapi.security import OAuth2PasswordBearer
from jose import JWTError, jwt
from passlib.context import CryptContext
from sqlalchemy.orm import Session

from .config import settings
from .database import get_db
from . import models, schemas

pwd_context = CryptContext(schemes=["bcrypt_sha256"], deprecated="auto")

oauth2_scheme = OAuth2PasswordBearer(tokenUrl="/auth/login", auto_error=False)

def verify_password(plain_password: str, hashed_password: str) -> bool:
    return pwd_context.verify(plain_password, hashed_password)

def get_password_hash(password: str) -> str:
    return pwd_context.hash(password)

def create_access_token(data: dict, expires_delta: Optional[timedelta] = None) -> str:
    to_encode = data.copy()
    expire = datetime.now(timezone.utc) + (expires_delta or timedelta(minutes=settings.access_token_expire_minutes))
    to_encode.update({"exp": expire})
    encoded_jwt = jwt.encode(to_encode, settings.jwt_secret_key, algorithm=settings.jwt_algorithm)
    return encoded_jwt

def decode_access_token(token: str) -> schemas.TokenData:
    try:
        payload = jwt.decode(token, settings.jwt_secret_key, algorithms=[settings.jwt_algorithm])
        user_id = payload.get("sub")
        role: str = payload.get("role")
        if user_id is None or role is None:
            raise HTTPException(status_code=status.HTTP_401_UNAUTHORIZED, detail="Invalid token payload")
        return schemas.TokenData(user_id=str(user_id), role=role)
    except JWTError:
        raise HTTPException(status_code=status.HTTP_401_UNAUTHORIZED, detail="Could not validate token")

def get_token_from_request(request: Request, token: Optional[str] = Depends(oauth2_scheme)) -> str:
    if token:
        return token
    cookie_token = request.cookies.get(settings.access_token_cookie_name)
    if not cookie_token:
        raise HTTPException(status_code=status.HTTP_401_UNAUTHORIZED, detail="Not authenticated")
    return cookie_token

def get_current_user(
    token: str = Depends(get_token_from_request),
    db: Session = Depends(get_db),
) -> models.User:
    token_data = decode_access_token(token)
    try:
        user_uuid = uuid.UUID(token_data.user_id)
    except ValueError:
        raise HTTPException(status_code=status.HTTP_401_UNAUTHORIZED, detail="Invalid token payload")
    user = db.query(models.User).filter(models.User.id == user_uuid).first()
    # --- AGREGA ESTOS PRINTS ---
    if not user:
        print(f"❌ DEBUG: Usuario {user_uuid} no encontrado en DB")
        raise HTTPException(status_code=401, detail="User not found")
        
    if not user.is_active:
        print(f"❌ DEBUG: Usuario {user.email} está INACTIVO")
        raise HTTPException(status_code=403, detail="Inactive user") # Ojo si esto es 403
        
    if not user.is_verified:
         print(f"❌ DEBUG: Usuario {user.email} NO VERIFICADO")
         # Si tienes una linea aquí que lanza 403, esa es la culpable
    
    print(f"✅ DEBUG: Usuario {user.email} autenticado correctamente. Rol: {user.role.name if user.role else 'Sin Rol'}")
    # ---------------------------
    if not user or not user.is_active:
        raise HTTPException(status_code=status.HTTP_401_UNAUTHORIZED, detail="Inactive or not found user")
    return user

def require_role(required_role: str):
    def role_dependency(current_user: models.User = Depends(get_current_user)):
        if not current_user.role or current_user.role.name != required_role:
            raise HTTPException(status_code=status.HTTP_403_FORBIDDEN, detail="Insufficient permissions")
        return current_user

    return role_dependency