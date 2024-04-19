use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "tipo_calle", rename_all = "lowercase")]
pub enum TipoCalle {
    Calle,
    Avenida,
    Prolongacion,
    Cerrada,
    Privada,
    Calzada,
}

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct CalleModelo {
    pub id_calle: i64,
    pub nombre: String,
    pub tipo: TipoCalle,
}

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct DomicilioModel {
    pub id_domicilio: i32,
    pub cp: String,
    pub colonia: String,
    pub calle_id: i64,
    pub entre_calle_id: i64,
    pub y_calle_id: i64,
    pub numero_exterior: String,
    pub numero_interior: Option<String>,
    pub geolocalizacion: Option<String>,
}
