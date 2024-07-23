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

#[derive(Debug, Deserialize)]
pub struct RefrescarTokenSchema {
    pub refresh_token: String,
}

#[derive(Deserialize)]
pub struct BuscarUsuarioQuery {
    pub palabra: Option<String>,
    pub limite: Option<i64>,
    pub offset: Option<i64>,
}
