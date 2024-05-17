use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct OcupacionPldModelo {
    pub id_ocupacion_pld: i32,
    pub nombre: String,
    pub cnbv: String,
    pub descripcion: String,
    pub factor_riesgo: i32,
}
