use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct FichaModelo {
    pub id_ficha: i32,
    pub folio: String,
    pub fecha: NaiveDateTime,
    pub persona: i32,
    pub usuario: i32,
    pub sucursal: i32,
    pub poliza: i32,
    pub operacion_fuente: i32,
    pub efectivo: f32,
    pub cheques: f32,
    pub transferencia: f32,
    pub tarjeta: f32,
    pub cancelada: bool,
    pub referencia: String,
    pub factura: i32,
    pub pagada: bool,
    pub instrumento: String,
}
