use serde::Deserialize;

#[derive(Deserialize)]
pub struct CrearRelacionDePersonaSchema {
    pub persona_relacionada: i32,
    pub parentesco: i32,
}
