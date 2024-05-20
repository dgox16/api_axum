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

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "clasificacion_persona", rename_all = "lowercase")]
pub enum ClasificacionPersona {
    Fisica,
    Moral,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "periodo_persona", rename_all = "lowercase")]
pub enum PeriodoPersona {
    Catorcenal,
    Semanal,
    Quincenal,
    Mensual,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "regimen_conyugal_persona", rename_all = "lowercase")]
pub enum RegimenConyugalPersona {
    NoAplica,
    BienesMancomunados,
    BienesSeparados,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "propietario_persona", rename_all = "lowercase")]
pub enum EsPropietarioPersona {
    Si,
    No,
    Autoempleo,
}
