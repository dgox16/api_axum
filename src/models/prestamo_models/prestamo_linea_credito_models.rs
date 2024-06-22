use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct PrestamoLineaCreditoModelo {
    id_prestamo_linea_credito: i32,
    linea_de_credito: Option<f32>,
    ultima_ministracion: Option<f32>,
    monto: Option<f32>,
    numero_abonos: Option<i32>,
    plazo: Option<i32>,
    servicio: Option<i32>,
    fuente_financiamiento: Option<i32>,
}
