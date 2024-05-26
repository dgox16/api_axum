use serde::Deserialize;

#[derive(Deserialize)]
pub struct CrearPersonaBeneficiarioSchema {
    pub entre_calle: i64,
    pub y_calle: i64,
}
