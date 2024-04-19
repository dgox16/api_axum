use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "tipo_poliza", rename_all = "lowercase")]
pub enum TipoPoliza {
    Egreso,
    Ingreso,
    Diario,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "aplicacion_poliza", rename_all = "snake_case")]
pub enum AplicacionPoliza {
    Normal,
    Condonacion,
    ChequeOrden,
    CierreDiario,
    CierreMensual,
    CierreAnual,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "fuente_poliza", rename_all = "lowercase")]
pub enum FuentePoliza {
    Operacion,
    Activos,
    Nomina,
    Gastos,
    Pasiva,
    Traslados,
    Traspasos,
}

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct PolizaModelo {
    pub id_poliza: i32,
    pub tipo: TipoPoliza,
    pub numero: Option<i32>,
    pub sucursal: i32,
    pub fecha_poliza: Option<DateTime<Utc>>,
    pub fecha_registro_poliza: Option<DateTime<Utc>>,
    pub concepto: String,
    pub usuario_autoriza: Option<i32>,
    pub usuario_elabora: i32,
    pub aplicacion: Option<AplicacionPoliza>,
    pub fuente: Option<FuentePoliza>,
    pub automatico: bool,
}
