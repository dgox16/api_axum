use chrono::NaiveDate;
use serde::Deserialize;

use crate::models::persona_models::persona_types::{ClasificacionPersona, RegimenConyugalPersona};

#[derive(Deserialize)]
pub struct CrearPersonaTutorSchema {
    pub clasificacion: ClasificacionPersona,
    pub entre_calle: i64,
    pub y_calle: i64,
    pub fecha_residencia: NaiveDate,
    pub lugar_nacimiento: String,
    pub estado_nacimiento: i32,
    pub regimen_conyugal: RegimenConyugalPersona,
    pub profesion: i32,
    pub escolaridad: i32,
    pub autorizo_compartir_informacion_ifai: bool,
    pub autorizo_publicidad: bool,
}
