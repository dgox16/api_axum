use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct ManerasEnterarse {
    id_manera_enterarse: i32,
    nombre: String,
}
