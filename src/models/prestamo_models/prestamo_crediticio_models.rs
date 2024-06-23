use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PrestamoCrediticioModelo {
    pub id_prestamo_crediticio: i32,
    pub fecha_buro: Option<NaiveDate>,
    pub folio_buro: Option<String>,
    pub calificacion_buro: Option<i32>,
    pub bc_score: Option<i32>,
    pub indice_capacidad_crediticia: Option<i32>,
}
