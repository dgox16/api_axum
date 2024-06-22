use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PrestamoSeguroModelo {
    id_prestamo_seguro: i32,
    tiene_seguro: Option<TieneSeguroPrestamo>,
    aseguradora: Option<i32>,
    afectacion: Option<f64>,
    tasa_interes: Option<f64>,
    tasa_moratoria: Option<f64>,
    tasa_iva: Option<f64>,
    pagos_otras_deudas: Option<f64>,
    interes_otras_deudas: Option<f64>,
}

#[derive(Clone, Debug, PartialEq, PartialOrd, sqlx::Type, Deserialize, Serialize)]
#[sqlx(type_name = "tiene_seguro_prestamo", rename_all = "snake_case")]
pub enum TieneSeguroPrestamo {
    No,
    Si,
    NoAplica,
    Hipoteca,
    Avales,
}
