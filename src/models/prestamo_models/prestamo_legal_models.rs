use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PrestamoLegalModelo {
    pub id_prestamo_legal: i32,
    pub esta_en_litigio: bool,
    pub emproblemado: String,
}
