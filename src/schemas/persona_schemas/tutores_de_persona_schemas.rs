use serde::Deserialize;

use crate::models::persona_models::tutores_de_persona_models::{QuienEsTutor, TipoTutor};

#[derive(Deserialize)]
pub struct CrearTutorDePersonaSchema {
    pub tutor: i32,
    pub tutor_migrado: String,
    pub quien_es_tutor: QuienEsTutor,
    pub documento_legal: String,
    pub documento: i32,
    pub tipo: TipoTutor,
}
