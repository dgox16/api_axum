use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegisterUserSchema {
    pub usuario: String,
    pub email: String,
    pub contraseña: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUserSchema {
    pub email: String,
    pub contraseña: String,
}
