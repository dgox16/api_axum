use chrono::{NaiveDate, NaiveDateTime};
use serde::Serialize;

use crate::models::contrato_captacion_models::TipoContratoCaptacion;

#[derive(Serialize)]
pub struct ListarContratoCaptacionRespuesta {
    pub id_contrato_captacion: i32,
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
    pub usuario: i32,
    pub fecha_interes: NaiveDate,
    pub autoriza_cancelacion: bool,
    pub usuario_autoriza_cancelacion: i32,
    pub saldo: f32,
}
