use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct OcupacionModelo {
    pub id_ocupacion: i32,
    pub nombre: String,
    pub ponderacion_5c: f32,
}

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct EscolaridadModelo {
    pub id_escolaridad: i32,
    pub nombre: String,
}
