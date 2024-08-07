use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct ObtenerBalanzaComprobacionQuery {
    pub fecha_inicial: NaiveDate,
    pub fecha_final: NaiveDate,
}
