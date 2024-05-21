use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "tipo_documento", rename_all = "lowercase")]
pub enum TipoDocumento {
    Personas,
    Captacion,
    Prestamos,
}

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct DocumentoModelo {
    pub id_documento: i32,
    pub nombre: String,
    pub tipo: Option<TipoDocumento>,
}

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct DocumentoDePersonaModelo {
    pub id_documento_de_persona: i32,
    pub id_persona: i32,
    pub documento: i32,
    pub numero_identificacion: String,
    pub es_identificacion: bool,
    pub vigencia: NaiveDate,
}
