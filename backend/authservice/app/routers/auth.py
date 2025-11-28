from datetime import datetime, timezone, timedelta
import secrets
import string
from email.message import EmailMessage
import smtplib

from fastapi import APIRouter, Depends, HTTPException, status, Response
from sqlalchemy.exc import IntegrityError
from sqlalchemy.orm import Session

from .. import models, schemas
from ..database import get_db
from ..security import get_password_hash, verify_password, create_access_token, get_current_user, require_role
from ..config import settings

router = APIRouter()

VERIFICATION_CODE_LENGTH = 6


def _generate_verification_code() -> str:
    return "".join(secrets.choice(string.digits) for _ in range(VERIFICATION_CODE_LENGTH))


def _send_verification_email(to_email: str, code: str) -> None:
    msg = EmailMessage()
    msg["Subject"] = "Código de verificación de correo"
    msg["From"] = settings.email_from
    msg["To"] = to_email
    msg.set_content(
        f"Tu código de verificación es: {code}.\n"
        f"Es válido por {settings.email_verification_code_minutes} minutos."
    )

    with smtplib.SMTP(settings.smtp_host, settings.smtp_port) as server:
        if settings.smtp_use_tls:
            server.starttls()
        if settings.smtp_user and settings.smtp_password:
            server.login(settings.smtp_user, settings.smtp_password)
        server.send_message(msg)


def _create_and_send_verification_code(user: models.User, db: Session) -> None:
    code = _generate_verification_code()
    expires_at = datetime.now(timezone.utc) + timedelta(
        minutes=settings.email_verification_code_minutes
    )

    user.verification_code = code
    user.verification_expires_at = expires_at

    db.add(user)
    db.commit()
    db.refresh(user)

    _send_verification_email(user.email, code)

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

    try:
        _create_and_send_verification_code(user, db)
    except Exception:
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail="No se pudo enviar el correo de verificación. Inténtalo nuevamente más tarde.",
        )

    return user


@router.post("/login", response_model=schemas.Token)
def login(response: Response, credentials: schemas.UserLogin, db: Session = Depends(get_db)):
    user = db.query(models.User).filter(models.User.email == credentials.email).first()
    if not user or not verify_password(credentials.password, user.password_hash):
        raise HTTPException(status_code=status.HTTP_401_UNAUTHORIZED, detail="Incorrect email or password")

    if not user.is_verified:
        raise HTTPException(status_code=status.HTTP_403_FORBIDDEN, detail="Email not verified")

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


@router.post("/send-verification-code")
def send_verification_code(
    payload: schemas.EmailVerificationRequest,
    db: Session = Depends(get_db),
):
    user = db.query(models.User).filter(models.User.email == payload.email).first()
    if not user:
        raise HTTPException(status_code=status.HTTP_404_NOT_FOUND, detail="User not found")

    if not user.is_active:
        raise HTTPException(status_code=status.HTTP_403_FORBIDDEN, detail="User is inactive")

    if user.is_verified:
        return {"detail": "Email already verified"}

    try:
        _create_and_send_verification_code(user, db)
    except Exception:
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail="No se pudo enviar el correo de verificación. Inténtalo nuevamente más tarde.",
        )

    return {"detail": "Verification code sent"}


@router.post("/verify-email")
def verify_email(
    payload: schemas.EmailVerificationConfirm,
    db: Session = Depends(get_db),
):
    user = db.query(models.User).filter(models.User.email == payload.email).first()
    if not user:
        raise HTTPException(status_code=status.HTTP_404_NOT_FOUND, detail="User not found")

    if not user.is_active:
        raise HTTPException(status_code=status.HTTP_403_FORBIDDEN, detail="User is inactive")

    if user.is_verified:
        return {"detail": "Email already verified"}

    if not user.verification_code or not user.verification_expires_at:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="No verification code has been requested",
        )

    now = datetime.now(timezone.utc)
    if now > user.verification_expires_at:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="Verification code has expired",
        )

    if payload.code != user.verification_code:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="Invalid verification code",
        )

    user.is_verified = True
    user.verification_code = None
    user.verification_expires_at = None

    db.add(user)
    db.commit()
    db.refresh(user)

    return {"detail": "Email verified successfully"}


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