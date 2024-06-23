use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PrestamoUltimoCalculoModelo {
    pub id_prestamo_ultimo_calculo: i32,
    pub dias_mora: i32,
    pub interes_moratorio: f32,
    pub interes_normal: f32,
    pub fecha_pago: NaiveDate,
    pub saldo_capital: f32,
    pub observaciones: String,
}
