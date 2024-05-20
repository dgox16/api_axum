use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::persona_enum::{ClasificacionPersona, RegimenConyugalPersona};

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct SucursalPersonaModelo {
    pub id_persona_sucursal: i32,
    pub id_persona: i32,
    pub clasificacion: ClasificacionPersona,
    pub entre_calle: i32,
    pub y_calle: i32,
    pub fecha_residencia: NaiveDate,
    pub lugar_nacimiento: String,
    pub estado_nacimiento: i32,
    pub regimen_conyugal: RegimenConyugalPersona,
    pub profesion: i32,
    pub escolaridad: i32,
    pub autorizo_compartir_informacion_ifai: bool,
    pub autorizo_publicidad: bool,
}
