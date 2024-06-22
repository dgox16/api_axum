use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PrestamoMigradoModelo {
    id_prestamo_migrado: i32,
    solicitud: Option<String>,
    cuenta: Option<String>,
    nombre_servicio_migrado: Option<String>,
    persona: Option<i32>,
    sucursal: Option<i32>,
    grupo_prestamo: Option<i32>,
    servicio_subgrupo: Option<i32>,
    usuario: Option<i32>,
}
