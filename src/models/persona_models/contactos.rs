use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "tipo_contacto", rename_all = "snake_case")]
pub enum TipoContacto {
    TelefonoFijo,
    TelefonoMovil,
    CorreoElectronico,
    Facebook,
    Whatsapp,
}

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct ContactoDePersonaModelo {
    pub id_contacto_de_persona: i32,
    pub id_persona: i32,
    pub contacto: String,
    pub tipo_contacto: TipoContacto,
    pub es_principal: bool,
}
