use serde::{Deserialize, Serialize};

use super::user_models::PropositoJWT;

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: i32,
    pub iat: usize,
    pub exp: usize,
    pub proposito: PropositoJWT,
}
