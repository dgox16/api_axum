use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct EscolaridadModelo {
    pub id_escolaridad: i32,
    pub nombre: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "tipo_lengua", rename_all = "lowercase")]
pub enum TipoLengua {
    Oficial,
    Indigena,
    Extranjera,
}

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct LenguasModelo {
    pub id_lengua: i32,
    pub nombre: String,
    pub tipo: TipoLengua,
}
