from pydantic import BaseModel
import os

class Settings(BaseModel):
    database_url: str = os.getenv("DATABASE_URL", "postgresql://authuser:authpassword@db:5432/authdb")
    jwt_secret_key: str = os.getenv("JWT_SECRET_KEY", "auth-secret-key-123")
    jwt_algorithm: str = os.getenv("JWT_ALGORITHM", "HS256")
    access_token_expire_minutes: int = int(os.getenv("ACCESS_TOKEN_EXPIRE_MINUTES", "30"))
    cookie_secure: bool = os.getenv("COOKIE_SECURE", "False").lower() == "true"
    access_token_cookie_name: str = "access_token"
    role_cookie_name: str = "user_role"

settings = Settings()