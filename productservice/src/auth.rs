use axum::{
    async_trait,
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode},
    middleware::Next,
    response::Response,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};
use crate::{state::AppState, errors::ApiError};

// In a real app, load this from environment variables!
const JWT_SECRET: &str = "your-super-secret-key";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // Subject (user id)
    pub role: String,
    pub exp: usize,  // Expiration time
}

#[derive(Debug, Clone)]
pub struct AuthenticatedUser {
    pub user_id: i32,
    pub role: String,
}

#[async_trait]
impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let bearer_token = TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, _state)
            .await
            .map_err(|_| ApiError::new(StatusCode::UNAUTHORIZED, "Missing or invalid Authorization header"))?;
        
        let token_data = decode::<Claims>(
            bearer_token.token(),
            &DecodingKey::from_secret(JWT_SECRET.as_ref()),
            &Validation::default(),
        )
        .map_err(|_| ApiError::new(StatusCode::UNAUTHORIZED, "Invalid token"))?;

        let user_id = token_data.claims.sub.parse::<i32>()
            .map_err(|_| ApiError::new(StatusCode::INTERNAL_SERVER_ERROR, "Invalid user ID in token"))?;

        Ok(AuthenticatedUser {
            user_id,
            role: token_data.claims.role,
        })
    }
}

pub fn is_admin(user: &AuthenticatedUser) -> Result<(), ApiError> {
    if user.role != "admin" {
        Err(ApiError::new(StatusCode::FORBIDDEN, "This action requires admin privileges"))
    } else {
        Ok(())
    }
}
