use serde::Deserialize;

use crate::models::poliza_models::{AplicacionPoliza, FuentePoliza, TipoPoliza};

#[derive(Deserialize)]
pub struct CrearPolizaEgresoSchema {
    pub beneficiario: String,
    pub banco: i32,
    pub cheque: String,
}

#[derive(Deserialize)]
pub struct CrearPolizaSchema {
    pub tipo: TipoPoliza,
    pub numero: Option<i32>,
    pub sucursal: i32,
    pub concepto: String,
    pub aplicacion: Option<AplicacionPoliza>,
    pub fuente: Option<FuentePoliza>,
    pub poliza_egreso: Option<CrearPolizaEgresoSchema>,
}
