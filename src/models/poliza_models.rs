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
    pub numero: i32,
    pub sucursal: i32,
    pub fecha_poliza: Option<DateTime<Utc>>,
    pub fecha_registro_poliza: Option<DateTime<Utc>>,
    pub concepto: String,
    pub usuario_autoriza: i32,
    pub usuario_elabora: i32,
    pub aplicacion: AplicacionPoliza,
    pub fuente: FuentePoliza,
    pub automatico: Option<bool>,
}

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct PolizaEgresoModelo {
    pub id_poliza_egreso: i32,
    pub poliza: i32,
    pub beneficiario: String,
    pub banco: i32,
    pub cheque: String,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "iva_detalle_poliza", rename_all = "snake_case")]
pub enum IvaDetallePoliza {
    NoAplica,
    Iva0,
    Iva8,
    Iva11,
    Iva16,
    Excento,
    Retenido,
}

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct DetallePolizaModelo {
    pub id_detalle_poliza: i32,
    pub poliza: i32,
    pub cuenta: i32,
    pub sucursal: i32,
    pub cargo: f32,
    pub abono: f32,
    pub proveedor: i32,
    pub concepto: String,
    pub iva: IvaDetallePoliza,
}

#[derive(Debug, sqlx::FromRow, Deserialize, Serialize)]
pub struct DetallePolizaFormateadoModelo {
    pub id_detalle_poliza: i32,
    pub poliza: i32,
    pub cuenta: String,
    pub sucursal: i32,
    pub cargo: f32,
    pub abono: f32,
    pub proveedor: i32,
    pub concepto: String,
    pub iva: IvaDetallePoliza,
}
