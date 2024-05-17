use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct OcupacionModelo {
    pub id_ocupacion: i32,
    pub nombre: String,
    pub ponderacion_5c: f32,
}
