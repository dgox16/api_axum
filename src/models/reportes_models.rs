use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct BalanzaComprobacionModelo {
    pub cuenta: String,
    pub nombre: String,
    pub total_cargo: f32,
    pub total_abono: f32,
    pub total: f32,
}
