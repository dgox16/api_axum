use serde::Deserialize;

use crate::models::poliza_models::{AplicacionPoliza, FuentePoliza, IvaDetallePoliza, TipoPoliza};

#[derive(Deserialize)]
pub struct CrearPolizaEgresoSchema {
    pub beneficiario: String,
    pub banco: i32,
    pub cheque: String,
}

#[derive(Deserialize)]
pub struct CrearDetallePolizaSchema {
    pub cuenta: i32,
    pub sucursal: i32,
    pub cargo: f32,
    pub abono: f32,
    pub proveedor: i32,
    pub concepto: String,
    pub iva: Option<IvaDetallePoliza>,
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
    pub detalles_poliza: Option<Vec<CrearDetallePolizaSchema>>,
}
