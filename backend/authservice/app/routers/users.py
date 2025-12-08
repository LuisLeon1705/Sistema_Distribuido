from typing import List, Optional
from uuid import UUID

from fastapi import APIRouter, Depends, HTTPException, status, Query
from sqlalchemy.exc import IntegrityError
from sqlalchemy.orm import Session

from .. import models, schemas
from ..database import get_db
from ..security import get_current_user, require_role, require_roles, get_password_hash

router = APIRouter()

# --- RUTAS ESTÁTICAS (Deben ir primero) ---

@router.get("/me", response_model=schemas.UserRead)
def read_user_me(current_user: models.User = Depends(get_current_user)):
    """Obtiene el perfil del usuario actual"""
    return current_user

@router.patch("/me", response_model=schemas.UserRead)
def update_me(
    user_update: schemas.UserSelfUpdate,
    db: Session = Depends(get_db),
    current_user: models.User = Depends(get_current_user),
):
    if user_update.username is not None:
        current_user.username = user_update.username
    if user_update.email is not None:
        current_user.email = user_update.email
    if user_update.phone_number is not None:
        current_user.phone_number = user_update.phone_number
    if user_update.password is not None:
        current_user.password_hash = get_password_hash(user_update.password)

    db.add(current_user)
    try:
        db.commit()
    except IntegrityError as e:
        db.rollback()
        msg = str(e.orig).lower()
        if "users_email_key" in msg or "email" in msg:
            raise HTTPException(status_code=400, detail="Email already registered")
        if "users_phone_number_key" in msg or "phone" in msg:
            raise HTTPException(status_code=400, detail="Phone number already registered")
        if "users_username_key" in msg or "username" in msg:
            raise HTTPException(status_code=400, detail="Username already registered")
        raise HTTPException(status_code=400, detail="Could not update user")

    db.refresh(current_user)
    return current_user


# --- RUTAS DE ADMINISTRACIÓN ---

@router.post("/", response_model=schemas.UserRead, status_code=status.HTTP_201_CREATED)
def create_user_admin(
    user_in: schemas.AdminUserCreate,
    db: Session = Depends(get_db),
    _: models.User = Depends(require_role("admin")),
):
    role_name = user_in.role_name or "customer"
    role = db.query(models.Role).filter(models.Role.name == role_name).first()
    if not role:
        raise HTTPException(status_code=400, detail="Role not found")

    user = models.User(
        username=user_in.username,
        email=user_in.email,
        phone_number=user_in.phone_number,
        password_hash=get_password_hash(user_in.password),
        role_id=role.id,
        is_active=user_in.is_active if user_in.is_active is not None else True,
        is_verified=user_in.is_verified if user_in.is_verified is not None else False,
    )

    db.add(user)
    try:
        db.commit()
    except IntegrityError as e:
        db.rollback()
        msg = str(e.orig).lower()
        if "users_email_key" in msg or "email" in msg:
            raise HTTPException(status_code=400, detail="Email already registered")
        if "users_phone_number_key" in msg or "phone" in msg:
            raise HTTPException(status_code=400, detail="Phone number already registered")
        if "users_username_key" in msg or "username" in msg:
            raise HTTPException(status_code=400, detail="Username already registered")
        raise HTTPException(status_code=400, detail="Could not create user")

    db.refresh(user)
    return user


@router.get("/", response_model=List[schemas.UserRead])
def list_users(
    db: Session = Depends(get_db),
    _: models.User = Depends(require_roles("admin", "inventory")),
    skip: int = Query(0, ge=0),
    limit: int = Query(50, ge=1, le=100),
    username: Optional[str] = None,
    email: Optional[str] = None,
    role: Optional[str] = None,
    is_active: Optional[bool] = None,
):
    query = db.query(models.User)

    if username is not None:
        query = query.filter(models.User.username.ilike(f"%{username}%"))
    if email is not None:
        query = query.filter(models.User.email.ilike(f"%{email}%"))
    if role is not None:
        query = query.join(models.Role).filter(models.Role.name == role)
    if is_active is not None:
        query = query.filter(models.User.is_active == is_active)

    users = query.offset(skip).limit(limit).all()
    return users


# --- RUTAS DINÁMICAS (Con :uuid para evitar conflictos) ---

@router.get("/{user_id:uuid}", response_model=schemas.UserRead)
def get_user_by_id(
    user_id: UUID,
    db: Session = Depends(get_db),
    _: models.User = Depends(require_roles("admin", "inventory")),
):
    user = db.query(models.User).filter(models.User.id == user_id).first()
    if not user:
        raise HTTPException(status_code=status.HTTP_404_NOT_FOUND, detail="User not found")
    return user


@router.patch("/{user_id:uuid}", response_model=schemas.UserRead)
def update_user_admin(
    user_id: UUID,
    user_update: schemas.UserUpdate,
    db: Session = Depends(get_db),
    _: models.User = Depends(require_role("admin")),
):
    user = db.query(models.User).filter(models.User.id == user_id).first()
    if not user:
        raise HTTPException(status_code=status.HTTP_404_NOT_FOUND, detail="User not found")

    if user_update.username is not None:
        user.username = user_update.username
    if user_update.email is not None:
        user.email = user_update.email
    if user_update.phone_number is not None:
        user.phone_number = user_update.phone_number
    if user_update.password is not None:
        user.password_hash = get_password_hash(user_update.password)
    if user_update.is_active is not None:
        user.is_active = user_update.is_active
    if user_update.is_verified is not None:
        user.is_verified = user_update.is_verified
    if user_update.role_name is not None:
        role = db.query(models.Role).filter(models.Role.name == user_update.role_name).first()
        if not role:
            raise HTTPException(status_code=400, detail="Role not found")
        user.role_id = role.id

    db.add(user)
    try:
        db.commit()
    except IntegrityError as e:
        db.rollback()
        msg = str(e.orig).lower()
        if "users_email_key" in msg or "email" in msg:
            raise HTTPException(status_code=400, detail="Email already registered")
        if "users_phone_number_key" in msg or "phone" in msg:
            raise HTTPException(status_code=400, detail="Phone number already registered")
        if "users_username_key" in msg or "username" in msg:
            raise HTTPException(status_code=400, detail="Username already registered")
        raise HTTPException(status_code=400, detail="Could not update user")

    db.refresh(user)
    return user


@router.delete("/{user_id:uuid}", status_code=status.HTTP_204_NO_CONTENT)
def delete_user(
    user_id: UUID,
    db: Session = Depends(get_db),
    _: models.User = Depends(require_role("admin")),
):
    user = db.query(models.User).filter(models.User.id == user_id).first()
    if not user:
        raise HTTPException(status_code=status.HTTP_404_NOT_FOUND, detail="User not found")

    db.delete(user)
    db.commit()
    return None
