use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PrestamoAnteriorModelo {
    pub id_prestamo_anterior: i32,
    pub incumplimiento_credito_anterior: Option<i32>,
    pub periodo_pactado_original: Option<i32>,
}
