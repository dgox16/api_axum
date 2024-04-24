use serde::Deserialize;

use crate::models::poliza_models::TipoPoliza;

#[derive(Deserialize)]
pub struct CrearPolizaEgresoSchema {
    pub beneficiario: String,
    pub banco: i32,
    pub cheque: String,
}

#[derive(Deserialize)]
pub struct CrearPolizaSchema {
    pub tipo: TipoPoliza,
    pub sucursal: i32,
    pub concepto: String,
    pub poliza_egreso: Option<CrearPolizaEgresoSchema>,
}
