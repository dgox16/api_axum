use serde::Deserialize;

use crate::models::persona_models::contactos_de_persona_models::TipoContacto;

#[derive(Deserialize)]
pub struct CrearContactoDePersonaSchema {
    pub contacto: String,
    pub tipo: TipoContacto,
    pub es_principal: bool,
}
