use serde::Deserialize;

use crate::models::poliza_models::TipoPoliza;

#[derive(Deserialize)]
pub struct CrearPolizaSchema {
    pub tipo: TipoPoliza,
    pub sucursal: i32,
    pub concepto: String,
}
