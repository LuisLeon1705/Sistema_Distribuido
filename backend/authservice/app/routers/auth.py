from datetime import datetime, timezone

from fastapi import APIRouter, Depends, HTTPException, status, Response
from sqlalchemy.exc import IntegrityError
from sqlalchemy.orm import Session

from .. import models, schemas
from ..database import get_db
from ..security import get_password_hash, verify_password, create_access_token, get_current_user, require_role
from ..config import settings

router = APIRouter()

@router.post("/register", response_model=schemas.UserRead, status_code=status.HTTP_201_CREATED)
def register(user_in: schemas.UserCreate, db: Session = Depends(get_db)):
    role = db.query(models.Role).filter(models.Role.name == "customer").first()
    if not role:
        raise HTTPException(status_code=500, detail="Default role 'customer' not configured")

    user = models.User(
        username=user_in.username,
        email=user_in.email,
        phone_number=user_in.phone_number,
        password_hash=get_password_hash(user_in.password),
        role_id=role.id,
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


@router.post("/login", response_model=schemas.Token)
def login(response: Response, credentials: schemas.UserLogin, db: Session = Depends(get_db)):
    user = db.query(models.User).filter(models.User.email == credentials.email).first()
    if not user or not verify_password(credentials.password, user.password_hash):
        raise HTTPException(status_code=status.HTTP_401_UNAUTHORIZED, detail="Incorrect email or password")

    if not user.is_active:
        raise HTTPException(status_code=status.HTTP_403_FORBIDDEN, detail="User is inactive")

    user.last_login_at = datetime.now(timezone.utc)
    db.add(user)
    db.commit()

    role_name = user.role.name if user.role else "customer"

    access_token = create_access_token(data={"sub": user.id, "role": role_name})

    response.set_cookie(
        key=settings.access_token_cookie_name,
        value=access_token,
        httponly=True,
        secure=settings.cookie_secure,
        samesite="lax",
    )
    response.set_cookie(
        key=settings.role_cookie_name,
        value=role_name,
        httponly=False,
        secure=settings.cookie_secure,
        samesite="lax",
    )

    return schemas.Token(access_token=access_token)


@router.post("/logout")
def logout(response: Response):
    response.delete_cookie(
        key=settings.access_token_cookie_name,
        httponly=True,
        secure=settings.cookie_secure,
        samesite="lax",
    )
    response.delete_cookie(
        key=settings.role_cookie_name,
        httponly=False,
        secure=settings.cookie_secure,
        samesite="lax",
    )

    return {"detail": "Logged out"}


@router.get("/me", response_model=schemas.UserRead)
def read_current_user(current_user: models.User = Depends(get_current_user)):
    return current_user


@router.get("/admin-only", response_model=schemas.UserRead)
def admin_only(current_user: models.User = Depends(require_role("admin"))):
    return current_user