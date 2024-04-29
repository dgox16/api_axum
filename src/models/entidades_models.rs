use serde::{Deserialize, Serialize};

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct SucursalModelo {
    pub id_sucursal: i32,
    pub nombre: String,
    pub encargado: i32,
    pub domicilio: i32,
}

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct BancoModelo {
    pub id_banco: i32,
    pub nombre: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "clasificacion_cuenta", rename_all = "lowercase")]
pub enum ClasificacionCuenta {
    Capitulo,
    Subcapitulo,
    Mayor,
    Menor,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "grupo_cuenta", rename_all = "snake_case")]
pub enum GrupoCuenta {
    Activo,
    Pasivo,
    Capital,
    ResultadoAcreedor,
    ResultadoDeudor,
    Orden,
    Puente,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "finalidad_cuenta", rename_all = "lowercase")]
pub enum FinalidadCuenta {
    Caja,
    Banco,
    Otros,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "naturaleza_cuenta", rename_all = "lowercase")]
pub enum NaturalezaCuenta {
    Deudora,
    Acreedora,
}

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct CuentaModelo {
    pub id_cuenta: i32,
    pub cuenta: String,
    pub cuenta_siti: String,
    pub nombre: String,
    pub clasificacion: ClasificacionCuenta,
    pub grupo: GrupoCuenta,
    pub finalidad: FinalidadCuenta,
    pub naturaleza: NaturalezaCuenta,
    pub afectable: bool,
    pub padre: String,
    pub nivel: i32,
    pub en_balance: bool,
    pub en_catalogo_minimo: bool,
    pub nombre_balance: String,
    pub nombre_siti: String,
    pub cuenta_padre_siti: String,
    pub cuenta_agrupar: String,
    pub orden_siti: i32,
    pub subcuenta_siti: bool,
    pub prorrateo: bool,
}
