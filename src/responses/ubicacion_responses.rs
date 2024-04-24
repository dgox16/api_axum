use serde::Serialize;

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct CalleRespuesta {
    pub id_calle: String,
    pub nombre: String,
    pub tipo: String,
}
