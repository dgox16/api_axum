use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "estado_prestamo", rename_all = "snake_case")]
pub enum EstadoPrestamo {
    Solicitud,
    Aprobado,
    Entregado,
    Vigente,
    Pagado,
    Renovado,
    CarteraVencida,
    Castigado,
    Cancelada,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "estado_prestamo", rename_all = "snake_case")]
pub enum EtapaPrestamo {
    Solicitud,
    Investigacion,
    Analisis,
    Autorizacion,
    Dispersion,
    Instrumentacion,
    MesaControl,
    Entrega,
    Entregado,
    LineaActiva,
    AutorizacionLegal,
    Legal,
    ProcesoCastigo,
    Castigo,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "ministracion_prestamo", rename_all = "snake_case")]
pub enum MinistracionPrestamo {
    UnaMinistracion,
    MinistracionVariable,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "tipo_pagares_prestamo", rename_all = "snake_case")]
pub enum TipoPagaresPrestamo {
    CapitalIgual,
    PagareIgual,
    AlVencimiento,
}

#[derive(Debug, Serialize, Deserialize)]
struct PrestamoFechaModelo {
    id_prestamo_fecha: i32,
    fecha_solicitud: Option<NaiveDateTime>,
    fecha_autorizacion: Option<NaiveDate>,
    fecha_entrega: Option<NaiveDateTime>,
    fecha_vencimiento: Option<NaiveDateTime>,
    fecha_vencimiento_linea: Option<NaiveDate>,
    fecha_interes: Option<NaiveDate>,
    fecha_interes_moratorio: Option<NaiveDate>,
    fecha_primer_incumplimiento: Option<NaiveDate>,
    fecha_primer_pagare: Option<NaiveDate>,
    fecha_ultima_ministracion: Option<NaiveDate>,
    fecha_antes_interes: Option<NaiveDate>,
    fecha_antes_moratorio: Option<NaiveDate>,
    fecha_devolucion_garantia: Option<NaiveDate>,
    fecha_ultima_investigacion: Option<NaiveDate>,
    estado: Option<EstadoPrestamo>,
    etapa: Option<EtapaPrestamo>,
    ministracion: Option<MinistracionPrestamo>,
    tipo_de_pagares: Option<TipoPagaresPrestamo>,
    monto_abono: Option<f32>,
    fecha_ajustar_primer_pagare: Option<NaiveDate>,
}
