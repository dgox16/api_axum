use chrono::NaiveDate;
use serde::Deserialize;

use crate::models::persona_models::{EstadoCivilPersona, SexoPersona, ViviendaPersona};

#[derive(Deserialize)]
pub struct CrearPersonaSchema {
    pub nombre: String,
    pub apellido_paterno: String,
    pub apellido_materno: String,
    pub tipo: i32,
    pub sexo: SexoPersona,
    pub cp: String,
    pub barrio: i32,
    pub ciudad: i32,
    pub calle: i64,
    pub numero_exterior: String,
    pub numero_interior: Option<String>,
    pub vivienda: ViviendaPersona,
    pub geolocalizacion: String,
    pub observaciones_geolocalizacion: String,
    pub fecha_nacimiento: NaiveDate,
    pub pais_nacimiento: i32,
    pub estado_civil: EstadoCivilPersona,
    pub persona_conyuge: Option<i32>,
    pub es_embargo_precautorio: bool,
    pub bloqueado_autoridad: bool,
    pub tercero_autorizado: i32,
}
