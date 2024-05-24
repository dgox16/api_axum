use chrono::NaiveDate;
use serde::Deserialize;

use crate::models::persona_models::persona_types::RegimenConyugalPersona;

#[derive(Deserialize)]
pub struct CrearPersonaConyugeSchema {
    pub fecha_residencia: NaiveDate,
    pub lugar_nacimiento: String,
    pub estado_nacimiento: i32,
    pub regimen_conyugal: RegimenConyugalPersona,
}
