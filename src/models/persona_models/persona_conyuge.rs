use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::persona_enum::RegimenConyugalPersona;

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct ConyugePersonaModelo {
    pub id_persona_conyuge: i32,
    pub id_persona: i32,
    pub fecha_residencia: NaiveDate,
    pub lugar_nacimiento: String,
    pub estado_nacimiento: i32,
    pub regimen_conyugal: RegimenConyugalPersona,
}
