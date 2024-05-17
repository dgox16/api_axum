use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct OcupacionModelo {
    pub id_ocupacion: i32,
    pub nombre: String,
    pub ponderacion_5c: f32,
}

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct ProfesionModelo {
    pub id_profesion: i32,
    pub profesion_migrado: i64,
    pub nombre: String,
    pub abreviatura: String,
}
