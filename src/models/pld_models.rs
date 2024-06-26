use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct OcupacionPldModelo {
    pub id_ocupacion_pld: i32,
    pub nombre: String,
    pub cnbv: String,
    pub descripcion: String,
    pub factor_riesgo: i32,
}

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct ActividadPldModelo {
    pub id_actividad_pld: i32,
    pub clave: String,
    pub subsector: i32,
    pub nombre: String,
    pub factor_riesgo: f32,
    pub se_va_utilizar: bool,
    pub maximo_ahorro: f32,
    pub maximo_prestamo: f32,
    pub ponderacion_5c: i32,
}

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct EspecificacionPldModelo {
    pub id_especificacion_pld: i32,
    pub ocupacion: i32,
    pub nombre_ocupacion: String,
    pub especificacion: String,
    pub actividad_economica: i32,
}

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct LavadoAntiguedadModelo {
    pub id_lavado_antiguedad: i32,
    pub nombre: String,
    pub antiguedad: i32,
    pub factor_riesgo: f32,
}
