use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CrearPersonaMenorSchema {
    pub entre_calle: i64,
    pub y_calle: i64,
    pub fecha_residencia: NaiveDate,
    pub lugar_nacimiento: String,
    pub estado_nacimiento: i32,
    pub escolaridad: i32,
    pub autorizo_compartir_informacion_ifai: bool,
    pub autorizo_publicidad: bool,
}
