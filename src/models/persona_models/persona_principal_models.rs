use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use super::persona_types::{EstadoCivilPersona, SexoPersona, ViviendaPersona};

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct PersonaModelo {
    pub id_persona: i32,
    pub nombre: String,
    pub apellido_paterno: String,
    pub apellido_materno: String,
    pub tipo: i32,
    pub sexo: SexoPersona,
    pub fecha_actualizacion: Option<DateTime<Utc>>,
    pub usuario_actualizo: i32,
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

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "grado_parentesco", rename_all = "snake_case")]
pub enum GradoParentesco {
    Primero,
    Segundo,
    Tercero,
    NoAplica,
}

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct ParentescoModelo {
    pub id_parentesco: i32,
    pub nombre: String,
    pub es_familiar: bool,
    pub grado: GradoParentesco,
    pub tipo_id: i32,
}
