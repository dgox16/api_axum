use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct BalanzaComprobacionModelo {
    pub cuenta: String,
    pub nombre: String,
    pub deudora_anterior: f32,
    pub acreedora_anterior: f32,
    pub cargo: f32,
    pub abono: f32,
    pub saldo_deudor: f32,
    pub saldo_acreedor: f32,
}
