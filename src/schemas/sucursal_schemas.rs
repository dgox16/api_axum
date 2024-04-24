use serde::Deserialize;

#[derive(Deserialize)]
pub struct NuevaSucursalSchema {
    pub nombre: String,
    pub encargado: i32,
    pub domicilio: i32,
}

#[derive(Deserialize)]
pub struct NuevoBancoSchema {
    pub nombre: String,
}
