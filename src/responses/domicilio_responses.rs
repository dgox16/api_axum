use serde::Serialize;

#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct NewCalleResponse {
    pub id_calle: String,
    pub nombre: String,
    pub tipo: String,
}
