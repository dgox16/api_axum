use serde::Deserialize;

use crate::models::ubicacion_models::{ClasificacionCiudad, TipoCalle, TipoCiudad};

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
pub struct BuscarPaisQuery {
    pub nombre: Option<String>,
}

#[derive(Deserialize)]
pub struct BuscarEstadoQuery {
    pub nombre: Option<String>,
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

#[derive(Deserialize)]
pub struct CrearCiudadSchema {
    pub clave_localidad: i32,
    pub estado: i32,
    pub municipio: i32,
    pub nombre: String,
    pub tipo: TipoCiudad,
    pub clasificacion: ClasificacionCiudad,
    pub numero_habitantes: i32,
    pub orden: i32,
    pub cp: String,
}
