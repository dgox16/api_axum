use chrono::NaiveDate;
use serde::Deserialize;

use crate::models::persona_models::persona_types::{ClasificacionPersona, PeriodoPersona};

#[derive(Deserialize)]
pub struct CrearPersonaAspiranteSchema {
    pub clasificacion: ClasificacionPersona,
    pub ocupacion_pld: i32,
    pub especificacion_pld: i32,
    pub actividad_pld: i32,
    pub antiguedad: i32,
    pub periodo: PeriodoPersona,
    pub frecuencia_captacion: i32,
    pub operacion_maxima_captacion: f32,
    pub perfil_frecuencia_prestamo: i32,
    pub operacion_maxima_prestamo: f32,
    pub ingresos_mensual: f32,
    pub egresos_mensual: f32,
    pub grado_afectacion: i32,
    pub afectacion: f32,
    pub entre_calle: i64,
    pub y_calle: i64,
    pub fecha_residencia: NaiveDate,
    pub lugar_nacimiento: String,
    pub estado_nacimiento: i32,
    pub profesion: i32,
    pub escolaridad: i32,
    pub autorizo_compartir_informacion_ifai: bool,
    pub autorizo_publicidad: bool,
}
