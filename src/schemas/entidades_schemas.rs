use serde::Deserialize;

use crate::models::entidades_models::{
    ClasificacionCuenta, FinalidadCuenta, GrupoCuenta, NaturalezaCuenta, OperacionProveedor,
    TipoProveedor,
};

#[derive(Deserialize)]
pub struct CrearSucursalSchema {
    pub nombre: String,
    pub encargado: i32,
    pub domicilio: i32,
}

#[derive(Deserialize)]
pub struct BuscarSucursalQuery {
    pub palabra: Option<String>,
    pub limite: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Deserialize)]
pub struct CrearBancoSchema {
    pub nombre: String,
}

#[derive(Deserialize)]
pub struct BuscarBancoQuery {
    pub nombre: Option<String>,
    pub limite: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Deserialize)]
pub struct CrearProveedorSchema {
    pub nombre: String,
    pub domicilio: i32,
    pub rfc: String,
    pub curp: String,
    pub telefono: String,
    pub tipo: Option<TipoProveedor>,
    pub operacion: Option<OperacionProveedor>,
    pub regimen: String,
    pub nombre_extranjero: Option<String>,
    pub pais_residencia: Option<i32>,
    pub pais_nacimiento: Option<i32>,
    pub banco: i32,
    pub cuenta_clabe: String,
}

#[derive(Deserialize)]
pub struct CrearCuentaSchema {
    pub cuenta: String,
    pub cuenta_siti: String,
    pub nombre: String,
    pub clasificacion: Option<ClasificacionCuenta>,
    pub grupo: Option<GrupoCuenta>,
    pub finalidad: Option<FinalidadCuenta>,
    pub naturaleza: Option<NaturalezaCuenta>,
    pub afectable: Option<bool>,
    pub padre: String,
    pub nivel: i32,
    pub en_balance: Option<bool>,
    pub en_catalogo_minimo: Option<bool>,
    pub nombre_balance: String,
    pub nombre_siti: String,
    pub cuenta_padre_siti: String,
    pub cuenta_agrupar: String,
    pub orden_siti: i32,
    pub subcuenta_siti: Option<bool>,
    pub prorrateo: Option<bool>,
}

#[derive(Deserialize)]
pub struct CrearEmpresaSchema {
    pub nombre: String,
    pub domicilio: i32,
    pub telefono: String,
    pub empleos_fijos: i32,
    pub frecuencia: i32,
}

#[derive(Deserialize)]
pub struct CrearFrecuenciaEmpresaSchema {
    pub nombre: String,
    pub dias: i32,
    pub meses: i32,
}
