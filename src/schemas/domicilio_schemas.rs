use serde::Deserialize;

#[derive(Deserialize)]
pub struct CrearNuevaCalleSchema {
    pub nombre: String,
    pub tipo: String,
}

#[derive(Deserialize)]
pub struct BuscarCalleQuery {
    pub palabra: Option<String>,
    pub limite: Option<i64>,
}
