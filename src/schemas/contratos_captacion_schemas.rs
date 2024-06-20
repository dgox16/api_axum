use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::models::contrato_captacion_models::TipoContratoCaptacion;

#[derive(Deserialize)]
pub struct ListarContratosCaptacionQuery {
    pub persona: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum TipoSaldoContratoCaptacion {
    Cargos,
    Abonos,
    Todos,
}

#[derive(Deserialize)]
pub struct ObtenerSaldoContratosCaptacionQuery {
    pub persona: i32,
    pub tipo: TipoSaldoContratoCaptacion,
}

#[derive(Deserialize, Serialize)]
pub enum CargoAbonoEnum {
    Cargo,
    Abono,
}

#[derive(Deserialize)]
pub struct AbonoCargoContratoCaptacionSchema {
    pub persona: i32,
    pub captacion: i32,
    pub abono: Option<f32>,
    pub cargo: Option<f32>,
    pub abono_cargo: CargoAbonoEnum,
}

#[derive(Deserialize)]
pub enum TipoPagoFicha {
    Efectivo,
    Cheques,
    Transferencia,
    Tarjeta,
}

#[derive(Deserialize)]
pub struct DepositoRetiroContratoCaptacionSchema {
    pub folio_ficha: String,
    pub persona: i32,
    pub sucursal: i32,
    pub poliza: i32,
    pub operacion_fuente: i32,
    pub tipo_pago: Option<TipoPagoFicha>,
    pub referencia: Option<String>,
    pub factura: i32,
    pub instrumento: Option<String>,
    pub subficha: i32,
    pub observacion: Option<String>,
    pub procesado: i32,
}

#[derive(Deserialize)]
pub struct CrearContratoCaptacionSchema {
    pub id_persona: i32,
    pub cuenta: String,
    pub servicio: i32,
    pub fecha: NaiveDate,
    pub no_dejar_retirar_antes_de: NaiveDate,
    pub fecha_contrato: NaiveDate,
    pub monto_autorizado: f32,
    pub numero_sesion: String,
    pub tipo_sesion: String,
    pub nombre: String,
    pub tipo: TipoContratoCaptacion,
    pub contrato_migrado: i64,
    pub fecha_desbloqueo: NaiveDateTime,
    pub usuario_desbloqueo: i32,
    pub fecha_libera_garantia: NaiveDateTime,
    pub usuario_libera_garantia: i32,
    pub monto_libera_garantia: f32,
    pub tercero_autorizado: bool,
    pub tasa_pactada: f32,
    pub bloqueada: bool,
    pub monto_bloqueado_adicional: f32,
    pub fecha_interes: NaiveDate,
    pub autoriza_cancelacion: bool,
    pub usuario_autoriza_cancelacion: i32,
}
