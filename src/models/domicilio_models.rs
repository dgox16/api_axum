use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "tipo_calle", rename_all = "UPPERCASE")]
pub enum TipoCalle {
    CA,
    AV,
    PR,
    CE,
    PV,
    CZ,
}

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct CalleModel {
    pub id_calle: i64,
    pub nombre: String,
    pub tipo: TipoCalle,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DomicilioModel {
    pub id_domicilio: i32,
    pub cp: String,
    pub colonia: String,
    pub calle_id: i64,
    pub entre_calle_id: i64,
    pub y_calle_id: i64,
    pub numero_exterior: String,
    pub numero_interior: String,
    pub geolocalizacion: String,
}