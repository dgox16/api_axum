use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PrestamoGarantiasModelo {
    pub id_prestamo_garantias: i32,
    pub id_prestamo_relacionado: i32,
    pub porcentaje_reciprocidad: f32,
    pub garantia_fija: bool,
    pub garantia_liquida: f32,
    pub id_servicio_garantia_liquida: i32,
    pub id_captacion_garantia: i32,
    pub id_inversion_garantia: i32,
    pub monto_garantia_liquida: f32,
    pub garantia_prendaria: f32,
    pub garantia_hipotecaria: f32,
    pub garantia_avales: f32,
}
