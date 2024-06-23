use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "grado_riesgo_prestamo", rename_all = "snake_case")]
pub enum GradoRiesgoPrestamo {
    Bajo,
    Medio,
    Alto,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrestamoAnalisisCapacidadModelo {
    pub id_prestamo_analisis_capacidad: i32,
    pub firma_a_ruego: Option<String>,
    pub cartera_pasiva: Option<i32>,
    pub capacidad_pago: Option<f32>,
    pub capacidad_pago_ajustada: Option<f32>,
    pub independencia_financiera: Option<f32>,
    pub solvencia_general: Option<f32>,
    pub indice_de_liquidez: Option<f32>,
    pub rentabilidad: Option<f32>,
    pub grado_de_riesgo: Option<GradoRiesgoPrestamo>,
}
