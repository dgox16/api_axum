use serde::Deserialize;

use crate::models::ubicacion_models::{
    ClasificacionCiudad, IndiceMarginacionBarrio, TipoCalle, TipoCiudad, TipoZonaBarrio,
};

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
pub struct CrearMunicipioSchema {
    pub nombre: String,
    pub estado: i32,
    pub factor_riesgo: i32,
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

#[derive(Deserialize)]
pub struct BuscarCiudadQuery {
    pub nombre: Option<String>,
    pub limite: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Deserialize)]
pub struct CrearBarrioSchema {
    pub ciudad: i32,
    pub nombre: String,
    pub cp: String,
    pub tipo_cp: i32,
    pub sindicatura: i32,
    pub tipo_zona: TipoZonaBarrio,
    pub numero_habitantes: i32,
    pub indice_marginacion: IndiceMarginacionBarrio,
    pub ponderacion_5c: f32,
    pub c_municipio: i32,
    pub unico_asentamiento: i32,
}

#[derive(Deserialize)]
pub struct BuscarBarrioQuery {
    pub nombre: Option<String>,
    pub limite: Option<i64>,
    pub offset: Option<i64>,
}
