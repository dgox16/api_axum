use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct RegisterUserSchema {
    pub usuario: String,
    pub email: String,
    pub contraseña: String,
}
