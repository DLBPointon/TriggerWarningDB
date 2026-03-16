use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use rocket::http::Status;
use rocket::request::{self, FromRequest, Request};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

#[derive(Clone)]
pub struct AuthConfig {
    pub jwt_secret: String,
    pub token_ttl_hours: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32, // user id
    pub admin: bool,
    pub exp: i64, // expiration as a UNIX timestamp (seconds)
}

impl AuthConfig {
    pub fn encode_token(
        &self,
        user_id: i32,
        admin: bool,
    ) -> Result<String, jsonwebtoken::errors::Error> {
        let exp = OffsetDateTime::now_utc() + Duration::hours(self.token_ttl_hours);
        let claims = Claims {
            sub: user_id,
            admin,
            exp: exp.unix_timestamp(),
        };
        let header = Header::new(Algorithm::HS256);
        encode(
            &header,
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )
    }

    pub fn decode_token(&self, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
        let mut validation = Validation::new(Algorithm::HS256);
        validation.validate_exp = true;
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.jwt_secret.as_bytes()),
            &validation,
        )?;
        Ok(token_data.claims)
    }
}

pub struct UserGuard {
    pub user_id: i32,
    pub admin: bool,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserGuard {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let Some(auth_header) = req.headers().get_one("Authorization") else {
            return request::Outcome::Error((Status::Unauthorized, ()));
        };

        let token = auth_header.strip_prefix("Bearer ").unwrap_or("");
        if token.is_empty() {
            return request::Outcome::Error((Status::Unauthorized, ()));
        }

        let Some(cfg) = req.rocket().state::<AuthConfig>() else {
            return request::Outcome::Error((Status::InternalServerError, ()));
        };

        match cfg.decode_token(token) {
            Ok(claims) => request::Outcome::Success(UserGuard {
                user_id: claims.sub,
                admin: claims.admin,
            }),
            Err(_) => request::Outcome::Error((Status::Unauthorized, ())),
        }
    }
}

pub struct AdminGuard {
    pub user_id: i32,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AdminGuard {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        match UserGuard::from_request(req).await {
            // If user == admin, then do the thing
            request::Outcome::Success(u) if u.admin => {
                request::Outcome::Success(AdminGuard { user_id: u.user_id })
            }

            // If user != admin, then it's successful but not enough permissions.
            request::Outcome::Success(_) => request::Outcome::Error((Status::Forbidden, ())),
            request::Outcome::Error(e) => request::Outcome::Error(e),
            request::Outcome::Forward(_) => request::Outcome::Error((Status::Unauthorized, ())),
        }
    }
}
