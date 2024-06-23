use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PrestamoEntregaModelo {
    pub id_prestamo_entrega: i32,
    pub poliza: i32,
    pub ficha: i32,
    pub cat: f32,
}
