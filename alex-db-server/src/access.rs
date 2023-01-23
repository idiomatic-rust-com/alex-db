use crate::error::AppError;
use alex_db_lib::db::Db;
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use std::{str::FromStr, sync::Arc};
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct Access {
    api_key_exists: bool,
    restricted_access: bool,
    x_auth_token: Option<Uuid>,
}

impl Access {
    pub fn granted(&self) -> bool {
        if self.restricted_access {
            match self.x_auth_token {
                None => false,
                Some(_x_auth_token) => self.api_key_exists,
            }
        } else {
            true
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Access
where
    Arc<Db>: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let db = Arc::from_ref(state);

        let token_header = parts.headers.get("X-Auth-Token");

        let x_auth_token = match token_header {
            None => None,
            Some(token_header) => {
                let res = Uuid::from_str(token_header.to_str()?)?;
                Some(res)
            }
        };

        let api_key_exists = match x_auth_token {
            None => false,
            Some(api_key) => db.api_key_exists(api_key)?,
        };

        let access = Access {
            api_key_exists,
            restricted_access: db.config.enable_security_api_keys,
            x_auth_token,
        };
        Ok(access)
    }
}
