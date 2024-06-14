use serde::Deserialize;

#[derive(Deserialize)]
pub struct CrearPuestoTrabajoSchema {
    pub nombre: String,
}
