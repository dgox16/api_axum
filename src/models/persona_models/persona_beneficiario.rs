use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct TutorPersonaModelo {
    pub id_persona_beneficiario: i32,
    pub id_persona: i32,
    pub entre_calle: i32,
    pub y_calle: i32,
}
