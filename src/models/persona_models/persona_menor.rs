use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct MenorPersonaModelo {
    pub id_persona_menor: i32,
    pub id_persona: i32,
    pub entre_calle: i32,
    pub y_calle: i32,
    pub fecha_residencia: NaiveDate,
    pub lugar_nacimiento: String,
    pub estado_nacimiento: i32,
    pub escolaridad: i32,
    pub autorizo_compartir_informacion_ifai: bool,
    pub autorizo_publicidad: bool,
}
