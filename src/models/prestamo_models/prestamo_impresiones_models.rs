use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PrestamoImpresionesModelo {
    pub id_prestamo_impresiones: i32,
    pub solicitud_impresa: bool,
}
