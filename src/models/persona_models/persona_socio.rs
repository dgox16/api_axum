use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::persona_enum::{
    ClasificacionPersona, EsPropietarioPersona, PeriodoPersona, RegimenConyugalPersona,
};

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct ManerasEnterarse {
    id_manera_enterarse: i32,
    nombre: String,
}

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct SocioPersonaModelo {
    pub id_persona_socio: i32,
    pub id_persona: i32,
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
    pub proveedor_recursos: i32,
    pub parentesco: i32,
    pub persona_recomendo: i32,
    pub manera_enterarse: i32,
    pub lengua: i32,
    pub empresa: i32,
    pub puesto: i32,
    pub fecha_trabajo: NaiveDate,
    pub ingreso_ordinario: f32,
    pub otros_ingresos: f32,
    pub es_propietario: EsPropietarioPersona,
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
