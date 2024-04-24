use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct SucursalModel {
    pub id_sucursal: i32,
    pub nombre: String,
    pub encargado: i32,
    pub domicilio: i32,
}

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct BancoModel {
    pub id_banco: i32,
    pub nombre: String,
}
