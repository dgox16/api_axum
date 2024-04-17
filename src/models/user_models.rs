use chrono::prelude::*;
use serde::{Deserialize, Serialize};

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct RoleModel {
    pub id: i32,
    pub nombre: String,
}

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct UserModel {
    pub id: i32,
    pub usuario: String,
    pub email: String,
    pub contraseña: String,
    pub id_rol: i32,
    #[serde(rename = "created_at")]
    pub created_at: Option<DateTime<Utc>>,
    #[serde(rename = "updated_at")]
    pub updated_at: Option<DateTime<Utc>>,
}
