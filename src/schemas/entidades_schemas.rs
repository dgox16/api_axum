use serde::Deserialize;

#[derive(Deserialize)]
pub struct CrearSucursalSchema {
    pub nombre: String,
    pub encargado: i32,
    pub domicilio: i32,
}

#[derive(Deserialize)]
pub struct CrearBancoSchema {
    pub nombre: String,
}
