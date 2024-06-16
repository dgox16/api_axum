use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct DetalleFichaModelo {
    pub id_detalle_ficha: i32,
    pub ficha: i32,
    pub servicio: Option<i32>,
    pub captacion: Option<i32>,
    pub prestamo: Option<i32>,
    pub inversion: Option<i32>,
    pub cargo: f32,
    pub abono: f32,
    pub operacion_fuente: i32,
    pub subficha: i32,
    pub observacion: String,
    pub proveedor: Option<i32>,
    pub procesado: i32,
}

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct DetalleFichaTemporalModelo {
    pub id_detalle_ficha_temporal: i32,
    pub persona: i32,
    pub servicio: Option<i32>,
    pub captacion: Option<i32>,
    pub prestamo: Option<i32>,
    pub inversion: Option<i32>,
    pub cargo: f32,
    pub abono: f32,
}
