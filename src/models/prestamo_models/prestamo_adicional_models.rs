use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PrestamoAdicionalModelo {
    pub id_prestamo_adicional: i32,
    pub modificado: i32,
    pub modalidad: String,
    pub tasa_subsidiada: f32,
    pub meses_de_gracia: i32,
}
