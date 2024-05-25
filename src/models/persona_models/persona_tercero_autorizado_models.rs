use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::persona_types::{ClasificacionPersona, RegimenConyugalPersona};

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct TerceroAutorizadoPersonaModelo {
    pub id_persona_tercero_autorizado: i32,
    pub id_persona: i32,
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
