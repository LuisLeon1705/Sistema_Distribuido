from pydantic import BaseModel
import os
from dotenv import load_dotenv

load_dotenv()

class Settings(BaseModel):
    database_url: str = os.getenv("DATABASE_URL", "postgresql://authuser:authpassword@db:5432/authdb")
    jwt_secret_key: str = os.getenv("JWT_SECRET_KEY", "5da4065109ac40d49a724461fcde8c0f5f233275b72eed2229a9af2ac2300155")
    jwt_algorithm: str = os.getenv("JWT_ALGORITHM", "HS256")
    access_token_expire_minutes: int = int(os.getenv("ACCESS_TOKEN_EXPIRE_MINUTES", "120"))
    cookie_secure: bool = False
    access_token_cookie_name: str = "access_token"
    role_cookie_name: str = "user_role"
    
    smtp_host: str = os.getenv("SMTP_HOST", "smtp.gmail.com")
    smtp_port: int = int(os.getenv("SMTP_PORT", "587"))
    smtp_user: str = os.getenv("SMTP_USER", "")
    smtp_password: str = os.getenv("SMTP_PASSWORD", "")
    smtp_use_tls: bool = os.getenv("SMTP_USE_TLS", "true").lower() == "true"
    email_from: str = os.getenv("EMAIL_FROM", "no-reply@example.com")
    email_verification_code_minutes: int = int(os.getenv("EMAIL_VERIFICATION_CODE_MINUTES", "3"))

settings = Settings()
