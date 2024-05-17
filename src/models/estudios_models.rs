use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct EscolaridadModelo {
    pub id_escolaridad: i32,
    pub nombre: String,
}
