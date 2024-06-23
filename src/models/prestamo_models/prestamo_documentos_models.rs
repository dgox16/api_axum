use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "clasificacion_prestamo", rename_all = "snake_case")]
pub enum ClasificacionPrestamo {
    Nuevo,
    Renovado,
    Estructurado,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "estrato_prestamo", rename_all = "snake_case")]
pub enum EstratoPrestamo {
    NoAplica,
    E1,
    E2,
    E3,
    E4,
    E5,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "subclasificacion_prestamo", rename_all = "snake_case")]
pub enum SubclasificacionPrestamo {
    Normal,
    MicroCredito,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrestamoDocumentosModelo {
    pub id_prestamo_documentos: i32,
    pub compromiso_ahorro: Option<f32>,
    pub folio: Option<i32>,
    pub contrato: Option<i32>,
    pub pagare: Option<i32>,
    pub opinion_analista: Option<String>,
    pub clasificacion: Option<ClasificacionPrestamo>,
    pub estrato: Option<EstratoPrestamo>,
    pub subclasificacion: Option<SubclasificacionPrestamo>,
    pub id_prestamo_migrado: Option<String>,
    pub reestructurado: Option<bool>,
    pub pagos_sostenidos: Option<i32>,
}
