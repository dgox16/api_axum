use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CrearDocumentoDePersonaSchema {
    pub documento: i32,
    pub numero_identificacion: String,
    pub es_identificacion: bool,
    pub vigencia: NaiveDate,
}
