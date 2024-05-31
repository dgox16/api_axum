use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "quien_es_tutor_de_persona", rename_all = "snake_case")]
pub enum QuienEsTutor {
    Madre,
    Padre,
    TutorLegal,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "tipo_tutor_de_persona", rename_all = "lowercase")]
pub enum TipoTutor {
    Principal,
    Segundo,
    Tercero,
}

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct TutorDePersonaModelo {
    pub id_tutor_de_persona: i32,
    pub id_persona: i32,
    pub tutor: i32,
    pub tutor_migrado: String,
    pub quien_es_tutor: QuienEsTutor,
    pub documento_legal: String,
    pub documento: i32,
    pub tipo: TipoTutor,
}
