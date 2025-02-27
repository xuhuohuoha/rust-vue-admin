//! JWT
//!

use axum::{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
    response::{IntoResponse, Response},
    Json, RequestPartsExt,
};
use axum_extra::{
    headers::{authorization::Bearer, Authorization},
    TypedHeader,
};
use chrono::{Duration, Local};
use jsonwebtoken::{
    decode, encode, errors::ErrorKind, DecodingKey, EncodingKey, Header, Validation,
};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{config::CONFIG, model::app_structs::UserInfoContext, serve::srv_online};

pub static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = &CONFIG.jwt.jwt_secret;
    Keys::new(secret.as_bytes())
});

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}
impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthPayload {
    /// 用户ID
    pub u_id: String,
    /// 用户账号
    pub u_code: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    /// 用户ID
    pub u_id: String,
    /// 用户账号
    pub u_code: String,
    /// Token
    pub token_id: String,
    /// 有效时间
    pub exp: i64,
}

// TODO 此处修改
// #[axum::async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;
    /// 将用户信息注入request
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let (_, token_v) = get_bear_token(parts).await?;
        // Decode the user data

        let token_data = match decode::<Claims>(&token_v, &KEYS.decoding, &Validation::default()) {
            Ok(token) => {
                let token_id = token.claims.token_id.clone();
                let (x, _) = srv_online::check_online(None, &token_id).await;
                if x {
                    token
                } else {
                    return Err(AuthError::CheckOutToken);
                }
            }
            Err(err) => match *err.kind() {
                ErrorKind::InvalidToken => {
                    return Err(AuthError::InvalidToken);
                }
                ErrorKind::ExpiredSignature => {
                    return Err(AuthError::MissingCredentials);
                }
                _ => {
                    return Err(AuthError::WrongCredentials);
                }
            },
        };
        let user = token_data.claims;
        parts.extensions.insert(UserInfoContext {
            u_id: user.u_id.clone(),
            token: user.token_id.clone(),
            u_code: user.u_code.clone(),
        });
        Ok(user)
    }
}

pub async fn get_bear_token(parts: &mut Parts) -> Result<(String, String), AuthError> {
    // Extract the token from the authorization header
    let TypedHeader(Authorization(bearer)) = parts
        .extract::<TypedHeader<Authorization<Bearer>>>()
        .await
        .map_err(|_| AuthError::InvalidToken)?;
    // Decode the user data
    let bearer_data = bearer.token();
    let cut = bearer_data.len() - scru128::new_string().len();
    Ok((
        bearer_data[cut..].to_string(),
        bearer_data[0..cut].to_string(),
    ))
}

pub async fn authorize(payload: AuthPayload, token_id: String) -> Result<AuthBody, AuthError> {
    if payload.u_id.is_empty() || payload.u_code.is_empty() {
        return Err(AuthError::MissingCredentials);
    }
    let iat = Local::now();
    let exp = iat + Duration::minutes(CONFIG.jwt.jwt_exp);
    let claims = Claims {
        u_id: payload.u_id.to_owned(),
        token_id: token_id.clone(),
        u_code: payload.u_code,
        exp: exp.timestamp(),
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding)
        .map_err(|_| AuthError::WrongCredentials)?;
    // Send the authorized token
    Ok(AuthBody::new(
        token,
        claims.exp,
        CONFIG.jwt.jwt_exp,
        token_id,
    ))
}

/// 授权错误类型
#[derive(Debug)]
pub enum AuthError {
    /// 凭据错误
    WrongCredentials,
    /// 凭据缺失
    MissingCredentials,
    /// 令牌创建
    TokenCreation,
    /// 令牌无效
    InvalidToken,
    /// 令牌验证
    CheckOutToken,
}
impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Token 凭据错误"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Token 凭据缺失"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token 令牌创建错误"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Token 令牌无效"),
            AuthError::CheckOutToken => (StatusCode::UNAUTHORIZED, "该账户已经退出"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

/// 授权消息体
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthBody {
    /// Token
    token: String,
    /// Token 类型
    token_type: String,
    /// 过期时间
    pub exp: i64,
    exp_in: i64,
}
impl AuthBody {
    fn new(access_token: String, exp: i64, exp_in: i64, token_id: String) -> Self {
        Self {
            token: access_token + &token_id,
            token_type: "Bearer".to_string(),
            exp,
            exp_in,
        }
    }
}
