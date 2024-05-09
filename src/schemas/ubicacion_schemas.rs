use serde::Deserialize;

use crate::models::ubicacion_models::TipoCalle;

#[derive(Deserialize)]
pub struct CrearCalleSchema {
    pub nombre: String,
    pub tipo: TipoCalle,
}

#[derive(Deserialize)]
pub struct BuscarCalleQuery {
    pub palabra: Option<String>,
    pub limite: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Deserialize)]
pub struct CrearDomicilioSchema {
    pub cp: String,
    pub colonia: String,
    pub calle_id: i64,
    pub entre_calle_id: i64,
    pub y_calle_id: i64,
    pub numero_exterior: String,
    pub numero_interior: Option<String>,
    pub geolocalizacion: Option<String>,
}
