use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "sexo_persona", rename_all = "snake_case")]
pub enum SexoPersona {
    Masculino,
    Femenino,
    PersonaMoral,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "vivienda_persona", rename_all = "snake_case")]
pub enum ViviendaPersona {
    Propio,
    Rentada,
    Familiares,
    Prestada,
    Otros,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "estado_civil_persona", rename_all = "snake_case")]
pub enum EstadoCivilPersona {
    Soltero,
    Casado,
    UnionLibre,
    Viudo,
    Otros,
    NoAplica,
}

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
