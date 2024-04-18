use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RegistroUsuarioSchema {
    pub usuario: String,
    pub email: String,
    pub contraseña: String,
}

#[derive(Debug, Deserialize)]
pub struct InicioSesionUsuarioSchema {
    pub usuario: String,
    pub contraseña: String,
}
