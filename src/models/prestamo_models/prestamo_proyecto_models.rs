use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct PrestamoProyectoModelo {
    id_prestamo_proyecto: i32,
    monto_del_proyecto: Option<f32>,
}
