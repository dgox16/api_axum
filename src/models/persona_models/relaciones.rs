use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct RelacionDePersonaModelo {
    pub id_relacion_de_persona: i32,
    pub id_persona: i32,
    pub persona_relacionada: i32,
    pub parentesco: i32,
}
