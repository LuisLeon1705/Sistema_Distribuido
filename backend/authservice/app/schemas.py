from datetime import datetime
from typing import Optional

from pydantic import BaseModel, EmailStr, field_validator
import re

PHONE_REGEX = re.compile(r"^[0-9+][0-9]{6,14}$")

class UserBase(BaseModel):
    username: str
    email: EmailStr
    phone_number: str

    @field_validator("phone_number")
    @classmethod
    def validate_phone(cls, v: str) -> str:
        if not PHONE_REGEX.match(v):
            raise ValueError("Invalid phone number format")
        return v

class UserCreate(UserBase):
    password: str

class AdminUserCreate(UserBase):
    password: str
    role_name: Optional[str] = None
    is_active: Optional[bool] = True
    is_verified: Optional[bool] = False

class UserUpdate(BaseModel):
    username: Optional[str] = None
    email: Optional[EmailStr] = None
    phone_number: Optional[str] = None
    password: Optional[str] = None
    is_active: Optional[bool] = None
    is_verified: Optional[bool] = None
    role_name: Optional[str] = None

    @field_validator("phone_number")
    @classmethod
    def validate_phone(cls, v: Optional[str]) -> Optional[str]:
        if v is not None and not PHONE_REGEX.match(v):
            raise ValueError("Invalid phone number format")
        return v

class UserSelfUpdate(BaseModel):
    username: Optional[str] = None
    email: Optional[EmailStr] = None
    phone_number: Optional[str] = None
    password: Optional[str] = None

    @field_validator("phone_number")
    @classmethod
    def validate_phone(cls, v: Optional[str]) -> Optional[str]:
        if v is not None and not PHONE_REGEX.match(v):
            raise ValueError("Invalid phone number format")
        return v

class UserLogin(BaseModel):
    email: EmailStr
    password: str

class RoleRead(BaseModel):
    id: int
    name: str

    class Config:
        from_attributes = True

class UserRead(BaseModel):
    id: int
    username: str
    email: EmailStr
    phone_number: str
    is_verified: bool
    is_active: bool
    role: RoleRead
    created_at: Optional[datetime]

    class Config:
        from_attributes = True

class Token(BaseModel):
    access_token: str
    token_type: str = "bearer"

class TokenData(BaseModel):
    user_id: int
    role: str