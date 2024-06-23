use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PrestamoAutorizacionModelo {
    id_prestamo_autorizacion: i32,
    id_autoriza: i32,
    autorizo_gerente: bool,
    comision: f32,
}
