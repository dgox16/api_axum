use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateNewCalleSchema {
    pub nombre: String,
    pub tipo: String,
}
