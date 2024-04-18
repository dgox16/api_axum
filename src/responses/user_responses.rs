use chrono::prelude::*;
use serde::Serialize;

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct UsuarioFormateado {
    pub id: String,
    pub usuario: String,
    pub email: String,
    pub rol: String,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
}

#[derive(Serialize, Debug)]
pub struct UsuarioDatos {
    pub usuario: UsuarioFormateado,
}

#[derive(Serialize, Debug)]
pub struct UsuarioRespuesta {
    pub status: String,
    pub data: UsuarioDatos,
}
