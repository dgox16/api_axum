use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PrestamoEstructuraDatosModelo {
    pub id_prestamo_estructura_datos: i32,
    pub monto_original_antes_de_reestructura: f32,
    pub adelantar: bool,
    pub anticipado: bool,
}
